//! HTTP server for analytics API

use crate::collector::EventCollector;
use crate::config::Config;
use crate::error::Result;
use crate::events::{EventBatch, EventEnvelope};
use crate::privacy::PrivacyFilter;
use crate::processor::EventProcessor;
use crate::storage::PostgresStorage;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct AnalyticsServer {
    config: Config,
}

impl AnalyticsServer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<()> {
        // Initialize storage
        let storage = Arc::new(
            PostgresStorage::new(&self.config.database.url).await?,
        );
        storage.init_schema().await?;

        // Create event processing pipeline
        let (tx, rx) = mpsc::unbounded_channel();

        let privacy_filter = Arc::new(PrivacyFilter::new(self.config.privacy.clone()));
        let collector = Arc::new(EventCollector::new(tx, privacy_filter));

        // Start event processor
        let processor = EventProcessor::new(
            rx,
            storage.clone(),
            self.config.storage.batch_size,
        );

        tokio::spawn(async move {
            if let Err(e) = processor.run().await {
                tracing::error!("Event processor error: {}", e);
            }
        });

        // Build router
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/api/v1/collect", post(collect_event))
            .route("/api/v1/collect/batch", post(collect_batch))
            .route("/api/v1/metrics", get(get_metrics))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(AppState { collector });

        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        info!("Starting server on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    collector: Arc<EventCollector>,
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn collect_event(
    State(state): State<AppState>,
    Json(envelope): Json<EventEnvelope>,
) -> impl IntoResponse {
    match state.collector.collect(envelope).await {
        Ok(_) => (StatusCode::ACCEPTED, "Event collected".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e),
        ),
    }
}

async fn collect_batch(
    State(state): State<AppState>,
    Json(batch): Json<EventBatch>,
) -> impl IntoResponse {
    match state.collector.collect_batch(batch).await {
        Ok(_) => (StatusCode::ACCEPTED, "Batch collected".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e),
        ),
    }
}

async fn get_metrics(State(state): State<AppState>) -> impl IntoResponse {
    let metrics = state.collector.metrics();
    Json(serde_json::json!({
        "events_collected": metrics.events_collected(),
        "batches_collected": metrics.batches_collected(),
        "errors": metrics.errors(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}

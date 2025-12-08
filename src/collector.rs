//! Event collector - High-performance event ingestion

use crate::error::{Error, Result};
use crate::events::{EventBatch, EventEnvelope};
use crate::privacy::PrivacyFilter;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};

/// Event collector handles incoming events
pub struct EventCollector {
    sender: mpsc::UnboundedSender<EventEnvelope>,
    privacy_filter: Arc<PrivacyFilter>,
    metrics: Arc<CollectorMetrics>,
}

impl EventCollector {
    /// Create a new event collector
    pub fn new(
        sender: mpsc::UnboundedSender<EventEnvelope>,
        privacy_filter: Arc<PrivacyFilter>,
    ) -> Self {
        Self {
            sender,
            privacy_filter,
            metrics: Arc::new(CollectorMetrics::default()),
        }
    }

    /// Collect a single event
    pub async fn collect(&self, mut envelope: EventEnvelope) -> Result<()> {
        // Apply privacy filters
        envelope = self.privacy_filter.apply(envelope).await?;

        // Validate event
        self.validate_event(&envelope)?;

        // Send to processing pipeline
        self.sender.send(envelope).map_err(|e| {
            self.metrics.increment_errors();
            Error::Unknown(format!("Failed to send event: {}", e))
        })?;

        self.metrics.increment_collected();
        debug!("Event collected successfully");

        Ok(())
    }

    /// Collect a batch of events
    pub async fn collect_batch(&self, batch: EventBatch) -> Result<()> {
        let batch_size = batch.size();
        for envelope in batch.events {
            self.collect(envelope).await?;
        }

        self.metrics.increment_batches();
        info!("Batch of {} events collected", batch_size);

        Ok(())
    }

    /// Validate event envelope
    fn validate_event(&self, envelope: &EventEnvelope) -> Result<()> {
        if envelope.measurement_id.is_empty() {
            return Err(Error::InvalidEvent(
                "Measurement ID is required".to_string(),
            ));
        }

        // Additional validation logic here
        Ok(())
    }

    /// Get collector metrics
    pub fn metrics(&self) -> Arc<CollectorMetrics> {
        Arc::clone(&self.metrics)
    }
}

/// Collector metrics
#[derive(Default)]
pub struct CollectorMetrics {
    events_collected: std::sync::atomic::AtomicU64,
    batches_collected: std::sync::atomic::AtomicU64,
    errors: std::sync::atomic::AtomicU64,
}

impl CollectorMetrics {
    fn increment_collected(&self) {
        self.events_collected
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn increment_batches(&self) {
        self.batches_collected
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn increment_errors(&self) {
        self.errors
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn events_collected(&self) -> u64 {
        self.events_collected
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn batches_collected(&self) -> u64 {
        self.batches_collected
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn errors(&self) -> u64 {
        self.errors.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::events::{Event, EventParams};

    #[tokio::test]
    async fn test_collect_event() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let config = Config::default();
        let privacy_filter = Arc::new(PrivacyFilter::new(config.privacy));
        let collector = EventCollector::new(tx, privacy_filter);

        let event = Event::PageView {
            page_title: "Test".to_string(),
            page_location: "https://test.com".to_string(),
            page_referrer: None,
            user_id: None,
            params: EventParams::default(),
        };

        let envelope = EventEnvelope::new("TEST123".to_string(), event);
        collector.collect(envelope).await.unwrap();

        assert!(rx.recv().await.is_some());
        assert_eq!(collector.metrics().events_collected(), 1);
    }
}

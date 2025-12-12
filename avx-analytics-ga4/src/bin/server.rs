//! Analytics server binary

use avx_analytics_ga4::config::Config;
use avx_analytics_ga4::server::AnalyticsServer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Load configuration
    let config = Config::from_env().unwrap_or_else(|_| {
        tracing::warn!("Failed to load config from env, using defaults");
        Config::default()
    });

    // Start server
    let server = AnalyticsServer::new(config);
    server.run().await?;

    Ok(())
}

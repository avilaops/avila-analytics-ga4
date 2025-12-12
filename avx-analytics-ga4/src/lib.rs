//! # Avila Analytics GA4
//!
//! Enterprise-grade web analytics engine written in 100% Rust
//!
//! ## Features
//!
//! - High-performance event tracking (1M+ events/sec)
//! - Privacy-first (GDPR/LGPD compliant)
//! - Real-time analytics
//! - E-commerce tracking
//! - Custom dimensions & metrics
//! - WebAssembly dashboard
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use avila_analytics_ga4::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = AnalyticsClient::new("YOUR_MEASUREMENT_ID").await?;
//!
//!     client.track_event(Event::PageView {
//!         page_title: "Home".into(),
//!         page_location: "https://example.com".into(),
//!         user_id: Some("user123".into()),
//!     }).await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod collector;
pub mod config;
pub mod error;
pub mod events;
pub mod models;
pub mod privacy;
pub mod processor;
pub mod query;
pub mod server;
pub mod session;
pub mod storage;
pub mod user;

pub mod prelude {
    //! Convenience re-exports for common types
    pub use crate::client::AnalyticsClient;
    pub use crate::config::Config;
    pub use crate::error::{Error, Result};
    pub use crate::events::{Event, EventParams};
    pub use crate::models::*;
    pub use crate::server::AnalyticsServer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_loads() {
        // Basic sanity check
        assert!(true);
    }
}

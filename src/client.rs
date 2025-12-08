//! Analytics client for tracking events

use crate::error::Result;
use crate::events::{Event, EventEnvelope};
use reqwest::Client;
use std::sync::Arc;

/// Analytics client for sending events
pub struct AnalyticsClient {
    measurement_id: String,
    endpoint: String,
    client: Arc<Client>,
}

impl AnalyticsClient {
    /// Create a new analytics client
    pub async fn new(measurement_id: impl Into<String>) -> Result<Self> {
        Self::with_endpoint(measurement_id, "http://localhost:8080")
    }

    /// Create client with custom endpoint
    pub fn with_endpoint(
        measurement_id: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self {
            measurement_id: measurement_id.into(),
            endpoint: endpoint.into(),
            client: Arc::new(Client::new()),
        })
    }

    /// Track a single event
    pub async fn track_event(&self, event: Event) -> Result<()> {
        let envelope = EventEnvelope::new(self.measurement_id.clone(), event);

        let url = format!("{}/api/v1/collect", self.endpoint);

        self.client
            .post(&url)
            .json(&envelope)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Track multiple events in batch
    pub async fn track_batch(&self, events: Vec<Event>) -> Result<()> {
        let envelopes: Vec<EventEnvelope> = events
            .into_iter()
            .map(|event| EventEnvelope::new(self.measurement_id.clone(), event))
            .collect();

        let url = format!("{}/api/v1/collect/batch", self.endpoint);

        self.client
            .post(&url)
            .json(&envelopes)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Set measurement ID
    pub fn set_measurement_id(&mut self, measurement_id: impl Into<String>) {
        self.measurement_id = measurement_id.into();
    }
}

/// Builder for analytics client
pub struct AnalyticsClientBuilder {
    measurement_id: Option<String>,
    endpoint: Option<String>,
    timeout_secs: Option<u64>,
}

impl AnalyticsClientBuilder {
    pub fn new() -> Self {
        Self {
            measurement_id: None,
            endpoint: None,
            timeout_secs: None,
        }
    }

    pub fn measurement_id(mut self, id: impl Into<String>) -> Self {
        self.measurement_id = Some(id.into());
        self
    }

    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    pub fn build(self) -> Result<AnalyticsClient> {
        let measurement_id = self
            .measurement_id
            .ok_or_else(|| crate::error::Error::Config("measurement_id required".into()))?;

        let endpoint = self
            .endpoint
            .unwrap_or_else(|| "http://localhost:8080".to_string());

        AnalyticsClient::with_endpoint(measurement_id, endpoint)
    }
}

impl Default for AnalyticsClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventParams;

    #[tokio::test]
    async fn test_client_creation() {
        let client = AnalyticsClient::with_endpoint("TEST123", "http://localhost:8080");
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_builder() {
        let client = AnalyticsClientBuilder::new()
            .measurement_id("TEST123")
            .endpoint("http://localhost:8080")
            .build();

        assert!(client.is_ok());
    }
}

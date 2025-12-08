//! Event processor - Transforms and enriches events

use crate::error::Result;
use crate::events::EventEnvelope;
use crate::storage::StorageEngine;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Event processor handles event transformation and storage
pub struct EventProcessor {
    receiver: mpsc::UnboundedReceiver<EventEnvelope>,
    storage: Arc<dyn StorageEngine>,
    batch_size: usize,
    buffer: Vec<EventEnvelope>,
}

impl EventProcessor {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<EventEnvelope>,
        storage: Arc<dyn StorageEngine>,
        batch_size: usize,
    ) -> Self {
        Self {
            receiver,
            storage,
            batch_size,
            buffer: Vec::with_capacity(batch_size),
        }
    }

    /// Start processing events
    pub async fn run(mut self) -> Result<()> {
        info!("Event processor started");

        while let Some(envelope) = self.receiver.recv().await {
            match self.process_event(envelope).await {
                Ok(_) => debug!("Event processed successfully"),
                Err(e) => error!("Failed to process event: {}", e),
            }

            // Flush if buffer is full
            if self.buffer.len() >= self.batch_size {
                self.flush().await?;
            }
        }

        // Flush remaining events
        if !self.buffer.is_empty() {
            self.flush().await?;
        }

        info!("Event processor stopped");
        Ok(())
    }

    /// Process a single event
    async fn process_event(&mut self, mut envelope: EventEnvelope) -> Result<()> {
        // Enrich event with additional data
        envelope = self.enrich_event(envelope).await?;

        // Add to buffer
        self.buffer.push(envelope);

        Ok(())
    }

    /// Enrich event with additional data
    async fn enrich_event(&self, envelope: EventEnvelope) -> Result<EventEnvelope> {
        // TODO: Add enrichment logic
        // - Geo-location lookup
        // - User agent parsing
        // - Session tracking
        // - User identification
        Ok(envelope)
    }

    /// Flush buffered events to storage
    async fn flush(&mut self) -> Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let events = std::mem::replace(
            &mut self.buffer,
            Vec::with_capacity(self.batch_size),
        );

        info!("Flushing {} events to storage", events.len());
        self.storage.store_events(events).await?;

        Ok(())
    }
}

/// Background task to periodically flush events
pub async fn flush_task(
    _storage: Arc<dyn StorageEngine>,
    interval_secs: u64,
) -> Result<()> {
    use tokio::time::{interval, Duration};

    let mut ticker = interval(Duration::from_secs(interval_secs));

    loop {
        ticker.tick().await;

        // Trigger storage flush if implemented
        debug!("Periodic flush triggered");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{Event, EventParams};
    use async_trait::async_trait;

    struct MockStorage;

    #[async_trait]
    impl StorageEngine for MockStorage {
        async fn store_events(&self, _events: Vec<EventEnvelope>) -> Result<()> {
            Ok(())
        }

        async fn get_event(&self, _id: uuid::Uuid) -> Result<Option<EventEnvelope>> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn test_event_processing() {
        let (tx, rx) = mpsc::unbounded_channel();
        let storage = Arc::new(MockStorage);
        let mut processor = EventProcessor::new(rx, storage, 10);

        let event = Event::Custom {
            name: "test".to_string(),
            params: EventParams::default(),
        };
        let envelope = EventEnvelope::new("TEST".to_string(), event);

        processor.process_event(envelope).await.unwrap();
        assert_eq!(processor.buffer.len(), 1);
    }
}

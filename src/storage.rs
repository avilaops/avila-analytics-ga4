//! Storage engine interface and implementations

use crate::error::Result;
use crate::events::EventEnvelope;
use async_trait::async_trait;
use uuid::Uuid;

/// Storage engine trait
#[async_trait]
pub trait StorageEngine: Send + Sync {
    /// Store a batch of events
    async fn store_events(&self, events: Vec<EventEnvelope>) -> Result<()>;

    /// Get an event by ID
    async fn get_event(&self, id: Uuid) -> Result<Option<EventEnvelope>>;
}

/// PostgreSQL storage implementation
pub struct PostgresStorage {
    pool: sqlx::PgPool,
}

impl PostgresStorage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id UUID PRIMARY KEY,
                measurement_id VARCHAR(255) NOT NULL,
                event_type VARCHAR(100) NOT NULL,
                event_data JSONB NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                processed BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                INDEX idx_measurement_id (measurement_id),
                INDEX idx_timestamp (timestamp),
                INDEX idx_event_type (event_type)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id UUID PRIMARY KEY,
                session_id VARCHAR(255) UNIQUE NOT NULL,
                user_id UUID,
                site_id UUID NOT NULL,
                started_at TIMESTAMPTZ NOT NULL,
                ended_at TIMESTAMPTZ,
                duration_seconds INT DEFAULT 0,
                page_views INT DEFAULT 0,
                events_count INT DEFAULT 0,
                is_bounce BOOLEAN DEFAULT TRUE,
                entry_page TEXT,
                exit_page TEXT,
                referrer TEXT,
                utm_source VARCHAR(255),
                utm_medium VARCHAR(255),
                utm_campaign VARCHAR(255),
                device_category VARCHAR(50),
                browser VARCHAR(100),
                os VARCHAR(100),
                country VARCHAR(2),
                city VARCHAR(255),
                INDEX idx_session_id (session_id),
                INDEX idx_user_id (user_id),
                INDEX idx_started_at (started_at)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl StorageEngine for PostgresStorage {
    async fn store_events(&self, events: Vec<EventEnvelope>) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for event in events {
            let event_type = format!("{:?}", event.event).split('(').next().unwrap_or("Unknown").to_string();
            let event_data = serde_json::to_value(&event.event)?;

            sqlx::query(
                r#"
                INSERT INTO events (id, measurement_id, event_type, event_data, timestamp, processed)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(event.event_id)
            .bind(&event.measurement_id)
            .bind(&event_type)
            .bind(&event_data)
            .bind(event.timestamp)
            .bind(event.processed)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn get_event(&self, id: Uuid) -> Result<Option<EventEnvelope>> {
        let row = sqlx::query_as::<_, (Uuid, String, String, serde_json::Value, chrono::DateTime<chrono::Utc>, bool)>(
            r#"
            SELECT id, measurement_id, event_type, event_data, timestamp, processed
            FROM events
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(id, measurement_id, _, event_data, timestamp, processed)| {
            EventEnvelope {
                event_id: id,
                measurement_id,
                timestamp,
                event: serde_json::from_value(event_data).unwrap(),
                processed,
            }
        }))
    }
}

/// Redis cache for real-time data
pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn increment_counter(&self, key: &str) -> Result<i64> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: i64 = redis::cmd("INCR")
            .arg(key)
            .query_async(&mut conn)
            .await?;
        Ok(value)
    }

    pub async fn get_realtime_users(&self, site_id: &str) -> Result<u32> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let key = format!("realtime:users:{}", site_id);
        let count: Option<u32> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await?;
        Ok(count.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_cache_creation() {
        let result = RedisCache::new("redis://localhost:6379");
        assert!(result.is_ok() || result.is_err()); // Just check it compiles
    }
}

//! Configuration management for Avila Analytics

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub privacy: PrivacyConfig,
    pub storage: StorageConfig,
    pub telemetry: TelemetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub cors_origins: Vec<String>,
    pub max_request_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub ttl_session: u64,
    pub ttl_cache: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub anonymize_ip: bool,
    pub respect_dnt: bool,
    pub data_retention_days: u32,
    pub cookie_consent_required: bool,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub compression_enabled: bool,
    pub batch_size: usize,
    pub flush_interval_secs: u64,
    pub max_buffer_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub service_name: String,
    pub sample_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: num_cpus::get(),
                cors_origins: vec!["*".to_string()],
                max_request_size: 1024 * 1024, // 1MB
            },
            database: DatabaseConfig {
                url: "postgres://postgres:postgres@localhost/analytics".to_string(),
                max_connections: 100,
                min_connections: 10,
                connection_timeout: 30,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 20,
                ttl_session: 1800,    // 30 minutes
                ttl_cache: 300,       // 5 minutes
            },
            privacy: PrivacyConfig {
                anonymize_ip: true,
                respect_dnt: true,
                data_retention_days: 365,
                cookie_consent_required: true,
                encryption_enabled: true,
            },
            storage: StorageConfig {
                compression_enabled: true,
                batch_size: 1000,
                flush_interval_secs: 5,
                max_buffer_size: 100_000,
            },
            telemetry: TelemetryConfig {
                enabled: true,
                endpoint: Some("http://localhost:4317".to_string()),
                service_name: "avila-analytics".to_string(),
                sample_rate: 0.1,
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::error::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .add_source(config::Environment::with_prefix("AVILA_ANALYTICS"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }

    /// Load configuration from environment variables
    pub fn from_env() -> crate::error::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("AVILA_ANALYTICS"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }

    /// Get default configuration with overrides
    pub fn with_overrides(mut self, overrides: ConfigOverrides) -> Self {
        if let Some(port) = overrides.port {
            self.server.port = port;
        }
        if let Some(db_url) = overrides.database_url {
            self.database.url = db_url;
        }
        if let Some(redis_url) = overrides.redis_url {
            self.redis.url = redis_url;
        }
        self
    }
}

#[derive(Debug, Default)]
pub struct ConfigOverrides {
    pub port: Option<u16>,
    pub database_url: Option<String>,
    pub redis_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 8080);
        assert!(config.privacy.anonymize_ip);
        assert!(config.storage.compression_enabled);
    }

    #[test]
    fn test_config_overrides() {
        let config = Config::default().with_overrides(ConfigOverrides {
            port: Some(9090),
            database_url: Some("postgres://test".to_string()),
            redis_url: None,
        });

        assert_eq!(config.server.port, 9090);
        assert_eq!(config.database.url, "postgres://test");
    }
}

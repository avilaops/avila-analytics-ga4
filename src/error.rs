//! Error types for Avila Analytics

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Redis error: {0}")]
    Redis(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Invalid event: {0}")]
    InvalidEvent(String),

    #[error("Privacy violation: {0}")]
    Privacy(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Database(err.to_string())
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::Redis(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err.to_string())
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Error::Config(err.to_string())
    }
}

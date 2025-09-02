//! Error handling for the onebox system

use thiserror::Error;

/// Custom error type for onebox operations
#[derive(Error, Debug)]
pub enum OneboxError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("TUN interface error: {0}")]
    Tun(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Encryption error: {0}")]
    Crypto(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("System error: {0}")]
    System(String),
}

/// Result type for onebox operations
pub type OneboxResult<T> = Result<T, OneboxError>;

impl From<serde_json::Error> for OneboxError {
    fn from(err: serde_json::Error) -> Self {
        OneboxError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for OneboxError {
    fn from(err: toml::de::Error) -> Self {
        OneboxError::Config(err.to_string())
    }
}

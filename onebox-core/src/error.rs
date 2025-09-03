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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_onebox_error_display() {
        assert_eq!(
            OneboxError::Config("test".to_string()).to_string(),
            "Configuration error: test"
        );
        assert_eq!(
            OneboxError::Network("test".to_string()).to_string(),
            "Network error: test"
        );
        assert_eq!(
            OneboxError::Tun("test".to_string()).to_string(),
            "TUN interface error: test"
        );
        assert_eq!(
            OneboxError::Auth("test".to_string()).to_string(),
            "Authentication error: test"
        );
        assert_eq!(
            OneboxError::Crypto("test".to_string()).to_string(),
            "Encryption error: test"
        );
        assert_eq!(
            OneboxError::Serialization("test".to_string()).to_string(),
            "Serialization error: test"
        );
        assert_eq!(
            OneboxError::System("test".to_string()).to_string(),
            "System error: test"
        );
    }

    #[test]
    fn test_io_error_from() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let onebox_error: OneboxError = io_error.into();
        assert_eq!(onebox_error.to_string(), "IO error: file not found");
    }

    #[test]
    fn test_serde_json_error_from() {
        let serde_error = serde_json::from_str::<serde_json::Value>("{,}");
        let onebox_error: OneboxError = serde_error.unwrap_err().into();
        assert!(onebox_error
            .to_string()
            .starts_with("Serialization error:"));
    }

    #[test]
    fn test_toml_de_error_from() {
        let toml_error = toml::from_str::<serde_json::Value>("invalid toml");
        let onebox_error: OneboxError = toml_error.unwrap_err().into();
        assert!(onebox_error.to_string().starts_with("Configuration error:"));
    }
}

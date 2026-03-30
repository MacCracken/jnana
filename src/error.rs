//! Error types for jnana.

use thiserror::Error;

/// Errors produced by jnana operations.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JnanaError {
    /// Entry not found in the registry.
    #[error("entry not found: {0}")]
    EntryNotFound(String),

    /// Domain not found.
    #[error("domain not found: {0}")]
    DomainNotFound(String),

    /// Profile not found.
    #[error("profile not found: {0}")]
    ProfileNotFound(String),

    /// Storage budget exceeded.
    #[error("storage budget exceeded: need {needed_mb}MB, have {available_mb}MB")]
    StorageExceeded { needed_mb: u64, available_mb: u64 },

    /// Source download or verification failed.
    #[error("source error: {0}")]
    Source(String),

    /// Content integrity check failed.
    #[error("integrity check failed for {name}: {reason}")]
    IntegrityFailed { name: String, reason: String },

    /// Parse error reading content files.
    #[error("parse error: {0}")]
    Parse(String),

    /// TOML deserialization error.
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, JnanaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = JnanaError::EntryNotFound("speed_of_light".into());
        assert_eq!(err.to_string(), "entry not found: speed_of_light");
    }

    #[test]
    fn error_storage_exceeded() {
        let err = JnanaError::StorageExceeded {
            needed_mb: 5000,
            available_mb: 2000,
        };
        assert!(err.to_string().contains("5000"));
    }

    #[test]
    fn error_integrity() {
        let err = JnanaError::IntegrityFailed {
            name: "wikipedia".into(),
            reason: "checksum mismatch".into(),
        };
        assert!(err.to_string().contains("wikipedia"));
    }

    #[test]
    fn error_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<JnanaError>();
        assert_sync::<JnanaError>();
    }

    #[test]
    fn error_domain_not_found() {
        let err = JnanaError::DomainNotFound("quantum".into());
        assert!(err.to_string().contains("quantum"));
    }

    #[test]
    fn error_profile_not_found() {
        let err = JnanaError::ProfileNotFound("turbo".into());
        assert!(err.to_string().contains("turbo"));
    }

    #[test]
    fn error_source() {
        let err = JnanaError::Source("download failed".into());
        assert!(err.to_string().contains("download failed"));
    }

    #[test]
    fn error_parse() {
        let err = JnanaError::Parse("invalid TOML".into());
        assert!(err.to_string().contains("invalid TOML"));
    }

    #[test]
    fn error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err: JnanaError = io_err.into();
        assert!(err.to_string().contains("missing"));
    }

    #[test]
    fn error_from_json() {
        let json_err = serde_json::from_str::<String>("not json").unwrap_err();
        let err: JnanaError = json_err.into();
        assert!(err.to_string().contains("JSON"));
    }
}

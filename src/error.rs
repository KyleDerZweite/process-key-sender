//! Custom error types for process-key-sender.
//!
//! This module provides structured error types using `thiserror` for better
//! error handling and more informative error messages.

use std::io;
use thiserror::Error;

/// Main error type for process-key-sender operations.
#[derive(Error, Debug)]
pub enum PksError {
    /// Process was not found after the specified number of retries.
    #[error("process '{name}' not found after {retries} attempts")]
    ProcessNotFound { name: String, retries: u32 },

    /// The specified key is invalid or unsupported.
    #[error("invalid key '{key}': {reason}")]
    InvalidKey { key: String, reason: String },

    /// Error parsing a key combination.
    #[error("invalid key combination '{combo}': {reason}")]
    InvalidKeyCombination { combo: String, reason: String },

    /// Configuration validation error.
    #[error("configuration error: {0}")]
    ConfigValidation(String),

    /// Error reading or parsing configuration file.
    #[error("failed to load config from '{path}': {reason}")]
    ConfigLoad { path: String, reason: String },

    /// Error writing configuration file.
    #[error("failed to save config to '{path}': {reason}")]
    ConfigSave { path: String, reason: String },

    /// Error parsing duration string.
    #[error("invalid duration '{value}': {reason}")]
    InvalidDuration { value: String, reason: String },

    /// Platform-specific operation is not supported.
    #[error("operation not supported on this platform: {0}")]
    UnsupportedPlatform(String),

    /// Error registering or handling hotkey.
    #[error("hotkey error: {0}")]
    Hotkey(String),

    /// Error sending key to window.
    #[error("failed to send key '{key}' to window {window_id}: {reason}")]
    KeySendFailed {
        key: String,
        window_id: u64,
        reason: String,
    },

    /// Error finding or focusing window.
    #[error("window error: {0}")]
    Window(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type alias for process-key-sender operations.
pub type Result<T> = std::result::Result<T, PksError>;

impl PksError {
    /// Create a new ProcessNotFound error.
    pub fn process_not_found(name: impl Into<String>, retries: u32) -> Self {
        Self::ProcessNotFound {
            name: name.into(),
            retries,
        }
    }

    /// Create a new InvalidKey error.
    pub fn invalid_key(key: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidKey {
            key: key.into(),
            reason: reason.into(),
        }
    }

    /// Create a new InvalidKeyCombination error.
    pub fn invalid_key_combination(combo: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidKeyCombination {
            combo: combo.into(),
            reason: reason.into(),
        }
    }

    /// Create a new ConfigValidation error.
    pub fn config_validation(message: impl Into<String>) -> Self {
        Self::ConfigValidation(message.into())
    }

    /// Create a new ConfigLoad error.
    pub fn config_load(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ConfigLoad {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Create a new InvalidDuration error.
    pub fn invalid_duration(value: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidDuration {
            value: value.into(),
            reason: reason.into(),
        }
    }

    /// Create a new UnsupportedPlatform error.
    pub fn unsupported_platform(message: impl Into<String>) -> Self {
        Self::UnsupportedPlatform(message.into())
    }

    /// Create a new Hotkey error.
    pub fn hotkey(message: impl Into<String>) -> Self {
        Self::Hotkey(message.into())
    }

    /// Create a new KeySendFailed error.
    pub fn key_send_failed(
        key: impl Into<String>,
        window_id: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self::KeySendFailed {
            key: key.into(),
            window_id,
            reason: reason.into(),
        }
    }

    /// Create a new Window error.
    pub fn window(message: impl Into<String>) -> Self {
        Self::Window(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PksError::process_not_found("game.exe", 10);
        assert_eq!(
            err.to_string(),
            "process 'game.exe' not found after 10 attempts"
        );

        let err = PksError::invalid_key("xyz", "unknown key");
        assert_eq!(err.to_string(), "invalid key 'xyz': unknown key");

        let err = PksError::config_validation("process_name cannot be empty");
        assert_eq!(
            err.to_string(),
            "configuration error: process_name cannot be empty"
        );
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let pks_err: PksError = io_err.into();
        assert!(matches!(pks_err, PksError::Io(_)));
    }
}

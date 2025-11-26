//! Configuration types and parsing for process-key-sender.
//!
//! This module provides the [`Config`] struct and related types for loading,
//! validating, and saving configuration files.
//!
//! # Example
//!
//! ```
//! use process_key_sender::Config;
//!
//! let json = r#"{
//!     "process_name": "notepad.exe",
//!     "key_sequence": [{"key": "space", "interval_after": "1000ms"}]
//! }"#;
//!
//! let config: Config = serde_json::from_str(json).unwrap();
//! assert_eq!(config.process_name, "notepad.exe");
//! ```

use anyhow::Result;
use serde::{Deserialize, Deserializer};
use std::time::Duration;

/// Main configuration struct for process-key-sender.
///
/// This struct contains all settings needed to run the automation,
/// including target process, keys to send, and timing options.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Name of the target process (e.g., "notepad.exe" or "game").
    pub process_name: String,

    /// Sequence of keys to send in order, with intervals between each.
    #[serde(default)]
    pub key_sequence: Vec<KeyAction>,

    /// Independent keys that run on separate timers simultaneously.
    #[serde(default)]
    pub independent_keys: Vec<IndependentKey>,

    /// Maximum attempts to find the target process before giving up.
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Global hotkey to pause/resume automation (e.g., "ctrl+alt+r").
    #[serde(default = "default_pause_hotkey")]
    pub pause_hotkey: String,

    /// Enable verbose output showing each key press.
    #[serde(default)]
    pub verbose: bool,

    /// Whether to loop the key sequence indefinitely.
    #[serde(default = "default_loop_sequence")]
    pub loop_sequence: bool,

    /// Number of times to repeat the sequence (0 = infinite).
    #[serde(default)]
    pub repeat_count: u32,

    /// Whether to restore focus to the original window after sending keys.
    #[serde(default = "default_restore_focus")]
    pub restore_focus: bool,
}

/// A single key action in a sequence.
///
/// Represents a key to press followed by a delay before the next action.
#[derive(Debug, Clone, Deserialize)]
pub struct KeyAction {
    /// The key to send (e.g., "space", "ctrl+s", "f5").
    pub key: String,

    /// Duration to wait after pressing this key.
    #[serde(deserialize_with = "deserialize_duration")]
    pub interval_after: Duration,
}

/// An independent key timer.
///
/// Runs on its own timer, independent of other keys.
#[derive(Debug, Clone, Deserialize)]
pub struct IndependentKey {
    /// The key to send (e.g., "r", "ctrl+s").
    pub key: String,

    /// Interval between key presses.
    #[serde(deserialize_with = "deserialize_duration")]
    pub interval: Duration,
}

// Custom deserializer for duration strings like "1000ms", "5s", "1m"
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_duration(&s).map_err(serde::de::Error::custom)
}

// Parse duration from string
pub fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim().to_lowercase();

    // Handle different formats
    if s.ends_with("ms") {
        let num_str = &s[..s.len() - 2];
        let ms: u64 = num_str
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid milliseconds value: {}", num_str))?;
        Ok(Duration::from_millis(ms))
    } else if s.ends_with('s') {
        let num_str = &s[..s.len() - 1];
        let secs: u64 = num_str
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid seconds value: {}", num_str))?;
        Ok(Duration::from_secs(secs))
    } else if s.ends_with('m') {
        let num_str = &s[..s.len() - 1];
        let mins: u64 = num_str
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid minutes value: {}", num_str))?;
        Ok(Duration::from_secs(mins * 60))
    } else {
        // Default to milliseconds if no suffix
        let ms: u64 = s
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid duration value: {}", s))?;
        Ok(Duration::from_millis(ms))
    }
}

// Default values
fn default_max_retries() -> u32 {
    10
}

fn default_pause_hotkey() -> String {
    "ctrl+alt+r".to_string()
}

fn default_loop_sequence() -> bool {
    true
}

fn default_restore_focus() -> bool {
    true
}

// ... rest of the config.rs implementation stays the same ...

impl Config {
    /// Load configuration from a JSON file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file '{}': {}", path, e))?;

        let config: Config = serde_json::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file '{}': {}", path, e))?;

        Ok(config)
    }

    /// Save configuration to a JSON file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        // Convert Duration back to string format for saving
        let config_for_save = self.clone();

        // We need to serialize with string durations for human readability
        let json = serde_json::to_string_pretty(&ConfigForSave::from(config_for_save))
            .map_err(|e| anyhow::anyhow!("Failed to serialize config: {}", e))?;

        std::fs::write(path, json)
            .map_err(|e| anyhow::anyhow!("Failed to write config file '{}': {}", path, e))?;

        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.process_name.trim().is_empty() {
            anyhow::bail!("process_name cannot be empty");
        }

        if self.key_sequence.is_empty() && self.independent_keys.is_empty() {
            anyhow::bail!("At least one key_sequence or independent_keys entry is required");
        }

        if !self.key_sequence.is_empty() && !self.independent_keys.is_empty() {
            anyhow::bail!(
                "Cannot specify both key_sequence and independent_keys. Choose one mode."
            );
        }

        if self.max_retries == 0 {
            anyhow::bail!("max_retries must be greater than 0");
        }

        // Validate key sequences
        for (i, key_action) in self.key_sequence.iter().enumerate() {
            if key_action.key.trim().is_empty() {
                anyhow::bail!("key_sequence[{}]: key cannot be empty", i);
            }
            if key_action.interval_after < Duration::from_millis(1) {
                anyhow::bail!("key_sequence[{}]: interval_after must be at least 1ms", i);
            }
        }

        // Validate independent keys
        for (i, independent_key) in self.independent_keys.iter().enumerate() {
            if independent_key.key.trim().is_empty() {
                anyhow::bail!("independent_keys[{}]: key cannot be empty", i);
            }
            if independent_key.interval < Duration::from_millis(1) {
                anyhow::bail!("independent_keys[{}]: interval must be at least 1ms", i);
            }
        }

        Ok(())
    }
}

// Helper struct for saving config with string durations
#[derive(serde::Serialize)]
struct ConfigForSave {
    process_name: String,
    key_sequence: Vec<KeyActionForSave>,
    independent_keys: Vec<IndependentKeyForSave>,
    max_retries: u32,
    pause_hotkey: String,
    verbose: bool,
    loop_sequence: bool,
    repeat_count: u32,
    restore_focus: bool,
}

#[derive(serde::Serialize)]
struct KeyActionForSave {
    key: String,
    interval_after: String,
}

#[derive(serde::Serialize)]
struct IndependentKeyForSave {
    key: String,
    interval: String,
}

impl From<Config> for ConfigForSave {
    fn from(config: Config) -> Self {
        ConfigForSave {
            process_name: config.process_name,
            key_sequence: config
                .key_sequence
                .into_iter()
                .map(|ka| KeyActionForSave {
                    key: ka.key,
                    interval_after: duration_to_string(ka.interval_after),
                })
                .collect(),
            independent_keys: config
                .independent_keys
                .into_iter()
                .map(|ik| IndependentKeyForSave {
                    key: ik.key,
                    interval: duration_to_string(ik.interval),
                })
                .collect(),
            max_retries: config.max_retries,
            pause_hotkey: config.pause_hotkey,
            verbose: config.verbose,
            loop_sequence: config.loop_sequence,
            repeat_count: config.repeat_count,
            restore_focus: config.restore_focus,
        }
    }
}

fn duration_to_string(duration: Duration) -> String {
    let ms = duration.as_millis();

    if ms >= 60000 && ms % 60000 == 0 {
        format!("{}m", ms / 60000)
    } else if ms >= 1000 && ms % 1000 == 0 {
        format!("{}s", ms / 1000)
    } else {
        format!("{}ms", ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(
            parse_duration("1000ms").unwrap(),
            Duration::from_millis(1000)
        );
        assert_eq!(parse_duration("5s").unwrap(), Duration::from_secs(5));
        assert_eq!(parse_duration("2m").unwrap(), Duration::from_secs(120));
        assert_eq!(parse_duration("500").unwrap(), Duration::from_millis(500));
        assert_eq!(
            parse_duration(" 1000MS ").unwrap(),
            Duration::from_millis(1000)
        );
    }

    #[test]
    fn test_parse_duration_errors() {
        assert!(parse_duration("abc").is_err());
        assert!(parse_duration("").is_err());
        assert!(parse_duration("1000x").is_err());
    }

    #[test]
    fn test_duration_to_string() {
        assert_eq!(duration_to_string(Duration::from_millis(1000)), "1s");
        assert_eq!(duration_to_string(Duration::from_millis(1500)), "1500ms");
        assert_eq!(duration_to_string(Duration::from_millis(60000)), "1m");
        assert_eq!(duration_to_string(Duration::from_millis(500)), "500ms");
    }

    #[test]
    fn test_config_parsing() {
        let json = r#"
        {
            "process_name": "test.exe",
            "independent_keys": [
                {
                    "key": "r",
                    "interval": "1000ms"
                },
                {
                    "key": "a",
                    "interval": "5s"
                }
            ]
        }
        "#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.process_name, "test.exe");
        assert_eq!(config.independent_keys.len(), 2);
        assert_eq!(config.independent_keys[0].key, "r");
        assert_eq!(
            config.independent_keys[0].interval,
            Duration::from_millis(1000)
        );
        assert_eq!(config.independent_keys[1].key, "a");
        assert_eq!(config.independent_keys[1].interval, Duration::from_secs(5));
        assert!(config.restore_focus); // Test default value
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config {
            process_name: "test.exe".to_string(),
            key_sequence: vec![],
            independent_keys: vec![IndependentKey {
                key: "r".to_string(),
                interval: Duration::from_millis(1000),
            }],
            max_retries: 10,
            pause_hotkey: "ctrl+alt+r".to_string(),
            verbose: false,
            loop_sequence: true,
            repeat_count: 0,
            restore_focus: true,
        };

        assert!(config.validate().is_ok());

        // Test empty process name
        config.process_name = "".to_string();
        assert!(config.validate().is_err());

        // Test no keys
        config.process_name = "test.exe".to_string();
        config.independent_keys.clear();
        assert!(config.validate().is_err());
    }
}

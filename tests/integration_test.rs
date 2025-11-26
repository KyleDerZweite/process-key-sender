use anyhow::Result;
use process_key_sender::config::{parse_duration, Config};
use process_key_sender::{KeySender, ProcessFinder};
use std::io::Write;
use std::time::Duration;
use tempfile::NamedTempFile;

#[test]
fn test_revolution_idle_config() {
    let json = r#"
    {
        "process_name": "Revolution Idle.exe",
        "key_sequence": [],
        "independent_keys": [
            {
                "key": "r",
                "interval": "1000ms"
            },
            {
                "key": "a", 
                "interval": "5000ms"
            }
        ],
        "max_retries": 10,
        "pause_hotkey": "ctrl+alt+r",
        "verbose": true,
        "loop_sequence": true,
        "repeat_count": 0
    }
    "#;

    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.process_name, "Revolution Idle.exe");
    assert_eq!(config.independent_keys.len(), 2);
    assert_eq!(config.independent_keys[0].key, "r");
    assert_eq!(
        config.independent_keys[0].interval,
        Duration::from_millis(1000)
    );
    assert_eq!(config.independent_keys[1].key, "a");
    assert_eq!(
        config.independent_keys[1].interval,
        Duration::from_millis(5000)
    );
    assert_eq!(config.max_retries, 10);
    assert_eq!(config.pause_hotkey, "ctrl+alt+r");
    assert!(config.verbose);
    assert!(config.loop_sequence);
    assert_eq!(config.repeat_count, 0);

    // Test validation
    assert!(config.validate().is_ok());
}

#[test]
fn test_key_sequence_config() {
    let json = r#"
    {
        "process_name": "notepad.exe",
        "key_sequence": [
            {
                "key": "1",
                "interval_after": "500ms"
            },
            {
                "key": "2",
                "interval_after": "500ms"
            },
            {
                "key": "space",
                "interval_after": "1s"
            }
        ],
        "independent_keys": [],
        "max_retries": 5,
        "verbose": false,
        "loop_sequence": false,
        "repeat_count": 3
    }
    "#;

    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.process_name, "notepad.exe");
    assert_eq!(config.key_sequence.len(), 3);
    assert_eq!(config.key_sequence[0].key, "1");
    assert_eq!(
        config.key_sequence[0].interval_after,
        Duration::from_millis(500)
    );
    assert_eq!(config.key_sequence[2].key, "space");
    assert_eq!(
        config.key_sequence[2].interval_after,
        Duration::from_secs(1)
    );
    assert_eq!(config.max_retries, 5);
    assert!(!config.verbose);
    assert!(!config.loop_sequence);
    assert_eq!(config.repeat_count, 3);

    // Test validation
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_file_operations() -> Result<()> {
    // Create a temporary file
    let mut temp_file = NamedTempFile::new()?;

    let json_content = r#"
    {
        "process_name": "test-app.exe",
        "key_sequence": [],
        "independent_keys": [
            {
                "key": "space",
                "interval": "2s"
            }
        ],
        "max_retries": 15,
        "pause_hotkey": "ctrl+shift+p",
        "verbose": true,
        "loop_sequence": true,
        "repeat_count": 0
    }
    "#;

    // Write JSON to file
    temp_file.write_all(json_content.as_bytes())?;

    // Load config from file
    let config = Config::from_file(temp_file.path().to_str().unwrap())?;

    assert_eq!(config.process_name, "test-app.exe");
    assert_eq!(config.independent_keys.len(), 1);
    assert_eq!(config.independent_keys[0].key, "space");
    assert_eq!(config.independent_keys[0].interval, Duration::from_secs(2));
    assert_eq!(config.max_retries, 15);
    assert_eq!(config.pause_hotkey, "ctrl+shift+p");

    // Test validation
    assert!(config.validate().is_ok());

    Ok(())
}

#[test]
fn test_duration_parsing_edge_cases() {
    // Valid cases
    assert_eq!(parse_duration("0ms").unwrap(), Duration::from_millis(0));
    assert_eq!(parse_duration("1000").unwrap(), Duration::from_millis(1000));
    assert_eq!(parse_duration("5S").unwrap(), Duration::from_secs(5)); // Case insensitive
    assert_eq!(parse_duration(" 2m ").unwrap(), Duration::from_secs(120)); // Whitespace

    // Invalid cases
    assert!(parse_duration("").is_err());
    assert!(parse_duration("abc").is_err());
    assert!(parse_duration("1000x").is_err());
    assert!(parse_duration("-1000ms").is_err());
}

#[test]
fn test_config_validation_errors() {
    // Empty process name
    let mut config = Config {
        process_name: "".to_string(),
        key_sequence: vec![],
        independent_keys: vec![],
        max_retries: 10,
        pause_hotkey: "ctrl+alt+r".to_string(),
        verbose: false,
        loop_sequence: true,
        repeat_count: 0,
        restore_focus: true,
    };

    assert!(config.validate().is_err());

    // No keys configured
    config.process_name = "test.exe".to_string();
    assert!(config.validate().is_err());

    // Zero retries
    config
        .independent_keys
        .push(process_key_sender::config::IndependentKey {
            key: "space".to_string(),
            interval: Duration::from_millis(1000),
        });
    config.max_retries = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_default_values() {
    let json = r#"
    {
        "process_name": "minimal.exe"
    }
    "#;

    // This should fail because no keys are provided
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.process_name, "minimal.exe");
    assert_eq!(config.max_retries, 10); // default
    assert_eq!(config.pause_hotkey, "ctrl+alt+r"); // default
    assert!(!config.verbose); // default false
    assert!(config.loop_sequence); // default true
    assert_eq!(config.repeat_count, 0); // default
    assert!(config.key_sequence.is_empty()); // default empty
    assert!(config.independent_keys.is_empty()); // default empty

    // Should fail validation due to no keys
    assert!(config.validate().is_err());
}

#[test]
fn test_complex_key_combinations() {
    let json = r#"
    {
        "process_name": "complex-app.exe",
        "independent_keys": [
            {
                "key": "ctrl+s",
                "interval": "30s"
            },
            {
                "key": "alt+tab",
                "interval": "10s"
            },
            {
                "key": "f5",
                "interval": "5m"
            }
        ]
    }
    "#;

    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.independent_keys.len(), 3);
    assert_eq!(config.independent_keys[0].key, "ctrl+s");
    assert_eq!(config.independent_keys[0].interval, Duration::from_secs(30));
    assert_eq!(config.independent_keys[1].key, "alt+tab");
    assert_eq!(config.independent_keys[1].interval, Duration::from_secs(10));
    assert_eq!(config.independent_keys[2].key, "f5");
    assert_eq!(
        config.independent_keys[2].interval,
        Duration::from_secs(300)
    ); // 5 minutes

    assert!(config.validate().is_ok());
}

#[test]
fn test_mixed_duration_formats() {
    let json = r#"
    {
        "process_name": "duration-test.exe",
        "key_sequence": [
            {
                "key": "1",
                "interval_after": "500ms"
            },
            {
                "key": "2", 
                "interval_after": "1s"
            },
            {
                "key": "3",
                "interval_after": "2000"
            }
        ]
    }
    "#;

    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(
        config.key_sequence[0].interval_after,
        Duration::from_millis(500)
    );
    assert_eq!(
        config.key_sequence[1].interval_after,
        Duration::from_secs(1)
    );
    assert_eq!(
        config.key_sequence[2].interval_after,
        Duration::from_millis(2000)
    );

    assert!(config.validate().is_ok());
}

// ProcessFinder tests

#[test]
fn test_process_finder_creation() {
    let finder = ProcessFinder::new();
    let finder2 = finder.clone();
    // Both should be valid instances
    drop(finder);
    drop(finder2);
}

#[test]
fn test_process_finder_default() {
    let finder = ProcessFinder::default();
    drop(finder);
}

#[test]
fn test_process_finder_nonexistent_process() {
    let mut finder = ProcessFinder::new();
    let result = finder.find_process_window("nonexistent_process_xyz_123456");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// KeySender tests

#[test]
fn test_key_sender_creation() {
    let sender = KeySender::new();
    assert!(sender.is_ok());
}

#[test]
fn test_key_sender_clone() {
    let sender = KeySender::new().unwrap();
    let sender2 = sender.clone();
    drop(sender);
    drop(sender2);
}

#[test]
fn test_key_validation_valid_keys() {
    let sender = KeySender::new().unwrap();

    // Letters
    assert!(sender.parse_key_for_validation("a").is_ok());
    assert!(sender.parse_key_for_validation("z").is_ok());
    assert!(sender.parse_key_for_validation("A").is_ok());

    // Numbers
    assert!(sender.parse_key_for_validation("0").is_ok());
    assert!(sender.parse_key_for_validation("9").is_ok());

    // Special keys
    assert!(sender.parse_key_for_validation("space").is_ok());
    assert!(sender.parse_key_for_validation("enter").is_ok());
    assert!(sender.parse_key_for_validation("tab").is_ok());
    assert!(sender.parse_key_for_validation("escape").is_ok());

    // Function keys
    assert!(sender.parse_key_for_validation("f1").is_ok());
    assert!(sender.parse_key_for_validation("f12").is_ok());

    // Arrow keys
    assert!(sender.parse_key_for_validation("left").is_ok());
    assert!(sender.parse_key_for_validation("right").is_ok());
    assert!(sender.parse_key_for_validation("up").is_ok());
    assert!(sender.parse_key_for_validation("down").is_ok());
}

#[test]
#[cfg(not(unix))]
fn test_key_validation_invalid_keys() {
    let sender = KeySender::new().unwrap();

    // Invalid keys should fail on Windows
    assert!(sender.parse_key_for_validation("invalid_key_xyz").is_err());
    assert!(sender.parse_key_for_validation("").is_err());
}

#[test]
fn test_key_validation_modifiers() {
    let sender = KeySender::new().unwrap();

    assert!(sender.parse_key_for_validation("ctrl").is_ok());
    assert!(sender.parse_key_for_validation("shift").is_ok());
    assert!(sender.parse_key_for_validation("alt").is_ok());
}

// Config save/load round-trip test

#[test]
fn test_config_save_load_roundtrip() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let config_path = temp_dir.path().join("test_config.json");

    let original = Config {
        process_name: "test.exe".to_string(),
        key_sequence: vec![process_key_sender::config::KeyAction {
            key: "space".to_string(),
            interval_after: Duration::from_millis(1500),
        }],
        independent_keys: vec![],
        max_retries: 15,
        pause_hotkey: "ctrl+shift+p".to_string(),
        verbose: true,
        loop_sequence: false,
        repeat_count: 5,
        restore_focus: false,
    };

    // Save
    original.save_to_file(config_path.to_str().unwrap())?;

    // Load
    let loaded = Config::from_file(config_path.to_str().unwrap())?;

    // Verify
    assert_eq!(loaded.process_name, original.process_name);
    assert_eq!(loaded.key_sequence.len(), original.key_sequence.len());
    assert_eq!(loaded.key_sequence[0].key, original.key_sequence[0].key);
    assert_eq!(
        loaded.key_sequence[0].interval_after,
        original.key_sequence[0].interval_after
    );
    assert_eq!(loaded.max_retries, original.max_retries);
    assert_eq!(loaded.pause_hotkey, original.pause_hotkey);
    assert_eq!(loaded.verbose, original.verbose);
    assert_eq!(loaded.loop_sequence, original.loop_sequence);
    assert_eq!(loaded.repeat_count, original.repeat_count);
    assert_eq!(loaded.restore_focus, original.restore_focus);

    Ok(())
}

// Error type tests

#[test]
fn test_error_types() {
    use process_key_sender::PksError;

    let err = PksError::process_not_found("test.exe", 10);
    assert!(err.to_string().contains("test.exe"));
    assert!(err.to_string().contains("10"));

    let err = PksError::invalid_key("xyz", "not recognized");
    assert!(err.to_string().contains("xyz"));

    let err = PksError::config_validation("missing field");
    assert!(err.to_string().contains("missing field"));
}

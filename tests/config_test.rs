use anyhow::Result;
use process_key_sender::config::{parse_duration, Config, IndependentKey, KeyAction};
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
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_file_operations() -> Result<()> {
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

    temp_file.write_all(json_content.as_bytes())?;

    let config = Config::from_file(temp_file.path().to_str().unwrap())?;

    assert_eq!(config.process_name, "test-app.exe");
    assert_eq!(config.independent_keys.len(), 1);
    assert_eq!(config.independent_keys[0].key, "space");
    assert_eq!(config.independent_keys[0].interval, Duration::from_secs(2));
    assert_eq!(config.max_retries, 15);
    assert_eq!(config.pause_hotkey, "ctrl+shift+p");
    assert!(config.validate().is_ok());

    Ok(())
}

#[test]
fn test_duration_parsing_edge_cases() {
    assert_eq!(parse_duration("0ms").unwrap(), Duration::from_millis(0));
    assert_eq!(parse_duration("1000").unwrap(), Duration::from_millis(1000));
    assert_eq!(parse_duration("5S").unwrap(), Duration::from_secs(5));
    assert_eq!(parse_duration(" 2m ").unwrap(), Duration::from_secs(120));

    assert!(parse_duration("").is_err());
    assert!(parse_duration("abc").is_err());
    assert!(parse_duration("1000x").is_err());
    assert!(parse_duration("-1000ms").is_err());
}

#[test]
fn test_config_validation_errors() {
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

    config.process_name = "test.exe".to_string();
    assert!(config.validate().is_err());

    config.independent_keys.push(IndependentKey {
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

    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.process_name, "minimal.exe");
    assert_eq!(config.max_retries, 10);
    assert_eq!(config.pause_hotkey, "ctrl+alt+r");
    assert!(!config.verbose);
    assert!(config.loop_sequence);
    assert_eq!(config.repeat_count, 0);
    assert!(config.key_sequence.is_empty());
    assert!(config.independent_keys.is_empty());
    assert!(config.restore_focus);
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
    );
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

#[test]
fn test_config_save_load_roundtrip() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let config_path = temp_dir.path().join("test_config.json");

    let original = Config {
        process_name: "test.exe".to_string(),
        key_sequence: vec![KeyAction {
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

    original.save_to_file(config_path.to_str().unwrap())?;
    let loaded = Config::from_file(config_path.to_str().unwrap())?;

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

#[test]
fn test_send_options_mapping() {
    let config = Config {
        process_name: "test.exe".to_string(),
        key_sequence: vec![KeyAction {
            key: "space".to_string(),
            interval_after: Duration::from_millis(1000),
        }],
        independent_keys: vec![],
        max_retries: 10,
        pause_hotkey: "ctrl+alt+r".to_string(),
        verbose: false,
        loop_sequence: true,
        repeat_count: 0,
        restore_focus: false,
    };

    assert!(!config.send_options().restore_focus);
}

#[test]
fn test_example_configs_deserialize_successfully() {
    let files = [
        "examples/configs/advanced-config.json",
        "examples/configs/config.json",
        "examples/configs/sequence-config.json",
        "examples/configs/single-key-config.json",
    ];

    for path in files {
        let config = Config::from_file(path).unwrap();
        assert!(config.validate().is_ok(), "{path} should be valid");
    }
}

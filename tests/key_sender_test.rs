use process_key_sender::{KeySender, SendOptions};

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

    assert!(sender.parse_key_for_validation("a").is_ok());
    assert!(sender.parse_key_for_validation("z").is_ok());
    assert!(sender.parse_key_for_validation("A").is_ok());
    assert!(sender.parse_key_for_validation("0").is_ok());
    assert!(sender.parse_key_for_validation("9").is_ok());
    assert!(sender.parse_key_for_validation("space").is_ok());
    assert!(sender.parse_key_for_validation("enter").is_ok());
    assert!(sender.parse_key_for_validation("tab").is_ok());
    assert!(sender.parse_key_for_validation("escape").is_ok());
    assert!(sender.parse_key_for_validation("f1").is_ok());
    assert!(sender.parse_key_for_validation("f12").is_ok());
    assert!(sender.parse_key_for_validation("left").is_ok());
    assert!(sender.parse_key_for_validation("right").is_ok());
    assert!(sender.parse_key_for_validation("up").is_ok());
    assert!(sender.parse_key_for_validation("down").is_ok());
    assert!(sender.parse_key_for_validation("insert").is_ok());
}

#[test]
fn test_key_validation_invalid_keys() {
    let sender = KeySender::new().unwrap();

    assert!(sender.parse_key_for_validation("invalid_key_xyz").is_err());
    assert!(sender.parse_key_for_validation("").is_err());
    assert!(sender.parse_key_for_validation("ctrl+").is_err());
    assert!(sender.parse_key_for_validation("ctrl+shift").is_err());
    assert!(sender.parse_key_for_validation("meta+s").is_err());
}

#[test]
fn test_key_validation_modifiers_and_combinations() {
    let sender = KeySender::new().unwrap();

    assert!(sender.parse_key_for_validation("ctrl").is_ok());
    assert!(sender.parse_key_for_validation("shift").is_ok());
    assert!(sender.parse_key_for_validation("alt").is_ok());
    assert!(sender.parse_key_for_validation("ctrl+s").is_ok());
    assert!(sender.parse_key_for_validation("ctrl+shift+f10").is_ok());
    assert!(sender.parse_key_for_validation("alt+tab").is_ok());
}

#[test]
fn test_send_options_default_restores_focus() {
    assert!(SendOptions::default().restore_focus);
}

#[test]
fn test_unix_send_reports_unsupported_platform() {
    #[cfg(unix)]
    {
        let sender = KeySender::new().unwrap();
        let error = sender.send_key_to_window(1, "space").unwrap_err();
        assert!(error.to_string().contains("not supported on Unix/Linux"));
    }
}

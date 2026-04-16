use assert_cmd::Command;
use clap::Parser;
use predicates::str::contains;
use process_key_sender::cli::Cli;

#[test]
fn test_cli_parses_current_flags() {
    let cli = Cli::parse_from([
        "pks",
        "--process",
        "game.exe",
        "--key",
        "space",
        "--interval",
        "1500ms",
        "--max-retries",
        "12",
        "--verbose",
        "--save-config",
        "config.json",
    ]);

    assert_eq!(cli.process.as_deref(), Some("game.exe"));
    assert_eq!(cli.key.as_deref(), Some("space"));
    assert_eq!(cli.interval, "1500ms");
    assert_eq!(cli.max_retries, 12);
    assert!(cli.verbose);
    assert_eq!(cli.save_config.as_deref(), Some("config.json"));
}

#[test]
fn test_cli_parses_config_only_mode() {
    let cli = Cli::parse_from(["pks", "--config", "examples/configs/config.json"]);

    assert_eq!(cli.config.as_deref(), Some("examples/configs/config.json"));
    assert_eq!(cli.interval, "1000ms");
    assert_eq!(cli.max_retries, 10);
    assert!(!cli.verbose);
}

#[test]
fn test_binary_help_output_lists_current_flags() {
    Command::cargo_bin("pks")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("--config"))
        .stdout(contains("--process"))
        .stdout(contains("--key"))
        .stdout(contains("--interval"))
        .stdout(contains("--verbose"))
        .stdout(contains("--save-config"))
        .stdout(contains("--max-retries"));
}

#[test]
fn test_binary_version_output_reports_current_release() {
    Command::cargo_bin("pks")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(contains("0.2.1"));
}

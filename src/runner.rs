use anyhow::Result;
use colored::Colorize;
use std::time::Duration;
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;
use crate::config::{parse_duration, Config, KeyAction};
use crate::{HotkeyManager, KeySender, PauseState, ProcessFinder};

/// Initialize tracing based on the requested verbosity.
pub fn init_tracing(verbose: bool) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        if verbose {
            EnvFilter::new("debug")
        } else {
            EnvFilter::new("info")
        }
    });

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .try_init();
}

/// Execute the CLI workflow.
pub async fn run(cli: Cli) -> Result<()> {
    let config = if let Some(config_file) = cli.config.as_deref() {
        load_config_file(config_file)?
    } else {
        create_config_from_args(&cli)?
    };

    if let Some(save_path) = cli.save_config.as_deref() {
        config.save_to_file(save_path)?;
        println!(
            "{} Configuration saved to: {}",
            "✓".green(),
            save_path.cyan()
        );
        return Ok(());
    }

    validate_config(&config)?;
    print_startup_info(&config);

    let mut process_finder = ProcessFinder::new();
    let key_sender = KeySender::new()?;

    let mut hotkey_manager = HotkeyManager::new()?;
    hotkey_manager.register_pause_hotkey(&config.pause_hotkey)?;
    let pause_state = hotkey_manager.pause_state();

    hotkey_manager.start_hotkey_listener().await?;

    run_automation(config, &mut process_finder, &key_sender, pause_state).await
}

fn load_config_file(config_file: &str) -> Result<Config> {
    println!(
        "{} Loading configuration from: {}",
        "📁".blue(),
        config_file.cyan()
    );

    match Config::from_file(config_file) {
        Ok(config) => {
            println!("{} Configuration loaded successfully", "✓".green());
            Ok(config)
        }
        Err(error) => {
            eprintln!("{} Failed to load configuration: {}", "✗".red(), error);
            anyhow::bail!("Configuration loading failed: {}", error);
        }
    }
}

fn create_config_from_args(cli: &Cli) -> Result<Config> {
    let process_name = cli
        .process
        .clone()
        .ok_or_else(|| anyhow::anyhow!("Process name is required. Use --process or --config."))?;

    let key = cli
        .key
        .clone()
        .ok_or_else(|| anyhow::anyhow!("Key is required. Use --key or --config."))?;

    let interval = parse_duration(&cli.interval)?;

    Ok(Config {
        process_name,
        key_sequence: vec![KeyAction {
            key,
            interval_after: interval,
        }],
        independent_keys: vec![],
        max_retries: cli.max_retries,
        pause_hotkey: "ctrl+alt+r".to_string(),
        verbose: cli.verbose,
        loop_sequence: true,
        repeat_count: 0,
        restore_focus: true,
    })
}

fn validate_config(config: &Config) -> Result<()> {
    config.validate()?;

    let key_sender = KeySender::new()?;

    for key_action in &config.key_sequence {
        validate_key(&key_sender, &key_action.key)?;
        warn_on_short_interval(key_action.interval_after, &key_action.key);
    }

    for independent_key in &config.independent_keys {
        validate_key(&key_sender, &independent_key.key)?;
        warn_on_short_interval(independent_key.interval, &independent_key.key);
    }

    Ok(())
}

fn validate_key(key_sender: &KeySender, key: &str) -> Result<()> {
    key_sender
        .parse_key_for_validation(key)
        .map_err(|error| anyhow::anyhow!("Invalid key '{}': {}", key, error))?;
    Ok(())
}

fn warn_on_short_interval(interval: Duration, key: &str) {
    if interval < Duration::from_millis(50) {
        println!(
            "{} Warning: Very short interval ({}ms) for key '{}' may cause issues",
            "⚠".yellow(),
            interval.as_millis(),
            key
        );
    }
}

fn print_startup_info(config: &Config) {
    println!(
        "\n{}",
        format!("Process Key Sender v{}", env!("CARGO_PKG_VERSION"))
            .bold()
            .cyan()
    );
    println!("{}", "═".repeat(40).cyan());

    println!(
        "{} Target Process: {}",
        "🎯".blue(),
        config.process_name.yellow()
    );
    println!(
        "{} Max Retries: {}",
        "🔄".blue(),
        config.max_retries.to_string().yellow()
    );
    println!(
        "{} Pause Hotkey: {}",
        "⏸".blue(),
        config.pause_hotkey.yellow()
    );
    println!(
        "{} Restore Focus: {}",
        "🪟".blue(),
        if config.restore_focus {
            "ON".green()
        } else {
            "OFF".red()
        }
    );
    println!(
        "{} Verbose Mode: {}",
        "📝".blue(),
        if config.verbose {
            "ON".green()
        } else {
            "OFF".red()
        }
    );

    if !config.key_sequence.is_empty() {
        println!("\n{} Key Sequence Mode:", "⌨".blue());
        for (index, key_action) in config.key_sequence.iter().enumerate() {
            println!(
                "  {}. {} (wait {}ms)",
                index + 1,
                key_action.key.cyan(),
                key_action.interval_after.as_millis().to_string().yellow()
            );
        }
        println!(
            "  {} Loop: {}",
            "🔁".blue(),
            if config.loop_sequence {
                "YES".green()
            } else {
                "NO".red()
            }
        );
        if config.repeat_count > 0 {
            println!(
                "  {} Repeat Count: {}",
                "🔢".blue(),
                config.repeat_count.to_string().yellow()
            );
        }
    }

    if !config.independent_keys.is_empty() {
        println!("\n{} Independent Keys Mode:", "⌨".blue());
        for independent_key in &config.independent_keys {
            println!(
                "  {} every {}ms",
                independent_key.key.cyan(),
                independent_key.interval.as_millis().to_string().yellow()
            );
        }
    }

    println!("{}", "═".repeat(40).cyan());
    println!(
        "{} Press {} to pause/resume globally",
        "⏸".blue(),
        config.pause_hotkey.yellow()
    );
    println!("{} Press Ctrl+C to stop\n", "ℹ".blue());
}

async fn run_automation(
    config: Config,
    process_finder: &mut ProcessFinder,
    key_sender: &KeySender,
    pause_state: PauseState,
) -> Result<()> {
    let window_id = find_target_process(&config, process_finder).await?;

    println!("{} Process found! Starting automation...", "✓".green());

    if !config.independent_keys.is_empty() {
        run_independent_keys(&config, key_sender, window_id, pause_state).await
    } else {
        run_key_sequence(&config, key_sender, window_id, pause_state).await
    }
}

async fn find_target_process(config: &Config, process_finder: &mut ProcessFinder) -> Result<u64> {
    println!(
        "{} Searching for process: {}",
        "🔍".blue(),
        config.process_name.yellow()
    );

    for attempt in 1..=config.max_retries {
        if config.verbose {
            println!("  Attempt {}/{}", attempt, config.max_retries);
        }

        match process_finder.find_process_window(&config.process_name) {
            Ok(Some(window_id)) => {
                println!(
                    "{} Found process window (ID: {})",
                    "✓".green(),
                    window_id.to_string().cyan()
                );
                return Ok(window_id);
            }
            Ok(None) => {
                if config.verbose {
                    println!("  Process not found, retrying...");
                }
            }
            Err(error) => {
                eprintln!("{} Error searching for process: {}", "✗".red(), error);
            }
        }

        if attempt < config.max_retries {
            sleep(Duration::from_millis(1000)).await;
        }
    }

    anyhow::bail!(
        "Could not find process '{}' after {} attempts",
        config.process_name,
        config.max_retries
    );
}

async fn run_independent_keys(
    config: &Config,
    key_sender: &KeySender,
    window_id: u64,
    pause_state: PauseState,
) -> Result<()> {
    println!("{} Starting independent keys automation...", "🚀".green());

    let mut handles = Vec::new();
    let mut pause_receiver = pause_state.get_pause_receiver();
    let send_options = config.send_options();

    for independent_key in &config.independent_keys {
        let key = independent_key.key.clone();
        let interval = independent_key.interval;
        let sender = key_sender.clone();
        let verbose = config.verbose;
        let pause_state = pause_state.clone();

        let handle = tokio::spawn(async move {
            loop {
                if pause_state.is_paused() {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                }

                match sender.send_key_to_window_with_options(window_id, &key, send_options) {
                    Ok(()) => {
                        if verbose {
                            println!("✓ Sent key: {}", key.cyan());
                        }
                    }
                    Err(error) => {
                        eprintln!("{} Error sending key '{}': {}", "✗".red(), key, error);
                    }
                }

                sleep(interval).await;
            }
        });

        handles.push(handle);
    }

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("\n{} Shutting down...", "🛑".yellow());
                break;
            }
            _ = pause_receiver.changed() => {}
        }
    }

    for handle in handles {
        handle.abort();
    }

    Ok(())
}

async fn run_key_sequence(
    config: &Config,
    key_sender: &KeySender,
    window_id: u64,
    pause_state: PauseState,
) -> Result<()> {
    println!("{} Starting key sequence automation...", "🚀".green());

    let mut iteration = 0u32;
    let mut pause_receiver = pause_state.get_pause_receiver();
    let send_options = config.send_options();

    loop {
        iteration += 1;

        if config.verbose {
            println!(
                "--- Sequence iteration {} ---",
                iteration.to_string().cyan()
            );
        }

        for (index, key_action) in config.key_sequence.iter().enumerate() {
            if tokio::time::timeout(Duration::from_millis(1), tokio::signal::ctrl_c())
                .await
                .is_ok()
            {
                println!("\n{} Shutting down...", "🛑".yellow());
                return Ok(());
            }

            while pause_state.is_paused() {
                if tokio::time::timeout(Duration::from_millis(100), tokio::signal::ctrl_c())
                    .await
                    .is_ok()
                {
                    println!("\n{} Shutting down...", "🛑".yellow());
                    return Ok(());
                }

                if pause_receiver.has_changed().unwrap_or(false) {
                    let _ = pause_receiver.borrow_and_update();
                }

                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            match key_sender.send_key_to_window_with_options(
                window_id,
                &key_action.key,
                send_options,
            ) {
                Ok(()) => {
                    if config.verbose {
                        println!("  {}. ✓ Sent key: {}", index + 1, key_action.key.cyan());
                    }
                }
                Err(error) => {
                    eprintln!(
                        "  {}. {} Error sending key '{}': {}",
                        index + 1,
                        "✗".red(),
                        key_action.key,
                        error
                    );
                }
            }

            sleep(key_action.interval_after).await;
        }

        if config.repeat_count > 0 && iteration >= config.repeat_count {
            println!(
                "{} Completed {} iterations",
                "✓".green(),
                config.repeat_count.to_string().cyan()
            );
            break;
        }

        if !config.loop_sequence {
            break;
        }
    }

    Ok(())
}

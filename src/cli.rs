use clap::{CommandFactory, Parser};

/// Command-line arguments for the `pks` binary.
#[derive(Debug, Clone, Parser)]
#[command(
    name = "Process Key Sender",
    bin_name = "pks",
    version,
    author,
    about = "Cross-platform keystroke automation tool for specific processes",
    long_about = None
)]
pub struct Cli {
    /// Configuration file path.
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<String>,

    /// Target process name (e.g., 'notepad.exe').
    #[arg(short = 'p', long = "process", value_name = "PROCESS")]
    pub process: Option<String>,

    /// Key to send (e.g., 'space', 'a', 'ctrl+c').
    #[arg(short = 'k', long = "key", value_name = "KEY")]
    pub key: Option<String>,

    /// Interval between key presses (e.g., '1000ms', '5s').
    #[arg(
        short = 'i',
        long = "interval",
        value_name = "DURATION",
        default_value = "1000ms"
    )]
    pub interval: String,

    /// Enable verbose output.
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Save current CLI arguments to a configuration file.
    #[arg(long = "save-config", value_name = "FILE")]
    pub save_config: Option<String>,

    /// Maximum retries to find the process.
    #[arg(long = "max-retries", value_name = "COUNT", default_value_t = 10)]
    pub max_retries: u32,
}

impl Cli {
    /// Expose clap's generated command for tests and docs.
    pub fn command() -> clap::Command {
        <Self as CommandFactory>::command()
    }
}

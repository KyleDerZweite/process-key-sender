//! # Process Key Sender
//!
//! A cross-platform command-line tool for sending keystrokes to specific processes
//! at configurable intervals.
//!
//! ## Features
//!
//! - Target specific processes by name
//! - Support for single keys, key sequences, and key combinations
//! - Independent key timers for simultaneous automation
//! - Global hotkey for pause/resume functionality
//! - JSON configuration file support
//! - Cross-platform support (Windows, Linux planned)
//!
//! ## Example
//!
//! ```no_run
//! use process_key_sender::{Config, KeySender, ProcessFinder};
//!
//! // Create components
//! let mut finder = ProcessFinder::new();
//! let sender = KeySender::new().unwrap();
//!
//! // Find a process
//! if let Ok(Some(pid)) = finder.find_process_window("notepad") {
//!     // Send a key
//!     sender.send_key_to_window(pid, "space").unwrap();
//! }
//! ```
//!
//! ## Configuration
//!
//! Configuration can be provided via JSON files:
//!
//! ```json
//! {
//!   "process_name": "app.exe",
//!   "key_sequence": [
//!     {"key": "space", "interval_after": "1000ms"}
//!   ],
//!   "pause_hotkey": "ctrl+alt+r"
//! }
//! ```

pub mod config;
pub mod error;
pub mod global_hotkey;
pub mod key_sender;
pub mod process_finder;

pub use config::Config;
pub use error::{PksError, Result};
pub use global_hotkey::HotkeyManager;
pub use key_sender::KeySender;
pub use process_finder::ProcessFinder;

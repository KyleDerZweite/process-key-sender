//! Process Key Sender - Cross-platform keystroke automation tool
//!
//! This library provides functionality to send keystrokes to specific processes
//! with configurable intervals and patterns.

pub mod config;
pub mod global_hotkey;
pub mod key_sender;
pub mod process_finder;

pub use config::Config;
pub use global_hotkey::HotkeyManager;
pub use key_sender::KeySender;
pub use process_finder::ProcessFinder;
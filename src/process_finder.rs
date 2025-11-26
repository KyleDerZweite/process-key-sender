//! Process discovery and window management.
//!
//! This module provides functionality to find running processes by name
//! and retrieve their window handles for key sending operations.

use anyhow::Result;
use sysinfo::{ProcessesToUpdate, System};

/// Finds processes by name and retrieves window identifiers.
///
/// Uses the `sysinfo` crate to enumerate running processes and match
/// them by name. On Windows, the PID is used to find the associated
/// window handle for key sending.
///
/// # Example
///
/// ```
/// use process_key_sender::ProcessFinder;
///
/// let mut finder = ProcessFinder::new();
/// match finder.find_process_window("notepad") {
///     Ok(Some(pid)) => println!("Found process with PID: {}", pid),
///     Ok(None) => println!("Process not found"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub struct ProcessFinder {
    system: System,
}

impl Clone for ProcessFinder {
    fn clone(&self) -> Self {
        Self {
            system: System::new(),
        }
    }
}

impl Default for ProcessFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessFinder {
    pub fn new() -> Self {
        Self {
            system: System::new(),
        }
    }

    pub fn find_process_window(&mut self, process_name: &str) -> Result<Option<u64>> {
        // Refresh all processes (new sysinfo 0.37+ API)
        self.system.refresh_processes(ProcessesToUpdate::All, true);

        let process_name_lower = process_name.to_lowercase();

        for (pid, process) in self.system.processes() {
            // process.name() returns &OsStr, convert to string for comparison
            let name = process.name().to_string_lossy().to_lowercase();
            if name.contains(&process_name_lower) {
                #[cfg(windows)]
                {
                    // For Windows, we'll use a simpler approach - just return the PID as window ID
                    // The key sender can work with this approach
                    return Ok(Some(pid.as_u32() as u64));
                }

                #[cfg(unix)]
                {
                    // For Unix, we'll use the PID as window ID for now
                    return Ok(Some(pid.as_u32() as u64));
                }
            }
        }

        Ok(None)
    }

    #[deprecated]
    #[allow(dead_code)]
    pub fn is_process_running(&mut self, process_name: &str) -> Result<bool> {
        // Refresh all processes (new sysinfo 0.37+ API)
        self.system.refresh_processes(ProcessesToUpdate::All, true);

        let process_name_lower = process_name.to_lowercase();

        for process in self.system.processes().values() {
            // process.name() returns &OsStr, convert to string for comparison
            let name = process.name().to_string_lossy().to_lowercase();
            if name.contains(&process_name_lower) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

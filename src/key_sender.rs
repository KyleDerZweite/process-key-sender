//! Keyboard input simulation for sending keystrokes to windows.
//!
//! This module provides platform-specific key sending functionality.
//! On Windows, it uses the Windows API (`SendInput`). On Unix/Linux,
//! configuration parsing and validation are available, but sending keys to
//! other processes is not currently supported.
//!
//! # Supported Keys
//!
//! - Letters: `a-z` (case insensitive)
//! - Numbers: `0-9`
//! - Function keys: `f1-f12`
//! - Special keys: `space`, `enter`, `tab`, `escape`, `backspace`, `delete`
//! - Arrow keys: `left`, `right`, `up`, `down`
//! - Navigation: `home`, `end`, `pageup`, `pagedown`
//! - Modifiers: `shift`, `ctrl`, `alt`
//! - Combinations: `ctrl+c`, `alt+tab`, `ctrl+shift+s`

use anyhow::Result;
use std::collections::HashMap;

#[cfg(windows)]
use winapi::shared::windef::HWND;
#[cfg(windows)]
use winapi::um::winuser::{
    BringWindowToTop, EnumWindows, GetForegroundWindow, GetWindowTextA, GetWindowThreadProcessId,
    IsWindowVisible, SendInput, SetActiveWindow, SetForegroundWindow, ShowWindow, INPUT,
    INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SW_RESTORE, VK_CONTROL, VK_ESCAPE, VK_MENU,
    VK_RETURN, VK_SHIFT, VK_SPACE, VK_TAB,
};

/// Sends keystrokes to target windows.
///
/// Handles key parsing, validation, and platform-specific key injection.
/// Supports single keys, modifier combinations, and key sequences.
///
/// # Example
///
/// ```no_run
/// use process_key_sender::KeySender;
///
/// let sender = KeySender::new().unwrap();
///
/// // Validate a key
/// sender.parse_key_for_validation("ctrl+s").unwrap();
///
/// // Send a key to a window (requires valid window ID)
/// // sender.send_key_to_window(12345, "space").unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SendOptions {
    pub restore_focus: bool,
}

impl Default for SendOptions {
    fn default() -> Self {
        Self {
            restore_focus: true,
        }
    }
}

pub struct KeySender {
    #[cfg(windows)]
    key_map: HashMap<String, u32>,
    #[cfg(unix)]
    #[allow(dead_code)]
    key_map: HashMap<String, u32>,
}

impl Clone for KeySender {
    fn clone(&self) -> Self {
        Self::new().unwrap()
    }
}

impl KeySender {
    pub fn new() -> Result<Self> {
        #[cfg(windows)]
        {
            let mut key_map = HashMap::new();

            // Special keys
            key_map.insert("space".to_string(), VK_SPACE as u32);
            key_map.insert("enter".to_string(), VK_RETURN as u32);
            key_map.insert("return".to_string(), VK_RETURN as u32);
            key_map.insert("tab".to_string(), VK_TAB as u32);
            key_map.insert("escape".to_string(), VK_ESCAPE as u32);
            key_map.insert("esc".to_string(), VK_ESCAPE as u32);
            key_map.insert("shift".to_string(), VK_SHIFT as u32);
            key_map.insert("ctrl".to_string(), VK_CONTROL as u32);
            key_map.insert("control".to_string(), VK_CONTROL as u32);
            key_map.insert("alt".to_string(), VK_MENU as u32);

            // Function keys
            for i in 1..=12 {
                key_map.insert(format!("f{}", i), (0x70 + i - 1) as u32);
            }

            // Number keys
            for i in 0..=9 {
                key_map.insert(i.to_string(), (0x30 + i) as u32);
            }

            // Letter keys
            for i in 0..26 {
                let letter = (b'a' + i) as char;
                key_map.insert(letter.to_string(), (0x41 + i) as u32); // VK_A to VK_Z
            }

            // Arrow keys
            key_map.insert("left".to_string(), 0x25);
            key_map.insert("up".to_string(), 0x26);
            key_map.insert("right".to_string(), 0x27);
            key_map.insert("down".to_string(), 0x28);

            // Additional keys
            key_map.insert("backspace".to_string(), 0x08);
            key_map.insert("delete".to_string(), 0x2E);
            key_map.insert("home".to_string(), 0x24);
            key_map.insert("end".to_string(), 0x23);
            key_map.insert("pageup".to_string(), 0x21);
            key_map.insert("pagedown".to_string(), 0x22);
            key_map.insert("insert".to_string(), 0x2D);

            Ok(Self { key_map })
        }

        #[cfg(unix)]
        {
            Ok(Self {
                key_map: HashMap::new(),
            })
        }
    }

    pub fn parse_key_for_validation(&self, key: &str) -> Result<()> {
        validate_key_expression(key)?;

        #[cfg(windows)]
        {
            if key.contains('+') {
                for part in key.split('+') {
                    let _ = self.parse_key_windows(part)?;
                }
            } else {
                let _ = self.parse_key_windows(key)?;
            }
            Ok(())
        }

        #[cfg(unix)]
        {
            Ok(())
        }
    }

    pub fn send_key_to_window(&self, window_id: u64, key: &str) -> Result<()> {
        self.send_key_to_window_with_options(window_id, key, SendOptions::default())
    }

    pub fn send_key_to_window_with_options(
        &self,
        window_id: u64,
        key: &str,
        options: SendOptions,
    ) -> Result<()> {
        #[cfg(windows)]
        {
            let pid = window_id as u32;

            if let Some(hwnd) = self.find_window_by_pid(pid) {
                self.send_key_with_focus_option(hwnd, key, options.restore_focus)
            } else {
                self.send_key_global_windows(key)
            }
        }

        #[cfg(unix)]
        {
            self.send_key_unix(window_id, key, options)
        }
    }

    #[cfg(windows)]
    fn find_window_by_pid(&self, target_pid: u32) -> Option<HWND> {
        struct EnumData {
            target_pid: u32,
            result: Option<HWND>,
        }

        let mut enum_data = EnumData {
            target_pid,
            result: None,
        };

        unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: isize) -> i32 {
            let enum_data = &mut *(lparam as *mut EnumData);

            unsafe {
                let mut window_pid = 0;
                GetWindowThreadProcessId(hwnd, &mut window_pid);

                if window_pid == enum_data.target_pid && IsWindowVisible(hwnd) != 0 {
                    let mut title = [0u8; 256];
                    let len = GetWindowTextA(hwnd, title.as_mut_ptr() as *mut i8, 256);

                    if len > 0 {
                        enum_data.result = Some(hwnd);
                        return 0; // Stop enumeration
                    }
                }
            }

            1 // Continue enumeration
        }

        unsafe {
            EnumWindows(Some(enum_proc), &mut enum_data as *mut _ as isize);
        }

        enum_data.result
    }

    #[cfg(windows)]
    fn send_key_with_focus_option(&self, hwnd: HWND, key: &str, restore_focus: bool) -> Result<()> {
        let original_window = unsafe { GetForegroundWindow() };
        let needs_focus_change = original_window != hwnd;

        if needs_focus_change {
            self.ensure_window_focus(hwnd)?;
        }

        let result = self.send_key_global_windows(key);

        if restore_focus && needs_focus_change && !original_window.is_null() {
            std::thread::sleep(std::time::Duration::from_millis(50));

            unsafe {
                SetForegroundWindow(original_window);
                SetActiveWindow(original_window);
            }
        }

        result
    }

    #[cfg(windows)]
    fn ensure_window_focus(&self, hwnd: HWND) -> Result<()> {
        unsafe {
            // Restore window if minimized
            ShowWindow(hwnd, SW_RESTORE);

            // Bring to top and set focus
            BringWindowToTop(hwnd);
            SetActiveWindow(hwnd);
            SetForegroundWindow(hwnd);

            // Minimal delay to ensure focus is established
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Ok(())
    }

    #[cfg(windows)]
    fn send_key_global_windows(&self, key: &str) -> Result<()> {
        if key.contains('+') {
            return self.send_key_combination_global_windows(key);
        }

        let vk_code = self.parse_key_windows(key)?;

        unsafe {
            // Key DOWN
            let mut input_down = INPUT {
                type_: INPUT_KEYBOARD,
                u: std::mem::zeroed(),
            };

            *input_down.u.ki_mut() = KEYBDINPUT {
                wVk: vk_code as u16,
                wScan: 0,
                dwFlags: 0, // Key down
                time: 0,
                dwExtraInfo: 0,
            };

            // Key UP
            let mut input_up = INPUT {
                type_: INPUT_KEYBOARD,
                u: std::mem::zeroed(),
            };

            *input_up.u.ki_mut() = KEYBDINPUT {
                wVk: vk_code as u16,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            };

            // Send key down
            let result1 = SendInput(1, &mut input_down, std::mem::size_of::<INPUT>() as i32);

            // Realistic key press duration
            std::thread::sleep(std::time::Duration::from_millis(30));

            // Send key up
            let result2 = SendInput(1, &mut input_up, std::mem::size_of::<INPUT>() as i32);

            if result1 == 0 || result2 == 0 {
                anyhow::bail!(
                    "SendInput failed for key '{}' (results: {}, {})",
                    key,
                    result1,
                    result2
                );
            }
        }

        Ok(())
    }

    #[cfg(windows)]
    fn send_key_combination_global_windows(&self, key_combo: &str) -> Result<()> {
        let parts: Vec<&str> = key_combo.split('+').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            anyhow::bail!("Invalid key combination format: {}", key_combo);
        }

        let mut modifier_codes = Vec::new();
        let main_key = parts.last().unwrap();

        // Parse modifiers
        for modifier in &parts[..parts.len() - 1] {
            let vk_code = self.parse_key_windows(modifier)?;
            modifier_codes.push(vk_code);
        }

        let main_key_code = self.parse_key_windows(main_key)?;

        unsafe {
            let mut inputs = Vec::new();

            // Press modifiers
            for &modifier_code in &modifier_codes {
                let mut input = INPUT {
                    type_: INPUT_KEYBOARD,
                    u: std::mem::zeroed(),
                };
                *input.u.ki_mut() = KEYBDINPUT {
                    wVk: modifier_code as u16,
                    wScan: 0,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                };
                inputs.push(input);
            }

            // Press main key
            let mut main_down = INPUT {
                type_: INPUT_KEYBOARD,
                u: std::mem::zeroed(),
            };
            *main_down.u.ki_mut() = KEYBDINPUT {
                wVk: main_key_code as u16,
                wScan: 0,
                dwFlags: 0,
                time: 0,
                dwExtraInfo: 0,
            };
            inputs.push(main_down);

            // Release main key
            let mut main_up = INPUT {
                type_: INPUT_KEYBOARD,
                u: std::mem::zeroed(),
            };
            *main_up.u.ki_mut() = KEYBDINPUT {
                wVk: main_key_code as u16,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            };
            inputs.push(main_up);

            // Release modifiers (reverse order)
            for &modifier_code in modifier_codes.iter().rev() {
                let mut input = INPUT {
                    type_: INPUT_KEYBOARD,
                    u: std::mem::zeroed(),
                };
                *input.u.ki_mut() = KEYBDINPUT {
                    wVk: modifier_code as u16,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                };
                inputs.push(input);
            }

            // Send all inputs at once
            let result = SendInput(
                inputs.len() as u32,
                inputs.as_mut_ptr(),
                std::mem::size_of::<INPUT>() as i32,
            );

            if result != inputs.len() as u32 {
                anyhow::bail!(
                    "SendInput failed for key combination '{}' (sent {}/{})",
                    key_combo,
                    result,
                    inputs.len()
                );
            }
        }

        Ok(())
    }

    #[cfg(windows)]
    fn parse_key_windows(&self, key: &str) -> Result<u32> {
        let key_lower = key.trim().to_lowercase();

        if let Some(&vk_code) = self.key_map.get(&key_lower) {
            return Ok(vk_code);
        }

        anyhow::bail!("Unsupported key: {}", key)
    }

    #[cfg(unix)]
    fn send_key_unix(&self, _window_id: u64, _key: &str, _options: SendOptions) -> Result<()> {
        anyhow::bail!("key sending is not supported on Unix/Linux")
    }
}

fn validate_key_expression(key: &str) -> Result<()> {
    let normalized = key.trim().to_lowercase();
    if normalized.is_empty() {
        anyhow::bail!("Key cannot be empty");
    }

    let parts: Vec<&str> = normalized.split('+').map(str::trim).collect();
    if parts.iter().any(|part| part.is_empty()) {
        anyhow::bail!("Invalid key combination: {}", key);
    }

    if parts.len() == 1 {
        return validate_single_key(parts[0], key);
    }

    let (main_key, modifiers) = parts.split_last().unwrap();
    for modifier in modifiers {
        if !is_modifier_key(modifier) {
            anyhow::bail!(
                "Invalid modifier '{}' in key combination '{}'",
                modifier,
                key
            );
        }
    }

    if !is_non_modifier_key(main_key) {
        anyhow::bail!(
            "Unsupported key '{}' in key combination '{}'",
            main_key,
            key
        );
    }

    Ok(())
}

fn validate_single_key(key: &str, original: &str) -> Result<()> {
    if is_modifier_key(key) || is_non_modifier_key(key) {
        return Ok(());
    }

    anyhow::bail!("Unsupported key: {}", original)
}

fn is_modifier_key(key: &str) -> bool {
    matches!(key, "shift" | "ctrl" | "control" | "alt")
}

fn is_non_modifier_key(key: &str) -> bool {
    matches!(
        key,
        "space"
            | "enter"
            | "return"
            | "tab"
            | "escape"
            | "esc"
            | "backspace"
            | "delete"
            | "insert"
            | "home"
            | "end"
            | "pageup"
            | "pagedown"
            | "left"
            | "right"
            | "up"
            | "down"
    ) || is_ascii_letter(key)
        || is_ascii_digit(key)
        || is_function_key(key)
}

fn is_ascii_letter(key: &str) -> bool {
    key.len() == 1 && key.as_bytes()[0].is_ascii_alphabetic()
}

fn is_ascii_digit(key: &str) -> bool {
    key.len() == 1 && key.as_bytes()[0].is_ascii_digit()
}

fn is_function_key(key: &str) -> bool {
    match key.strip_prefix('f') {
        Some(number) => matches!(number.parse::<u8>(), Ok(1..=12)),
        None => false,
    }
}

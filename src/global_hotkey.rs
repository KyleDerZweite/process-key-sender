use anyhow::Result;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::watch;

pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    is_paused: Arc<AtomicBool>,
    pause_sender: watch::Sender<bool>,
    pause_receiver: watch::Receiver<bool>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| anyhow::anyhow!("Failed to create GlobalHotKeyManager: {}", e))?;
        
        let is_paused = Arc::new(AtomicBool::new(false));
        let (pause_sender, pause_receiver) = watch::channel(false);

        Ok(Self {
            manager,
            is_paused,
            pause_sender,
            pause_receiver,
        })
    }

    pub fn register_pause_hotkey(&mut self, hotkey_str: &str) -> Result<()> {
        let hotkey = parse_hotkey(hotkey_str)?;
        
        self.manager.register(hotkey)
            .map_err(|e| anyhow::anyhow!("Failed to register hotkey '{}': {}", hotkey_str, e))?;

        println!("🔥 Global pause hotkey '{}' registered successfully", hotkey_str);
        Ok(())
    }

    pub fn get_pause_receiver(&self) -> watch::Receiver<bool> {
        self.pause_receiver.clone()
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::Relaxed)
    }

    pub async fn start_hotkey_listener(self: Arc<Self>) -> Result<()> {
        let receiver = GlobalHotKeyEvent::receiver();
        let manager = self.clone();

        tokio::task::spawn_blocking(move || {
            loop {
                if let Ok(event) = receiver.try_recv() {
                    if event.state == HotKeyState::Pressed {
                        let current_state = manager.is_paused.load(Ordering::Relaxed);
                        let new_state = !current_state;
                        
                        manager.is_paused.store(new_state, Ordering::Relaxed);
                        
                        if let Err(e) = manager.pause_sender.send(new_state) {
                            eprintln!("Failed to send pause state: {}", e);
                        }

                        if new_state {
                            println!("⏸️  Automation PAUSED (press hotkey again to resume)");
                        } else {
                            println!("▶️  Automation RESUMED");
                        }
                    }
                }
                
                // Small sleep to prevent busy waiting
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        Ok(())
    }
}

fn parse_hotkey(hotkey_str: &str) -> Result<global_hotkey::hotkey::HotKey> {
    use global_hotkey::hotkey::{HotKey, Modifiers};

    let binding = hotkey_str.to_lowercase();
    let parts: Vec<&str> = binding.split('+').map(|s| s.trim()).collect();
    
    if parts.is_empty() {
        return Err(anyhow::anyhow!("Empty hotkey string"));
    }

    let mut modifiers = Modifiers::empty();
    let mut key_code = None;

    for part in &parts {
        match *part {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "alt" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            "meta" | "cmd" | "super" => modifiers |= Modifiers::SUPER,
            key => {
                if key_code.is_some() {
                    return Err(anyhow::anyhow!("Multiple keys specified in hotkey: {}", hotkey_str));
                }
                key_code = Some(parse_key_code(key)?);
            }
        }
    }

    let code = key_code.ok_or_else(|| anyhow::anyhow!("No key specified in hotkey: {}", hotkey_str))?;

    Ok(HotKey::new(Some(modifiers), code))
}

fn parse_key_code(key: &str) -> Result<global_hotkey::hotkey::Code> {
    use global_hotkey::hotkey::Code;

    let code = match key {
        // Letters
        "a" => Code::KeyA,
        "b" => Code::KeyB,
        "c" => Code::KeyC,
        "d" => Code::KeyD,
        "e" => Code::KeyE,
        "f" => Code::KeyF,
        "g" => Code::KeyG,
        "h" => Code::KeyH,
        "i" => Code::KeyI,
        "j" => Code::KeyJ,
        "k" => Code::KeyK,
        "l" => Code::KeyL,
        "m" => Code::KeyM,
        "n" => Code::KeyN,
        "o" => Code::KeyO,
        "p" => Code::KeyP,
        "q" => Code::KeyQ,
        "r" => Code::KeyR,
        "s" => Code::KeyS,
        "t" => Code::KeyT,
        "u" => Code::KeyU,
        "v" => Code::KeyV,
        "w" => Code::KeyW,
        "x" => Code::KeyX,
        "y" => Code::KeyY,
        "z" => Code::KeyZ,
        
        // Numbers
        "0" => Code::Digit0,
        "1" => Code::Digit1,
        "2" => Code::Digit2,
        "3" => Code::Digit3,
        "4" => Code::Digit4,
        "5" => Code::Digit5,
        "6" => Code::Digit6,
        "7" => Code::Digit7,
        "8" => Code::Digit8,
        "9" => Code::Digit9,
        
        // Function keys
        "f1" => Code::F1,
        "f2" => Code::F2,
        "f3" => Code::F3,
        "f4" => Code::F4,
        "f5" => Code::F5,
        "f6" => Code::F6,
        "f7" => Code::F7,
        "f8" => Code::F8,
        "f9" => Code::F9,
        "f10" => Code::F10,
        "f11" => Code::F11,
        "f12" => Code::F12,
        
        // Special keys
        "space" => Code::Space,
        "enter" | "return" => Code::Enter,
        "tab" => Code::Tab,
        "escape" | "esc" => Code::Escape,
        "backspace" => Code::Backspace,
        "delete" => Code::Delete,
        "insert" => Code::Insert,
        "home" => Code::Home,
        "end" => Code::End,
        "pageup" => Code::PageUp,
        "pagedown" => Code::PageDown,
        
        // Arrow keys
        "up" | "arrowup" => Code::ArrowUp,
        "down" | "arrowdown" => Code::ArrowDown,
        "left" | "arrowleft" => Code::ArrowLeft,
        "right" | "arrowright" => Code::ArrowRight,
        
        _ => return Err(anyhow::anyhow!("Unsupported key: {}", key)),
    };

    Ok(code)
}

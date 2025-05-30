# 🚀 Feature: Global Hotkey Support for Pause/Resume Functionality

## 📋 Description

This PR implements system-wide global hotkey functionality that allows users to pause and resume automation regardless of which window has focus. Previously, the pause feature only worked when the terminal window was active, significantly limiting its usefulness.

## 🎯 Problem Solved

**Before**: Users could only pause automation using Ctrl+C when the terminal window was focused, making it difficult to quickly pause automation when working in other applications.

**After**: Users can now pause/resume automation globally using a configurable hotkey combination (default: `Ctrl+Alt+R`) that works regardless of window focus.

## ✨ Key Features

- **🌐 Global Hotkey Support**: Works system-wide, not limited to terminal focus
- **🔄 Toggle Functionality**: Single hotkey toggles between pause and resume states
- **⚙️ Configurable**: Hotkey combination can be customized via configuration
- **🎛️ Visual Feedback**: Clear console messages when pausing/resuming
- **🔧 Cross-Platform**: Supports Windows and Linux (with X11)
- **⚡ Async Integration**: Seamlessly integrated with existing tokio-based architecture

## 🔧 Technical Implementation

### New Components

1. **`src/global_hotkey.rs`**: New module containing:
   - `HotkeyManager` struct for managing global hotkeys
   - Hotkey parsing and registration functionality
   - Asynchronous pause state management using `tokio::sync::watch`
   - Support for various key combinations

2. **Updated Dependencies**:
   - Added `global-hotkey = "0.5"` for cross-platform hotkey support

3. **Integration Points**:
   - Modified `main.rs` to initialize and integrate hotkey manager
   - Updated automation loops to respect pause state
   - Enhanced startup information display

### Supported Hotkey Formats

- `ctrl+alt+r` (default)
- `ctrl+shift+p`
- `alt+f1`
- `ctrl+alt+space`
- And many more combinations...

## 📦 Dependencies

### New Dependency
```toml
global-hotkey = "0.5"
```

### System Requirements (Linux)
```bash
# Ubuntu/Debian
sudo apt-get install libx11-dev libxtst-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev

# Fedora/RHEL
sudo dnf install libX11-devel libXtst-devel libXrandr-devel libXinerama-devel libXcursor-devel libXi-devel
```

## 🔄 Usage Examples

### Configuration File
```json
{
  "process_name": "notepad.exe",
  "pause_hotkey": "ctrl+alt+r",
  "key_sequence": [
    {
      "key": "space",
      "interval_after": "1000ms"
    }
  ]
}
```

### Command Line
```bash
# Use default pause hotkey (ctrl+alt+r)
pks --process notepad.exe --key space --interval 1000ms

# Display startup info showing pause hotkey
🚀 Process Key Sender v0.1.1
═══════════════════════════════════════════
🎯 Target Process: notepad.exe
⏸ Pause Hotkey: ctrl+alt+r
📝 Verbose Mode: OFF
═══════════════════════════════════════════
⏸ Press ctrl+alt+r to pause/resume globally
ℹ Press Ctrl+C to stop
```

## 🧪 Testing

### Manual Testing Performed
1. ✅ Hotkey registration and initialization
2. ✅ Global hotkey detection across different applications
3. ✅ Pause/resume functionality in both automation modes
4. ✅ Visual feedback and console output
5. ✅ Cross-platform compilation (Windows/Linux)

### Test Scenarios
- **Scenario 1**: Start automation, switch to browser, press hotkey → ✅ Pauses
- **Scenario 2**: While paused, press hotkey again → ✅ Resumes
- **Scenario 3**: Hotkey works while in different applications → ✅ Works
- **Scenario 4**: Ctrl+C still terminates application → ✅ Works

## 🔒 Breaking Changes

**None** - This is a purely additive feature that maintains full backward compatibility.

## 📋 Checklist

- [x] Code follows project coding standards
- [x] Self-review completed
- [x] Functionality tested manually
- [x] Documentation updated
- [x] Backward compatibility maintained
- [x] Error handling implemented
- [x] Cross-platform support verified

## 🎛️ Configuration Schema

The pause hotkey can be configured in the JSON configuration file:

```json
{
  "pause_hotkey": "ctrl+alt+r"  // Default value
}
```

Supported modifiers: `ctrl`, `alt`, `shift`, `meta`/`cmd`/`super`
Supported keys: Letters, numbers, function keys, arrow keys, space, enter, etc.

## 🚀 Future Enhancements

- [ ] Support for multiple hotkeys with different actions
- [ ] Hotkey customization via command line arguments
- [ ] macOS support (currently Windows/Linux only)
- [ ] Configuration validation and error recovery

## 📝 Related Issues

This PR addresses the core limitation mentioned in user feedback where pause functionality was not accessible when working in other applications.

---

**Ready for Review** 🎉

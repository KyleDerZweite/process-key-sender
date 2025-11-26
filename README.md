# Process Key Sender (pks)

A cross-platform command-line tool for sending keystrokes to specific processes at configurable intervals. Perfect for gaming automation, testing, or any scenario where you need to send repeated keystrokes to a specific application.

## ⚠️ **IMPORTANT DISCLAIMER**

**This tool is intended for educational purposes and use with offline/single-player games only.**

### **ANTI-CHEAT WARNING**
- **DO NOT use this tool with online multiplayer games**
- **DO NOT use this tool with games that have anti-cheat systems** (EasyAntiCheat, BattlEye, Vanguard, etc.)
- **Using automation tools in online games may result in:**
  - Permanent account bans
  - Hardware ID bans
  - Violation of Terms of Service
  - Loss of game progress and purchases

### **Safe Use Cases**
- Single-player offline games
- Local development and testing
- Automation of non-gaming applications
- Educational and research purposes
- Personal productivity tools

### **Gaming Guidelines**
- Always check the game's Terms of Service before use
- Only use with games that explicitly allow automation tools
- Prefer official game features (auto-run, key rebinding) when available
- Use responsibly and ethically

**The developers of this tool are not responsible for any consequences resulting from its misuse. Users assume all risks and responsibilities when using this software.**

---

## Features

- **Target specific processes** - Send keys only to the process you specify
- **Flexible key support** - Support for letters, special keys, and key combinations
- **Key sequences** - Send multiple keys with different intervals
- **Configurable intervals** - Set custom delays between keystrokes
- **Loop control** - Run sequences once or loop indefinitely
- **Global pause/resume** - System-wide hotkey support (works regardless of window focus)
- **Smart process detection** - Automatically finds and monitors target processes
- **Configuration files** - Save and load settings from JSON files
- **Colorized output** - Beautiful terminal interface with status indicators
- **Cross-platform** - Works on Windows and Linux

## Installation

### From Source
```bash
git clone https://github.com/KyleDerZweite/process-key-sender.git
cd process-key-sender
cargo build --release
```

### From Crates.io (coming soon)
```bash
cargo install process-key-sender
```

## Usage

### Basic Usage
```bash
# Send 'R' key to game.exe every 1000ms
pks --process game.exe

# Send 'F' key every 500ms
pks --process myapp.exe --key f --interval 500

# Enable verbose output
pks --process notepad.exe --key space --verbose
```

### Key Sequences
```bash
# Send multiple keys with different intervals
pks --process game.exe --sequence "r:1000,space:500,e:2000"

# Complex sequence for crafting in games
pks --process rpg.exe --sequence "e:500,tab:200,enter:300,escape:1000"

# Simple sequence with default intervals
pks --process app.exe --sequence "w,a,s,d"

# Run sequence only once (no looping)
pks --process game.exe --sequence "f1:1000,f2:1000" --loop-sequence=false

# Repeat sequence exactly 5 times
pks --process game.exe --sequence "r:2000,space:1000" --repeat-count 5
```

### Key Combinations
```bash
# Send Ctrl+C combination
pks --process editor.exe --key "ctrl+c" --interval 5000

# Send Alt+Tab
pks --process app.exe --key "alt+tab" --interval 3000

# Complex sequence with combinations
pks --process ide.exe --sequence "ctrl+s:2000,f5:1000,ctrl+shift+f10:5000"
```

### Advanced Usage
```bash
# Load configuration from file
pks --config my-config.json

# Save current settings to configuration file
pks --process game.exe --sequence "r:1000,e:500" --save-config my-config.json

# Maximum retries to find process
pks --process game.exe --max-retries 20
```

## Global Hotkey Control

The global hotkey feature allows you to pause and resume automation **regardless of which window has focus**. This is especially useful when you need to quickly pause automation while working in other applications.

### Default Hotkey
- **Default**: `Ctrl+Alt+R`
- **Action**: Toggle pause/resume
- **Scope**: System-wide (works in any application)

### Usage
1. Start automation with any command
2. Switch to any other application (browser, game, etc.)
3. Press `Ctrl+Alt+R` to pause automation
4. Press `Ctrl+Alt+R` again to resume
5. Visual feedback appears in the terminal

### Custom Hotkeys
```bash
# Set custom pause hotkey in configuration
{
  "process_name": "game.exe",
  "pause_hotkey": "ctrl+shift+p",
  "key_sequence": [...]
}
```

### Supported Key Combinations
```
Modifiers: ctrl, alt, shift, meta/cmd/super
Keys: a-z, 0-9, f1-f12, space, enter, arrow keys, etc.

Examples:
- ctrl+alt+r
- ctrl+shift+p  
- alt+f1
- ctrl+alt+space
```

### System Requirements (Linux)
```bash
# Ubuntu/Debian
sudo apt-get install libx11-dev libxtst-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev

# Fedora/RHEL
sudo dnf install libX11-devel libXtst-devel libXrandr-devel libXinerama-devel libXcursor-devel libXi-devel
```
```

### Command Line Options

```
Options:
  -p, --process <PROCESS>           Name of the target process (e.g., "game.exe", "notepad")
  -k, --key <KEY>                   Single key to send [default: r]
  -i, --interval <INTERVAL>         Interval between keystrokes in milliseconds [default: 1000]
  -s, --sequence <SEQUENCE>         Key sequence with custom intervals (e.g., "r:1000,space:500,e:2000")
  -r, --max-retries <MAX_RETRIES>   Maximum number of attempts to find the process [default: 10]
      --pause-hotkey <PAUSE_HOTKEY> Hotkey to pause/resume (e.g., "ctrl+alt+r")
      --loop-sequence               Loop the sequence indefinitely [default: true]
      --repeat-count <COUNT>        Number of times to repeat the sequence (0 = infinite) [default: 0]
  -v, --verbose                     Enable verbose output
  -c, --config <CONFIG>             Load configuration from file
      --save-config <SAVE_CONFIG>   Save current configuration to file
  -h, --help                        Print help
  -V, --version                     Print version
```

### Sequence Format

Key sequences use the format: `key1:interval1,key2:interval2,key3:interval3`

- **Key**: Any supported key (see supported keys below)
- **Interval**: Time to wait after pressing the key (in milliseconds)
- **Separator**: Use commas to separate key-interval pairs
- **Optional interval**: If no interval is specified, default is 1000ms

Examples:
```bash
# Basic sequence
"r,space,e"  # Each key waits 1000ms

# Custom intervals
"r:2000,space:500,e:1500"

# Mixed format
"r:2000,space,e:500"  # space uses default 1000ms

# Key combinations in sequence
"ctrl+c:1000,tab:500,enter:2000"
```

### Supported Keys

#### Letters and Numbers
- **Letters**: `a-z` (case insensitive)
- **Numbers**: `0-9`

#### Special Keys
- **Space**: `space`
- **Enter**: `enter`, `return`
- **Tab**: `tab`
- **Escape**: `escape`, `esc`
- **Backspace**: `backspace`
- **Delete**: `delete`

#### Arrow Keys
- **Arrows**: `left`, `right`, `up`, `down`

#### Navigation Keys
- **Navigation**: `home`, `end`, `pageup`, `pagedown`

#### Function Keys
- **Function**: `f1` through `f12`

#### Modifier Keys
- **Modifiers**: `shift`, `ctrl`/`control`, `alt`

#### Key Combinations
- **Format**: `modifier+key` (e.g., `ctrl+c`, `alt+tab`, `ctrl+shift+s`)
- **Multiple modifiers**: `ctrl+shift+f10`

## Configuration Files

You can save and load configurations using JSON files:

```json
{
  "process_name": "game.exe",
  "key_sequence": [
    {
      "key": "r",
      "interval_after": "1000ms"
    },
    {
      "key": "space",
      "interval_after": "500ms"
    },
    {
      "key": "e",
      "interval_after": "2000ms"
    }
  ],
  "max_retries": 10,
  "pause_hotkey": "ctrl+alt+r",
  "verbose": false,
  "loop_sequence": true,
  "repeat_count": 0
}
```

## Examples

### Safe Gaming Automation (Offline/Single-player only)

#### Auto-reload and healing sequence
```bash
# Reload, wait, use health potion, wait longer
pks --process single-player-fps.exe --sequence "r:1500,h:3000"
```

#### Complex crafting sequence
```bash
# Open inventory, navigate, craft, close
pks --process rpg.exe --sequence "i:500,tab:200,tab:200,space:300,escape:1000"
```

#### Farming sequence with movement
```bash
# Move forward, interact, wait for animation, move back
pks --process farming-sim.exe --sequence "w:1000,e:2000,s:1000" --repeat-count 10
```

#### Auto-save sequence
```bash
# Regular auto-save in single-player game
pks --process game.exe --sequence "f5:30000" --verbose
```

### Development Testing

#### Web development refresh cycle
```bash
# Save, switch to browser, refresh, switch back
pks --process vscode.exe --sequence "ctrl+s:500,alt+tab:200,f5:1000,alt+tab:500"
```

#### Build and test cycle
```bash
# Build, wait for completion, run tests
pks --process terminal.exe --sequence "ctrl+c:100,up:100,enter:5000,ctrl+c:100"
```

### Productivity Automation

#### Presentation auto-advance
```bash
# Auto-advance slides every 10 seconds
pks --process powerpoint.exe --key "right" --interval 10000
```

#### Document review cycle
```bash
# Page down, wait to read, repeat
pks --process document-viewer.exe --sequence "pagedown:5000" --repeat-count 20
```

### Accessibility Assistance

#### Regular interaction for users with limited mobility
```bash
# Gentle, regular interaction to keep applications active
pks --process app.exe --sequence "shift:30000" --verbose
```

## Platform Support

- **Windows** - Full support using Windows API
- **Linux** - Basic support (X11 implementation in progress)
- **macOS** - Planned support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup
```bash
git clone https://github.com/KyleDerZweite/process-key-sender.git
cd process-key-sender
cargo build
cargo test
```

### TODO
- [ ] Complete Linux/X11 implementation
- [ ] Add macOS support
- [x] Global hotkey implementation for pause/resume
- [ ] GUI version
- [ ] More key types and special characters
- [ ] Process window title filtering
- [ ] Randomized intervals within ranges
- [ ] Conditional sequences based on screen colors/patterns
- [ ] Recording and playback of manual key sequences

## Ethical Usage Guidelines

### ✅ **Recommended Uses**
- Educational programming projects
- Accessibility tools for users with disabilities
- Testing and quality assurance
- Personal productivity automation
- Single-player game convenience features
- Development and debugging tools

### ❌ **Prohibited Uses**
- Gaining unfair advantages in competitive online games
- Violating Terms of Service of any software
- Circumventing game mechanics in multiplayer environments
- Any form of cheating in online games
- Commercial exploitation without proper licensing

## Legal Considerations

- This software is provided for educational and legitimate automation purposes
- Users are responsible for compliance with all applicable laws and regulations
- Users must respect the Terms of Service of all software they interact with
- The authors disclaim responsibility for any misuse of this software

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust 🦀
- Uses `clap` for CLI parsing
- Uses `sysinfo` for cross-platform process detection
- Windows implementation uses `winapi`
- Terminal colors by `colored`

## Author

**KyleDerZweite** - [GitHub](https://github.com/KyleDerZweite)

**Remember: Use responsibly and ethically. When in doubt, don't use it with online games!**
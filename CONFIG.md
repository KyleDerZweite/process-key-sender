# Configuration File Guide

Process Key Sender uses JSON configuration files to define automation behavior. This guide explains all available options.

## 📁 File Format

Configuration files use JSON format with the following structure:

```json
{
  "process_name": "target-process.exe",
  "key_sequence": [...],
  "independent_keys": [...],
  "max_retries": 10,
  "pause_hotkey": "ctrl+alt+r",
  "verbose": true,
  "loop_sequence": true,
  "repeat_count": 0
}
```

## 📂 Example Configurations

Ready-to-use example configuration files are available in the `example-configs/` directory:

* `config.json` - Basic independent keys setup
* `single-key-config.json` - Simple single key automation
* `sequence-config.json` - Sequential key presses
* `advanced-config.json` - Advanced multi-key automation

### Using Example Configs

```bash
# Use an example directly
pks --config example-configs/config.json

# Copy and customize
cp example-configs/config.json my-custom-config.json
# Edit my-custom-config.json with your settings
pks --config my-custom-config.json
```

## ⚙️ Configuration Options

### `process_name` (required)

* **Type:** String
* **Description:** Name of the target process (with or without `.exe`)
* **Examples:**

    * "notepad.exe"
    * "MyGame"
    * "productivity-app.exe"

### `key_sequence` (optional)

* **Type:** Array of key actions
* **Description:** Keys sent in sequence, one after another
* **Format:** Each action has `"key"` and `"interval_after"`
* **Example:**

```json
"key_sequence": [
  {"key": "1", "interval_after": "1000ms"},
  {"key": "2", "interval_after": "500ms"},
  {"key": "space", "interval_after": "2000ms"}
]
```

### `independent_keys` (optional)

* **Type:** Array of independent key timers
* **Description:** Keys sent simultaneously on separate timers
* **Format:** Each key has `"key"` and `"interval"`
* **Example:**

```json
"independent_keys": [
  {"key": "r", "interval": "1000ms"},
  {"key": "a", "interval": "5000ms"}
]
```

### `max_retries` (optional)

* **Type:** Number
* **Default:** 10
* **Description:** Maximum attempts to find the target process
* **Range:** 1-100

### `pause_hotkey` (optional)

* **Type:** String
* **Default:** "ctrl+alt+r"
* **Description:** Global hotkey to pause/resume (not yet implemented)
* **Examples:** "ctrl+alt+p", "shift+f12", "ctrl+shift+space"

### `verbose` (optional)

* **Type:** Boolean
* **Default:** false
* **Description:** Enable detailed output showing each key press

### `loop_sequence` (optional)

* **Type:** Boolean
* **Default:** true
* **Description:** Whether to repeat the key sequence indefinitely

### `repeat_count` (optional)

* **Type:** Number
* **Default:** 0 (infinite)
* **Description:** Number of times to repeat the sequence (0 = infinite)

## 🎹 Supported Keys

### Letter Keys

* "a" through "z" (case insensitive)

### Number Keys

* "0" through "9"

### Special Keys

* "space" - Space bar
* "enter" or "return" - Enter key
* "tab" - Tab key
* "escape" or "esc" - Escape key
* "backspace" - Backspace key
* "delete" - Delete key

### Arrow Keys

* "left", "right", "up", "down"

### Function Keys

* "f1" through "f12"

### Modifier Keys

* "shift", "ctrl" or "control", "alt"

### Navigation Keys

* "home", "end", "pageup", "pagedown"

### Key Combinations

Use `+` to combine keys:

* "ctrl+c"
* "alt+tab"
* "ctrl+shift+s"

## ⏱️ Time Formats

* **Milliseconds:** "1000ms" or "1000"
* **Seconds:** "1s" (converts to 1000ms)
* **Minutes:** "1m" (converts to 60000ms)

## 🎯 Usage Modes

### Mode 1: Single Key

Send one key repeatedly:

```json
{
  "process_name": "app.exe",
  "key_sequence": [
    {"key": "space", "interval_after": "1000ms"}
  ]
}
```

### Mode 2: Key Sequence

Send keys in order, then repeat:

```json
{
  "process_name": "app.exe",
  "key_sequence": [
    {"key": "1", "interval_after": "500ms"},
    {"key": "2", "interval_after": "500ms"},
    {"key": "3", "interval_after": "1000ms"}
  ]
}
```

### Mode 3: Independent Keys

Send multiple keys simultaneously on different timers:

```json
{
  "process_name": "app.exe",
  "independent_keys": [
    {"key": "r", "interval": "1000ms"},
    {"key": "a", "interval": "5000ms"},
    {"key": "ctrl+s", "interval": "30000ms"}
  ]
}
```

## 📝 Example Use Cases

### Accessibility Tool

Auto-save functionality for users with limited mobility:

```json
{
  "process_name": "text-editor.exe",
  "independent_keys": [
    {"key": "ctrl+s", "interval": "30000ms"}
  ],
  "verbose": false
}
```

### Productivity Automation

Automate repetitive data entry tasks:

```json
{
  "process_name": "data-entry.exe",
  "key_sequence": [
    {"key": "tab", "interval_after": "100ms"},
    {"key": "enter", "interval_after": "500ms"}
  ],
  "repeat_count": 50
}
```

### Development Testing

Automated application testing:

```json
{
  "process_name": "test-app.exe",
  "independent_keys": [
    {"key": "f5", "interval": "5000ms"},
    {"key": "ctrl+r", "interval": "10000ms"}
  ],
  "verbose": true
}
```

## 🔧 Loading Configurations

```bash
# Use example configuration
pks --config example-configs/config.json

# Load custom configuration
pks --config my-config.json

# Override specific settings
pks --config my-config.json --verbose --process "different-app.exe"

# Save current CLI args to config
pks --process "app.exe" --key "space" --save-config my-new-config.json
```

## 🚨 Important Notes

* Only one mode at a time: Use either `key_sequence` **or** `independent_keys`, not both
* Process names: Include `.exe` for Windows processes
* Key case: Key names are case-insensitive
* Interval limits: Minimum recommended interval is 50ms
* Ethical usage: Only use with applications you own or have permission to automate

## ❗ Troubleshooting

### Common Issues

* **Process not found:** Check exact process name with Task Manager
* **Keys not working:** Ensure target application has focus
* **JSON errors:** Validate JSON syntax with online tools
* **Permission errors:** Run as administrator if needed (Windows)

### Validation

The tool will validate your configuration and display helpful error messages for:

* Missing required fields
* Invalid key names
* Malformed time intervals
* JSON syntax errors

## 📁 Directory Structure

```
process-key-sender/
├── example-configs/          # Ready-to-use example configurations
│   ├── config.json
│   ├── single-key-config.json
│   ├── sequence-config.json
│   └── advanced-config.json
├── CONFIG.md                 # This configuration guide
├── README.md                 # Main project documentation
└── target/release/pks.exe    # Compiled executable
```

For more examples and advanced configurations, check the `example-configs/` directory!

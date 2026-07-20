# Process Key Sender — Send Keystrokes to a Target Application

[![CI](https://github.com/KyleDerZweite/process-key-sender/actions/workflows/ci.yml/badge.svg)](https://github.com/KyleDerZweite/process-key-sender/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/KyleDerZweite/process-key-sender)](LICENSE)
[![Release](https://img.shields.io/github/v/release/KyleDerZweite/process-key-sender?display_name=tag)](https://github.com/KyleDerZweite/process-key-sender/releases/tag/v0.2.1)

`pks` is a Windows keystroke automation CLI that finds a target application by process name and sends it single keys, keyboard shortcuts, or timed key sequences. Use the command line for simple repeating keys or JSON config files for sequences and independent timers.

## Safety

Use this tool only for legitimate local automation, accessibility, testing, and offline applications. Do not use it with online games, anti-cheat protected software, or services that prohibit automation.

## Platform Status

- Windows: supported target platform for process lookup and key sending.
- Unix/Linux: config parsing, validation, and most tests work, but key sending is not implemented.

More detail: [docs/platform-support.md](docs/platform-support.md)

## Supported Keyboard Input

- Letters, numbers, arrows, navigation keys, and `F1`–`F12`
- Special keys such as `space`, `enter`, `tab`, `escape`, and `delete`
- Shortcuts and key combinations such as `ctrl+c`, `alt+tab`, and `ctrl+shift+s`
- Timed sequences and independently repeating keys

On Windows, `pks` finds the chosen application's visible window, focuses it, sends the keyboard input through the Windows `SendInput` API, and can restore the previously focused window afterward.

## Build

```bash
git clone https://github.com/KyleDerZweite/process-key-sender.git
cd process-key-sender
cargo build --release
```

## Quick Start

Run a single key on an interval:

```bash
cargo run -- --process notepad.exe --key space --interval 1000ms
```

Run from a config file:

```bash
cargo run -- --config examples/configs/sequence-config.json
```

Save CLI arguments as a config file:

```bash
cargo run -- --process notepad.exe --key space --save-config my-config.json
```

## Documentation

- CLI reference: [docs/cli.md](docs/cli.md)
- Configuration guide: [docs/configuration.md](docs/configuration.md)
- Platform support: [docs/platform-support.md](docs/platform-support.md)

## Contributing

Development setup, verification commands, and contribution guidelines are in [CONTRIBUTING.md](CONTRIBUTING.md).

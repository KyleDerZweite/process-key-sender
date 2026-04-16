# Process Key Sender

[![CI](https://github.com/KyleDerZweite/process-key-sender/actions/workflows/ci.yml/badge.svg)](https://github.com/KyleDerZweite/process-key-sender/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/KyleDerZweite/process-key-sender)](LICENSE)
[![Release](https://img.shields.io/github/v/release/KyleDerZweite/process-key-sender?display_name=tag)](https://github.com/KyleDerZweite/process-key-sender/releases/tag/v0.2.1)

`pks` is a Rust CLI for sending keystrokes to a target process on a schedule. It supports quick single-key automation from the command line and richer sequence or timer setups through JSON config files.

## Safety

Use this tool only for legitimate local automation, accessibility, testing, and offline applications. Do not use it with online games, anti-cheat protected software, or services that prohibit automation.

## Platform Status

- Windows: supported target platform for process lookup and key sending.
- Unix/Linux: config parsing, validation, and most tests work, but key sending is not implemented.

More detail: [docs/platform-support.md](docs/platform-support.md)

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

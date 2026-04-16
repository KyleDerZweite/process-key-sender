# Contributing to Process Key Sender

## Project Scope

This project is for legitimate local automation, accessibility, testing, and offline applications. Contributions that target cheating, anti-cheat bypasses, or terms-of-service violations are out of scope.

## Setup

Prerequisites:

- Rust 1.85 or newer
- Git
- Windows for end-to-end key sending validation
- Linux is fine for general development and most tests

```bash
git clone https://github.com/KyleDerZweite/process-key-sender.git
cd process-key-sender
cargo build
```

Run an example config:

```bash
cargo run -- --config examples/configs/config.json
```

## Verification

Run these before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo test --doc
cargo build --release --locked
cargo deny check
```

## Change Expectations

- Keep CLI flags and config field names backward compatible in `0.2.x`.
- Add tests with functional changes.
- Update docs when behavior changes.
- Prefer targeted refactors over large rewrites.
- Use clear commit messages such as `fix(config): validate key combinations on unix`.

## Pull Requests

Include:

- a short summary of the change,
- testing performed,
- any platform limitations or follow-up risks.

## Reporting Bugs

When opening an issue, include:

- operating system and version,
- `rustc --version`,
- `pks --version`,
- the command or config file used,
- the observed error message or output.

## License and Conduct

By contributing, you agree that your work will be released under the MIT License. Participation in the project is governed by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

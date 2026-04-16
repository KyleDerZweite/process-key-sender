# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-04-16

### Changed
- Split the binary entrypoint into dedicated CLI and runner modules
- Reorganized repository docs into `docs/` and moved example configs to `examples/configs/`
- Simplified the README and aligned contributor and security documentation with current behavior
- Streamlined CI into a smaller quality/build workflow and refreshed Dependabot grouping

### Fixed
- Removed duplicate module wiring in `src/main.rs`, eliminating double compilation of internal modules
- Made `restore_focus` an active runtime option instead of dead configuration
- Tightened key validation so unsupported keys fail consistently on Unix and Windows
- Fixed `.gitignore` so tracked example configs remain versioned and `Cargo.lock` stays reproducible

## [0.2.0] - 2025-11-26

### Added
- GitHub Actions CI workflow (test, clippy, fmt, build, cargo-deny)
- Custom error types with `thiserror` for better error handling
- Tracing infrastructure for structured logging
- Comprehensive rustdoc documentation for all public APIs
- Additional unit and integration tests (35 total tests)
- `Default` trait implementation for `ProcessFinder`

### Changed
- Updated `sysinfo` to 0.37 API (breaking change fix)
- Fixed Cargo.toml edition from invalid "2024" to "2021"
- Removed duplicate `parse_duration` function (now only in config module)
- Improved code formatting and clippy compliance

### Fixed
- Compilation errors with `sysinfo` 0.37 API changes
- `process.name()` now properly converts `OsStr` to string
- Fixed clippy warnings for `for_kv_map` and `redundant_pattern_matching`

## [0.1.1] - 2025-05-29

### Added
- Global hotkey support for pause/resume functionality
- `global-hotkey` crate integration

## [0.1.0] - 2025-05-29

### Added
- Initial release of Process Key Sender
- Cross-platform keystroke automation for specific processes
- Support for single keys, key sequences, and independent key timers
- Configuration file support (JSON format)
- Comprehensive CLI interface with clap
- Process detection and monitoring
- Safety disclaimers and ethical usage guidelines
- Support for key combinations (Ctrl+C, Alt+Tab, etc.)
- Verbose logging and colored terminal output
- Windows implementation with winapi
- Example configuration files

### Features
- **Independent Key Mode**: Send multiple keys on different timers simultaneously
- **Sequential Mode**: Send keys in a specific sequence with custom intervals
- **Process Targeting**: Automatically find and target specific processes
- **Configuration Files**: Save and load settings from JSON files
- **Cross-platform**: Windows support (Linux planned)

[Unreleased]: https://github.com/KyleDerZweite/process-key-sender/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/KyleDerZweite/process-key-sender/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/KyleDerZweite/process-key-sender/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/KyleDerZweite/process-key-sender/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/KyleDerZweite/process-key-sender/releases/tag/v0.1.0

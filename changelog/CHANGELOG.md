# Changelog

All notable changes to the onebox-rs project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Project foundation and scaffolding
- Basic CLI framework for client and server
- Configuration system with TOML support
- Logging infrastructure with configurable levels
- Error handling with anyhow

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2025-09-02

### Added
- Initial project structure with Cargo workspace
- `onebox-core` library crate with configuration and error handling
- `onebox-client` binary with CLI interface
- `onebox-server` binary with CLI interface
- Basic configuration system using serde
- CLI framework using clap
- Logging system using tracing
- Error handling using anyhow
- Async runtime support using tokio

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## Version History

- [0.1.0](./0.1.0.md) - Initial release with project foundation
- [Unreleased](./UNRELEASED.md) - Upcoming changes

## Changelog Format

Each version follows the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format:

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** in case of vulnerabilities

## Links

- [Product Requirements Document](../docs/PRD.md)
- [Software Requirements Specification](../docs/SRS.md)
- [Implementation Tasks](../docs/TASKS.md)
- [Test Plan](../docs/TEST_PLAN.md)
- [Test Execution Results](../docs/TEST_EXECUTION.md)

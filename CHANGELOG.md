# Changelog

All notable changes to the Rest element are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.6.0] — 2026 Q2

### Added

- Comprehensive README overhaul with architecture details
- Benefit-focused crate-level rustdoc in Library.rs
- See Also section linking to architecture overview and related Elements

### Changed

- Separated binary entry point from library code
- Standardized test file formatting
- Made shell scripts POSIX-compliant
- Standardized quote style and import ordering
- Rebuilt bin/Rest binary artifact

## [0.5.0] — 2026 Q1

### Added

- Compile subcommand for standalone TypeScript compilation
- Phase 3 advanced build features (incremental builds, watch mode)
- Comprehensive test suite for OXC compiler
- Static class property transformation for VS Code compatibility

### Changed

- Migrated compiler backend from SWC to OXC ecosystem
- Updated to workspace dependencies
- Restructured binary entry point with dedicated main.rs
- Updated .gitignore to exclude build artifacts
- Removed ephemeral Target/ build artifacts from tracking

### Fixed

- Watch.rs: Used spawn_blocking for compilation inside watcher loop
- Compile.rs: Used spawn_blocking for CPU-bound compilation
- Made compile_file synchronous with std::fs to fix Send trait issues
- Fixed SWC.rs Program::mutate/process logic
- Fixed Sequential.rs and Parallel.rs GlobSet and Group usage
- Resolved various import and module declaration warnings

## [0.4.0] — 2025 Q4

### Changed

- Updated dependencies

## [0.3.0] — 2025 Q3

### Changed

- Updated dependencies

## [0.2.0] — 2025 Q2

### Changed

- Updated contact email to Community@Editor.Land
- Updated dependencies

## [0.1.0] — 2025 Q1

### Changed

- Updated dependencies

## [0.0.1] — 2024 Q3

### Added

- Initial OXC-powered JavaScript bundler for VS Code platform code
- SWC compiler integration for TypeScript transformation
- Parallel and sequential build commands with GlobSet support
- File watcher with automatic recompilation
- CI/CD workflows with GitHub Actions
- Dependabot configuration for automated dependency updates

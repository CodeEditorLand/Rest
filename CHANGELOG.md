# Changelog

All notable changes to Rest (OXC JS Bundler) are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/).

## [v2.1] - Q2 2026: Full Workbench Lift

### Changed

- Test file formatting standardized (1,806 insertions/deletions)
- README overhauled (+174 lines)
- Binary.rs separated from Library.rs entry point
- Shell scripts made POSIX-compliant

## [v2.0] - Q1 2026: Editor Launch Sprint

### March 11: SWC → OXC Migration (Pivotal Commit)

43 files changed, 3,821 insertions, 2,659 deletions.

#### Added

- 7 new OXC modules in `Source/Fn/OXC/`:
  - `Compiler.rs` (332 lines) - main orchestration with memory-safe allocator
    scoping, metric tracking, compile ID logging
  - `Transformer.rs` (224 lines) - AST transformation (TypeScript strip, JSX)
  - `Compile.rs` (166 lines) - compilation entry point with file I/O
  - `Parser.rs` (154 lines) - ultra-fast TypeScript parsing, ESTree compat
  - `Codegen.rs` (142 lines) - code generation with source maps
  - `Watch.rs` (62 lines) - file system monitoring via `notify` crate
  - `mod.rs` (11 lines)

#### Removed

- SWC dependencies: swc_common, swc_ecma_ast, swc_ecma_parser,
  swc_ecma_transforms_base, swc_ecma_transforms_typescript,
  swc_ecma_codegen, swc_ecma_visit

#### Added (Dependencies)

- OXC crates: oxc_allocator, oxc_parser, oxc_transformer, oxc_minifier,
  oxc_codegen, oxc_semantic, oxc_span, oxc_ast

### March 13-14: Test Suite

#### Added

- 6 test files (~3,800 lines):
  - `tests/unit/codegen_tests.rs` (1,224 lines)
  - `tests/unit/parser_tests.rs` (1,057 lines)
  - `tests/unit/transformer_tests.rs` (891 lines)
  - `tests/integration/vscode_compatibility.rs` (427 lines)
  - `tests/unit/oxc_compiler.rs` (293 lines)
  - `tests/lib.rs` (15 lines)

### Critical Fix: OXC Allocator Lifetime

Problem: allocator dropped prematurely during code generation (use-after-free).
Solution: entire Parse → Transform → Codegen pipeline scoped in single block,
allocator lifetime extends through all stages.

## [v1.3] - Q4 2025: Dependency Maintenance

### Changed

- Dependency updates maintained via Dependabot
- No source changes

## [v1.2] - Q3 2025: Full Stack Integration

### Changed

- Dependency bumps continued
- Architecture stable

## [v1.1] - Q2 2025: Architecture Buildout

### Changed

- Incremental improvements and testing
- 105 commits, mostly sync operations

## [v1.0] - Q1 2025: Integration Phase

### Changed

- Bug fixes, sync operations
- 104 commits

## [v0.2] - Q4 2024: Architecture Solidification

### Changed

- Stabilization: 42 commits
- Dependency management
- License work

## [v0.1] - Q3 2024: Rapid Development

**Created September 14, 2024** (commit 31c95a1).

### Added

- Initial build: 33 files, 1,596 insertions
- `Source/Fn/Binary/Command.rs` - CLI argument orchestration
- `Source/Fn/Binary/Entry.rs` - command entry execution
- `Source/Fn/Binary/Parallel.rs` - Rayon-based parallel compilation
- `Source/Fn/Binary/Sequential.rs` - sequential fallback
- `Source/Fn/Build.rs` (87 lines) - high-level build coordination
- `Source/Fn/Bundle/` - ESBuild wrapper + config parsing (286+172+246 lines)
- `Source/Fn/NLS/` - i18n bundle/extract/replace (167+110+98 lines)
- `Source/Fn/SWC/` - SWC compiler integration (pre-OXC era)
- `Source/Fn/Transform/private_field.rs` - TypeScript AST transforms
- `Source/Fn/Worker/` - worker pool management (bootstrap, compile, detect)
- `Source/Struct/Binary/Command*.rs` - CLI structure definitions
- `Source/Struct/CompilerConfig.rs` (166 lines) - compiler configuration DTO
- Cargo.toml: Edition 2024 (nightly), outputs: staticlib + cdylib + rlib

### Dependencies (First Release)

- SWC ecosystem (swc_common, swc_ecma_*, swc_ecma_codegen, swc_ecma_visit)
- tokio, rayon, serde_json, tracing, git2, walkdir, notify, globset, dashmap

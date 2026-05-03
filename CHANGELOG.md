# Changelog - Rest

Rest is our JS bundler - the Rust crate that compiles VS Code platform source
into the bundled tree Cocoon and Sky load at runtime. We migrated from SWC to
OXC in March 2026 and have been tracking the OXC API ever since. This file
records what we built in our voice, version by version. Format adapted from
[Keep a Changelog](https://keepachangelog.com/).

## [v2.1] - Full Workbench Lift (April 2026)

We tracked OXC forward, separated the binary entry point cleanly, and brought
Rest in line with the project's naming rules.

### Added

- **CHANGELOG overhaul** with Keep-a-Changelog format and detailed versioning
  (`b751938`, 2026-04-17; `e7cbd0c`, 2026-04-16).
- **Comprehensive README overhaul** with architecture details (`9000a3a`,
  2026-04-05) and benefit-focused rewrites (`c3beceb`, `8949250`, 2026-04-04).
- **Crate-level rustdoc** rewritten benefit-first in `Library.rs` (`6db511a`,
  2026-04-04).

### Changed

- **OXC 0.127 API migration** for codegen and transform paths (`f0ddbf9`,
  2026-04-22). Rest now tracks the current OXC release.
- **Binary entry point switched from `Library.rs` to `Main.rs`** (`cf44ea7`,
  2026-04-27) - completes the binary/library separation started in `34e61fc`
  (2026-04-04).
- **Modules and files renamed to PascalCase** (`053dc5c`, 2026-04-17). The
  lingering `private_field` module also moved to PascalCase (`c9c42a2`).
- **Unused imports cleared**, test modules restructured (`2b0ca73`, 2026-04-18).
- **Test-file formatting standardised** (`f9d8ec0`, 2026-04-11).
- **Imports reordered**, quote style standardised (`f30f8ce`, 2026-04-06).
- **Shell scripts made POSIX-compliant** (`9132872`, 2026-04-06), redirection
  spacing and quote style standardised (`5c19e4f`, `cd100e9`).
- **Rayon bumped 1.11.0 → 1.12.0** (`f4d5f52`, 2026-04-22).

## [v2.0] - Editor Launch (OXC Migration, March 2026)

The pivotal cycle. Rest left SWC behind and rebuilt the compilation surface on
OXC.

### Added

- **OXC compiler ecosystem** as Rest's bundling foundation (`be30545`,
  2026-03-11). End of the SWC era.
- **Compile subcommand** for standalone TypeScript compilation (`789d672`,
  2026-03-05).
- **Phase 3 advanced build features** (`0289235`, 2026-03-05).
- **Static class-property transformation** + VS Code compatibility improvements
  (`bae0f1b`, 2026-03-14).
- **Comprehensive test suite for the OXC compiler** (`1e2f408`, 2026-03-13;
  expanded `40032e8`, 2026-03-14).

### Changed

- **Rebuilt binary** after OXC migration and the new compiler features
  (`150547b`, 2026-03-13).
- **Binary entry point separated from library code** (`34e61fc`, 2026-04-04).
  The split would later let us swap entry-point ownership cleanly in `cf44ea7`
  above.
- **Documentation moved** from inside the crate to `Documentation/Rest/`
  (`84d39ae`, 2026-03-13).
- **Workspace dependencies adopted** (`7ab9a14`, 2026-03-04; `7804a6e`,
  2026-01-30) so Rest stops carrying its own pinned versions.
- **Rust dependencies refreshed** (`a01cb9a`, 2026-02-27).
- **Workbench fix** (`90c1470`, 2026-02-14) - the consumer side in Mountain
  learned to drive Rest cleanly during the editor-launch boot path.

### Fixed (January 2026 Stability Sweep)

- **Send-trait issues** resolved: made `compile_file` synchronous using
  `std::fs` (`a47c3bb`, 2026-01-22).
- **`spawn_blocking` for CPU-bound compilation** inside the watcher loop and the
  compile path (`44e5470`, `7d974a8`, 2026-01-22).
- **`Source/Struct/SWC.rs`** - use `Program::mutate`/`process` logic, avoid
  `TsConfig` import via `Default` (`4479343`, 2026-01-22).
- **`Source/Fn/Binary/Command/{Sequential,Parallel}.rs`** - group usage logic,
  GlobSet integration (`feecf58`, `fc59b36`, 2026-01-22).
- **DashMap + Group module added to `Build.rs`** (`592b6ce`, 2026-01-22).
- **`Source/Fn/SWC/Watch.rs` + `Watch/Compile.rs`** - imports and notify usage
  corrected (`0c13500`, `4fd941a`, 2026-01-22).
- **`mut` removed from module declaration** (`e180ea2`, 2026-01-22), `Entry`
  renamed to `_Entry` to suppress unused-var warning (`77bad79`).

## [v1.x] - Maintenance Cycle (Q2 2025 - Q4 2025)

### Changed

- **Re-licensed to CC0 1.0** (`a06c20c`, 2025-05-22), then **migrated to Land
  Public License v1.0** (`90721bd`, 2025-05-10).
- **Documentation URLs normalised to lowercase** (`3931010`, 2025-05-20).
- **Continuous Dependabot rolls** through the year - no substantive source work
  between June 2025 and December 2025; the SWC bundler was stable and the OXC
  migration was scheduled for Q1 2026.

## [v1.0] - Integration Phase (Q1 2025)

Maintenance-heavy cycle - 152 commits across Q1 2025, almost entirely Dependabot
rolls. The single substantive change was a contact-email update (`af90c01`,
2025-03-21).

## [v0.2] - Architecture Solidification (Q4 2024)

The crate sat in stabilisation through October-December 2024. SWC bundler
surface mature; CI, README, license metadata firmed up. No substantive feature
work - the OXC pivot was still 14 months away.

## [v0.1] - Rapid Development (September - October 2024)

Initial scaffold and SWC integration. The early September commits (`31c95a1`,
`0699527`, `2b1d2ad`, `f299f47`, etc., 2024-09-14 through 2024-09-21)
established the bundler shape. Through this window Rest existed as a standalone
repository (`BinaryRest` per the first PR), before being absorbed into the Land
monorepo's Element layout.

## [v0.0] - Project Inception (September 2024)

First commit (`31c95a1`, 2024-09-14). Rest started life as the JS bundler we
needed to feed Output the VS Code platform code Cocoon would later consume. The
implementation rode SWC for the first seventeen months before the OXC migration
in v2.0.

//! SWC-compatible TypeScript compiler module (uses OXC backend).
//!
//! Provides the same CLI interface and original SWC compiler functionality
//! but uses OXC for parsing and transformation under the hood.
//!
//! ## Modules
//!
//! * [`Compile`] — Compilation entry point (OXC-backed).
//! * [`Watch`] — File watching and incremental recompilation.

pub mod Compile;

pub mod Watch;

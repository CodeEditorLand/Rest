//! # Struct
//!
//! Configuration structures, CLI types, and compiler configuration used
//! throughout the Rest build pipeline.
//!
//! Re-exports Binary command options, SWC compiler config, and advanced
//! CompilerConfig types.

pub mod Binary;

pub mod SWC;

// Phase 3 advanced features
pub mod CompilerConfig;

// Re-export key types for testing convenience
pub use SWC::{ModuleFormat, Option as BuildOption};
pub use Binary::Command::Option as CommandOption;
pub use CompilerConfig as AdvancedCompilerConfig;

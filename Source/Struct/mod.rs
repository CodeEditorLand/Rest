pub mod Binary;

pub mod SWC;

// Phase 3 advanced features
pub mod CompilerConfig;

// Re-export key types for testing convenience
pub use SWC::{CompilerConfig, ModuleFormat, Option as BuildOption};
pub use Binary::Command::Option as CommandOption;

pub mod Binary;
pub mod OXC;
pub mod SWC;

pub mod Build;

// Phase 3 advanced features
pub mod Transform;
pub mod NLS;
pub mod Worker;
pub mod Bundle;

// Re-export compiler for testing
pub use OXC::Compiler;

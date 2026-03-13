//! Test library for Rest compiler
//!
//! This module re-exports all necessary components for integration tests.

pub mod vscode_compatibility;

// Re-export compiler components for tests
pub use crate::{
    Fn::OXC::{self, Compiler},
    Struct::{CompilerConfig, SWC},
};

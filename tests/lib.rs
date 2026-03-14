//! Test library for Rest compiler
//!
//! This module re-exports all necessary components for integration tests.

pub mod vscode_compatibility;
pub mod unit;
pub mod unit::parser_tests;
pub mod unit::transformer_tests;
pub mod unit::codegen_tests;

// Re-export compiler components for tests
pub use crate::{
    Fn::OXC::{self, Compiler},
    Struct::{CompilerConfig, SWC},
};

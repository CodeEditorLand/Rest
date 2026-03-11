//! OXC-based TypeScript compiler module
//!
//! This module provides TypeScript to JavaScript compilation using the OXC
//! parser and transformer, replacing the previous SWC-based implementation.

pub mod Compiler;
pub mod Codegen;
pub mod Compile;
pub mod Parser;
pub mod Transformer;
pub mod Watch;

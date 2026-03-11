//! SWC-compatible TypeScript compiler module (using OXC backend)
//!
//! This module provides the same CLI interface and functionality as the
//! original SWC compiler but uses OXC for parsing and transformation under the
//! hood.

pub mod Compile;
pub mod Watch;

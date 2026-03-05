//! Custom AST transforms for VSCode compatibility
//! 
//! This module provides custom transforms that SWC doesn't include by default:
//! - Private field conversion (#field -> __field)
//! - Additional VSCode-specific transformations

pub mod private_field;

pub use private_field::PrivateFieldTransform;
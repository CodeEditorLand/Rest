//! Custom AST transforms for VSCode compatibility
//!
//! This module provides custom transforms that SWC doesn't include by default:
//! - Private field conversion (#field -> __field)
//! - Additional VSCode-specific transformations

#[path = "PrivateField.rs"]
pub mod PrivateField;

pub use PrivateField::PrivateFieldTransform;

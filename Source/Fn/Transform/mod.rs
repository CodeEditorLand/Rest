//! Custom AST transforms for VSCode compatibility.
//!
//! Provides custom transforms that SWC does not include by default:
//! - Private field conversion (`#field` → `__field`)
//! - Additional VSCode-specific transformations
//!
//! ## Modules
//!
//! * [`PrivateField`] — Converts TypeScript private fields to regular properties.

#[path = "PrivateField.rs"]
pub mod PrivateField;

pub use PrivateField::PrivateFieldTransform;

#![allow(
	non_snake_case,
	non_camel_case_types,
	non_upper_case_globals,
	dead_code,
	unused_imports,
	unused_variables,
	unused_assignments
)]

//! # Rest: JavaScript Bundler for VS Code Platform Code
//!
//! Rest compiles VS Code's TypeScript and JavaScript source into optimized
//! bundles that Cocoon can load at runtime. Uses OXC (Oxidation Compiler)
//! for native-speed parsing, transformation, and minification.
//!
//! ## Architecture
//!
//! See the [architecture documentation](https://github.com/editor-land/CodeEditorLand/blob/main/Documentation/GitHub/Architecture.md)
//! for the full architectural overview.
//!
//! ## What Rest Produces
//!
//! VS Code's source code is a massive TypeScript/JavaScript codebase. Rest
//! takes the platform code (workbench, services, platform layers) and bundles
//! it into self-contained modules that the Cocoon extension host can
//! `require()`.
//!
//! The output lands in the `Output` Element, ready for production use.
//!
//! ## Why Not Webpack or Esbuild
//!
//! Rest uses OXC because it runs at native speed as a Rust library. No Node.js
//! process is needed for bundling; the entire build pipeline stays in Rust.
//!
//! ## Modules
//!
//! - [`Fn`]: Core bundling functions and OXC integration
//! - [`Struct`]: Configuration structures and CLI command definitions

#[allow(dead_code)]
#[tokio::main]
/// Initialises the command structure and dispatches the asynchronous
/// command pipeline.
///
/// Sets up the CLI argument parser, resolves the subcommand (e.g.
/// `compile`), and runs the selected operation through the Rest build
/// pipeline.
///
/// # Panics
///
/// Does not panic.
///
/// # Example
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() { (Struct::Binary::Command::Struct::Fn().Fn)().await }
/// ```
async fn main() { (Struct::Binary::Command::Struct::Fn().Fn)().await }

/// Core bundling functions and OXC integration.
pub mod Fn;

/// Configuration structures and CLI command definitions.
pub mod Struct;

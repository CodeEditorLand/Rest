#![allow(non_snake_case)]

//! # Rest: JavaScript Bundler for VS Code Platform Code
//!
//! Rest compiles VS Code's TypeScript and JavaScript source into optimized
//! bundles that Cocoon can load at runtime. Built on OXC (Oxidation Compiler)
//! for native-speed parsing, transformation, and minification.
//!
//! ## What Rest Produces
//!
//! VS Code's source code is a massive TypeScript/JavaScript codebase. Rest
//! takes the platform code (workbench, services, platform layers) and bundles
//! it into self-contained modules that the Cocoon extension host can `require()`.
//!
//! The output lands in the `Output` Element, ready for production use.
//!
//! ## Why Not Webpack or esbuild
//!
//! Rest uses OXC because it runs at native speed as a Rust library. No Node.js
//! process needed for bundling. The entire build pipeline stays in Rust.
//!
//! ## Modules
//!
//! - [`Fn`]: Core bundling functions and OXC integration
//! - [`Struct`]: Configuration structures and CLI command definitions

#[allow(dead_code)]
#[tokio::main]
/// The main entry point for the application.
///
/// This function initializes the command structure and executes the
/// asynchronous function defined within it. The function is marked with the
/// `#[tokio::main]` attribute to enable asynchronous execution using the Tokio
/// runtime.
///
/// # Panics
///
/// This function does not panic.
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() { (Struct::Binary::Command::Struct::Fn().Fn)().await }

/// ```

pub mod Fn;

pub mod Struct;

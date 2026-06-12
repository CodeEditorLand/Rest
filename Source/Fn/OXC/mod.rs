//! OXC-based TypeScript compiler module.
//!
//! Provides TypeScript-to-JavaScript compilation using the OXC parser and
//! transformer, replacing the previous SWC-based implementation.
//!
//! ## Modules
//!
//! * [`Compiler`] — Compiler orchestration (parse → transform → codegen).
//! * [`Codegen`] — JavaScript code generation from AST.
//! * [`Compile`] — High-level compilation entry point.
//! * [`Parser`] — TypeScript source parsing.
//! * [`Transformer`] — TypeScript-to-JavaScript AST transformation.
//! * [`Watch`] — File watching and recompilation.

pub mod Compiler;

pub mod Codegen;

pub mod Compile;

pub mod Parser;

pub mod Transformer;

pub mod Watch;

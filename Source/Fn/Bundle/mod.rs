//! Bundling and esbuild integration
//!
//! This module provides:
//! - Simple bundling capabilities for the Rest compiler
//! - Optional esbuild integration for complex builds
//! - Multi-file compilation support

#[path = "Config.rs"]
pub mod Config;
#[path = "Builder.rs"]
pub mod Builder;
#[path = "ESBuild.rs"]
pub mod ESBuild;

pub use Config::BundleConfig;
pub use Builder::BundleBuilder;
pub use ESBuild::EsbuildWrapper;

/// Result of a bundling operation
#[derive(Debug, Clone)]
pub struct BundleResult {
	/// Path to the bundled output file
	pub output_path:String,
	/// Source map path (if generated)
	pub source_map_path:Option<String>,
	/// List of bundled files
	pub bundled_files:Vec<String>,
	/// Bundle hash for cache invalidation
	pub hash:String,
}

/// Represents a bundle entry (file to be bundled)
#[derive(Debug, Clone)]
pub struct BundleEntry {
	/// Path to the source file
	pub source:String,
	/// Module name (for ESM exports)
	pub module_name:Option<String>,
	/// Whether this is an entry point
	pub is_entry:bool,
}

impl BundleEntry {
	pub fn new(source:impl Into<String>) -> Self { Self { source:source.into(), module_name:None, is_entry:false } }

	pub fn entry(source:impl Into<String>) -> Self { Self { source:source.into(), module_name:None, is_entry:true } }

	pub fn with_module_name(mut self, name:impl Into<String>) -> Self {
		self.module_name = Some(name.into());
		self
	}
}

/// Bundling mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BundleMode {
	/// Single file compilation (current behavior)
	#[default]
	SingleFile,
	/// Bundle multiple files into one
	Bundle,
	/// Watch mode - rebuild on changes
	Watch,
	/// Build with esbuild (for complex cases)
	Esbuild,
}

impl BundleMode {
	pub fn requires_bundle(&self) -> bool {
		matches!(self, BundleMode::Bundle | BundleMode::Esbuild | BundleMode::Watch)
	}
}

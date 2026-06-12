//! Bundling and esbuild integration.
//!
//! Provides simple bundling capabilities for the Rest compiler, optional
//! esbuild integration for complex builds, and multi-file compilation support.
//!
//! ## Modules
//!
//! * [`Config`] — Bundle configuration options.
//! * [`Builder`] — Bundle builder orchestrating compilation.
//! * [`ESBuild`] — Optional esbuild wrapper for complex builds.

#[path = "Config.rs"]
pub mod Config;

#[path = "Builder.rs"]
pub mod Builder;

#[path = "ESBuild.rs"]
pub mod ESBuild;

pub use Config::BundleConfig;
pub use Builder::BundleBuilder;
pub use ESBuild::EsbuildWrapper;

/// Result of a bundling operation.
#[derive(Debug, Clone)]
pub struct BundleResult {
	/// Path to the bundled output file.
	pub output_path:String,

	/// Source map path (if generated).
	pub source_map_path:Option<String>,

	/// List of bundled files.
	pub bundled_files:Vec<String>,

	/// Bundle hash for cache invalidation.
	pub hash:String,
}

/// A bundle entry describing a file to be bundled.
#[derive(Debug, Clone)]
pub struct BundleEntry {
	/// Path to the source file.
	pub source:String,

	/// Module name (for ESM exports).
	pub module_name:Option<String>,

	/// Whether this is an entry point.
	pub is_entry:bool,
}

impl BundleEntry {
	/// Creates a new bundle entry with the given source path.
	pub fn new(source:impl Into<String>) -> Self { Self { source:source.into(), module_name:None, is_entry:false } }

	/// Creates a new entry-point bundle entry.
	pub fn entry(source:impl Into<String>) -> Self { Self { source:source.into(), module_name:None, is_entry:true } }

	/// Sets the module name for this bundle entry.
	pub fn with_module_name(mut self, name:impl Into<String>) -> Self {
		self.module_name = Some(name.into());

		self
	}
}

/// Bundling mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BundleMode {
	/// Single-file compilation (current default behavior).
	#[default]
	SingleFile,

	/// Bundle multiple files into one.
	Bundle,

	/// Watch mode — rebuild on changes.
	Watch,

	/// Build with esbuild (for complex cases).
	Esbuild,
}

impl BundleMode {
	/// Returns `true` if this mode requires actual bundling (is an entry point).
	pub fn requires_bundle(&self) -> bool {
		matches!(self, BundleMode::Bundle | BundleMode::Esbuild | BundleMode::Watch)
	}
}

//! Web Worker compilation support
//!
//! This module provides:
//! - Worker file detection and classification
//! - Worker bootstrap code generation
//! - Module handling for worker contexts

#[path = "Detect.rs"]
pub mod Detect;

#[path = "Bootstrap.rs"]
pub mod Bootstrap;

#[path = "Compile.rs"]
pub mod Compile;

pub use Detect::WorkerDetector;
pub use Bootstrap::WorkerBootstrap;
pub use Compile::WorkerCompiler;

/// Configuration for worker compilation
#[derive(Debug, Clone, Default)]
pub struct WorkerConfig {
	/// Enable worker compilation
	pub enabled:bool,

	/// Output directory for worker bundles
	pub output_dir:String,

	/// Whether to inline dependencies
	pub inline_dependencies:bool,

	/// Worker type: "classic" or "module"
	pub worker_type:WorkerType,

	/// Additional scripts to include in worker bootstrap
	pub bootstrap_scripts:Vec<String>,

	/// Whether to generate source maps for workers
	pub source_maps:bool,
}

impl WorkerConfig {
	/// Create a new [`WorkerConfig`] with default settings.
	pub fn new() -> Self {
		Self {
			enabled:true,

			output_dir:"out/workers".to_string(),

			inline_dependencies:true,

			worker_type:WorkerType::Module,

			bootstrap_scripts:Vec::new(),

			source_maps:true,
		}
	}
}

/// Type of web worker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorkerType {
	/// Classic worker (shared worker context)
	Classic,

	/// Module worker (ES modules)
	#[default]
	Module,
}

impl WorkerType {
	/// Return the worker type as a static string identifier.
	pub fn as_str(&self) -> &'static str {
		match self {
			WorkerType::Classic => "classic",

			WorkerType::Module => "module",
		}
	}
}

/// Information about a detected worker file
#[derive(Debug, Clone)]
pub struct WorkerInfo {
	/// Path to the worker source file
	pub source_path:String,

	/// Path to the output worker bundle
	pub output_path:String,

	/// Name of the worker (derived from filename)
	pub name:String,

	/// The type of worker
	pub worker_type:WorkerType,

	/// Dependencies that need to be bundled
	pub dependencies:Vec<String>,

	/// Whether this is a shared worker
	pub is_shared:bool,
}

impl WorkerInfo {
	/// Create a new [`WorkerInfo`] for the given source path and type.
	pub fn new(source_path:impl Into<String>, worker_type:WorkerType) -> Self {
		let source_path = source_path.into();

		let name = std::path::Path::new(&source_path)
			.file_stem()
			.and_then(|s| s.to_str())
			.unwrap_or("worker")
			.to_string();

		Self {
			source_path:source_path.clone(),

			output_path:source_path,

			name,

			worker_type,

			dependencies:Vec::new(),

			is_shared:false,
		}
	}
}

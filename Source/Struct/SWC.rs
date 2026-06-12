//! SWC-compatible compiler configuration and option types.
//!
//! Defines compiler configuration, option, and metrics structures used by the
//! Rest compiler's SWC-compatible (OXC-backed) pipeline.

/// Information about compiled files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
	path:PathBuf,

	last_modified:SystemTime,
}

/// Module format options for code generation.
///
/// Determines whether the compiler produces ESM (`import`/`export`),
/// CommonJS (`require`/`module.exports`), or IIFE bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ModuleFormat {
	/// CommonJS module format (default)
	#[default]
	CommonJs,

	/// ECMAScript Modules (ESM)
	EsModule,

	/// Asynchronous Module Definition (AMD)
	Amd,

	/// UMD (Universal Module Definition)
	Umd,

	/// No modules - preserve original imports/exports
	None,
}

impl ModuleFormat {
	/// Parses module format from string ("commonjs", "esmodule", "amd", "umd",
	/// "none").
	pub fn from_str(s:&str) -> Self {
		match s.to_lowercase().as_str() {
			"esmodule" | "esm" | "esnext" | "es" => ModuleFormat::EsModule,

			"amd" => ModuleFormat::Amd,

			"umd" => ModuleFormat::Umd,

			"none" => ModuleFormat::None,

			_ => ModuleFormat::CommonJs,
		}
	}

	/// Returns the module format as a lowercase string.
	pub fn as_str(&self) -> &'static str {
		match self {
			ModuleFormat::CommonJs => "commonjs",

			ModuleFormat::EsModule => "esmodule",

			ModuleFormat::Amd => "amd",

			ModuleFormat::Umd => "umd",

			ModuleFormat::None => "none",
		}
	}
}

/// SWC-compatible compiler configuration for OXC-backed compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
	/// Target ECMAScript version (e.g., "es2024")
	pub Target:String,

	/// Module system ("commonjs", "esmodule")
	pub Module:String,

	/// Enable strict mode
	pub Strict:bool,

	/// Emit decorators metadata
	pub EmitDecoratorsMetadata:bool,

	/// Enable tree-shaking to remove unused code
	pub TreeShaking:bool,

	/// Enable minification to reduce output size
	pub Minify:bool,

	/// Module format (commonjs, esmodule, amd, umd, none)
	pub ModuleFormat:ModuleFormat,
}

/// Compilation options passed through the Rest build pipeline.
#[derive(Debug, Clone)]
pub struct Option {
	/// Entry points for compilation
	pub entry:Vec<Vec<String>>,

	/// File path separator character
	pub separator:char,

	/// File pattern to match (e.g., "**/*.ts")
	pub pattern:String,

	/// Compiler configuration
	pub config:CompilerConfig,

	/// Output directory for compiled files
	pub output:String,

	/// VSCode compatibility: use defineForClassFields (false by default for
	/// VSCode)
	pub use_define_for_class_fields:bool,
}

impl Default for Option {
	fn default() -> Self {
		Self {
			entry:vec![],

			separator:'/',

			pattern:"**/*.ts".to_string(),

			config:CompilerConfig::default(),

			output:"out".to_string(),

			// VSCode compatibility: false ensures class fields use define pattern
			use_define_for_class_fields:false,
		}
	}
}

/// Compiler metrics tracking count, elapsed time, and error count.
#[derive(Debug, Default)]
pub struct CompilerMetrics {
	/// Total files processed
	pub Count:usize,

	/// Total elapsed time
	pub Elapsed:Duration,

	/// Total error count
	pub Error:usize,
}

impl Default for CompilerConfig {
	fn default() -> Self {
		Self {
			Target:"es2024".to_string(),

			Module:"commonjs".to_string(),

			Strict:true,

			EmitDecoratorsMetadata:true,

			TreeShaking:false,

			Minify:false,

			ModuleFormat:ModuleFormat::CommonJs,
		}
	}
}

use std::{
	path::PathBuf,
	time::{Duration, SystemTime},
};

use serde::{Deserialize, Serialize};

impl CompilerConfig {
	/// Checks if JSX is enabled from the target file extension.
	pub fn jsx(&self) -> bool {
		false // JSX is detected at parse time from file extension
	}

	/// Returns the module format as a string.
	pub fn module_format(&self) -> String {
		match self.ModuleFormat {
			ModuleFormat::CommonJs => "commonjs".to_string(),

			ModuleFormat::EsModule => "esmodule".to_string(),

			ModuleFormat::Amd => "amd".to_string(),

			ModuleFormat::Umd => "umd".to_string(),

			ModuleFormat::None => "none".to_string(),
		}
	}
}

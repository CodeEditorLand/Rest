#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
	path:PathBuf,
	last_modified:SystemTime,
}

/// Module format options for code generation
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
	pub fn from_str(s:&str) -> Self {
		match s.to_lowercase().as_str() {
			"esmodule" | "esm" | "esnext" | "es" => ModuleFormat::EsModule,
			"amd" => ModuleFormat::Amd,
			"umd" => ModuleFormat::Umd,
			"none" => ModuleFormat::None,
			_ => ModuleFormat::CommonJs,
		}
	}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
	pub Target:String,
	pub Module:String,
	pub Strict:bool,
	pub EmitDecoratorsMetadata:bool,
	/// Enable tree-shaking to remove unused code
	pub TreeShaking:bool,
	/// Enable minification to reduce output size
	pub Minify:bool,
	/// Module format (commonjs, esmodule, amd, umd, none)
	pub ModuleFormat:ModuleFormat,
}

#[derive(Debug, Clone)]
pub struct Option {
	pub entry:Vec<Vec<String>>,
	pub separator:char,
	pub pattern:String,
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
			/// VSCode compatibility: false ensures class fields use define
			/// pattern
			use_define_for_class_fields:false,
		}
	}
}

#[derive(Debug, Default)]
pub struct CompilerMetrics {
	pub Count:usize,
	pub Elapsed:Duration,
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
	/// Check if JSX is enabled
	pub fn jsx(&self) -> bool {
		false // JSX is detected at parse time from file extension
	}

	/// Get module format as string
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

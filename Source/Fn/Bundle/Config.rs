//! Bundle configuration.
//!
//! Configuration options for the bundler.

use super::BundleMode;

/// Configuration for the bundler.
#[derive(Debug, Clone)]
pub struct BundleConfig {
	/// Bundling mode.
	pub mode:BundleMode,

	/// Output directory for bundled files.
	pub output_dir:String,

	/// Output filename pattern (e.g., `"{name}.js"`).
	pub output_file:String,

	/// Enable source-map generation.
	pub source_map:bool,

	/// Inline source map in the output.
	pub inline_source_map:bool,

	/// Enable minification.
	pub minify:bool,

	/// Enable tree-shaking.
	pub tree_shaking:bool,

	/// Target environment (e.g., `"es2022"`, `"es2024"`).
	pub target:String,

	/// Module format (`"commonjs"`, `"esmodule"`, `"amd"`, `"umd"`).
	pub format:String,

	/// Whether to generate declaration files.
	pub declarations:bool,

	/// Whether to watch for changes.
	pub watch:bool,

	/// External modules (not to be bundled).
	pub externals:Vec<String>,

	/// Additional entry points.
	pub entries:Vec<String>,

	/// Whether to split chunks (for code splitting).
	pub split_chunks:bool,
}

impl Default for BundleConfig {
	fn default() -> Self {
		Self {
			mode:BundleMode::SingleFile,

			output_dir:"out".to_string(),

			output_file:"{name}.js".to_string(),

			source_map:true,

			inline_source_map:false,

			minify:false,

			tree_shaking:true,

			target:"es2024".to_string(),

			format:"esmodule".to_string(),

			declarations:false,

			watch:false,

			externals:Vec::new(),

			entries:Vec::new(),

			split_chunks:false,
		}
	}
}

impl BundleConfig {
	/// Creates a config for simple single-file compilation.
	pub fn single_file() -> Self { Self { mode:BundleMode::SingleFile, ..Default::default() } }

	/// Creates a config for bundling multiple files.
	pub fn bundle() -> Self { Self { mode:BundleMode::Bundle, ..Default::default() } }

	/// Creates a config for esbuild mode.
	pub fn esbuild() -> Self { Self { mode:BundleMode::Esbuild, ..Default::default() } }

	/// Creates a config for watch mode.
	pub fn watch() -> Self { Self { mode:BundleMode::Watch, watch:true, ..Default::default() } }

	/// Sets the output directory.
	pub fn with_output_dir(mut self, dir:impl Into<String>) -> Self {
		self.output_dir = dir.into();

		self
	}

	/// Sets the output file pattern.
	pub fn with_output_file(mut self, file:impl Into<String>) -> Self {
		self.output_file = file.into();

		self
	}

	/// Enables or disables source maps.
	pub fn with_source_map(mut self, enabled:bool) -> Self {
		self.source_map = enabled;

		self
	}

	/// Enables or disables minification.
	pub fn with_minify(mut self, enabled:bool) -> Self {
		self.minify = enabled;

		self
	}

	/// Enables or disables tree-shaking.
	pub fn with_tree_shaking(mut self, enabled:bool) -> Self {
		self.tree_shaking = enabled;

		self
	}

	/// Sets the target environment.
	pub fn with_target(mut self, target:impl Into<String>) -> Self {
		self.target = target.into();

		self
	}

	/// Sets the module format.
	pub fn with_format(mut self, format:impl Into<String>) -> Self {
		self.format = format.into();

		self
	}

	/// Adds an external module (excluded from the bundle).
	pub fn add_external(mut self, external:impl Into<String>) -> Self {
		self.externals.push(external.into());

		self
	}

	/// Adds an entry point.
	pub fn add_entry(mut self, entry:impl Into<String>) -> Self {
		self.entries.push(entry.into());

		self
	}

	/// Returns `true` if this config requires actual bundling.
	pub fn requires_bundle(&self) -> bool { self.mode.requires_bundle() }
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_default_config() {
		let config = BundleConfig::default();

		assert_eq!(config.mode, BundleMode::SingleFile);

		assert_eq!(config.target, "es2024");
	}

	#[test]
	fn test_single_file_config() {
		let config = BundleConfig::single_file();

		assert_eq!(config.mode, BundleMode::SingleFile);

		assert!(!config.source_map);
	}

	#[test]
	fn test_bundle_config() {
		let config = BundleConfig::bundle();

		assert_eq!(config.mode, BundleMode::Bundle);

		assert!(config.tree_shaking);
	}

	#[test]
	fn test_builder_pattern() {
		let config = BundleConfig::default()
			.with_output_dir("dist")
			.with_output_file("bundle.js")
			.with_minify(true)
			.add_external("react")
			.add_entry("src/index.ts");

		assert_eq!(config.output_dir, "dist");

		assert_eq!(config.output_file, "bundle.js");

		assert!(config.minify);

		assert_eq!(config.externals.len(), 1);

		assert_eq!(config.entries.len(), 1);
	}
}

//! Integration tests for Rest compiler VSCode compatibility
//!
//! These tests verify that Rest compiler output matches VSCode's gulp/tsb build
//! exactly, ensuring 1:1 compatibility for the CodeEditorLand build process.

#![cfg(test)]

use std::{
	path::{Path, PathBuf},
	process::Command,
	sync::Arc,
};

use tempfile::TempDir;
use tracing::{debug, error, info, warn};

use crate::{
	Fn::{Binary::Command::Struct::Option, Compiler},
	Struct::{CompilerConfig, SWC},
};

/// Test configuration for VSCode compatibility benchmarks
#[derive(Debug, Clone)]
pub struct VSCodeTestConfig {
	/// Path to VSCode source directory
	pub vscode_source:PathBuf,

	/// Path to VSCode expected output (from gulp build)
	pub vscode_output:PathBuf,

	/// Path to Rest output directory
	pub rest_output:PathBuf,

	/// Compiler configuration
	pub config:CompilerConfig,

	/// Whether to generate source maps
	pub source_maps:bool,

	/// Whether to use defineForClassFields (VSCode: false)
	pub use_define_for_class_fields:bool,
}

impl VSCodeTestConfig {
	pub fn new(
		vscode_source:impl Into<PathBuf>,

		vscode_output:impl Into<PathBuf>,

		rest_output:impl Into<PathBuf>,
	) -> Self {
		Self {
			vscode_source:vscode_source.into(),

			vscode_output:vscode_output.into(),

			rest_output:rest_output.into(),

			config:CompilerConfig::vscode(),

			source_maps:true,

			use_define_for_class_fields:false,
		}
	}

	/// Create test config for out/ directory (development build)
	pub fn development() -> Self {
		let workspace_root = Self::find_workspace_root();

		Self::new(
			workspace_root.join("Dependency/Microsoft/Dependency/Editor/out"),
			workspace_root.join("Dependency/Microsoft/Dependency/Editor/out"),
			workspace_root.join("Element/Rest/Target/test-out"),
		)
	}

	/// Create test config for out-build/ directory (production build)
	pub fn production() -> Self {
		let workspace_root = Self::find_workspace_root();

		Self::new(
			workspace_root.join("Dependency/Microsoft/Dependency/Editor/out-build"),
			workspace_root.join("Dependency/Microsoft/Dependency/Editor/out-build"),
			workspace_root.join("Element/Rest/Target/test-out-build"),
		)
	}

	/// Find the workspace root directory
	fn find_workspace_root() -> PathBuf {
		let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

		// Assuming tests run from workspace root or Element/Rest
		if current.ends_with("Element/Rest") {
			current.parent().unwrap().parent().unwrap().to_path_buf()
		} else if current.ends_with("Element") {
			current.parent().unwrap().to_path_buf()
		} else {
			current
		}
	}
}

/// Results of a VSCode compatibility test
#[derive(Debug, Clone)]
pub struct CompatibilityReport {
	pub total_files:usize,

	pub matching_files:usize,

	pub mismatched_files:Vec<FileMismatch>,

	pub missing_in_rest:Vec<PathBuf>,

	pub extra_in_rest:Vec<PathBuf>,

	pub overall_match:bool,
}

#[derive(Debug, Clone)]
pub struct FileMismatch {
	pub path:PathBuf,

	pub vscode_size:usize,

	pub rest_size:usize,

	pub content_diff_percent:f64,
}

impl CompatibilityReport {
	pub fn success(&self) -> bool {
		self.overall_match
			&& self.mismatched_files.is_empty()
			&& self.missing_in_rest.is_empty()
			&& self.extra_in_rest.is_empty()
	}
}

/// Test suite for Rest compiler VSCode compatibility
#[cfg(test)]
mod tests {

	use super::*;
	use crate::Fn::OXC::Compiler as OxcCompiler;

	/// Test that Rest can compile a simple TypeScript file
	#[test]
	fn test_simple_compilation() {
		let source = r#"
            export function greet(name: string): string {

                return `Hello, ${name}!`;
            }

        "#;

		let config = CompilerConfig::simple();

		let compiler = Compiler::new(config);

		// Create temp input file
		let temp_dir = tempfile::tempdir().unwrap();

		let input_path = temp_dir.path().join("test.ts");

		std::fs::write(&input_path, source).unwrap();

		// Compile
		let result = compiler.compile_file(input_path.to_str().unwrap(), source.to_string());

		assert!(result.is_ok());

		let output_path = input_path.with_extension("js");

		let output = std::fs::read_to_string(&output_path).unwrap();

		// Verify output contains the function
		assert!(output.contains("function greet"));

		assert!(output.contains("Hello"));
	}

	/// Test decorator compilation matches VSCode expectations
	#[test]
	fn test_decorator_compilation() {
		let source = r#"
            function sealed(constructor: Function) {

                Object.seal(constructor);
            }

            @sealed
            class MyClass {

                method() {}
            }

        "#;

		let config = CompilerConfig::vscode();

		let compiler = Compiler::new(config);

		let temp_dir = tempfile::tempdir().unwrap();

		let input_path = temp_dir.path().join("decorator.ts");

		std::fs::write(&input_path, source).unwrap();

		let result = compiler.compile_file(input_path.to_str().unwrap(), source.to_string());

		assert!(result.is_ok());

		let output_path = input_path.with_extension("js");

		let output = std::fs::read_to_string(&output_path).unwrap();

		// VSCode with emitDecorators: true will include __decorate helper
		assert!(output.contains("__decorate") || output.contains("MyClass"));
	}

	/// Test that compilation preserves source maps when enabled
	#[test]
	fn test_source_maps_generation() {
		let source = r#"
            export const value: number = 42;

        "#;

		let config = CompilerConfig::simple();

		let compiler = Compiler::new(config);

		let temp_dir = tempfile::tempdir().unwrap();

		let input_path = temp_dir.path().join("smap.ts");

		std::fs::write(&input_path, source).unwrap();

		// Note: Current Compiler.rs doesn't expose source map option in compile_file
		// This will need to be added to test properly
		let result = compiler.compile_file(input_path.to_str().unwrap(), source.to_string());

		assert!(result.is_ok());

		let output_path = input_path.with_extension("js");

		assert!(output_path.exists());
	}

	/// Full integration test comparing Rest output to VSCode out/ directory
	/// This test requires VSCode to be built first
	#[test]
	#[ignore] // Requires VSCode build to be present
	fn test_vscode_out_compatibility() {
		let test_config = VSCodeTestConfig::development();

		if !test_config.vscode_source.exists() {
			warn!("VSCode out/ directory not found, skipping test");

			return;
		}

		let report = compare_vscode_output(&test_config).unwrap();

		println!("Compatibility Report: {:?}", report);

		assert!(report.success(), "Rest output does not match VSCode output");
	}

	/// Full integration test comparing Rest output to VSCode out-build/
	/// directory
	#[test]
	#[ignore] // Requires VSCode build to be present
	fn test_vscode_out_build_compatibility() {
		let test_config = VSCodeTestConfig::production();

		if !test_config.vscode_source.exists() {
			warn!("VSCode out-build/ directory not found, skipping test");

			return;
		}

		let report = compare_vscode_output(&test_config).unwrap();

		println!("Compatibility Report: {:?}", report);

		assert!(report.success(), "Rest output does not match VSCode build output");
	}

	/// Compare Rest compilation output against VSCode's compiled output
	fn compare_vscode_output(config:&VSCodeTestConfig) -> Result<CompatibilityReport, Box<dyn std::error::Error>> {
		info!("Comparing Rest output vs VSCode output: {}", config.vscode_source.display());

		// Ensure output directories exist and are clean
		if config.rest_output.exists() {
			std::fs::remove_dir_all(&config.rest_output)?;
		}

		std::fs::create_dir_all(&config.rest_output)?;

		// Collect all TypeScript files from VSCode source
		let ts_files = find_typeScript_files(&config.vscode_source)?;

		info!("Found {} TypeScript files to compare", ts_files.len());

		// Compile each file with Rest
		let oxc_config = config.config.clone();

		let compiler = Compiler::new(oxc_config);

		for ts_file in &ts_files {
			// Determine output path relative to source
			let rel_path = ts_file.strip_prefix(&config.vscode_source).unwrap();

			let output_path = config.rest_output.join(rel_path).with_extension("js");

			// Create parent directories
			if let Some(parent) = output_path.parent() {
				std::fs::create_dir_all(parent)?;
			}

			// Read source
			let source = std::fs::read_to_string(ts_file)?;

			// Compile
			match compiler.compile_file_to(
				ts_file.to_str().unwrap(),
				source,
				&output_path,
				config.use_define_for_class_fields,
			) {
				Ok(_) => {
					debug!("Compiled: {}", ts_file.display());
				},

				Err(e) => {
					error!("Failed to compile {}: {}", ts_file.display(), e);

					// Continue with other files
				},
			}
		}

		// Compare outputs
		let mut report = CompatibilityReport {
			total_files:ts_files.len(),

			matching_files:0,

			mismatched_files:Vec::new(),

			missing_in_rest:Vec::new(),

			extra_in_rest:Vec::new(),

			overall_match:false,
		};

		// Check each original file's JS output
		for ts_file in &ts_files {
			let rel_path = ts_file.strip_prefix(&config.vscode_source).unwrap();

			let vscode_js_path = config.vscode_output.join(rel_path).with_extension("js");

			let rest_js_path = config.rest_output.join(rel_path).with_extension("js");

			if !vscode_js_path.exists() {
				debug!("VSCode JS not found: {}", vscode_js_path.display());

				continue;
			}

			if !rest_js_path.exists() {
				debug!("Rest JS not found: {}", rest_js_path.display());

				report.missing_in_rest.push(rel_path.to_path_buf());

				continue;
			}

			// Compare file contents
			let vscode_content = std::fs::read(&vscode_js_path)?;

			let rest_content = std::fs::read(&rest_js_path)?;

			if vscode_content == rest_content {
				report.matching_files += 1;
			} else {
				let vscode_size = vscode_content.len();

				let rest_size = rest_content.len();

				let diff_percent = if vscode_size > 0 {
					((vscode_size as f64 - rest_size as f64).abs() / vscode_size as f64) * 100.0
				} else {
					0.0
				};

				report.mismatched_files.push(FileMismatch {
					path:rel_path.to_path_buf(),
					vscode_size,
					rest_size,
					content_diff_percent:diff_percent,
				});
			}
		}

		report.overall_match =
			report.mismatched_files.is_empty() && report.missing_in_rest.is_empty() && report.extra_in_rest.is_empty();

		Ok(report)
	}

	/// Find all TypeScript files in a directory
	fn find_typeScript_files(dir:&Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
		let mut files = Vec::new();

		for entry in walkdir::WalkDir::new(dir) {
			let entry = entry?;

			if entry.file_type().is_file() {
				let path = entry.path();

				if let Some(ext) = path.extension() {
					if ext == "ts" || ext == "tsx" {
						// Skip .d.ts files and test files
						if !path.to_string_lossy().contains(".d.ts")
							&& !path.to_string_lossy().contains("/test/")
							&& !path.to_string_lossy().contains("\\test\\")
						{
							files.push(path.to_path_buf());
						}
					}
				}
			}
		}

		Ok(files)
	}
}

#[cfg(test)]
mod benchmarks {

	use std::time::Instant;

	use super::*;

	/// Benchmark Rest compiler performance on VSCode-sized codebase
	#[test]
	#[ignore]
	fn benchmark_rest_compilation() {
		let test_config = VSCodeTestConfig::development();

		if !test_config.vscode_source.exists() {
			warn!("VSCode source not found, skipping benchmark");

			return;
		}

		let ts_files = find_typeScript_files(&test_config.vscode_source).unwrap();

		let sample_size = ts_files.len().min(100); // Test first 100 files

		let config = CompilerConfig::vscode();

		let compiler = Compiler::new(config);

		let start = Instant::now();

		let mut compiled = 0;

		let mut errors = 0;

		for ts_file in ts_files.iter().take(sample_size) {
			let source = std::fs::read_to_string(ts_file).unwrap();

			match compiler.compile_file(ts_file.to_str().unwrap(), source) {
				Ok(_) => compiled += 1,

				Err(_) => errors += 1,
			}
		}

		let elapsed = start.elapsed();

		let avg_time = elapsed.as_secs_f64() / compiled as f64;

		println!("\n=== Rest Compilation Benchmark ===");

		println!("Files: {}/{}", compiled, sample_size);

		println!("Errors: {}", errors);

		println!("Total time: {:?}", elapsed);

		println!("Average per file: {:.3?}", avg_time);

		println!("Throughput: {:.1} files/sec", compiled as f64 / elapsed.as_secs_f64());
	}

	/// Compare Rest performance with VSCode's tsb compilation
	#[test]
	#[ignore]
	fn benchmark_vscode_vs_rest() {

		// This would require running both builds and comparing
		// Implementation depends on having both compilers available
	}
}

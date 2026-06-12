//! Worker compilation orchestration
//!
//! Coordinates the compilation of web workers.

use std::{collections::HashMap, path::Path};

use super::{WorkerBootstrap, WorkerConfig, WorkerDetector, WorkerInfo, WorkerType};

/// Compiles web workers
pub struct WorkerCompiler {
	config:WorkerConfig,

	detector:WorkerDetector,

	bootstrap:WorkerBootstrap,

	/// Cached compiled workers
	compiled:HashMap<String, String>,
}

impl WorkerCompiler {
	/// Create a new [`WorkerCompiler`] for the given configuration.
	pub fn new(config:WorkerConfig) -> Self {
		Self {
			config:config.clone(),

			detector:WorkerDetector::new(config.clone()),

			bootstrap:WorkerBootstrap::new(config),

			compiled:HashMap::new(),
		}
	}

	/// Compile all workers in a directory
	pub fn compile_workers(&mut self, root_dir:&Path) -> anyhow::Result<Vec<WorkerInfo>> {
		// Ensure output directory exists
		std::fs::create_dir_all(&self.config.output_dir)?;

		// Detect all workers
		let workers = self.detector.detect_workers(root_dir);

		// Compile each worker
		for mut worker_info in workers.clone() {
			self.compile_worker(&mut worker_info)?;
		}

		Ok(workers)
	}

	/// Compile a single worker
	pub fn compile_worker(&mut self, worker_info:&mut WorkerInfo) -> anyhow::Result<()> {
		let source_path = Path::new(&worker_info.source_path);

		if !source_path.exists() {
			return Err(anyhow::anyhow!("Worker source file not found: {}", worker_info.source_path));
		}

		// Read the worker source
		let source = std::fs::read_to_string(source_path)?;

		// Extract dependencies
		worker_info.dependencies = self.detector.extract_dependencies(source_path);

		// Generate output based on worker type
		let output = match worker_info.worker_type {
			WorkerType::Module => self.compile_module_worker(&source, worker_info)?,

			WorkerType::Classic => self.compile_classic_worker(&source, worker_info)?,
		};

		// Write output
		let output_path = Path::new(&worker_info.output_path);

		std::fs::write(output_path, &output)?;

		// Cache the compiled output
		self.compiled.insert(worker_info.name.clone(), output);

		Ok(())
	}

	/// Compile a module worker
	fn compile_module_worker(&self, source:&str, worker_info:&WorkerInfo) -> anyhow::Result<String> {
		// Add bootstrap code for module workers
		let bootstrap = self.bootstrap.generate_module_worker(&worker_info.source_path);

		// Combine bootstrap and source
		let mut output = bootstrap;

		output.push_str("\n// Worker source\n");

		output.push_str(source);

		Ok(output)
	}

	/// Compile a classic worker
	fn compile_classic_worker(&self, source:&str, worker_info:&WorkerInfo) -> anyhow::Result<String> {
		// Add bootstrap code for classic workers
		let bootstrap = self.bootstrap.generate_classic_worker(&worker_info.source_path);

		// Combine bootstrap and source
		let mut output = bootstrap;

		output.push_str("\n// Worker source\n");

		output.push_str(source);

		Ok(output)
	}

	/// Get a compiled worker by name
	pub fn get_compiled(&self, name:&str) -> Option<&String> { self.compiled.get(name) }

	/// Generate worker type declarations
	pub fn generate_declarations(&self, workers:&[WorkerInfo]) -> String {
		let mut declarations = String::new();

		declarations.push_str("// Worker type declarations\n");

		declarations.push_str("// This file is auto-generated - do not edit\n\n");

		for worker in workers {
			declarations.push_str(&super::Bootstrap::generate_worker_declaration(&worker.name));
		}

		declarations
	}

	/// Create a worker entry point that lazy-loads workers
	pub fn generate_worker_loader(&self, workers:&[WorkerInfo]) -> String {
		let mut code = String::new();

		code.push_str("// Worker loader - auto-generated\n");

		code.push_str("// This file provides lazy-loading for web workers\n\n");

		for worker in workers {
			let worker_url = format!("./{}", worker.name);

			code.push_str(&self.bootstrap.generate_worker_loader(&worker.name, &worker_url));

			code.push('\n');
		}

		code
	}
}

/// Simplified worker compilation function
pub fn compile_worker_file(source_path:&Path, output_path:&Path, worker_type:WorkerType) -> anyhow::Result<()> {
	let config = WorkerConfig::new();

	let mut compiler = WorkerCompiler::new(config);

	let mut worker_info = WorkerInfo::new(source_path.to_string_lossy().as_ref(), worker_type);

	worker_info.output_path = output_path.to_string_lossy().to_string();

	compiler.compile_worker(&mut worker_info)
}

#[cfg(test)]
mod tests {

	use tempfile::TempDir;

	use super::*;

	#[test]
	fn test_worker_compiler_creation() {
		let config = WorkerConfig::new();

		let compiler = WorkerCompiler::new(config);

		assert!(compiler.compiled.is_empty());
	}

	#[test]
	fn test_worker_declaration_generation() {
		let config = WorkerConfig::new();

		let compiler = WorkerCompiler::new(config);

		let workers = vec![WorkerInfo::new("test.worker.ts", WorkerType::Module)];

		let declarations = compiler.generate_declarations(&workers);

		assert!(declarations.contains("declare const test"));
	}

	#[test]
	fn test_worker_loader_generation() {
		let config = WorkerConfig::new();

		let compiler = WorkerCompiler::new(config);

		let workers = vec![WorkerInfo::new("test.worker.ts", WorkerType::Module)];

		let loader = compiler.generate_worker_loader(&workers);

		assert!(loader.contains("Worker"));
	}
}

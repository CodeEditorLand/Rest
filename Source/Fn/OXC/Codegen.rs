//! OXC Code Generation module
//!
//! This module provides code generation from the transformed AST to JavaScript
//! source code.
//!
//! DIAGNOSTIC LOGGING:
//! - Tracks codegen lifecycle and memory access patterns

use std::sync::atomic::{AtomicUsize, Ordering};

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_codegen::{Codegen, CodegenOptions, CodegenReturn};
use oxc_span::SourceType;
use tracing::{debug, error, info, trace, warn};

/// Codegen configuration options
#[derive(Debug, Clone)]
pub struct CodegenConfig {
	/// Whether to generate minified output
	pub minify:bool,
	/// Whether to generate source maps
	pub source_map:bool,
	/// Source map file name (without extension)
	pub source_map_name:String,
	/// Whether to preserve comments
	pub comments:bool,
}

impl Default for CodegenConfig {
	fn default() -> Self { Self { minify:false, source_map:false, source_map_name:String::new(), comments:false } }
}

impl CodegenConfig {
	/// Create a new codegen configuration
	pub fn new(minify:bool, _source_map:bool, _source_map_name:String, comments:bool) -> Self {
		Self { minify, source_map:_source_map, source_map_name:_source_map_name, comments }
	}
}

/// Result of code generation
pub struct CodegenResult {
	/// The generated JavaScript source code
	pub code:String,
	/// The length of the generated code
	pub code_len:usize,
}

/// Post-process generated JavaScript to match VSCode's static class property
/// format. Converts `static x = expr;` into `static { this.x = expr; }`.
/// This is needed because OXC 0.48's class properties plugin does not emit
/// legacy static initializer blocks by default.
fn transform_static_class_properties(code:&str) -> String {
	// This regex matches: static <name> = <expression>;
	// Captures the property name and the initializer expression.
	let re = match regex::Regex::new(r"(?m)^\s*static\s+([a-zA-Z_$][\w$]*)\s*=\s*([^;]+);") {
		Ok(re) => re,
		Err(e) => {
			// If regex compilation fails, return original code and log.
			error!("transform_static_class_properties: regex compile error: {}", e);
			return code.to_string();
		},
	};
	re.replace_all(code, "static { this.$1 = $2; }").into_owned()
}

/// Generate JavaScript source code from a transformed AST
///
/// # Arguments
/// * `allocator` - The allocator used for the AST
/// * `program` - The transformed program AST
/// * `_source_type` - The source type (JavaScript, JSX, etc.)
/// * `config` - Codegen configuration options
///
/// # Returns
/// A CodegenResult containing the generated source code
static CODEGEN_COUNT:AtomicUsize = AtomicUsize::new(0);

#[tracing::instrument(skip(_allocator, program, config))]
pub fn codegen<'a>(
	_allocator:&Allocator,
	program:&Program<'a>,
	_source_type:SourceType,
	config:&CodegenConfig,
) -> Result<CodegenResult, String> {
	let codegen_id = CODEGEN_COUNT.fetch_add(1, Ordering::SeqCst);

	info!("[Codegen #{codegen_id}] Starting code generation");
	trace!("[Codegen #{codegen_id}] Program address: {:p}", program);
	trace!(
		"[Codegen #{codegen_id}] Program body ptr: {:p}, len: {}",
		program.body.as_ptr(),
		program.body.len()
	);
	debug!(
		"[Codegen #{codegen_id}] Config: minify={}, comments={}",
		config.minify, config.comments
	);

	// Configure codegen options
	let options = CodegenOptions { minify:config.minify, comments:config.comments, ..Default::default() };
	trace!("[Codegen #{codegen_id}] CodegenOptions configured");

	// Create codegen instance and generate code
	let codegen_start = std::time::Instant::now();
	let CodegenReturn { code, .. } = Codegen::new().with_options(options).build(program);
	info!(
		"[Codegen #{codegen_id}] Code generation completed in {:?}",
		codegen_start.elapsed()
	);

	let code_len = code.len();
	debug!("[Codegen #{codegen_id}] Generated {} bytes of code", code_len);
	trace!(
		"[Codegen #{codegen_id}] First 100 chars of output: {:?}",
		code.chars().take(100).collect::<String>()
	);

	info!("[Codegen #{codegen_id}] SUCCESS: Generated {} bytes", code_len);
	// Transform OXC output to match VSCode static class property format
	let transformed_code = transform_static_class_properties(&code);
	Ok(CodegenResult { code:transformed_code, code_len })
}

/// Write the generated code to a file
///
/// # Arguments
/// * `output_path` - The path to write the output file
/// * `result` - The codegen result containing source text
pub fn write_output(output_path:&std::path::Path, result:&CodegenResult) -> Result<(), std::io::Error> {
	// Create parent directories if they don't exist
	if let Some(parent) = output_path.parent() {
		std::fs::create_dir_all(parent)?;
	}

	// Write the source code
	std::fs::write(output_path, &result.code)?;

	debug!("Written output to {}", output_path.display());

	Ok(())
}

//! OXC TypeScript Transformer module
//!
//! This module provides TypeScript to JavaScript transformation using the OXC
//! transformer. It handles TypeScript type stripping, decorator transformation,
//! and ECMAScript version transpilation.
//!
//! DIAGNOSTIC LOGGING:
//! - Tracks transformer lifecycle and memory access
//! - Logs allocator addresses to detect use-after-free

use std::{
	path::Path,
	sync::atomic::{AtomicUsize, Ordering},
};

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{
	CompilerAssumptions,
	EnvOptions,
	JsxOptions,
	JsxRuntime,
	TransformOptions,
	Transformer,
	TypeScriptOptions,
};
use tracing::{debug, info, trace, warn};

/// Transformer configuration options
#[derive(Debug, Clone)]
pub struct TransformerConfig {
	/// Target ECMAScript version (e.g., "es2024")
	pub target:String,
	/// Module format (commonjs, esmodule, etc.)
	pub module_format:String,
	/// Whether to emit decorator metadata
	pub emit_decorator_metadata:bool,
	/// Whether to use define for class fields (VSCode compatibility)
	pub use_define_for_class_fields:bool,
	/// Whether to support JSX
	pub jsx:bool,
	/// Whether to enable tree-shaking
	pub tree_shaking:bool,
	/// Whether to enable minification
	pub minify:bool,
}

impl Default for TransformerConfig {
	fn default() -> Self {
		Self {
			target:"es2024".to_string(),
			module_format:"commonjs".to_string(),
			emit_decorator_metadata:true,
			use_define_for_class_fields:false,
			jsx:false,
			tree_shaking:false,
			minify:false,
		}
	}
}

impl TransformerConfig {
	/// Create a new transformer configuration
	pub fn new(
		target:String,
		_module_format:String,
		_emit_decorator_metadata:bool,
		use_define_for_class_fields:bool,
		jsx:bool,
		_tree_shaking:bool,
		_minify:bool,
	) -> Self {
		Self {
			target,
			module_format:_module_format,
			emit_decorator_metadata:_emit_decorator_metadata,
			use_define_for_class_fields,
			jsx,
			tree_shaking:_tree_shaking,
			minify:_minify,
		}
	}
}

/// Transform a parsed AST from TypeScript to JavaScript
///
/// # Arguments
/// * `allocator` - The allocator used for the AST
/// * `program` - The parsed program AST (mutable)
/// * `source_path` - The source file path
/// * `source_type` - The source type (TypeScript, JSX, etc.)
/// * `config` - Transformer configuration options
///
/// # Returns
/// Result containing transformation errors if any
static TRANSFORM_COUNT:AtomicUsize = AtomicUsize::new(0);

#[tracing::instrument(skip(allocator, program, config))]
pub fn transform<'a>(
	allocator:&'a Allocator,
	program:&mut Program<'a>,
	source_path:&str,
	_source_type:SourceType,
	config:&TransformerConfig,
) -> Result<(), Vec<String>> {
	let transform_id = TRANSFORM_COUNT.fetch_add(1, Ordering::SeqCst);

	info!("[Transform #{transform_id}] Starting transformation of: {}", source_path);
	trace!("[Transform #{transform_id}] Allocator address: {:p}", allocator);
	trace!("[Transform #{transform_id}] Program address: {:p}", program);
	trace!(
		"[Transform #{transform_id}] Program body ptr before: {:p}, len: {}",
		program.body.as_ptr(),
		program.body.len()
	);
	debug!(
		"[Transform #{transform_id}] Config: target={}, module={}, use_define={}",
		config.target, config.module_format, config.use_define_for_class_fields
	);

	// Build semantic information required for transformations
	let semantic_start = std::time::Instant::now();
	let semantic_ret = SemanticBuilder::new().build(program);
	info!(
		"[Transform #{transform_id}] Semantic build completed in {:?}",
		semantic_start.elapsed()
	);

	if !semantic_ret.errors.is_empty() {
		let errors:Vec<String> = semantic_ret.errors.iter().map(|e| e.to_string()).collect();
		warn!("[Transform #{transform_id}] Semantic errors: {:?}", errors);
		return Err(errors);
	}

	// Extract the unified scoping (symbol table + scope tree) from semantic.
	// OXC 0.127 collapsed the separate `SymbolTable` + `ScopeTree` accessors
	// into a single `Scoping` value returned by `into_scoping()`. The older
	// `into_symbol_table_and_scope_tree()` (0.48-era API) no longer exists,
	// and `Transformer::build_with_symbols_and_scopes` has likewise been
	// replaced by `build_with_scoping` below.
	let scoping = semantic_ret.semantic.into_scoping();
	trace!(
		"[Transform #{transform_id}] Extracted scoping: {} symbols, {} scopes",
		scoping.symbols_len(),
		scoping.scopes_len()
	);

	// Configure TypeScript transformation
	// Set only_remove_type_imports to true to preserve all value exports
	// This ensures modules with runtime code (like profiling.ts) emit JavaScript
	let mut typescript_options = TypeScriptOptions::default();
	typescript_options.only_remove_type_imports = true;
	trace!("[Transform #{transform_id}] TypeScript options configured (only_remove_type_imports=true)");

	// Configure JSX transformation if enabled
	let jsx_options = if config.jsx {
		JsxOptions { runtime:JsxRuntime::Automatic, ..JsxOptions::default() }
	} else {
		// Disable JSX by setting a dummy runtime
		JsxOptions { runtime:JsxRuntime::Classic, ..JsxOptions::default() }
	};
	trace!("[Transform #{transform_id}] JSX options configured");

	// Configure environment options based on target
	let env_options_start = std::time::Instant::now();
	let env_options = EnvOptions::from_target(&config.target).unwrap_or_default();
	trace!(
		"[Transform #{transform_id}] Env options from target '{}' in {:?}",
		config.target,
		env_options_start.elapsed()
	);

	// Configure compiler assumptions for VSCode compatibility.
	// The `use_define_for_class_fields` flag from TypeScript:
	// - false => loose mode (direct assignment) => set_public_class_fields = true
	// - true => strict mode (defineProperty) => set_public_class_fields = false
	let mut assumptions = CompilerAssumptions::default();
	assumptions.set_public_class_fields = !config.use_define_for_class_fields;
	trace!(
		"[Transform #{transform_id}] Compiler assumptions configured (set_public_class_fields={})",
		assumptions.set_public_class_fields
	);

	// Create transform options with all VSCode compatibility settings
	let transform_options = TransformOptions {
		typescript:typescript_options,
		jsx:jsx_options,
		env:env_options,
		assumptions,
		..TransformOptions::default()
	};
	trace!("[Transform #{transform_id}] TransformOptions configured with plugins");
	trace!("[Transform #{transform_id}] TransformOptions created");

	// Create transformer and apply transformation using OXC 0.127 API.
	let transformer_start = std::time::Instant::now();
	let transformer = Transformer::new(allocator, Path::new(source_path), &transform_options);
	info!(
		"[Transform #{transform_id}] Transformer created in {:?}",
		transformer_start.elapsed()
	);
	trace!("[Transform #{transform_id}] Transformer allocator address: {:p}", allocator);

	let build_start = std::time::Instant::now();
	let transform_ret = transformer.build_with_scoping(scoping, program);
	info!(
		"[Transform #{transform_id}] build_with_scoping completed in {:?}",
		build_start.elapsed()
	);
	trace!(
		"[Transform #{transform_id}] Program body ptr after: {:p}, len: {}",
		program.body.as_ptr(),
		program.body.len()
	);

	if !transform_ret.errors.is_empty() {
		let errors:Vec<String> = transform_ret.errors.iter().map(|e| e.to_string()).collect();
		warn!("[Transform #{transform_id}] Transformation errors: {:?}", errors);
		return Err(errors);
	}

	info!(
		"[Transform #{transform_id}] SUCCESS: Transformation completed for {}",
		source_path
	);
	Ok(())
}

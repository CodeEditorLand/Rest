//! OXC TypeScript Compiler.
//!
//! Provides the main compiler orchestration for TypeScript-to-JavaScript
//! compilation using the OXC parser, transformer, and codegen subsystems.
//!
//! Diagnostic logging tracks the full lifecycle from parse through codegen,
//! including memory allocation timestamps and pointer addresses for
//! debugging use-after-free issues.

use std::{
	path::Path,
	sync::{
		Arc,
		Mutex,
		atomic::{AtomicUsize, Ordering},
	},
	time::{Duration, Instant},
};

use tracing::{debug, info, trace, warn};

use super::{
	Codegen::{self, CodegenConfig},
	Parser::{self, ParserConfig},
	Transformer::{self, TransformerConfig},
};

static COMPILE_ID:AtomicUsize = AtomicUsize::new(0);

/// Compiler metrics: count, elapsed time, and error count.
#[derive(Debug, Default)]
pub struct CompilerMetrics {
	/// Number of files compiled.
	pub count:usize,

	/// Total elapsed time.
	pub elapsed:Duration,

	/// Number of errors.
	pub error:usize,
}

/// OXC-based TypeScript compiler.
///
/// Orchestrates the parse → transform → codegen pipeline for each
/// source file.
pub struct Compiler {
	/// Compiler configuration.
	pub config:crate::Struct::SWC::CompilerConfig,

	/// Compiler metrics.
	pub outlook:Arc<Mutex<CompilerMetrics>>,
}

impl Compiler {
	/// Creates a new OXC compiler with the given configuration.
	pub fn new(config:crate::Struct::SWC::CompilerConfig) -> Self {
		Self { config, outlook:Arc::new(Mutex::new(CompilerMetrics::default())) }
	}

	/// Compiles a TypeScript file from source text, returning the output path.
	///
	/// ## Parameters
	///
	/// * `file_path` — Path to the source file.
	/// * `input` — TypeScript source code.
	///
	/// ## Returns
	///
	/// The output file path on success, or an error describing the failure.
	#[tracing::instrument(skip(self, input))]
	pub fn compile_file(&self, file_path:&str, input:String) -> anyhow::Result<String> {
		let compile_id = COMPILE_ID.fetch_add(1, Ordering::SeqCst);

		let begin = Instant::now();

		info!("[Compile #{compile_id}] Starting compilation of: {}", file_path);

		trace!("[Compile #{compile_id}] Input size: {} bytes", input.len());

		// All compilation steps happen in one scope to ensure allocator stays alive
		let (codegen_result, output_path) = {
			info!("[Compile #{compile_id}] Step 1: Parsing TypeScript source");

			let parser_config = self.get_parser_config();

			let mut parse_result = Parser::parse(&input, file_path, &parser_config)
				.map_err(|errors| anyhow::anyhow!("Parse errors: {:?}", errors))?;

			info!("[Compile #{compile_id}] Step 1 complete: Parsed {} successfully", file_path);

			debug!(
				"[Compile #{compile_id}] ParseResult.allocator address: {:p}",
				&parse_result.allocator
			);

			debug!(
				"[Compile #{compile_id}] ParseResult.program address: {:p}",
				&parse_result.program
			);

			trace!(
				"[Compile #{compile_id}] AST body pointer: {:p}",
				parse_result.program.body.as_ptr()
			);

			// Transform - borrow program mutably from parse_result
			info!("[Compile #{compile_id}] Step 2: Transforming AST");

			let transformer_config = self.get_transformer_config();

			// SAFETY: We borrow program with 'static lifetime from parse_result.
			// This is safe because:
			// 1. parse_result stays in scope for the entire inner block
			// 2. The program and allocator are dropped together when parse_result is
			//    dropped
			// 3. We never use program after parse_result is dropped
			let program = unsafe {
				std::mem::transmute::<&mut oxc_ast::ast::Program<'static>, &mut oxc_ast::ast::Program<'_>>(
					&mut parse_result.program,
				)
			};

			let source_type = oxc_span::SourceType::from_path(file_path).unwrap_or(oxc_span::SourceType::ts());

			debug!(
				"[Compile #{compile_id}] Transformer config: target={}, module={}",
				transformer_config.target, transformer_config.module_format
			);

			Transformer::transform(&parse_result.allocator, program, file_path, source_type, &transformer_config)
				.map_err(|errors| anyhow::anyhow!("Transform errors: {:?}", errors))?;

			info!(
				"[Compile #{compile_id}] Step 2 complete: Transformed {} successfully",
				file_path
			);

			trace!(
				"[Compile #{compile_id}] Program after transform - body pointer: {:p}",
				program.body.as_ptr()
			);

			// Generate code
			info!("[Compile #{compile_id}] Step 3: Generating code");

			let codegen_config = self.get_codegen_config();

			let codegen_result = Codegen::codegen(&parse_result.allocator, program, source_type, &codegen_config)
				.map_err(|e| anyhow::anyhow!("Codegen error: {}", e))?;

			info!(
				"[Compile #{compile_id}] Step 3 complete: Generated {} bytes",
				codegen_result.code.len()
			);

			let output_path = Path::new(file_path).with_extension("js");

			(codegen_result, output_path)
		}; // parse_result dropped here - allocator and AST freed together

		// Write output to file (outside the scope)
		if let Some(parent) = output_path.parent() {
			std::fs::create_dir_all(parent)?;
		}

		let write_start = Instant::now();

		std::fs::write(&output_path, &codegen_result.code)?;

		trace!("[Compile #{compile_id}] File write completed in {:?}", write_start.elapsed());

		let elapsed = begin.elapsed();

		{
			let mut outlook = self.outlook.lock().unwrap();

			outlook.count += 1;

			outlook.elapsed += elapsed;
		}

		info!("[Compile #{compile_id}] COMPLETE: Compiled {} in {:?}", file_path, elapsed);

		Ok(output_path.to_string_lossy().to_string())
	}

	/// Compiles a TypeScript file and writes the output to a specific path.
	///
	/// ## Parameters
	///
	/// * `file_path` — Path to the source file.
	/// * `input` — TypeScript source code.
	/// * `output_path` — Destination path for the compiled JavaScript.
	/// * `use_define_for_class_fields` — VSCode compatibility setting.
	///
	/// ## Returns
	///
	/// The output file path on success, or an error describing the failure.
	#[tracing::instrument(skip(self, input))]
	pub fn compile_file_to(
		&self,

		file_path:&str,

		input:String,

		output_path:&Path,

		use_define_for_class_fields:bool,
	) -> anyhow::Result<String> {
		let compile_id = COMPILE_ID.fetch_add(1, Ordering::SeqCst);

		let begin = Instant::now();

		info!(
			"[Compile #{compile_id}] START compile_file_to: {} -> {}",
			file_path,
			output_path.display()
		);

		trace!(
			"[Compile #{compile_id}] Input size: {} bytes, use_define_for_class_fields={}",
			input.len(),
			use_define_for_class_fields
		);

		// CRITICAL FIX: Wrap entire compilation in its own scope to ensure
		// the allocator is dropped before the next file is processed.
		// This prevents OXC internal state corruption when processing
		// multiple files sequentially.
		let codegen_result = {
			// Parse the TypeScript source
			info!("[Compile #{compile_id}] Step 1/4: Parsing {}", file_path);

			let parse_start = Instant::now();

			let parser_config = self.get_parser_config();

			let mut parse_result = Parser::parse(&input, file_path, &parser_config)
				.map_err(|errors| anyhow::anyhow!("Parse errors: {:?}", errors))?;

			info!(
				"[Compile #{compile_id}] Step 1/4 complete: Parse in {:?}",
				parse_start.elapsed()
			);

			debug!(
				"[Compile #{compile_id}] Memory addresses: allocator={:p}, program={:p}",
				&parse_result.allocator, &parse_result.program
			);

			trace!(
				"[Compile #{compile_id}] AST body slice: ptr={:p}, len={}",
				parse_result.program.body.as_ptr(),
				parse_result.program.body.len()
			);

			// Transform the AST - borrow program mutably, don't move it out
			info!("[Compile #{compile_id}] Step 2/4: Transforming");

			let transform_start = Instant::now();

			let mut transformer_config = self.get_transformer_config();

			transformer_config.use_define_for_class_fields = use_define_for_class_fields;

			// SAFETY: We borrow program with 'static lifetime from parse_result.
			// This is safe because:
			// 1. parse_result stays in scope for the entire inner block
			// 2. The program and allocator are dropped together when parse_result is
			// dropped
			// 3. We never use program after parse_result is dropped
			let program = unsafe {
				std::mem::transmute::<&mut oxc_ast::ast::Program<'static>, &mut oxc_ast::ast::Program<'_>>(
					&mut parse_result.program,
				)
			};

			let source_type = oxc_span::SourceType::from_path(file_path).unwrap_or(oxc_span::SourceType::ts());

			Transformer::transform(&parse_result.allocator, program, file_path, source_type, &transformer_config)
				.map_err(|errors| anyhow::anyhow!("Transform errors: {:?}", errors))?;

			info!(
				"[Compile #{compile_id}] Step 2/4 complete: Transform in {:?}",
				transform_start.elapsed()
			);

			trace!(
				"[Compile #{compile_id}] Program after transform - body ptr: {:p}",
				program.body.as_ptr()
			);

			// Generate code - parse_result still alive
			info!("[Compile #{compile_id}] Step 3/4: Codegen");

			let codegen_start = Instant::now();

			let codegen_config = self.get_codegen_config();

			let codegen_result = Codegen::codegen(&parse_result.allocator, program, source_type, &codegen_config)
				.map_err(|e| anyhow::anyhow!("Codegen error: {}", e))?;

			info!(
				"[Compile #{compile_id}] Step 3/4 complete: Codegen in {:?}, output={}(bytes)",
				codegen_start.elapsed(),
				codegen_result.code.len()
			);

			// Force drop of parse_result to ensure allocator is freed
			// before returning from this scope
			let _ = program;

			// parse_result will be dropped at end of scope
			codegen_result
		}; // parse_result and allocator dropped here - critical for preventing segfault

		// Create parent directories if they don't exist
		if let Some(parent) = output_path.parent() {
			std::fs::create_dir_all(parent)?;
		}

		// Write to the specified output path
		let write_start = Instant::now();

		std::fs::write(output_path, &codegen_result.code)?;

		trace!("[Compile #{compile_id}] Step 4/4: File write in {:?}", write_start.elapsed());

		let elapsed = begin.elapsed();

		{
			let mut outlook = self.outlook.lock().unwrap();

			outlook.count += 1;

			outlook.elapsed += elapsed;
		}

		info!(
			"[Compile #{compile_id}] COMPLETE: {} -> {} in {:?}",
			file_path,
			output_path.display(),
			elapsed
		);

		Ok(output_path.to_string_lossy().to_string())
	}

	/// Returns the parser configuration derived from the compiler config.
	fn get_parser_config(&self) -> ParserConfig {
		ParserConfig::new(
			self.config.Target.clone(),
			self.config.jsx(),
			true, // decorators always enabled for TypeScript
			true, // typescript
		)
	}

	/// Returns the transformer configuration derived from the compiler config.
	fn get_transformer_config(&self) -> TransformerConfig {
		TransformerConfig::new(
			self.config.Target.clone(),
			self.config.module_format(),
			self.config.EmitDecoratorsMetadata,
			false, // default: VSCode compatible
			self.config.jsx(),
			self.config.TreeShaking,
			self.config.Minify,
		)
	}

	/// Returns the codegen configuration derived from the compiler config.
	fn get_codegen_config(&self) -> CodegenConfig {
		CodegenConfig::new(
			self.config.Minify,
			false, // source maps disabled by default
			String::new(),
			false, // comments disabled by default
		)
	}
}

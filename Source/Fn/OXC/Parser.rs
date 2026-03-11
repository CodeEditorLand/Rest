//! OXC TypeScript Parser module
//!
//! This module provides TypeScript source code parsing using the OXC parser.
//!
//! DIAGNOSTIC LOGGING:
//! - All operations log with tracing::debug! for memory lifecycle tracking
//! - Use RUST_LOG=debug to see detailed parser operations

use std::sync::atomic::{AtomicUsize, Ordering};

use oxc_allocator::Allocator;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use tracing::{debug, info, trace, warn};

/// Result of parsing a TypeScript source file
///
/// CRITICAL FIX FOR SEGFAULT:
/// The Program has 'static lifetime via safe transmute. ParseResult owns BOTH
/// the Program AND the Allocator, ensuring they're dropped together.
///
/// The ORIGINAL segfault occurred when code did:
///   let mut program = parse_result.program;  // Moves Program out!
///   Transformer::transform(&parse_result.allocator, ...);
///   // parse_result dropped here -> allocator freed -> program is dangling!
///
/// THE FIX in Compiler.rs - NEVER move Program out:
///   let program = &mut parse_result.program;  // Borrow mutably
///   Transformer::transform(&parse_result.allocator, program, ...);
///   // parse_result stays in scope -> allocator stays alive -> no segfault!
pub struct ParseResult {
	/// The parsed AST program with 'static lifetime (safe transmute)
	pub program:oxc_ast::ast::Program<'static>,
	/// The allocator used for the AST - owns the memory for the Program
	/// CRITICAL: Must not be dropped separately from program
	pub allocator:Allocator,
	/// Any parsing errors encountered
	pub errors:Vec<String>,
	/// File path for debugging
	#[allow(dead_code)]
	pub file_path:String,
}

/// Parser configuration options
#[derive(Debug, Clone)]
pub struct ParserConfig {
	/// Target ECMAScript version (e.g., "es2024")
	pub target:String,
	/// Whether to support JSX syntax
	pub jsx:bool,
	/// Whether to support decorators
	pub decorators:bool,
	/// Whether to support TypeScript
	pub typescript:bool,
}

impl Default for ParserConfig {
	fn default() -> Self { Self { target:"es2024".to_string(), jsx:false, decorators:true, typescript:true } }
}

impl ParserConfig {
	/// Create a new parser configuration
	pub fn new(target:String, jsx:bool, decorators:bool, typescript:bool) -> Self {
		Self { target, jsx, decorators, typescript }
	}
}

/// Parse TypeScript source code into an AST
///
/// # Arguments
/// * `source_text` - The TypeScript source code to parse
/// * `file_path` - The path to the source file (used for determining file type)
/// * `config` - Parser configuration options
///
/// # Returns
/// A ParseResult containing the parsed AST and any errors
static PARSE_COUNT:AtomicUsize = AtomicUsize::new(0);

#[tracing::instrument(skip(source_text, config))]
pub fn parse(source_text:&str, file_path:&str, config:&ParserConfig) -> Result<ParseResult, Vec<String>> {
	let parse_id = PARSE_COUNT.fetch_add(1, Ordering::SeqCst);

	info!("[Parser #{parse_id}] Starting parse of: {}", file_path);
	trace!("[Parser #{parse_id}] Source text length: {} bytes", source_text.len());

	let allocator = Allocator::default();
	trace!("[Parser #{parse_id}] Allocator created at: {:p}", &allocator);

	// Determine source type based on file extension and config
	let source_type = determine_source_type(file_path, config);
	info!("[Parser #{parse_id}] Parsing {} as {:?}", file_path, source_type);

	let parse_start = std::time::Instant::now();
	// Parse the source code
	let parser_return:ParserReturn = Parser::new(&allocator, source_text, source_type).parse();
	info!("[Parser #{parse_id}] Parse completed in {:?}", parse_start.elapsed());

	let errors:Vec<String> = parser_return.errors.iter().map(|e| e.to_string()).collect();

	if !errors.is_empty() {
		warn!("[Parser #{parse_id}] Parsing errors for {}: {:?}", file_path, errors);
		return Err(errors);
	}

	// SAFETY: Transmute program lifetime to 'static.
	// This is SAFE because:
	// 1. ParseResult owns BOTH the Program AND the Allocator
	// 2. The Program's memory comes from the Allocator
	// 3. Both are dropped together when ParseResult is dropped
	// 4. The Program NEVER outlives the Allocator
	//
	// CRITICAL BUG FIX: The original segfault happened because code did:
	//   let mut program = parse_result.program;  // Moves Program out!
	//   Transformer::transform(&parse_result.allocator, ...);
	//   // parse_result dropped -> allocator freed -> program is dangling!
	//
	// THE FIX in Compiler.rs:
	//   let program = &mut parse_result.program;  // Borrow mutably, don't move!
	//   Transformer::transform(&parse_result.allocator, program, ...);
	//   // parse_result stays in scope -> allocator stays alive
	let program = unsafe {
		std::mem::transmute::<oxc_ast::ast::Program<'_>, oxc_ast::ast::Program<'static>>(parser_return.program)
	};

	debug!("[Parser #{parse_id}] Program transmute complete at: {:p}", &program);
	trace!(
		"[Parser #{parse_id}] AST body pointer: {:p}, len: {}",
		program.body.as_ptr(),
		program.body.len()
	);

	info!(
		"[Parser #{parse_id}] SUCCESS: ParseResult<'static> created (allocator={:p})",
		&allocator
	);

	Ok(ParseResult { program, allocator, errors:vec![], file_path:file_path.to_string() })
}

/// Determine the source type based on file path and configuration
fn determine_source_type(file_path:&str, _config:&ParserConfig) -> SourceType {
	let path = std::path::Path::new(file_path);
	let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

	match extension {
		"ts" | "mts" => SourceType::ts(),
		"tsx" => SourceType::tsx(),
		"mjs" => SourceType::mjs(),
		"cjs" => SourceType::cjs(),
		"js" => SourceType::unambiguous(),
		"jsx" => SourceType::jsx(),
		_ => SourceType::unambiguous(),
	}
}

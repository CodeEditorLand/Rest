//! OXC-based compilation module
//!
//! This module provides the compilation functionality using the OXC compiler.
//!
//! DIAGNOSTIC LOGGING:
//! - Full lifecycle tracking of file compilation
//! - Memory allocation tracking for each file
//! - File path and nesting level analysis

use std::{
	io::Write,
	sync::atomic::{AtomicUsize, Ordering},
};

use tracing::{debug, error, info, trace, warn};

static FILE_PROCESS_COUNT:AtomicUsize = AtomicUsize::new(0);

/// Calculate directory nesting depth for a path
fn get_nesting_depth(path:&str) -> usize {
	path.chars().filter(|&c| c == std::path::MAIN_SEPARATOR || c == '/').count()
}

#[tracing::instrument(skip(options))]
/// Compiles TypeScript files from input directory to output directory
///
/// # Arguments
///
/// * `options` - Compilation options including entry, pattern, config, output
///   directory
/// * `_parallel` - Whether to use parallel compilation (currently unused - runs
///   sequentially)
pub async fn Fn(options:crate::Struct::SWC::Option, _parallel:bool) -> anyhow::Result<()> {
	info!("=== OXC Compilation START ===");
	tracing_subscriber::fmt::init();

	let compiler = std::sync::Arc::new(crate::Fn::OXC::Compiler::Compiler::new(options.config.clone()));

	// Get the input base path
	let input_base = options.entry[0][0].clone();
	let output_base = options.output.clone();
	let pattern = options.pattern.clone();

	info!("Compilation from {} to {}", input_base, output_base);
	debug!("Pattern: {}, Parallel: {}", pattern, _parallel);

	// Use walkdir to find all TypeScript files in the input directory
	let walk_start = std::time::Instant::now();
	let ts_files:Vec<String> = walkdir::WalkDir::new(&input_base)
		.follow_links(true)
		.into_iter()
		.filter_map(|e| {
			let entry = e.ok()?;
			let path = entry.path();
			if path.is_file() && path.to_string_lossy().ends_with(&pattern) {
				Some(path.to_string_lossy().to_string())
			} else {
				None
			}
		})
		.collect();
	info!(
		"File discovery completed in {:?}, found {} TypeScript files",
		walk_start.elapsed(),
		ts_files.len()
	);

	// Analyze file distribution by nesting depth
	let mut depth_dist = std::collections::HashMap::new();
	for f in &ts_files {
		let depth = get_nesting_depth(f);
		*depth_dist.entry(depth).or_insert(0) += 1;
	}
	debug!("File distribution by depth: {:?}", depth_dist);

	// Sort files by nesting depth to process nested files last (diagnostic)
	let mut sorted_files = ts_files.clone();
	sorted_files.sort_by_key(|f| get_nesting_depth(f));

	trace!("File list (sorted by depth):");
	for (i, f) in sorted_files.iter().enumerate() {
		trace!("  [{}] {} (depth={})", i, f, get_nesting_depth(f));
	}

	// Process files sequentially to avoid OXC globals issues
	let mut count = 0;
	let mut error = 0;
	let mut current_file = 0;

	let total_files = sorted_files.len();
	info!("Starting sequential file processing ({} files)...", total_files);
	for file_path in sorted_files {
		current_file += 1;
		let file_id = FILE_PROCESS_COUNT.fetch_add(1, Ordering::SeqCst);
		let depth = get_nesting_depth(&file_path);

		print!(".");
		std::io::stdout().flush().unwrap();

		info!(
			"[File #{file_id}] Processing [{}/{}]: {} (depth={})",
			current_file, total_files, file_path, depth
		);

		match tokio::fs::read_to_string(&file_path).await {
			Ok(input) => {
				trace!("[File #{file_id}] Read {} bytes", input.len());

				// Calculate relative path from input base
				let input_path = std::path::Path::new(&file_path);
				let base_path = std::path::Path::new(&input_base);
				let relative_path = input_path.strip_prefix(base_path).unwrap_or(input_path);

				// Create output path preserving directory structure
				let output_path = std::path::Path::new(&output_base).join(relative_path).with_extension("js");

				debug!("[File #{file_id}] Output path: {}", output_path.display());

				let compile_start = std::time::Instant::now();
				match compiler.compile_file_to(&file_path, input, &output_path, options.use_define_for_class_fields) {
					Ok(output) => {
						info!(
							"[File #{file_id}] SUCCESS in {:?}: {} -> {}",
							compile_start.elapsed(),
							file_path,
							output
						);
						count += 1;
					},
					Err(e) => {
						error!(
							"[File #{file_id}] FAILED in {:?}: {} - Error: {}",
							compile_start.elapsed(),
							file_path,
							e
						);
						error += 1;
					},
				}
			},
			Err(e) => {
				error!("[File #{file_id}] READ FAILED: {} - Error: {}", file_path, e);
				error += 1;
			},
		}
	}

	println!();

	let outlook = compiler.outlook.lock().unwrap();

	info!("=== OXC Compilation COMPLETE ===");
	info!(
		"Total: {} files, Successful: {}, Failed: {}, Time: {:?}",
		outlook.count, count, error, outlook.elapsed
	);

	// Print summary
	println!("\n=== Compilation Summary ===");
	println!("Total files processed: {}", outlook.count);
	println!("Successful: {}", count);
	println!("Failed: {}", error);
	println!("Time elapsed: {:?}\n", outlook.elapsed);

	Ok(())
}

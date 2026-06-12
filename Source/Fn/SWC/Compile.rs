//! SWC-compatible compilation module (uses OXC backend).
//!
//! Provides the same compilation functionality as the original SWC compiler
//! but uses OXC for parsing and transformation.

#[tracing::instrument(skip(options))]
/// Compiles TypeScript files from the input directory to the output directory
/// using the OXC-backed compiler pipeline.
///
/// ## Parameters
///
/// * `options` — Compilation options including entry paths, pattern, config,
///   and output directory.
/// * `_parallel` — Whether to use parallel compilation (currently unused;
///   runs sequentially).
pub async fn Fn(options:crate::Struct::SWC::Option, _parallel:bool) -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();

	// Use OXC compiler instead of SWC
	let compiler = std::sync::Arc::new(crate::Fn::OXC::Compiler::Compiler::new(options.config.clone()));

	// Get the input base path
	let input_base = options.entry[0][0].clone();

	let output_base = options.output.clone();

	let pattern = options.pattern.clone();

	println!("Starting compilation from {} to {}", input_base, output_base);

	// Use walkdir to find all TypeScript files in the input directory
	// Exclude .d.ts declaration files to match VSCode's build process
	let ts_files:Vec<String> = walkdir::WalkDir::new(&input_base)
		.follow_links(true)
		.into_iter()
		.filter_map(|e| {
			let entry = e.ok()?;

			let path = entry.path();

			let path_str = path.to_string_lossy();

			// Skip .d.ts declaration files (like VSCode's noDeclarationsFilter)
			if path_str.ends_with(".d.ts") {
				None
			} else if path.is_file() && path_str.ends_with(&pattern) {
				Some(path_str.to_string())
			} else {
				None
			}
		})
		.collect();

	println!("Found {} TypeScript files in {}", ts_files.len(), input_base);

	// Process files sequentially to avoid OXC globals issues
	let mut count = 0;

	let mut error = 0;

	for file_path in ts_files {
		print!(".");

		std::io::stdout().flush().unwrap();

		match tokio::fs::read_to_string(&file_path).await {
			Ok(input) => {
				// Calculate relative path from input base
				let input_path = std::path::Path::new(&file_path);

				let base_path = std::path::Path::new(&input_base);

				let relative_path = input_path.strip_prefix(base_path).unwrap_or(input_path);

				// Create output path preserving directory structure
				let output_path = std::path::Path::new(&output_base).join(relative_path).with_extension("js");

				match compiler.compile_file_to(&file_path, input, &output_path, options.use_define_for_class_fields) {
					Ok(output) => {
						debug!("Compiled: {} -> {}", file_path, output);

						count += 1;
					},

					Err(e) => {
						error!("Compilation error for {}: {}", file_path, e);

						error += 1;
					},
				}
			},

			Err(e) => {
				error!("Failed to read file {}: {}", file_path, e);

				error += 1;
			},
		}
	}

	println!();

	let outlook = compiler.outlook.lock().unwrap();

	info!(
		"Compilation complete. Processed {} files in {:?}. {} successful, {} failed.",
		outlook.count, outlook.elapsed, count, error
	);

	// Print summary
	println!("\n=== Compilation Summary ===");

	println!("Total files processed: {}", outlook.count);

	println!("Successful: {}", count);

	println!("Failed: {}", error);

	println!("Time elapsed: {:?}\n", outlook.elapsed);

	Ok(())
}

use std::io::Write;

use tracing::{debug, error, info};

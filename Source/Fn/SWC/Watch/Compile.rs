//! OXC-based watch compile module
//!
//! This module provides watch-mode compilation using the OXC compiler.

use std::io::Write;

#[tracing::instrument(skip(options))]
/// Compile all watched files using SWC with the given options.
pub async fn Fn(options:crate::Struct::SWC::Option) -> anyhow::Result<()> {
	let compiler = std::sync::Arc::new(crate::Fn::OXC::Compiler::Compiler::new(options.config.clone()));

	// Get the input base path
	let input_base = options.entry[0][0].clone();

	let output_base = options.output.clone();

	let pattern = options.pattern.clone();

	println!("Starting watch compilation from {} to {}", input_base, output_base);

	// Use walkdir to find all TypeScript files in the input directory
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

	println!("Found {} TypeScript files in {}", ts_files.len(), input_base);

	// Process files sequentially
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
		"Watch compilation complete. Processed {} files in {:?}. {} successful, {} failed.",
		outlook.count, outlook.elapsed, count, error
	);

	Ok(())
}

use tracing::{debug, error, info};

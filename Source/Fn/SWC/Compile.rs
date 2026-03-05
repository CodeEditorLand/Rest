#[tracing::instrument(skip(Option))]
/// Compiles TypeScript files from input directory to output directory
/// 
/// # Arguments
/// 
/// * `Option` - Compilation options including entry, pattern, config, output directory
/// * `_Parallel` - Whether to use parallel compilation (currently unused - runs sequentially)
pub async fn Fn(Option:crate::Struct::SWC::Option, _Parallel:bool) -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	
	let Compiler = std::sync::Arc::new(crate::Struct::SWC::Compiler::new(Option.config.clone()));

	// Get the input base path
	let InputBase = Option.entry[0][0].clone();
	let OutputBase = Option.output.clone();
	let Pattern = Option.pattern.clone();

	println!("Starting compilation from {} to {}", InputBase, OutputBase);

	// Use walkdir to find all TypeScript files in the input directory
	let ts_files: Vec<String> = WalkDir::new(&InputBase)
		.follow_links(true)
		.into_iter()
		.filter_map(|e| {
			let entry = e.ok()?;
			let path = entry.path();
			if path.is_file() && path.to_string_lossy().ends_with(&Pattern) {
				Some(path.to_string_lossy().to_string())
			} else {
				None
			}
		})
		.collect();

	println!("Found {} TypeScript files in {}", ts_files.len(), InputBase);

	// Process files sequentially to avoid SWC globals issues
	let mut Count = 0;
	let mut Error = 0;
	
	for file_path in ts_files {
		print!(".");
		std::io::stdout().flush().unwrap();
		
		match tokio::fs::read_to_string(&file_path).await {
			Ok(input) => {
				// Calculate relative path from input base
				let input_path = std::path::Path::new(&file_path);
				let base_path = std::path::Path::new(&InputBase);
				let relative_path = input_path.strip_prefix(base_path).unwrap_or(input_path);

				// Create output path preserving directory structure
				let output_path = Path::new(&OutputBase).join(relative_path).with_extension("js");

				match Compiler.compile_file_to(&file_path, input, &output_path, Option.use_define_for_class_fields) {
					Ok(output) => {
						debug!("Compiled: {} -> {}", file_path, output);
						Count += 1;
					},
					Err(e) => {
						error!("Compilation error for {}: {}", file_path, e);
						Error += 1;
					},
				}
			},
			Err(e) => {
				error!("Failed to read file {}: {}", file_path, e);
				Error += 1;
			},
		}
	}

	println!();

	let Outlook = Compiler.Outlook.lock().unwrap();

	info!(
		"Compilation complete. Processed {} files in {:?}. {} successful, {} failed.",
		Outlook.Count, Outlook.Elapsed, Count, Error
	);

	// Print summary
	println!("\n=== Compilation Summary ===");
	println!("Total files processed: {}", Outlook.Count);
	println!("Successful: {}", Count);
	println!("Failed: {}", Error);
	println!("Time elapsed: {:?}\n", Outlook.Elapsed);

	Ok(())
}

use std::{path::Path, io::Write};

use walkdir::WalkDir;
use tracing::{error, debug, info};
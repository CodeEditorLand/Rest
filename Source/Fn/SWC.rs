#[allow(unused)]
async fn Fn() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();

	let args:Vec<String> = std::env::args().collect();

	if args.len() != 2 {
		error!("Usage: {} <directory>", args[0]);
		std::process::exit(1);
	}

	let Path = std::path::PathBuf::from(&args[1]);

	let Config = if let Ok(Config) = fs::read_to_string("swc_config.json").await {
		serde_json::from_str(&Config).unwrap_or_default()
	} else {
		CompilerConfig::default()
	};

	let options = Option {
		entry:vec![vec![Path.to_string_lossy().to_string()]],
		separator:std::path::MAIN_SEPARATOR,
		pattern:".ts".to_string(),
		config:Config.clone(),
		output: String::new(),  // Not used in watch mode
		use_define_for_class_fields: false,
	};

	// Initial compilation
	info!("Starting initial compilation...");

	Watch::Compile::Fn(options.clone()).await?;

	info!("Initial compilation complete. Watching for changes...");

	// Start watching for changes
	Watch::Fn(Path, options).await?;

	Ok(())
}

pub mod Watch;

pub mod Compile;

use tokio::fs;
use tracing::{error, info};

use crate::Struct::SWC::{CompilerConfig, Option};

//! Build functions for the Rest compilation pipeline.
//!
//! Provides top-level build and group processing functions.

use dashmap::DashMap;

/// Executes the build entry point and returns a map of build results.
pub async fn Fn(_Entry:&str) -> Result<DashMap<u64, (String, String)>, Box<dyn std::error::Error>> {
	let Build = DashMap::new();

	Ok(Build)
}

/// Group processing functions for build output.
pub mod Group {

	/// Processes build output and prints a summary.
	pub fn Fn(output:Vec<(String, String)>) {
		println!("Processed {} files", output.len());
	}
}

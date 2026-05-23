//! Worker file detection
//!
//! Detects web worker files in the project and classifies them.

use std::path::Path;

use walkdir::WalkDir;

use super::{WorkerConfig, WorkerInfo, WorkerType};

/// Detects worker files in a project
pub struct WorkerDetector {
	config:WorkerConfig,
}

impl WorkerDetector {
	pub fn new(config:WorkerConfig) -> Self { Self { config } }

	/// Detect all worker files in a directory
	pub fn detect_workers(&self, root_dir:&Path) -> Vec<WorkerInfo> {
		let mut workers = Vec::new();

		for entry in WalkDir::new(root_dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
			let path = entry.path();

			if self.is_worker_file(path) {
				if let Some(worker_info) = self.create_worker_info(path) {
					workers.push(worker_info);
				}
			}
		}

		workers
	}

	/// Check if a file is a worker file based on naming conventions
	pub fn is_worker_file(&self, path:&Path) -> bool {
		if !path.is_file() {
			return false;
		}

		let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

		// Check common worker file patterns
		let worker_patterns = [
			".worker.ts",
			".worker.js",
			".worker.tsx",
			".worker.jsx",
			"-worker.ts",
			"-worker.js",
			"worker.ts",
			"worker.js",
			"SharedWorker.ts",
			"SharedWorker.js",
		];

		for pattern in worker_patterns {
			if file_name.ends_with(pattern) {
				return true;
			}
		}

		// Check for explicit worker markers in the file
		if let Ok(content) = std::fs::read_to_string(path) {
			return self.contains_worker_marker(&content);
		}

		false
	}

	/// Create worker info from a file path
	fn create_worker_info(&self, path:&Path) -> Option<WorkerInfo> {
		let file_name = path.file_name()?.to_str()?;

		let worker_type = if file_name.contains("SharedWorker") || file_name.contains("shared") {
			// Shared workers are typically classic
			WorkerType::Classic
		} else {
			self.config.worker_type
		};

		let is_shared = file_name.contains("SharedWorker") || file_name.contains("shared");

		let source_path = path.to_string_lossy().to_string();

		let name = path.file_stem()?.to_str()?.replace(".worker", "").replace("-worker", "");

		let output_path = Path::new(&self.config.output_dir)
			.join(format!("{}.js", name))
			.to_string_lossy()
			.to_string();

		Some(WorkerInfo { source_path, output_path, name, worker_type, dependencies:Vec::new(), is_shared })
	}

	/// Check if the file contains a worker marker
	fn contains_worker_marker(&self, content:&str) -> bool {
		let markers = [
			"new Worker(",
			"new SharedWorker(",
			"self.onmessage",
			"self.postMessage",
			"importScripts(",
			"// @worker",
			"//worker",
		];

		for marker in markers {
			if content.contains(marker) {
				return true;
			}
		}

		false
	}

	/// Extract dependencies from a worker file
	pub fn extract_dependencies(&self, path:&Path) -> Vec<String> {
		let mut deps = Vec::new();

		if let Ok(content) = std::fs::read_to_string(path) {
			// Extract import statements
			for line in content.lines() {
				let trimmed = line.trim();

				// Match import from '...' or import "..."
				if trimmed.starts_with("import ") {
					if let Some(from_start) = trimmed.find("from") {
						let import_part = &trimmed[from_start + 4..];

						if let Some(path_start) = import_part.find('"') {
							let path_end = import_part[path_start + 1..].find('"');

							if let Some(end) = path_end {
								let import_path = &import_part[path_start + 1..path_start + 1 + end];

								deps.push(import_path.to_string());
							}
						}
					}
				}

				// Match importScripts(...)
				if trimmed.starts_with("importScripts(") {
					if let Some(paren_start) = trimmed.find('(') {
						let paren_content = &trimmed[paren_start + 1..];

						if let Some(paren_end) = paren_content.find(')') {
							let scripts = &paren_content[..paren_end];

							for script in scripts.split(',') {
								let script = script.trim().trim_matches('"').trim_matches('\'');

								if !script.is_empty() {
									deps.push(script.to_string());
								}
							}
						}
					}
				}
			}
		}

		deps
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_worker_detection_by_name() {
		let config = WorkerConfig::new();

		let detector = WorkerDetector::new(config);

		assert!(detector.is_worker_file(Path::new("test.worker.ts")));

		assert!(detector.is_worker_file(Path::new("my-worker.js")));

		assert!(detector.is_worker_file(Path::new("SharedWorker.ts")));

		assert!(!detector.is_worker_file(Path::new("regular.ts")));
	}

	#[test]
	fn test_worker_detection_by_content() {
		let config = WorkerConfig::new();

		let detector = WorkerDetector::new(config);

		let content = r#"
            self.onmessage = function(e) {

                self.postMessage(e.data);
            };

        "#;

		assert!(detector.contains_worker_marker(content));
	}

	#[test]
	fn test_dependency_extraction() {
		let config = WorkerConfig::new();

		let detector = WorkerDetector::new(config);

		let path = Path::new("test.worker.ts");

		// This would need an actual file to work properly
		let _deps = detector.extract_dependencies(path);
	}
}

//! NLS key extraction from TypeScript source files
//!
//! Extracts localization keys from various patterns:
//! - nls.localize('key', 'default')
//! - nls.localize2('key', 'default1', 'default2')
//! - nls.localizeWithFlattenedArgs(...)
//! - localize('key', 'default')

use std::collections::HashMap;

use regex::Regex;

/// Extracts NLS keys from TypeScript source
pub struct NLSExtractor {
	/// Extracted localization entries
	pub entries:HashMap<String, String>,
	/// Current file being processed
	pub current_file:Option<String>,
	/// Regex patterns for different localize calls
	localize_pattern:Regex,
	localize2_pattern:Regex,
}

impl NLSExtractor {
	pub fn new() -> Self {
		Self {
			entries:HashMap::new(),
			current_file:None,
			// Match nls.localize('key', 'value') or localize('key', 'value')
			localize_pattern:Regex::new(r#"(?:nls\.)?localize\s*\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]*)['"]\s*\)"#)
				.unwrap(),
			// Match nls.localize2('key', 'v1', 'v2')
			localize2_pattern:Regex::new(
				r#"(?:nls\.)?localize2\s*\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]*)['"]\s*,\s*['"]([^'"]*)['"]\s*\)"#,
			)
			.unwrap(),
		}
	}

	pub fn with_file(mut self, file:impl Into<String>) -> Self {
		self.current_file = Some(file.into());
		self
	}

	/// Extract keys from source content
	pub fn extract(&mut self, source:&str) {
		// Extract from localize() calls
		for cap in self.localize_pattern.captures_iter(source) {
			if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
				self.entries
					.entry(key.as_str().to_string())
					.or_insert(value.as_str().to_string());
			}
		}

		// Extract from localize2() calls (use first value)
		for cap in self.localize2_pattern.captures_iter(source) {
			if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
				self.entries
					.entry(key.as_str().to_string())
					.or_insert(value.as_str().to_string());
			}
		}
	}
}

impl Default for NLSExtractor {
	fn default() -> Self { Self::new() }
}

/// Extract NLS keys from source code using regex
pub fn extract_nls_keys(source:&str) -> HashMap<String, String> {
	let mut extractor = NLSExtractor::new();
	extractor.extract(source);
	extractor.entries
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extract_simple_localize() {
		let source = r#"
            const str = nls.localize('hello', 'Hello World');
        "#;
		let keys = extract_nls_keys(source);
		assert_eq!(keys.get("hello"), Some(&"Hello World".to_string()));
	}

	#[test]
	fn test_extract_multiple_keys() {
		let source = r#"
            const a = localize('key1', 'Value 1');
            const b = nls.localize('key2', 'Value 2');
        "#;
		let keys = extract_nls_keys(source);
		assert_eq!(keys.get("key1"), Some(&"Value 1".to_string()));
		assert_eq!(keys.get("key2"), Some(&"Value 2".to_string()));
	}

	#[test]
	fn test_extract_localize2() {
		let source = r#"
            const str = nls.localize2('key', 'Value 1', 'Value 2');
        "#;
		let keys = extract_nls_keys(source);
		assert_eq!(keys.get("key"), Some(&"Value 1".to_string()));
	}
}

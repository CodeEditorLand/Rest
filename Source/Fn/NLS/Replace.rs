//! NLS key replacement transform
//!
//! Replaces NLS keys with actual localized strings at build time.
//! This is used when inlining translations into the output.

use std::collections::HashMap;

use regex::Regex;

/// Replaces NLS localization calls with actual string values
pub struct NLSReplacer {
	/// The localization bundle to use for replacements
	bundle:HashMap<String, String>,

	/// Whether to preserve the localize call structure
	preserve_calls:bool,

	/// Regex patterns for localize calls
	localize_pattern:Regex,

	_localize2_pattern:Regex,
}

impl NLSReplacer {
	/// Create a new [`NLSReplacer`] with the given translation bundle.
	pub fn new(bundle:HashMap<String, String>) -> Self {
		Self {
			bundle,

			preserve_calls:false,

			localize_pattern:Regex::new(r#"(?:nls\.)?localize\s*\(\s*['"]([^'"]+)['"]"#).unwrap(),

			_localize2_pattern:Regex::new(r#"(?:nls\.)?localize2\s*\(\s*['"]([^'"]+)['"]"#).unwrap(),
		}
	}

	/// Set whether to preserve original function call syntax during
	/// replacement.
	pub fn with_preserve_calls(mut self, preserve:bool) -> Self {
		self.preserve_calls = preserve;

		self
	}

	/// Replace NLS keys in source code
	pub fn replace(&self, source:&str) -> String {
		let mut result = source.to_string();

		// Replace localize calls
		if let Some(caps) = self.localize_pattern.captures(&result) {
			if let Some(key) = caps.get(1) {
				let key_str = key.as_str();

				if let Some(value) = self.bundle.get(key_str) {
					let pattern = format!(r#"nls.localize\s*\(\s*['"]{}.*?\)"#, regex::escape(key_str));

					let re = Regex::new(&pattern).unwrap();

					if self.preserve_calls {
						// Keep the call but with replacement
						result = re
							.replace_all(&result, format!("/* localize('{}') */ '{}'", key_str, value))
							.to_string();
					} else {
						// Replace with just the string value
						result = re.replace_all(&result, format!("'{}'", value)).to_string();
					}
				}
			}
		}

		result
	}
}

/// Replace NLS keys in source code with translations
pub fn replace_nls_keys(source:&str, bundle:&HashMap<String, String>) -> String {
	let replacer = NLSReplacer::new(bundle.clone());

	replacer.replace(source)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_replacer_creation() {
		let bundle = HashMap::new();

		let replacer = NLSReplacer::new(bundle);

		assert!(!replacer.preserve_calls);
	}

	#[test]
	fn test_replacer_with_preserve() {
		let bundle = HashMap::new();

		let replacer = NLSReplacer::new(bundle).with_preserve_calls(true);

		assert!(replacer.preserve_calls);
	}

	#[test]
	fn test_replace_keys() {
		let mut bundle = HashMap::new();

		bundle.insert("hello".to_string(), "Hello World".to_string());

		let source = r#"nls.localize('hello', 'default')"#;

		let result = replace_nls_keys(source, &bundle);

		assert!(result.contains("Hello World"));
	}
}

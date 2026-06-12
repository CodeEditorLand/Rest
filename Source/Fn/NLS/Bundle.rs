//! NLS bundle generation and management.
//!
//! Creates and manages localization bundles for different languages.

use std::{collections::HashMap, path::Path};

use super::{LocalizationBundle, NLSConfig};

/// Generates NLS bundles from extracted keys for multiple languages.
pub struct NLSBundle {
	_config:NLSConfig,

	/// Bundle for each language.
	bundles:HashMap<String, LocalizationBundle>,
}

impl NLSBundle {
	/// Creates a new [`NLSBundle`] with the given configuration.
	pub fn new(config:NLSConfig) -> Self {
		let mut bundles = HashMap::new();

		// Initialize bundles for each language
		for lang in &config.languages {
			bundles.insert(lang.clone(), LocalizationBundle::new(lang));
		}

		Self { _config:config, bundles }
	}

	/// Adds a localization entry to a specific language.
	pub fn add_entry(&mut self, language:&str, key:impl Into<String>, value:impl Into<String>) {
		if let Some(bundle) = self.bundles.get_mut(language) {
			bundle.add_entry(key, value);
		}
	}

	/// Adds entries from a key-value map to a language.
	pub fn add_entries(&mut self, language:&str, entries:&HashMap<String, String>) {
		if let Some(bundle) = self.bundles.get_mut(language) {
			for (key, value) in entries {
				bundle.add_entry(key.clone(), value.clone());
			}

			bundle.compute_hash();
		}
	}

	/// Generates bundle JSON files for all languages in the output directory.
	///
	/// ## Errors
	///
	/// Returns `std::io::Error` if directory creation or file writing fails.
	pub fn generate(&self, output_dir:&Path) -> std::io::Result<()> {
		use std::fs;

		// Create output directory if it doesn't exist
		fs::create_dir_all(output_dir)?;

		for (lang, bundle) in &self.bundles {
			let filename = format!("{}.json", lang);

			let path = output_dir.join(&filename);

			let json =
				serde_json::to_string_pretty(bundle).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

			fs::write(path, json)?;
		}

		Ok(())
	}

	/// Returns the bundle for a specific language, if present.
	pub fn get_bundle(&self, language:&str) -> Option<&LocalizationBundle> { self.bundles.get(language) }

	/// Returns all language bundles.
	pub fn all_bundles(&self) -> &HashMap<String, LocalizationBundle> { &self.bundles }

	/// Loads a bundle from a JSON file.
	///
	/// ## Errors
	///
	/// Returns `std::io::Error` if the file cannot be read, parsed, or if the
	/// language in the file does not match the expected language.
	pub fn load_bundle(language:&str, path:&Path) -> std::io::Result<LocalizationBundle> {
		let content = std::fs::read_to_string(path)?;

		let bundle:LocalizationBundle =
			serde_json::from_str(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

		// Verify language matches
		if bundle.language != language {
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				format!("Language mismatch: expected {}, got {}", language, bundle.language),
			));
		}

		Ok(bundle)
	}

	/// Converts a bundle into a `HashMap` for use with `NLSReplacer`.
	pub fn to_hashmap(&self, language:&str) -> HashMap<String, String> {
		let mut map = HashMap::new();

		if let Some(bundle) = self.bundles.get(language) {
			for entry in &bundle.entries {
				map.insert(entry.key.clone(), entry.value.clone());
			}
		}

		map
	}
}

/// Generates a VSCode-compatible NLS bundle from a key-value map.
pub fn generate_vscode_bundle(entries:&HashMap<String, String>, language:&str) -> LocalizationBundle {
	let mut bundle = LocalizationBundle::new(language);

	for (key, value) in entries {
		bundle.add_entry(key, value);
	}

	bundle.compute_hash();

	bundle
}

/// Formats a bundle as a VSCode `nls.metadata.json`-style JSON value.
pub fn format_metadata(bundle:&LocalizationBundle) -> serde_json::Value {
	let mut metadata = serde_json::Map::new();

	for entry in &bundle.entries {
		let mut item = serde_json::Map::new();

		item.insert("value".to_string(), serde_json::Value::String(entry.value.clone()));

		if let Some(comment) = &entry.comment {
			item.insert("comment".to_string(), serde_json::Value::String(comment.clone()));
		}

		metadata.insert(entry.key.clone(), serde_json::Value::Object(item));
	}

	serde_json::Value::Object(metadata)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_bundle_creation() {
		let config = NLSConfig { languages:vec!["en".to_string(), "de".to_string()], ..Default::default() };

		let bundle = NLSBundle::new(config);

		assert!(bundle.get_bundle("en").is_some());

		assert!(bundle.get_bundle("de").is_some());
	}

	#[test]
	fn test_add_entry() {
		let config = NLSConfig { languages:vec!["en".to_string()], ..Default::default() };

		let mut bundle = NLSBundle::new(config);

		bundle.add_entry("en", "hello", "Hello World");

		let en_bundle = bundle.get_bundle("en").unwrap();

		assert_eq!(en_bundle.entries.len(), 1);

		assert_eq!(en_bundle.entries[0].key, "hello");
	}

	#[test]
	fn test_to_hashmap() {
		let config = NLSConfig { languages:vec!["en".to_string()], ..Default::default() };

		let mut bundle = NLSBundle::new(config);

		bundle.add_entry("en", "hello", "Hello World");

		let map = bundle.to_hashmap("en");

		assert_eq!(map.get("hello"), Some(&"Hello World".to_string()));
	}
}

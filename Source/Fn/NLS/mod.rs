//! NLS (National Language Support) processing for VSCode localization.
//!
//! Handles extraction of localization keys from TypeScript source, generation
//! of localization bundle files, and replacement of localization keys with
//! actual strings at build time.
//!
//! ## Modules
//!
//! * [`Extract`] — Key extraction from source code.
//! * [`Replace`] — Key replacement with localized strings.
//! * [`Bundle`] — Bundle generation and management.

#[path = "Extract.rs"]
pub mod Extract;

#[path = "Replace.rs"]
pub mod Replace;

#[path = "Bundle.rs"]
pub mod Bundle;

pub use Extract::NLSExtractor;
pub use Replace::NLSReplacer;
pub use Bundle::NLSBundle;

/// Configuration for NLS processing.
#[derive(Debug, Clone, Default)]
pub struct NLSConfig {
	/// Source language for localization (default: `"en"`).
	pub source_lang:String,

	/// Output directory for generated localization files.
	pub output_dir:String,

	/// Whether to inline translations into the output.
	pub inline:bool,

	/// File pattern for localization keys (default: `"*.nls.*"`).
	pub key_pattern:String,

	/// Additional languages to generate.
	pub languages:Vec<String>,
}

impl NLSConfig {
	/// Creates a new [`NLSConfig`] with default settings.
	pub fn new() -> Self {
		Self {
			source_lang:"en".to_string(),

			output_dir:"out/nls".to_string(),

			inline:false,

			key_pattern:"*.nls.*".to_string(),

			languages:vec!["en".to_string()],
		}
	}
}

/// A localization entry mapping a key to its translated value.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalizationEntry {
	/// The key used to identify this string.
	pub key:String,

	/// The localized string value.
	pub value:String,

	/// Optional comment for translators.
	pub comment:Option<String>,
}

/// A collection of localization entries for a specific language.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LocalizationBundle {
	/// Language code (e.g., `"en"`, `"de"`, `"fr"`).
	pub language:String,

	/// Hash of the source strings for cache invalidation.
	pub hash:String,

	/// The localization entries.
	pub entries:Vec<LocalizationEntry>,
}

impl LocalizationBundle {
	/// Creates a new [`LocalizationBundle`] for the given language.
	pub fn new(language:&str) -> Self { Self { language:language.to_string(), hash:String::new(), entries:Vec::new() } }

	/// Adds a localization entry to this bundle.
	pub fn add_entry(&mut self, key:impl Into<String>, value:impl Into<String>) {
		self.entries
			.push(LocalizationEntry { key:key.into(), value:value.into(), comment:None });
	}

	/// Computes a hash for cache invalidation from all entries.
	pub fn compute_hash(&mut self) {
		use std::{
			collections::hash_map::DefaultHasher,
			hash::{Hash, Hasher},
		};

		let mut hasher = DefaultHasher::new();

		for entry in &self.entries {
			entry.key.hash(&mut hasher);

			entry.value.hash(&mut hasher);
		}

		self.hash = format!("{:x}", hasher.finish());
	}
}

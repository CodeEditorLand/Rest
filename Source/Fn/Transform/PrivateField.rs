//! Private field conversion transform
//!
//! Converts TypeScript private fields (#field) to regular properties (__field)
//! for VSCode compatibility. VSCode does this for performance reasons.

/// Configuration for the private field transform
#[derive(Debug, Clone, Default)]
pub struct Config {
	/// Prefix to use for converted private fields
	pub prefix:String,

	/// Whether to preserve the original private identifier in comments
	pub preserve_comments:bool,
}

impl Config {
	/// Create a new Config with default values.
	pub fn new() -> Self { Self { prefix:"__".to_string(), preserve_comments:true } }
}

/// Transform that converts TypeScript private fields (#field) to regular
/// properties
///
/// VSCode performs this conversion for performance reasons, as private fields
/// using Symbol have more overhead than regular properties.
///
/// This module provides the configuration and interface. The actual transform
/// is applied during compilation using the SWC visitor pattern.
pub struct PrivateFieldTransform {
	config:Config,
}

impl PrivateFieldTransform {
	/// Create a new [`PrivateFieldTransform`] with default config.
	pub fn new() -> Self { Self { config:Config::new() } }

	/// Configure the private field transform with a custom config.
	pub fn with_config(config:Config) -> Self { Self { config } }

	/// Convert a private identifier to a regular identifier name
	pub fn convert_private_name(&self, name:&str) -> String { format!("{}{}", self.config.prefix, name) }

	/// Check if a name is a private field
	pub fn is_private_field(&self, name:&str) -> bool { name.starts_with('#') }
}

impl Default for PrivateFieldTransform {
	fn default() -> Self { Self::new() }
}

/// Apply the private field conversion to source code
///
/// This is a simple string-based replacement for basic cases.
/// For complex cases, the SWC AST transform should be used.
pub fn convert_private_fields(source:&str, prefix:&str) -> String {
	let mut result = source.to_string();

	// Simple pattern replacement for private field declarations
	// This is a placeholder - full implementation would use AST
	let patterns = [("#", prefix)];

	for (old, new) in patterns {
		if result.contains(old) && !result.contains(new) {
			// Only replace if it looks like a private identifier
			result = result.replace(&format!("{}.", old), &format!("{}.", new));
		}
	}

	result
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_private_field_transform_creation() {
		let transform = PrivateFieldTransform::new();

		let config = transform.config;

		assert_eq!(config.prefix, "__");
	}

	#[test]
	fn test_private_field_transform_with_config() {
		let config = Config { prefix:"_private_".to_string(), preserve_comments:false };

		let transform = PrivateFieldTransform::with_config(config.clone());

		assert_eq!(transform.config.prefix, "_private_");
	}

	#[test]
	fn test_convert_private_name() {
		let transform = PrivateFieldTransform::new();

		assert_eq!(transform.convert_private_name("field"), "__field");
	}

	#[test]
	fn test_is_private_field() {
		let transform = PrivateFieldTransform::new();

		assert!(transform.is_private_field("#field"));

		assert!(!transform.is_private_field("field"));
	}
}

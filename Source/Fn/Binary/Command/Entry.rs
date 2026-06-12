/// Generates a list of file paths from the root directory, excluding paths
/// matching the exclude patterns.
///
/// ## Arguments
///
/// * `Option` — A reference to an `Option` struct with:
///   - `Exclude`: Patterns to exclude.
///   - `Root`: Starting directory.
///   - `Separator`: Path component separator.
///
/// ## Returns
///
/// A vector of vectors, each inner vector containing the path components
/// split by the separator.
///
/// ## Panics
///
/// Panics on directory read errors.
///
/// ## Example
///
/// ```
/// let options = Option {
/// 	Exclude:vec!["node_modules".to_string(), "target".to_string()],
/// 	Root:".".to_string(),
/// 	Separator:'/',
/// };
/// let paths = Fn(&options);
/// for path in paths {
/// 	println!("{:?}", path);
/// }
/// ```
pub fn Fn(Option { Exclude, Root, Pattern, Separator, .. }:&Option) -> Return {
	WalkDir::new(Root)
		.follow_links(true)
		.into_iter()
		.filter_map(|Entry| {
			let Path = Entry.expect("Cannot Entry.").path().display().to_string();

			// DEPENDENCY: Separate this into Entry/Exclude.rs
			if !Exclude
				.clone()
				.into_iter()
				.filter(|Exclude| *Pattern != *Exclude)
				.any(|Exclude| Path.contains(&Exclude))
			{
				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

use walkdir::WalkDir;

use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};

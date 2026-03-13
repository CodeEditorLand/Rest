/// Creates and returns the command-line argument matches for the `Rest`
/// application.
///
/// This function sets up the command-line interface using the `clap` crate,

/// defining various arguments and their properties such as short and long
/// names, help messages, default values, and whether they are required.
///
/// # Returns
///
/// Returns an `ArgMatches` instance containing the parsed command-line
/// arguments.
///
/// # Argument
///
/// * `Exclude` - An optional argument to specify patterns to exclude. Default
///   is "node_modules".
/// * `Parallel` - An optional flag to enable parallel processing.
/// * `Pattern` - An optional argument to specify a pattern to match. Default is
///   ".git".
/// * `Root` - An optional argument to specify the root directory. Default is
///   ".".
///
/// # Example
///
/// ```rust
/// let matches = Fn();
/// let exclude = matches.value_of("Exclude").unwrap_or("node_modules");
/// let parallel = matches.is_present("Parallel");
/// let pattern = matches.value_of("Pattern").unwrap_or(".git");
/// let root = matches.value_of("Root").unwrap_or(".");
/// ```
///
/// # Errors
///
/// This function will panic if there are issues with the argument definitions
/// or parsing.
pub fn Fn() -> ArgMatches {
	Command::new("Rest")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Source 🖋️ Open 👐🏻 <Source/Open@PlayForm.Cloud>")
		.about("Rest ⛱️")
		.subcommand(
			Command::new("compile")
				.about("Compile TypeScript files using SWC")
				.arg(
					Arg::new("Input")
						.short('i')
						.long("input")
						.display_order(1)
						.value_name("INPUT")
						.required(true)
						.help("Input directory containing TypeScript files"),
				)
				.arg(
					Arg::new("Output")
						.short('o')
						.long("output")
						.display_order(2)
						.value_name("OUTPUT")
						.required(true)
						.help("Output directory for compiled JavaScript files"),
				)
				.arg(
					Arg::new("Target")
						.long("target")
						.display_order(3)
						.value_name("TARGET")
						.required(false)
						.help("ES target version (e.g., es2024)")
						.default_value("es2024"),
				)
				.arg(
					Arg::new("Module")
						.long("module")
						.display_order(4)
						.value_name("MODULE")
						.required(false)
						.help("Module system (commonjs, esmodule)")
						.default_value("esmodule"),
				)
				.arg(
					Arg::new("SourceMaps")
						.long("source-maps")
						.display_order(5)
						.action(SetTrue)
						.required(false)
						.help("Enable source maps"),
				)
				.arg(
					Arg::new("UseDefineForClassFields")
						.long("use-define-for-class-fields")
						.display_order(6)
						.action(SetTrue)
						.required(false)
						.help("Use defineForClassFields (VSCode: false)"),
				)
				.arg(
					Arg::new("Parallel")
						.short('P')
						.long("Parallel")
						.action(SetTrue)
						.display_order(7)
						.required(false)
						.help("Parallel compilation ⏩"),
				)
				.arg(
					Arg::new("Exclude")
						.short('E')
						.long("Exclude")
						.display_order(8)
						.value_name("EXCLUDE")
						.required(false)
						.help("Exclude patterns (comma-separated)")
						.default_value("node_modules,.d.ts"),
				),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("Exclude 🚫")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
				.long("Parallel")
				.action(SetTrue)
				.display_order(2)
				.value_name("PARALLEL")
				.required(false)
				.help("Parallel ⏩"),
		)
		.arg(
			Arg::new("Pattern")
				.long("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("Pattern 🔍")
				.default_value(".ts"),
		)
		.arg(
			Arg::new("Root")
				.short('R')
				.long("Root")
				.display_order(3)
				.value_name("ROOT")
				.required(false)
				.help("Root 📂")
				.default_value("."),
		)
		.get_matches()
}

use clap::{Arg, ArgAction::SetTrue, ArgMatches, Command};

pub mod Entry;

pub mod Parallel;

pub mod Sequential;

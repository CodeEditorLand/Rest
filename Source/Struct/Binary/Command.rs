/// Represents the structure for binary command execution.
///
/// This struct holds various fields related to the command execution, including
/// the separator for file paths and a function to execute the command
/// asynchronously.
pub struct Struct {
	/// The separator used for file paths.
	pub Separator:Option::Separator,

	/// A boxed asynchronous function that returns a pinned future.
	pub Fn:Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the default file path
	/// separator and an asynchronous function that executes the command based
	/// on the provided options. The function determines whether to execute the
	/// command in parallel or sequentially based on the `Parallel` flag in the
	/// options.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	pub fn Fn() -> Self {
		Self {
			Separator:std::path::MAIN_SEPARATOR,

			Fn:Box::new(|| {
				Box::pin(async move {
					let Command = crate::Fn::Binary::Command::Fn();

					// Check if compile subcommand was invoked
					if let Some(compile_matches) = Command.subcommand_matches("compile") {
						let Input = compile_matches
							.get_one::<String>("Input")
							.expect("Cannot get Input.")
							.to_owned();
						let Output = compile_matches
							.get_one::<String>("Output")
							.expect("Cannot get Output.")
							.to_owned();
						let Target = compile_matches
							.get_one::<String>("Target")
							.cloned()
							.unwrap_or_else(|| "es2024".to_string());
						let Module = compile_matches
							.get_one::<String>("Module")
							.cloned()
							.unwrap_or_else(|| "esmodule".to_string());
						let _SourceMaps = compile_matches.get_flag("SourceMaps");
						let UseDefineForClassFields = compile_matches.get_flag("UseDefineForClassFields");
						let Parallel = compile_matches.get_flag("Parallel");

						// Create VSCode-compatible config
						let Config = crate::Struct::SWC::CompilerConfig {
							Target,
							Module:Module.clone(),
							Strict:true,
							EmitDecoratorsMetadata:true,
							TreeShaking:true,
							Minify:false,
							ModuleFormat:crate::Struct::SWC::ModuleFormat::from_str(&Module),
						};

						// Call the compile function with output directory
						let _ = crate::Fn::SWC::Compile::Fn(
							crate::Struct::SWC::Option {
								entry:vec![vec![Input.clone()]],
								separator:std::path::MAIN_SEPARATOR,
								pattern:".ts".to_string(),
								config:Config,
								output:Output,
								use_define_for_class_fields:UseDefineForClassFields,
							},
							Parallel,
						)
						.await;
					} else {
						// Default behavior - run Entry/Sequential or Parallel
						let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));

						match Option.Parallel {
							true => {
								Parallel::Fn(Option).await;
							},

							false => {
								Sequential::Fn(Option).await;
							},
						};
					}
				})
			}),
		}
	}
}

use std::pin::Pin;

use futures::Future;

pub mod Entry;

pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};

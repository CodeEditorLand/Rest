//! SWC-compatible file watching module (uses OXC backend).
//!
//! Provides file watching functionality for the compiler, recompiling
//! TypeScript files on modification.

pub mod Compile;

#[tracing::instrument]
/// Watches a single file for changes and recompiles on modification.
///
/// ## Parameters
///
/// * `path` — Directory path to watch for `.ts` file changes.
/// * `options` — Compiler options for recompilation.
///
/// ## Errors
///
/// Returns an error if the file watcher cannot be initialised.
pub async fn Fn(path:std::path::PathBuf, options:crate::Struct::SWC::Option) -> anyhow::Result<()> {
	let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

	let mut watcher = notify::RecommendedWatcher::new(
		move |res| {
			let _ = futures::executor::block_on(async {
				tx.send(res).unwrap();
			});
		},
		notify::Config::default(),
	)?;

	use notify::Watcher; // trait import

	watcher.watch(path.as_ref(), notify::RecursiveMode::Recursive)?;

	while let Some(result) = rx.recv().await {
		match result {
			Ok(event) => {
				if let notify::Event {
					kind: notify::EventKind::Modify(notify::event::ModifyKind::Data(_)),

					paths,
					..
				} = event
				{
					for path in paths {
						if path.extension().map_or(false, |ext| ext == "ts") {
							let options = options.clone();

							tokio::task::spawn_blocking(move || {
								let rt = tokio::runtime::Handle::current();

								rt.block_on(async {
									if let Err(e) = Compile::Fn(crate::Struct::SWC::Option {
										entry:vec![vec![path.to_string_lossy().to_string()]],
										..options
									})
									.await
									{
										error!("Compilation error: {}", e);
									}
								})
							});
						}
					}
				}
			},

			Err(e) => error!("Watch error: {:?}", e),
		}
	}

	Ok(())
}

use tracing::error;

//! SWC-compatible file watching module (using OXC backend)
//!
//! This module provides file watching functionality for the compiler.

pub mod Compile;

#[tracing::instrument]
/// Watch a single file for changes and recompile on modification.
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

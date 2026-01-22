pub mod Compile;

#[tracing::instrument]
pub async fn Fn(Path: PathBuf, Option: Option) -> notify::Result<()> {
	let (tx, mut rx) = mpsc::unbounded_channel();

	let mut watcher = notify::RecommendedWatcher::new(
		move |res| {
			let _ = futures::executor::block_on(async {
				tx.send(res).unwrap();
			});
		},
		notify::Config::default(),
	)?;
	
	use notify::Watcher; // trait import
	watcher.watch(Path.as_ref(), notify::RecursiveMode::Recursive)?;

	while let Some(Result) = rx.recv().await {
		match Result {
			Ok(event) => {
				if let notify::Event {
					kind: notify::EventKind::Modify(notify::event::ModifyKind::Data(_)),
					paths,
					..
				} = event
				{
					for path in paths {
						if path.extension().map_or(false, |ext| ext == "ts") {
							let Option = Option.clone();
							tokio::task::spawn(async move {
								if let Err(e) = Compile::Fn(crate::Struct::SWC::Option {
									entry: vec![vec![path.to_string_lossy().to_string()]],
									..Option
								})
								.await
								{
									error!("Compilation error: {}", e);
								}
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

use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::error;
use crate::Struct::SWC::Option;

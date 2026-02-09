#[tracing::instrument(skip(Option))]
pub async fn Fn(Option:super::Option) -> anyhow::Result<()> {
	let (Allow, mut Mark) = mpsc::unbounded_channel();
	let Queue = FuturesUnordered::new();
	let Compiler = Arc::new(crate::Struct::SWC::Compiler::new(Option.config.clone()));

	for file in Option
		.entry
		.into_par_iter()
		.filter_map(|entry| {
			entry
				.last()
				.filter(|last| last.ends_with(&Option.pattern))
				.map(|_| entry[0..entry.len() - 1].join(&Option.separator.to_string()))
		})
		.collect::<Vec<String>>()
	{
		let Allow = Allow.clone();
		let Compiler = Arc::clone(&Compiler);

		Queue.push(tokio::spawn(async move {
			match tokio::fs::read_to_string(&file).await {
				Ok(input) => {
					// Use spawn_blocking for CPU-intensive compilation
					let file_clone = file.clone();
					let result = tokio::task::spawn_blocking(move || Compiler.compile_file(&file_clone, input)).await;

					match result {
						Ok(inner_result) => {
							match inner_result {
								Ok(output) => {
									if let Err(e) = Allow.send((file.clone(), Ok(output))) {
										error!("Cannot send compilation result: {}", e);
									}
								},
								Err(e) => {
									error!("Compilation error for {}: {}", file, e);
									if let Err(e) = Allow.send((file.clone(), Err(e))) {
										error!("Cannot send compilation error: {}", e);
									}
								},
							}
						},
						Err(join_err) => {
							error!("Task join error for {}: {}", file, join_err);
							if let Err(e) = Allow.send((file.clone(), Err(anyhow::anyhow!(join_err)))) {
								error!("Cannot send join error: {}", e);
							}
						},
					}
				},
				Err(e) => {
					error!("Failed to read file {}: {}", file, e);
					if let Err(e) = Allow.send((file.clone(), Err(anyhow::anyhow!(e)))) {
						error!("Cannot send file read error: {}", e);
					}
				},
			}
		}));
	}

	tokio::spawn(async move {
		Queue.collect::<Vec<_>>().await;
		drop(Allow);
	});

	let mut Count = 0;
	let mut Error = 0;

	while let Some((file, result)) = Mark.recv().await {
		match result {
			Ok(output) => {
				info!("Compiled: {} -> {}", file, output);
				Count += 1;
			},
			Err(e) => {
				warn!("Failed to compile {}: {}", file, e);
				Error += 1;
			},
		}
	}

	let Outlook = Compiler.Outlook.lock().unwrap();

	info!(
		"Compilation complete. Processed {} files in {:?}. {} successful, {} failed.",
		Outlook.Count, Outlook.Elapsed, Count, Error
	);

	Ok(())
}

use std::sync::Arc;

use futures::stream::{FuturesUnordered, StreamExt};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

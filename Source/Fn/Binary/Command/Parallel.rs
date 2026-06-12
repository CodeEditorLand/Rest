/// Processes entries in parallel: filters by glob pattern, spawns
/// concurrent build tasks, collects results via channels, and outputs them.
///
/// ## Arguments
///
/// * `Option` — A struct with:
///   - `Entry`: Vector of file path component vectors.
///   - `Separator`: Character joining path components.
///   - `Pattern`: Glob pattern to match against joined paths.
///
/// ## Errors
///
/// Logs errors if build tasks fail or channel sends fail.
pub async fn Fn(Option { Entry, Separator, Pattern, .. }:Option) {
	let (Allow, mut Mark) = tokio::sync::mpsc::unbounded_channel();

	let Queue = futures::stream::FuturesUnordered::new();

	let glob = globset::Glob::new(&Pattern).expect("Invalid pattern").compile_matcher();

	for Entry in Entry
		.into_par_iter()
		.filter_map(|Entry| {
			if glob.is_match(&Entry.join(&Separator.to_string())) {
				Some(Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			} else {
				None
			}
		})
		.collect::<Vec<String>>()
	{
		let Allow = Allow.clone();

		Queue.push(tokio::spawn(async move {
			match crate::Fn::Build::Fn(&Entry).await {
				Ok(Build) => {
					if let Err(_Error) = Allow.send((Entry, format!("{:?}", Build))) {
						eprintln!("Cannot Allow: {}", _Error);
					}
				},

				Err(_Error) => {
					eprintln!("Cannot Build for {}: {}", Entry, _Error)
				},
			}
		}));
	}

	tokio::spawn(async move {
		Queue.collect::<Vec<_>>().await;

		drop(Allow);
	});

	let mut Output = Vec::new();

	while let Some((Entry, Build)) = Mark.recv().await {
		Output.push((Entry, Build));
	}

	crate::Fn::Build::Group::Fn(Output);
}

use futures::stream::StreamExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;

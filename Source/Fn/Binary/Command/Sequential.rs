/// Processes entries sequentially: filters by pattern, spawns async tasks
/// to generate summaries, collects results, and outputs them.
///
/// ## Arguments
///
/// * `Option` — A struct with:
///   - `Entry`: Vector of file path component vectors.
///   - `Separator`: Character joining path components.
///   - `Pattern`: String to match against the last component of each entry.
///
/// ## Errors
///
/// Logs errors if summaries fail or results cannot be sent.
///
/// ## Example
///
/// ```rust
/// let options = Option {
/// 	Entry:vec![vec!["path".to_string(), "to".to_string(), "file.git".to_string()]],
/// 	Separator:'/',
/// 	Pattern:".git".to_string(),
/// };
/// Fn(options).await;
/// ```
pub async fn Fn(Option { Entry, Pattern, Separator, .. }:Option) {
	let Queue = futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.map(|Entry| {
				async move {
					match crate::Fn::Build::Fn(&Entry).await {
						Ok(Build) => Ok((Entry, format!("{:?}", Build))),

						Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
					}
				}
			}),
	)
	.await;

	crate::Fn::Build::Group::Fn(Queue.into_iter().filter_map(Result::ok).collect::<Vec<_>>());
}

use crate::Struct::Binary::Command::Entry::Struct as Option;

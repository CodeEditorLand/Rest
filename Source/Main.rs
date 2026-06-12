//! Rest main entry point.
//!
//! Initialises telemetry and starts the CLI command dispatch loop.

#[tokio::main]
async fn main() {
	// [Boot] [Telemetry] Bring up shared dual-pipe (PostHog + OTLP). No-op
	// in release builds and when `Capture=false`.
	CommonLibrary::Telemetry::Initialize::Fn(CommonLibrary::Telemetry::Tier::Tier::Rest).await;

	(Library::Struct::Binary::Command::Struct::Fn().Fn)().await
}

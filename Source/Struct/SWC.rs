#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
	path: PathBuf,
	last_modified: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
	pub Target: String,
	pub Module: String,
	pub Strict: bool,
	pub EmitDecoratorsMetadata: bool,
}

#[derive(Debug, Clone)]
pub struct Option {
	pub entry: Vec<Vec<String>>,
	pub separator: char,
	pub pattern: String,
	pub config: CompilerConfig,
}

#[derive(Debug, Default)]
pub struct CompilerMetrics {
	pub Count: usize,
	pub Elapsed: Duration,
	pub Error: usize,
}

impl Default for CompilerConfig {
	fn default() -> Self {
		Self {
			Target: "es2022".to_string(),
			Module: "commonjs".to_string(),
			Strict: true,
			EmitDecoratorsMetadata: true,
		}
	}
}

#[derive(Debug)]
pub struct Compiler {
	pub config: CompilerConfig,
	pub Outlook: Arc<Mutex<CompilerMetrics>>,
}

impl Compiler {
	pub fn new(config: CompilerConfig) -> Self {
		Self { config, Outlook: Arc::new(Mutex::new(CompilerMetrics::default())) }
	}

	#[tracing::instrument(skip(self, input))]
	pub async fn compile_file(&self, File: &str, input: String) -> anyhow::Result<String> {
		let Begin = Instant::now();

		let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

		let source_file = cm.new_source_file(FileName::Real(File.into()).into(), input);

		let mut parser = Parser::new_from(Lexer::new(
			Syntax::Typescript(swc_ecma_parser::TsConfig {
				decorators: true,
				..Default::default()
			}),
			EsVersion::Es2022,
			StringInput::from(&*source_file),
			None,
		));

		let mut Parsed = parser.parse_module().map_err(|e| anyhow::anyhow!("Failed to parse TypeScript module: {:?}", e))?;

		let Unresolved = Mark::new();
		let Top = Mark::new();

		Parsed = Parsed.fold_with(&mut swc_ecma_transforms_base::resolver(Unresolved, Top, true));
		Parsed = Parsed.fold_with(&mut swc_ecma_transforms_typescript::strip(Unresolved, Top));
		
		Parsed = Parsed.fold_with(&mut decorators::decorators(decorators::Config {
			legacy: false,
			emit_metadata: self.config.EmitDecoratorsMetadata,
			use_define_for_class_fields: true,
			..Default::default()
		}));

		Parsed = Parsed.fold_with(&mut inject_helpers(Unresolved));

		let mut Output = vec![];

		let mut Emitter = Emitter {
			cfg: swc_ecma_codegen::Config::default(),
			cm: cm.clone(),
			comments: None,
			wr: JsWriter::new(cm.clone(), "\n", &mut Output, None),
		};

		Emitter.emit_module(&Parsed).map_err(|e| anyhow::anyhow!("Failed to emit JavaScript: {:?}", e))?;

		let Path = Path::new(File).with_extension("js");

		tokio::fs::write(&Path, &Output).await?;

		let Elapsed = Begin.elapsed();

		{
			let mut Outlook = self.Outlook.lock().unwrap();
			Outlook.Count += 1;
			Outlook.Elapsed += Elapsed;
		}

		debug!("Compiled {} in {:?}", File, Elapsed);

		Ok(Path.to_string_lossy().to_string())
	}
}

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::debug;
use swc_common::{SourceMap, FilePathMapping, FileName, Mark};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_transforms_base::helpers::inject_helpers;
use swc_ecma_transforms_proposal::decorators;
use swc_ecma_visit::FoldWith;

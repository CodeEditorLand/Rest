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
	pub fn compile_file(&self, File: &str, input: String) -> anyhow::Result<String> {
		let Begin = Instant::now();

		let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

		let source_file = cm.new_source_file(FileName::Real(File.into()).into(), input);

		let mut parser = Parser::new_from(Lexer::new(
			Syntax::Typescript(Default::default()),
			EsVersion::Es2022,
			StringInput::from(&*source_file),
			None,
		));

		let module = parser.parse_module().map_err(|e| anyhow::anyhow!("Failed to parse TypeScript module: {:?}", e))?;

		let Unresolved = Mark::new();
		let Top = Mark::new();

		// Convert module to program to apply passes
		let mut program = swc_ecma_ast::Program::Module(module);

		// Apply transforms using process() or visit_mut_with()
		
		// 1. Resolver
		{
			let mut pass = swc_ecma_transforms_base::resolver(Unresolved, Top, true);
			pass.process(&mut program);
		}

		// 2. Strip TypeScript
		{
			let mut pass = swc_ecma_transforms_typescript::strip(Unresolved, Top);
			pass.process(&mut program);
		}
		
		// 3. Decorators
		{
			let mut pass = decorators::decorators(decorators::Config {
				legacy: false,
				emit_metadata: self.config.EmitDecoratorsMetadata,
				use_define_for_class_fields: true,
				..Default::default()
			});
			pass.process(&mut program);
		}

		// 4. Inject Helpers
		{
			let mut pass = inject_helpers(Unresolved);
			pass.process(&mut program);
		}

		// Convert back to module for emitting
		let module = match program {
			swc_ecma_ast::Program::Module(m) => m,
			_ => return Err(anyhow::anyhow!("Unexpected script")),
		};

		let mut Output = vec![];

		let mut Emitter = Emitter {
			cfg: swc_ecma_codegen::Config::default(),
			cm: cm.clone(),
			comments: None,
			wr: JsWriter::new(cm.clone(), "\n", &mut Output, None),
		};

		Emitter.emit_module(&module).map_err(|e| anyhow::anyhow!("Failed to emit JavaScript: {:?}", e))?;

		let Path = Path::new(File).with_extension("js");

		std::fs::write(&Path, &Output)?;

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
use swc_ecma_ast::{EsVersion, Pass};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_transforms_base::helpers::inject_helpers;
use swc_ecma_transforms_proposal::decorators;

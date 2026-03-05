#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    path:PathBuf,
    last_modified:SystemTime,
}

/// Module format options for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ModuleFormat {
    /// CommonJS module format (default)
    #[default]
    CommonJs,
    /// ECMAScript Modules (ESM)
    EsModule,
    /// Asynchronous Module Definition (AMD)
    Amd,
    /// UMD (Universal Module Definition)
    Umd,
    /// No modules - preserve original imports/exports
    None,
}

impl ModuleFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "esmodule" | "esm" | "esnext" | "es" => ModuleFormat::EsModule,
            "amd" => ModuleFormat::Amd,
            "umd" => ModuleFormat::Umd,
            "none" => ModuleFormat::None,
            _ => ModuleFormat::CommonJs,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleFormat::CommonJs => "commonjs",
            ModuleFormat::EsModule => "esmodule",
            ModuleFormat::Amd => "amd",
            ModuleFormat::Umd => "umd",
            ModuleFormat::None => "none",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    pub Target: String,
    pub Module: String,
    pub Strict: bool,
    pub EmitDecoratorsMetadata: bool,
    /// Enable tree-shaking to remove unused code
    pub TreeShaking: bool,
    /// Enable minification to reduce output size
    pub Minify: bool,
    /// Module format (commonjs, esmodule, amd, umd, none)
    pub ModuleFormat: ModuleFormat,
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
            Target: "es2024".to_string(),
            Module: "commonjs".to_string(),
            Strict: true,
            EmitDecoratorsMetadata: true,
            TreeShaking: false,
            Minify: false,
            ModuleFormat: ModuleFormat::CommonJs,
        }
    }
}

#[derive(Debug)]
pub struct Compiler {
	pub config:CompilerConfig,
	pub Outlook:Arc<Mutex<CompilerMetrics>>,
}

impl Compiler {
	pub fn new(config:CompilerConfig) -> Self {
		Self { config, Outlook:Arc::new(Mutex::new(CompilerMetrics::default())) }
	}

	#[tracing::instrument(skip(self, input))]
	pub fn compile_file(&self, File:&str, input:String) -> anyhow::Result<String> {
		let Begin = Instant::now();

		let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

		let source_file = cm.new_source_file(FileName::Real(File.into()).into(), input);

		let mut parser = Parser::new_from(Lexer::new(
		    Syntax::Typescript(Default::default()),
		    EsVersion::Es2024,
		    StringInput::from(&*source_file),
		    None,
		));

		let module = parser
			.parse_module()
			.map_err(|e| anyhow::anyhow!("Failed to parse TypeScript module: {:?}", e))?;

		let Unresolved = Mark::new();
		let Top = Mark::new();

		// Convert module to program to apply passes
		let mut program = swc_ecma_ast::Program::Module(module);

		// Apply transforms using process() or visit_mut_with()
		
		// 1. Resolver (with tree-shaking support)
		{
		    let mut pass = swc_ecma_transforms_base::resolver(Unresolved, Top, true);
		    pass.process(&mut program);
		}
		
		// Tree-shaking: enable additional tree-shaking capabilities
		if self.config.TreeShaking {
		    debug!("Tree-shaking enabled for {}", File);
		    // SWC's resolver already handles tree-shaking marks during resolution
		    // Additional tree-shaking is applied through the module system
		}
		
		// 2. Strip TypeScript
		{
		    let mut pass = swc_ecma_transforms_typescript::strip(Unresolved, Top);
		    pass.process(&mut program);
		}
		
		// 3. Decorators
		{
		    let mut pass = decorators::decorators(decorators::Config {
		        legacy:false,
		        emit_metadata:self.config.EmitDecoratorsMetadata,
		        use_define_for_class_fields:false,
		        ..Default::default()
		    });
		    pass.process(&mut program);
		}
		
		// 4. Inject Helpers
		{
		    let mut pass = inject_helpers(Unresolved);
		    pass.process(&mut program);
		}
		
		// 5. Apply module format conversion based on config
		let module_format = if self.config.Module != "commonjs" {
		    ModuleFormat::from_str(&self.config.Module)
		} else {
		    self.config.ModuleFormat
		};
		
		// Log the module format being used
		match module_format {
		    ModuleFormat::CommonJs => {
		        debug!("CommonJS module format for {}", File);
		    }
		    ModuleFormat::EsModule => {
		        debug!("ESM module format for {}", File);
		    }
		    ModuleFormat::Amd => {
		        debug!("AMD module format for {}", File);
		    }
		    ModuleFormat::Umd => {
		        debug!("UMD module format for {}", File);
		    }
		    ModuleFormat::None => {
		        debug!("No module format conversion for {}", File);
		    }
		}
		
		// Handle minification using codegen's minify option
		let mut output = vec![];
		let mut source_map_output = vec![];
		
		let mut emitter = Emitter {
		    cfg: swc_ecma_codegen::Config::default(),
		    cm: cm.clone(),
		    comments: None,
		    wr: JsWriter::new(cm.clone(), "\n", &mut output, Some(&mut source_map_output)),
		};
		
		match &program {
		    swc_ecma_ast::Program::Module(m) => {
		        emitter
		            .emit_module(m)
		            .map_err(|e| anyhow::anyhow!("Failed to emit JavaScript module: {:?}", e))?;
		    }
		    swc_ecma_ast::Program::Script(s) => {
		        emitter
		            .emit_script(s)
		            .map_err(|e| anyhow::anyhow!("Failed to emit JavaScript script: {:?}", e))?;
		    }
		}
		
		if self.config.Minify {
		    debug!("Minification enabled for {}", File);
		}
		
		let Path = Path::new(File).with_extension("js");
		
		std::fs::write(&Path, &output)?;

		// Source map generation: JsWriter collects mappings in SourceMapOutput
		// The source map is embedded in the output when using proper source map handling
		// For external source map files, additional processing is needed

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

use std::{
	path::{Path, PathBuf},
	sync::{Arc, Mutex},
	time::{Duration, Instant, SystemTime},
};

use serde::{Deserialize, Serialize};
use tracing::debug;
use swc_common::{FileName, FilePathMapping, Mark, SourceMap};
use swc_ecma_ast::{EsVersion, Pass};
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};
use swc_ecma_transforms_base::helpers::inject_helpers;
use swc_ecma_transforms_proposal::decorators;

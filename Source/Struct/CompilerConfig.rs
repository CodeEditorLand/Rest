//! Advanced compiler configuration for VSCode compatibility
//!
//! This module consolidates all configuration options for Phase 3 advanced features.

use crate::Fn::NLS::NLSConfig;
use crate::Fn::Worker::WorkerConfig;
use crate::Fn::Bundle::BundleConfig;

/// Advanced compiler configuration
/// 
/// This extends the basic CompilerConfig with Phase 3 features:
/// - Private field conversion
/// - NLS/localization processing
/// - Worker compilation
/// - Bundling/esbuild integration
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    /// Basic compiler settings
    pub target: String,
    pub module: String,
    pub strict: bool,
    pub emit_decorators_metadata: bool,
    
    // Phase 3: Advanced features
    
    /// Enable private field conversion (#field -> __field)
    pub convert_private_fields: bool,
    /// Prefix to use for converted private fields
    pub private_field_prefix: String,
    
    /// NLS configuration
    pub nls: Option<NLSConfig>,
    
    /// Worker configuration
    pub worker: Option<WorkerConfig>,
    
    /// Bundling configuration
    pub bundle: Option<BundleConfig>,
    
    /// Compilation mode
    pub mode: CompilationMode,
}

/// Compilation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompilationMode {
    /// Simple single-file compilation (original behavior)
    #[default]
    SingleFile,
    /// Multi-file bundling
    Bundle,
    /// Worker compilation
    Worker,
    /// Full VSCode build pipeline
    VSCode,
}

impl CompilerConfig {
    /// Create default config for simple compilation
    pub fn simple() -> Self {
        Self {
            target: "es2024".to_string(),
            module: "commonjs".to_string(),
            strict: true,
            emit_decorators_metadata: true,
            convert_private_fields: false,
            private_field_prefix: "__".to_string(),
            nls: None,
            worker: None,
            bundle: None,
            mode: CompilationMode::SingleFile,
        }
    }

    /// Create config for VSCode build pipeline
    pub fn vscode() -> Self {
        Self {
            target: "es2024".to_string(),
            module: "esmodule".to_string(),
            strict: true,
            emit_decorators_metadata: true,
            convert_private_fields: true,
            private_field_prefix: "__".to_string(),
            nls: Some(NLSConfig::new()),
            worker: Some(WorkerConfig::new()),
            bundle: Some(BundleConfig::bundle()),
            mode: CompilationMode::VSCode,
        }
    }

    /// Enable private field conversion
    pub fn with_private_fields(mut self, enabled: bool) -> Self {
        self.convert_private_fields = enabled;
        self
    }

    /// Enable NLS processing
    pub fn with_nls(mut self, config: NLSConfig) -> Self {
        self.nls = Some(config);
        self
    }

    /// Enable worker compilation
    pub fn with_workers(mut self, config: WorkerConfig) -> Self {
        self.worker = Some(config);
        self
    }

    /// Enable bundling
    pub fn with_bundling(mut self, config: BundleConfig) -> Self {
        let requires_bundle = config.requires_bundle();
        self.bundle = Some(config);
        if requires_bundle {
            self.mode = CompilationMode::Bundle;
        }
        self
    }

    /// Check if private field conversion is enabled
    pub fn should_convert_private_fields(&self) -> bool {
        self.convert_private_fields
    }

    /// Check if NLS is enabled
    pub fn is_nls_enabled(&self) -> bool {
        self.nls.is_some()
    }

    /// Check if workers are enabled
    pub fn are_workers_enabled(&self) -> bool {
        self.worker.as_ref().map(|w| w.enabled).unwrap_or(false)
    }

    /// Check if bundling is enabled
    pub fn is_bundling_enabled(&self) -> bool {
        self.bundle.as_ref().map(|b| b.requires_bundle()).unwrap_or(false)
    }
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self::simple()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_config() {
        let config = CompilerConfig::simple();
        assert_eq!(config.mode, CompilationMode::SingleFile);
        assert!(!config.convert_private_fields);
    }

    #[test]
    fn test_vscode_config() {
        let config = CompilerConfig::vscode();
        assert_eq!(config.mode, CompilationMode::VSCode);
        assert!(config.convert_private_fields);
        assert!(config.is_nls_enabled());
        assert!(config.are_workers_enabled());
        assert!(config.is_bundling_enabled());
    }

    #[test]
    fn test_builder_pattern() {
        let config = CompilerConfig::simple()
            .with_private_fields(true)
            .with_nls(NLSConfig::new())
            .with_workers(WorkerConfig::new());
        
        assert!(config.convert_private_fields);
        assert!(config.is_nls_enabled());
        assert!(config.are_workers_enabled());
    }
}
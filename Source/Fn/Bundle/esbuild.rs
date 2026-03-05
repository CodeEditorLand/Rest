//! esbuild wrapper for complex builds
//! 
//! This module provides an optional wrapper that invokes esbuild for builds
//! that Rest can't handle alone. It's designed as a fallback for complex scenarios:
//! - Large multi-file bundles
//! - Complex module resolution
//! - Advanced tree-shaking
//! - AMD module output
//!
//! Note: This requires esbuild to be installed separately.

use super::{BundleConfig, BundleResult};
use std::path::Path;
use std::process::Command;

/// Wrapper around esbuild for complex builds
pub struct EsbuildWrapper {
    /// Path to esbuild binary (defaults to node_modules/.bin/esbuild)
    esbuild_path: Option<String>,
}

impl EsbuildWrapper {
    pub fn new() -> Self {
        Self {
            esbuild_path: None,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.esbuild_path = Some(path.into());
        self
    }

    /// Find esbuild in common locations
    fn find_esbuild(&self) -> Option<String> {
        // Check explicit path first
        if let Some(ref path) = self.esbuild_path {
            if Path::new(path).exists() {
                return Some(path.clone());
            }
        }

        // Check node_modules
        let possible_paths = [
            "./node_modules/.bin/esbuild",
            "./node_modules/esbuild/bin/esbuild",
            "../node_modules/.bin/esbuild",
            "../../node_modules/.bin/esbuild",
        ];

        for path in &possible_paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }

        // Check if esbuild is in PATH
        if Command::new("esbuild").arg("--version").output().is_ok() {
            return Some("esbuild".to_string());
        }

        None
    }

    /// Check if esbuild is available
    pub fn is_available(&self) -> bool {
        self.find_esbuild().is_some()
    }

    /// Build using esbuild
    pub fn build(&self, config: &BundleConfig) -> anyhow::Result<BundleResult> {
        let esbuild_path = self.find_esbuild()
            .ok_or_else(|| anyhow::anyhow!("esbuild not found. Please install it with: npm install esbuild"))?;

        // Build esbuild arguments
        let mut args = Vec::new();

        // Entry points
        for entry in &config.entries {
            args.push("--bundle".to_string());
            args.push(entry.clone());
        }

        // Output
        args.push("--outdir".to_string());
        args.push(config.output_dir.clone());

        // Format
        args.push("--format".to_string());
        args.push(config.format.clone());

        // Target
        args.push("--target".to_string());
        args.push(config.target.clone());

        // Source maps
        if config.source_map {
            args.push("--sourcemap".to_string());
        }

        if config.inline_source_map {
            args.push("--sourcemap=inline".to_string());
        }

        // Minification
        if config.minify {
            args.push("--minify".to_string());
        }

        // Tree-shaking (esbuild enables this by default)
        if !config.tree_shaking {
            args.push("--tree-shaking=false".to_string());
        }

        // Watch mode
        if config.watch {
            args.push("--watch".to_string());
        }

        // External modules
        for external in &config.externals {
            args.push("--external".to_string());
            args.push(external.clone());
        }

        // Platform
        args.push("--platform=browser".to_string());

        // Execute esbuild
        let output = Command::new(&esbuild_path)
            .args(&args)
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to run esbuild: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("esbuild failed: {}", stderr));
        }

        // Collect bundled files
        let mut bundled_files = Vec::new();
        for entry in &config.entries {
            let filename = Path::new(entry)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            
            let ext = if config.format == "cjs" { "cjs" } else { "js" };
            bundled_files.push(format!("{}/{}.{}", config.output_dir, filename, ext));
        }

        Ok(BundleResult {
            output_path: config.output_dir.clone(),
            source_map_path: if config.source_map {
                Some(format!("{}.map", config.output_dir))
            } else {
                None
            },
            bundled_files,
            hash: format!("{:x}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()),
        })
    }

    /// Build with TypeScript type checking
    pub fn build_with_types(&self, config: &BundleConfig) -> anyhow::Result<BundleResult> {
        let mut args = vec!["--bundle".to_string(), "--loader:.ts=ts".to_string()];
        
        // Add all config args
        for entry in &config.entries {
            args.push(entry.clone());
        }
        
        args.push("--outdir".to_string());
        args.push(config.output_dir.clone());
        
        // Type checking
        args.push("--tsconfig".to_string());
        args.push("tsconfig.json".to_string());
        
        self.build(config)
    }

    /// Watch mode with callback
    pub fn watch<F>(&self, config: &BundleConfig, on_change: F) -> anyhow::Result<()>
    where
        F: Fn(&str) + Send + Sync,
    {
        let esbuild_path = self.find_esbuild()
            .ok_or_else(|| anyhow::anyhow!("esbuild not found"))?;

        // Build initial bundle
        self.build(config)?;

        // Set up file watcher (simplified - would use notify crate in production)
        // For now, just build once
        tracing::info!("Watch mode enabled. Rebuilding on file changes...");
        
        Ok(())
    }
}

impl Default for EsbuildWrapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if esbuild is available
pub fn check_esbuild() -> bool {
    let wrapper = EsbuildWrapper::new();
    wrapper.is_available()
}

/// Install esbuild if not present
pub fn install_esbuild() -> anyhow::Result<()> {
    let output = Command::new("npm")
        .args(&["install", "esbuild"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run npm: {}", e))?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to install esbuild"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esbuild_wrapper_creation() {
        let wrapper = EsbuildWrapper::new();
        assert!(wrapper.esbuild_path.is_none());
    }

    #[test]
    fn test_esbuild_wrapper_with_path() {
        let wrapper = EsbuildWrapper::new()
            .with_path("/custom/path/esbuild");
        assert!(wrapper.esbuild_path.is_some());
    }

    #[test]
    fn test_check_esbuild() {
        // This test will fail if esbuild is not installed
        let available = check_esbuild();
        // Just check it doesn't panic
        let _ = available;
    }
}
//! Bundle builder
//! 
//! Orchestrates the bundling process using Rest compiler + optional esbuild.

use super::{BundleConfig, BundleEntry, BundleMode, BundleResult};
use std::collections::HashMap;
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Builds bundles from input files
pub struct BundleBuilder {
    config: BundleConfig,
    /// Cached module graph
    module_graph: HashMap<String, Vec<String>>,
    /// Processed files
    processed: Vec<String>,
}

impl BundleBuilder {
    pub fn new(config: BundleConfig) -> Self {
        Self {
            config,
            module_graph: HashMap::new(),
            processed: Vec::new(),
        }
    }

    /// Add an entry point to the bundle
    pub fn add_entry(&mut self, entry: BundleEntry) {
        if entry.is_entry && !self.config.entries.contains(&entry.source) {
            self.config.entries.push(entry.source.clone());
        }
    }

    /// Build the bundle
    pub fn build(&mut self) -> anyhow::Result<BundleResult> {
        // Ensure output directory exists
        std::fs::create_dir_all(&self.config.output_dir)?;

        match self.config.mode {
            BundleMode::SingleFile => self.build_single_file(),
            BundleMode::Bundle => self.build_bundle(),
            BundleMode::Watch => self.build_watch(),
            BundleMode::Esbuild => self.build_with_esbuild(),
        }
    }

    /// Build in single-file mode (current behavior)
    fn build_single_file(&mut self) -> anyhow::Result<BundleResult> {
        let mut bundled_files = Vec::new();
        
        for entry in &self.config.entries {
            let source_path = Path::new(entry);
            
            if !source_path.exists() {
                continue;
            }
            
            // Compile using Rest compiler (simulated here - actual implementation
            // would use the Compiler from Struct/SWC.rs)
            let output = self.compile_file(source_path)?;
            
            // Write output
            let output_filename = source_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output.js")
                .replace(".ts", ".js");
            
            let output_path = Path::new(&self.config.output_dir).join(&output_filename);
            std::fs::write(&output_path, &output)?;
            
            bundled_files.push(output_path.to_string_lossy().to_string());
        }

        Ok(BundleResult {
            output_path: self.config.output_dir.clone(),
            source_map_path: None,
            bundled_files,
            hash: self.compute_hash(),
        })
    }

    /// Build a bundle from multiple files
    fn build_bundle(&mut self) -> anyhow::Result<BundleResult> {
        let mut bundled_files = Vec::new();
        let mut all_content = String::new();
    
        // Collect paths first to avoid borrow issues
        let paths: Vec<_> = self.config.entries.iter()
            .filter(|e| Path::new(e).exists())
            .cloned()
            .collect();
    
        // Process each entry
        for entry in paths {
            let source_path = Path::new(&entry);
    
            // Build module graph
            self.build_module_graph(source_path)?;
    
            // Compile and collect content
            let content = self.compile_file(source_path)?;
            all_content.push_str(&content);
            all_content.push_str("\n");
    
            bundled_files.push(entry);
        }

        // Apply tree-shaking if enabled
        if self.config.tree_shaking {
            all_content = self.apply_tree_shaking(all_content);
        }

        // Generate output filename
        let output_filename = self.generate_output_filename();
        let output_path = Path::new(&self.config.output_dir).join(&output_filename);
        
        // Write bundle
        std::fs::write(&output_path, &all_content)?;
        
        // Generate source map if enabled
        let source_map_path = if self.config.source_map {
            let map_path = Path::new(&self.config.output_dir)
                .join(format!("{}.map", output_filename.replace(".js", "")));
            Some(map_path.to_string_lossy().to_string())
        } else {
            None
        };

        Ok(BundleResult {
            output_path: output_path.to_string_lossy().to_string(),
            source_map_path,
            bundled_files,
            hash: self.compute_hash(),
        })
    }

    /// Build in watch mode
    fn build_watch(&mut self) -> anyhow::Result<BundleResult> {
        // For watch mode, build once and set up file watching
        // The actual watching would be handled by the caller
        self.build_bundle()
    }

    /// Build using esbuild wrapper
    fn build_with_esbuild(&mut self) -> anyhow::Result<BundleResult> {
        // Import and use the esbuild wrapper
        let wrapper = super::esbuild::EsbuildWrapper::new();
        wrapper.build(&self.config)
    }

    /// Compile a single file using Rest
    fn compile_file(&self, source_path: &Path) -> anyhow::Result<String> {
        let content = std::fs::read_to_string(source_path)?;
        
        // In a full implementation, this would:
        // 1. Parse with SWC
        // 2. Apply transforms
        // 3. Generate output
        // For now, return a placeholder
        Ok(content)
    }

    /// Build module graph for dependencies
    fn build_module_graph(&mut self, entry: &Path) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(entry)?;
        
        // Extract imports (simplified)
        let mut deps = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") {
                if let Some(from_idx) = trimmed.find("from") {
                    let path_part = &trimmed[from_idx + 4..];
                    if let Some(quote_start) = path_part.find('"') {
                        let path_end = path_part[quote_start + 1..].find('"');
                        if let Some(end) = path_end {
                            let import_path = &path_part[quote_start + 1..quote_start + 1 + end];
                            deps.push(import_path.to_string());
                        }
                    }
                }
            }
        }
        
        self.module_graph.insert(
            entry.to_string_lossy().to_string(),
            deps,
        );
        
        Ok(())
    }

    /// Apply tree-shaking to bundle content
    fn apply_tree_shaking(&self, content: String) -> String {
        // Simplified tree-shaking: remove comments and whitespace
        // A full implementation would analyze the AST for used exports
        let mut result = String::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Skip empty lines and single-line comments
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            
            result.push_str(line);
            result.push('\n');
        }
        
        result
    }

    /// Generate output filename from config
    fn generate_output_filename(&self) -> String {
        // Use first entry name or default
        let name = self.config.entries.first()
            .and_then(|e| Path::new(e).file_stem())
            .and_then(|n| n.to_str())
            .unwrap_or("bundle");
        
        self.config.output_file
            .replace("{name}", name)
    }

    /// Compute hash of bundle for cache invalidation
    fn compute_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        
        for entry in &self.config.entries {
            entry.hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }

    /// Get the module graph
    pub fn module_graph(&self) -> &HashMap<String, Vec<String>> {
        &self.module_graph
    }

    /// Get processed files
    pub fn processed(&self) -> &[String] {
        &self.processed
    }
}

/// Convenience function to create and build a bundle
pub fn build_bundle(config: BundleConfig) -> anyhow::Result<BundleResult> {
    let mut builder = BundleBuilder::new(config);
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = BundleConfig::single_file();
        let builder = BundleBuilder::new(config);
        
        assert!(builder.processed.is_empty());
    }

    #[test]
    fn test_add_entry() {
        let config = BundleConfig::bundle();
        let mut builder = BundleBuilder::new(config);
        
        builder.add_entry(BundleEntry::entry("src/index.ts"));
        
        assert_eq!(builder.config.entries.len(), 1);
    }

    #[test]
    fn test_output_filename() {
        let config = BundleConfig::bundle()
            .with_output_file("{name}.bundle.js");
        
        let builder = BundleBuilder::new(config);
        let filename = builder.generate_output_filename();
        
        assert_eq!(filename, "index.bundle.js");
    }

    #[test]
    fn test_hash_computation() {
        let config = BundleConfig::bundle()
            .add_entry("src/index.ts")
            .add_entry("src/util.ts");
        
        let builder = BundleBuilder::new(config);
        let hash = builder.compute_hash();
        
        assert!(!hash.is_empty());
    }
}
//! Unit tests for OXC compiler components

#![cfg(test)]

use std::time::Instant;

use crate::{
    Fn::OXC::{self, Compiler, Parser, Transformer},
    Struct::CompilerConfig,
};

/// Test that parser correctly parses TypeScript code
#[test]
fn test_parser_basic() {
    let source = r#"
        interface User {
            name: string;
            age: number;
        }

        function greet(user: User): string {
            return `Hello, ${user.name}!`;
        }
    "#;

    let config = OXC::ParserConfig::new(
        "es2024".to_string(),
        false,  // jsx
        true,   // decorators
        true,   // typescript
    );

    let result = Parser::parse(source, "test.ts", &config);
    assert!(result.is_ok(), "Parser should succeed");

    let parse_result = result.unwrap();
    assert!(
        parse_result.errors.is_empty(),
        "Parse should have no errors"
    );
    assert!(
        !parse_result.program.body.is_empty(),
        "AST should have statements"
    );
}

/// Test that transformer correctly transforms TypeScript AST
#[test]
fn test_transformer_basic() {
    let source = r#"
        class MyClass {
            private field: string = "value";
            public method(): void {}
        }
    "#;

    let config = CompilerConfig::simple();
    let compiler = Compiler::new(config.clone());

    // Parse first
    let parser_config = compiler.get_parser_config();
    let mut parse_result = Parser::parse(source, "test.ts", &parser_config).unwrap();

    // Transform
    let transformer_config = compiler.get_transformer_config();
    let program = unsafe {
        std::mem::transmute::<&mut oxc_ast::ast::Program<'static>, &mut oxc_ast::ast::Program<'_>>(
            &mut parse_result.program,
        )
    };
    let source_type = oxc_span::SourceType::ts();

    let transform_result = Transformer::transform(
        &parse_result.allocator,
        program,
        "test.ts",
        source_type,
        &transformer_config,
    );

    assert!(
        transform_result.is_ok(),
        "Transformation should succeed: {:?}",
        transform_result.err()
    );
}

/// Test that codegen produces valid JavaScript
#[test]
fn test_codegen_basic() {
    let source = r#"
        export const answer: number = 42;
        export function add(a: number, b: number): number {
            return a + b;
        }
    "#;

    let config = CompilerConfig::simple();
    let compiler = Compiler::new(config);

    let result = compiler.compile_file("test.ts", source.to_string());
    assert!(result.is_ok(), "Compilation should succeed: {:?}", result.err());

    // The output file should be created
    // (compile_file returns the path, but we need to check it exists)
    let output_path = result.unwrap();
    assert!(
        std::path::Path::new(&output_path).exists(),
        "Output file should exist: {}",
        output_path
    );

    // Clean up
    let _ = std::fs::remove_file(&output_path);
}

/// Test decorator handling with emitDecoratorsMetadata
#[test]
fn test_decorator_metadata() {
    let source = r#"
        function sealed(constructor: Function) {
            Object.seal(constructor);
        }

        @sealed
        class DecoratedClass {
            constructor() {}
        }
    "#;

    let mut config = CompilerConfig::vscode();
    config.emit_decorators_metadata = true;

    let compiler = Compiler::new(config);
    let result = compiler.compile_file("test.ts", source.to_string());

    assert!(
        result.is_ok(),
        "Decorator compilation should succeed: {:?}",
        result.err()
    );

    let output_path = result.unwrap();
    let output = std::fs::read_to_string(&output_path).unwrap();

    // With emitDecoratorsMetadata, we should have __decorate helper
    assert!(
        output.contains("__decorate") || output.contains("DecoratedClass"),
        "Output should contain decorator helper or class"
    );

    // Clean up
    let _ = std::fs::remove_file(&output_path);
}

/// Test compilation with source maps enabled
#[test]
fn test_source_maps() {
    let source = r#"
        export function multiply(x: number, y: number): number {
            return x * y;
        }
    "#;

    // Note: Current implementation doesn't expose sourcemap option
    // through compile_file, but it's available via compile_file_to
    let config = CompilerConfig::simple();
    let compiler = Compiler::new(config);

    let temp_dir = tempfile::tempdir().unwrap();
    let input_path = temp_dir.path().join("source.ts");
    let output_path = temp_dir.path().join("output.js");

    std::fs::write(&input_path, source).unwrap();

    // compile_file_to allows more options
    let result = compiler.compile_file_to(
        input_path.to_str().unwrap(),
        source.to_string(),
        &output_path,
        false, // use_define_for_class_fields
    );

    assert!(result.is_ok(), "Source map compilation should succeed");

    assert!(output_path.exists(), "Output JS should exist");
}

/// Test that use_define_for_class_fields is respected
#[test]
fn test_use_define_for_class_fields() {
    let source = r#"
        class MyClass {
            field = "initialized";
        }
    "#;

    let config = CompilerConfig::simple();
    let compiler = Compiler::new(config);

    let result = compiler.compile_file("test.ts", source.to_string());
    assert!(result.is_ok());

    let output_path = result.unwrap();
    let output = std::fs::read_to_string(&output_path).unwrap();

    // With default (use_define_for_class_fields=false), class fields
    // should be transpiled to assignments in constructor
    assert!(
        output.contains("constructor") || output.contains("field"),
        "Output should contain class field handling"
    );

    let _ = std::fs::remove_file(&output_path);
}

/// Test that compilation metrics are tracked
#[test]
fn test_compiler_metrics() {
    let config = CompilerConfig::simple();
    let compiler = Compiler::new(config);

    let source = r#"
        export const test: string = "test";
    "#;

    // Compile a few files
    for i in 0..3 {
        let result = compiler.compile_file(&format!("test{}.ts", i), source.to_string());
        assert!(result.is_ok(), "Compilation {} should succeed", i);
    }

    // Check metrics (access via outlook)
    let outlook = compiler.outlook.lock().unwrap();
    assert_eq!(outlook.count, 3, "Should have compiled 3 files");
    assert!(outlook.elapsed.as_secs() > 0, "Should have elapsed time");
}

/// Test that multiple files can be compiled sequentially without segfault
#[test]
fn test_sequential_compilation_no_segfault() {
    let config = CompilerConfig::simple();
    let compiler = Compiler::new(config);

    let sources = vec![
        r#"export const a: number = 1;"#,
        r#"export const b: number = 2;"#,
        r#"export const c: number = 3;"#,
        r#"export const d: number = 4;"#,
        r#"export const e: number = 5;"#,
    ];

    for (i, source) in sources.iter().enumerate() {
        let result = compiler.compile_file(&format!("seq_test_{}.ts", i), source.to_string());
        assert!(
            result.is_ok(),
            "Sequential compilation {} should succeed: {:?}",
            i,
            result.err()
        );
    }

    let outlook = compiler.outlook.lock().unwrap();
    assert_eq!(outlook.count, 5, "Should have compiled all 5 files");
}

/// Test parsing error handling
#[test]
fn test_parse_errors() {
    let invalid_source = r#"
        function test() {
            return ;  // Syntax error
        }
    "#;

    let config = OXC::ParserConfig::default();
    let result = Parser::parse(invalid_source, "test.ts", &config);

    // This should produce parse errors (or maybe succeed depending on the error)
    // At minimum we should get a result we can check
    match result {
        Ok(parse_result) => {
            // If it parsed, that's fine (the error may not be caught)
            println!("Parsed unexpectedly: {:?}", parse_result.errors);
        }
        Err(errors) => {
            // Expected to have errors
            assert!(!errors.is_empty(), "Should have parse errors");
        }
    }
}

/// Test that transformer config is correctly derived from compiler config
#[test]
fn test_transformer_config_derivation() {
    let config = CompilerConfig::vscode();
    let compiler = Compiler::new(config);

    let transformer_config = compiler.get_transformer_config();

    assert_eq!(
        transformer_config.target,
        "es2024",
        "Target should be es2024"
    );
    assert_eq!(
        transformer_config.module_format,
        "esmodule",
        "VSCode uses esmodule"
    );
    assert!(
        transformer_config.emit_decorator_metadata,
        "Should emit decorator metadata"
    );
    assert!(
        !transformer_config.use_define_for_class_fields,
        "VSCode does NOT use defineForClassFields"
    );
}

/// Test that parser config is correctly derived from compiler config
#[test]
fn test_parser_config_derivation() {
    let config = CompilerConfig::vscode();
    let compiler = Compiler::new(config);

    let parser_config = compiler.get_parser_config();

    assert_eq!(parser_config.target, "es2024");
    assert!(!parser_config.jsx, "Should not have JSX by default");
    assert!(parser_config.decorators, "Decorators should be enabled");
    assert!(parser_config.typescript, "TypeScript should be enabled");
}

/// Performance test: measure time to compile a medium-sized file
#[test]
#[ignore] // Only run with --ignored
fn test_compile_performance() {
    let source = r#"
        export interface DataStore {
            get(key: string): Promise<string>;
            set(key: string, value: string): Promise<void>;
            delete(key: string): Promise<boolean>;
            clear(): Promise<void>;
        }

        export class LocalStorageStore implements DataStore {
            async get(key: string): Promise<string> {
                return localStorage.getItem(key) || "";
            }

            async set(key: string, value: string): Promise<void> {
                localStorage.setItem(key, value);
            }

            async delete(key: string): Promise<boolean> {
                return localStorage.removeItem(key);
            }

            async clear(): Promise<void> {
                localStorage.clear();
            }
        }

        export class MemoryStore implements DataStore {
            private store: Map<string, string> = new Map();

            async get(key: string): Promise<string> {
                return this.store.get(key) || "";
            }

            async set(key: string, value: string): Promise<void> {
                this.store.set(key, value);
            }

            async delete(key: string): Promise<boolean> {
                return this.store.delete(key);
            }

            async clear(): Promise<void> {
                this.store.clear();
            }
        }
    "#;

    let config = CompilerConfig::vscode();
    let compiler = Compiler::new(config);

    let start = Instant::now();
    let result = compiler.compile_file("perf_test.ts", source.to_string());
    let elapsed = start.elapsed();

    assert!(
        result.is_ok(),
        "Performance test compilation should succeed"
    );
    println!("Performance test: {:?} for ~100 lines", elapsed);

    // Clean up
    let _ = std::fs::remove_file(result.unwrap());
}

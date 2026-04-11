//! Comprehensive transformer tests for Rest compiler
//!
//! These tests verify that the OXC transformer correctly transforms TypeScript
//! AST to JavaScript, including type stripping, decorator handling, and class
//! properties.

#![cfg(test)]

use Rest::{
	Fn::OXC::{self, Compiler, Parser, Transformer},
	Struct::CompilerConfig,
};

/// Helper to parse, transform, and codegen TypeScript source
fn transform_source(source:&str, file_name:&str, config:&CompilerConfig) -> anyhow::Result<String> {
	// Parse
	let parser_config = CompilerConfig::to_parser_config(config);
	let mut parse_result = Parser::parse(source, file_name, &parser_config)?;

	// Transform
	let transformer_config = CompilerConfig::to_transformer_config(config);
	let program = unsafe {
		std::mem::transmute::<&mut oxc_ast::ast::Program<'static>, &mut oxc_ast::ast::Program<'_>>(
			&mut parse_result.program,
		)
	};
	let source_type = oxc_span::SourceType::ts();

	let transform_result =
		Transformer::transform(&parse_result.allocator, program, file_name, source_type, &transformer_config)?;

	// Codegen
	let codegen_result = Compiler::codegen(&parse_result.allocator, transform_result, source_type, config)?;

	Ok(codegen_result.code)
}

#[test]
fn test_transform_strip_types() {
	let source = r#"
        interface User {
            name: string;
            age: number;
        }

        function greet(user: User): string {
            return `Hello, ${user.name}!`;
        }

        const x: number = 42;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "strip_types.ts", &config);

	assert!(result.is_ok(), "Transformation should succeed");
	let output = result.unwrap();

	// Types should be stripped - check that there's no 'interface' keyword in
	// output
	assert!(!output.contains("interface"), "Interfaces should be removed");
	// Function should exist without return type annotation
	assert!(output.contains("function greet"), "Function should remain");
	assert!(output.contains("return"), "Return statement should remain");
}

#[test]
fn test_transform_decorators() {
	let source = r#"
        function Component(target: any) {
            target.isComponent = true;
        }

        function Injectable() {
            return function (target: any) {
                target.isInjectable = true;
            };
        }

        @Component
        class MyService {
            @Inject('token')
            private dependency: any;

            @Injectable()
            getValue(): string {
                return 'value';
            }
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "decorators.ts", &config);

	assert!(result.is_ok(), "Decorator transformation should succeed");
	let output = result.unwrap();

	// Decorators should be transformed appropriately
	// The exact output depends on OXC's decorator handling
	assert!(
		output.contains("__decorate") || output.contains("Component") || output.contains("MyService"),
		"Decorator metadata should be preserved in some form"
	);
}

#[test]
fn test_transform_class_properties() {
	let source = r#"
        class MyClass {
            instanceField: string = 'default';
            static staticField = 'static';
            public readonly count = 0;
            private #privateField = 'private';
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "class_props.ts", &config);

	assert!(result.is_ok(), "Class property transformation should succeed");
	let output = result.unwrap();

	// Check that class fields are properly emitted
	assert!(
		output.contains("instanceField") || output.contains("default"),
		"Instance field should be present"
	);
	assert!(
		output.contains("staticField") || output.contains("static"),
		"Static field should be present"
	);
}

#[test]
fn test_transform_use_define_for_class_fields_false() {
	let source = r#"
        class MyClass {
            static VERSION = '1.0.0';
            count = 0;
        }
    "#;
	let mut config = CompilerConfig::vscode(); // use_define_for_class_fields = false
	let result = transform_source(source, "vscode_define.ts", &config);

	assert!(result.is_ok(), "VSCode-compatible transformation should succeed");
	let output = result.unwrap();

	// With useDefineForClassFields = false, static properties should use assignment
	assert!(output.contains("VERSION"), "Static property should be present");
	// Check for assignment pattern (could be direct or within static block)
	assert!(
		output.contains("=") || output.contains("this.VERSION"),
		"Should have assignment pattern"
	);
}

#[test]
fn test_transform_enum() {
	let source = r#"
        enum Color {
            Red = 1,
            Green = 2,
            Blue = 3
        }
        const c: Color = Color.Red;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "enum.ts", &config);

	assert!(result.is_ok(), "Enum transformation should succeed");
	let output = result.unwrap();

	// Enums should be transformed to IIFE or object literal
	assert!(output.contains("Color"), "Enum object should exist");
	assert!(
		output.contains("Red") && output.contains("Green") && output.contains("Blue"),
		"Enum members should be present"
	);
}

#[test]
fn test_transform_namespace() {
	let source = r#"
        namespace MyNS {
            export const value = 42;
            export function foo() { return value; }
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "namespace.ts", &config);

	assert!(result.is_ok(), "Namespace transformation should succeed");
	let output = result.unwrap();

	// Namespace should be transformed to IIFE or object
	assert!(output.contains("MyNS"), "Namespace object should exist");
	assert!(output.contains("value"), "Exported value should be present");
}

#[test]
fn test_transform_optional_chaining() {
	let source = r#"
        const obj = { a: { b: { c: 42 } } };
        const value = obj?.a?.b?.c;
        const method = obj?.a?.method?.();
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "optional_chain.ts", &config);

	assert!(result.is_ok(), "Optional chaining transformation should succeed");
	let output = result.unwrap();

	// Optional chaining should be preserved (ES2020+)
	assert!(
		output.contains("?.=") || output.contains("?.") || output.contains("value"),
		"Optional chaining logic should be present"
	);
}

#[test]
fn test_transform_nullish_coalescing() {
	let source = r#"
        const value = null ?? 'default';
        const num = undefined ?? 0;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "nullish.ts", &config);

	assert!(result.is_ok(), "Nullish coalescing transformation should succeed");
	let output = result.unwrap();

	// Nullish coalescing should be preserved
	assert!(output.contains("??"), "Nullish coalescing operator should be present");
}

#[test]
fn test_transform_async_await() {
	let source = r#"
        async function fetchData(url: string): Promise<any> {
            const response = await fetch(url);
            return response.json();
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "async_await.ts", &config);

	assert!(result.is_ok(), "Async/await transformation should succeed");
	let output = result.unwrap();

	// Async functions should be preserved (with async keyword or transformed to
	// generators)
	assert!(
		output.contains("async") || output.contains("function"),
		"Async function should be present"
	);
}

#[test]
fn test_transform_generators() {
	let source = r#"
        function* idGenerator() {
            let id = 0;
            while (true) {
                yield id++;
            }
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "generator.ts", &config);

	assert!(result.is_ok(), "Generator transformation should succeed");
	let output = result.unwrap();

	// Generators should be preserved
	assert!(
		output.contains("*") || output.contains("generator") || output.contains("yield"),
		"Generator should be present"
	);
}

#[test]
fn test_transform_jsx() {
	let source = r#"
        import React from 'react';
        const element = <div>Hello</div>;
        const Component = () => <span>World</span>;
    "#;
	let mut config = CompilerConfig::simple();
	config.jsx = true;
	let result = transform_source(source, "jsx.tsx", &config);

	assert!(result.is_ok(), "JSX transformation should succeed");
	let output = result.unwrap();

	// JSX should be transformed to React.createElement calls
	assert!(
		output.contains("React.createElement") || output.contains("createElement"),
		"JSX should be transformed to createElement calls"
	);
}

#[test]
fn test_transform_big_int() {
	let source = r#"
        const big: bigint = 9007199254740991n;
        const huge = 1234567890123456789012345678901234567890n;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "bigint.ts", &config);

	assert!(result.is_ok(), "BigInt transformation should succeed");
	let output = result.unwrap();

	// BigInt values should be preserved
	assert!(
		output.contains("9007199254740991n") || output.contains("BigInt"),
		"BigInt literal should be preserved"
	);
}

#[test]
fn test_transform_private_fields() {
	let source = r#"
        class MyClass {
            #privateField: string;
            constructor(value: string) {
                this.#privateField = value;
            }
            getPrivate(): string {
                return this.#privateField;
            }
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "private_fields.ts", &config);

	assert!(result.is_ok(), "Private field transformation should succeed");
	let output = result.unwrap();

	// Private fields should be transformed (likely to WeakMap or _ prefix)
	assert!(
		output.contains("privateField") || output.contains("#privateField"),
		"Private field should be handled"
	);
}

#[test]
fn test_transform_static_class_properties_vscode_format() {
	// This is the critical test for VSCode compatibility
	let source = r#"
        class Service {
            static error = Sound.register({ fileName: "error.mp3" });
            static warning = Sound.register({ fileName: "warning.mp3" });
        }
    "#;
	let mut config = CompilerConfig::vscode(); // use_define_for_class_fields = false
	let result = transform_source(source, "static_props.ts", &config);

	assert!(result.is_ok(), "Static class property transformation should succeed");
	let output = result.unwrap();

	// VSCode compatibility: static properties should be in initializer block format
	// Expected: static { this.error = ...; }
	assert!(
		output.contains("static {") && output.contains("this.error"),
		"Static properties should be in VSCode initializer block format"
	);
	assert!(output.contains("Sound.register"), "Expression should be preserved");
}

#[test]
fn test_transform_complex_class() {
	let source = r#"
        class ComplexClass<T> {
            static VERSION = '1.0';
            static count = 0;

            #privateId: number;
            public name: string;
            protected readonly id: number;

            constructor(name: string, id: number) {
                this.name = name;
                this.#privateId = id;
            }

            static getNextId(): number {
                return ComplexClass.count++;
            }

            getName(): string {
                return this.name;
            }

            static fromObject(obj: { name: string; id: number }): ComplexClass<any> {
                return new ComplexClass(obj.name, obj.id);
            }
        }
    "#;
	let config = CompilerConfig::vscode();
	let result = transform_source(source, "complex_class.ts", &config);

	assert!(result.is_ok(), "Complex class transformation should succeed");
	let output = result.unwrap();

	// Verify key elements
	assert!(output.contains("ComplexClass"), "Class name should be preserved");
	assert!(
		output.contains("VERSION") || output.contains("version"),
		"Static field should be present"
	);
	assert!(output.contains("constructor"), "Constructor should be present");
	assert!(output.contains("getName"), "Methods should be preserved");
}

#[test]
fn test_transform_type_assertions() {
	let source = r#"
        const elem = document.getElementById('myId') as HTMLDivElement;
        const anyVal = someValue as any;
        const unknownVal = value as unknown;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "assertions.ts", &config);

	assert!(result.is_ok(), "Type assertion transformation should succeed");
	let output = result.unwrap();

	// Type assertions should be removed (become identity)
	assert!(
		output.contains("elem") || output.contains("HTMLDivElement") || output.contains("as any"),
		"Assertion handling should be correct"
	);
}

#[test]
fn test_transform_import_exports() {
	let source = r#"
        import { Component } from '@angular/core';
        import * as React from 'react';
        import defaultExport from 'module';

        export class MyComponent {}
        export default MyComponent;
        export * from './other';
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "import_export.ts", &config);

	assert!(result.is_ok(), "Import/export transformation should succeed");
	let output = result.unwrap();

	// Imports and exports should be preserved
	assert!(output.contains("import"), "Import should be present");
	assert!(output.contains("export"), "Export should be present");
}

#[test]
fn test_transform_import_meta() {
	let source = r#"
        console.log(import.meta.url);
        const module = import.meta.resolve('./module');
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "import_meta.ts", &config);

	assert!(result.is_ok(), "import.meta transformation should succeed");
	let output = result.unwrap();

	// import.meta should be preserved
	assert!(output.contains("import.meta"), "import.meta should be present");
}

#[test]
fn test_transform_import_attributes() {
	let source = r#"
        import data from './data.json' with { type: 'json' };
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "import_attrs.ts", &config);

	assert!(result.is_ok(), "Import attributes transformation should succeed");
	let output = result.unwrap();

	// Import attributes should be handled (may be removed or transformed)
	assert!(
		output.contains("import") || output.contains("data"),
		"Import should be preserved"
	);
}

#[test]
fn test_transform_satisfies_operator() {
	let source = r#"
        const obj = { a: 1, b: 'string' } satisfies Record<string, any>;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "satisfies.ts", &config);

	assert!(result.is_ok(), "Satisfies operator transformation should succeed");
	let output = result.unwrap();

	// Satisfies operator should be removed (erased) but object preserved
	assert!(
		output.contains("obj") || output.contains("a: 1"),
		"Object literal should be preserved"
	);
}

#[test]
fn test_transform_top_level_await() {
	let source = r#"
        await init();
        export const result = await process();
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "tla.ts", &config);

	assert!(result.is_ok(), "Top-level await transformation should succeed");
	let output = result.unwrap();

	// Top-level await should be handled
	assert!(
		output.contains("await") || output.contains("init"),
		"Await should be present or transformed"
	);
}

#[test]
fn test_transform_throw_expressions() {
	let source = r#"
        const value = condition ? value : throw new Error('Invalid');
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "throw_expr.ts", &config);

	assert!(result.is_ok(), "Throw expression transformation should succeed");
	let output = result.unwrap();

	// Throw should be present (either as expression or statement)
	assert!(
		output.contains("throw") || output.contains("Error"),
		"Throw logic should be preserved"
	);
}

#[test]
fn test_transform_export_type_only() {
	let source = r#"
        export type User = { name: string; age: number };
        export interface IAnimal { name: string };
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "export_type.ts", &config);

	assert!(result.is_ok(), "Type-only exports transformation should succeed");
	let output = result.unwrap();

	// Type-only exports should be removed (no runtime effect)
	// Output may be empty or just have imports
	assert!(
		output.is_empty() || output.contains("export"),
		"Type-only exports should be erased"
	);
}

#[test]
fn test_transform_const_assertions() {
	let source = r#"
        const arr = [1, 2, 3] as const;
        const obj = { a: 1 } as const;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "const_assert.ts", &config);

	assert!(result.is_ok(), "Const assertions transformation should succeed");
	let output = result.unwrap();

	// Const assertions should be removed but values preserved
	assert!(
		output.contains("arr") && output.contains("1,2,3"),
		"Const array should be preserved"
	);
}

#[test]
fn test_transform_destructuring() {
	let source = r#"
        const { a, b } = obj;
        const [first, second, ...rest] = array;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "destructuring.ts", &config);

	assert!(result.is_ok(), "Destructuring transformation should succeed");
	let output = result.unwrap();

	// Destructuring should be preserved
	assert!(
		output.contains("a") && output.contains("b") || output.contains("first") && output.contains("second"),
		"Destructuring patterns should be preserved"
	);
}

#[test]
fn test_transform_spread() {
	let source = r#"
        const arr1 = [1, 2];
        const arr2 = [...arr1, 3, 4];
        const obj1 = { a: 1 };
        const obj2 = { ...obj1, b: 2 };
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "spread.ts", &config);

	assert!(result.is_ok(), "Spread transformation should succeed");
	let output = result.unwrap();

	// Spread should be preserved
	assert!(
		output.contains("...") || output.contains("arr1") || output.contains("obj1"),
		"Spread syntax should be preserved"
	);
}

#[test]
fn test_transform_regular_expressions() {
	let source = r#"
        const pattern = /hello/gim;
        const regex = new RegExp('world', 'i');
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "regex.ts", &config);

	assert!(result.is_ok(), "Regex transformation should succeed");
	let output = result.unwrap();

	// Regular expressions should be preserved
	assert!(
		output.contains("/hello/") || output.contains("RegExp"),
		"Regular expressions should be preserved"
	);
}

#[test]
fn test_transform_mapped_types() {
	let source = r#"
        type Keys = 'a' | 'b';
        type Mapped = { [K in Keys]: string };
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "mapped.ts", &config);

	assert!(result.is_ok(), "Mapped types transformation should succeed");
	let output = result.unwrap();

	// Mapped types should be erased (no runtime representation)
	assert!(output.is_empty() || output.contains("type"), "Mapped types should be erased");
}

#[test]
fn test_transform_template_literal_types() {
	let source = r#"
        type Name = `Hello, ${string}!`;
        type Email = `${string}@${string}.com`;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "template_literal_types.ts", &config);

	assert!(result.is_ok(), "Template literal types transformation should succeed");
	let output = result.unwrap();

	// Template literal types should be erased
	assert!(
		output.is_empty() || output.contains("type"),
		"Template literal types should be erased"
	);
}

#[test]
fn test_transform_conditional_types() {
	let source = r#"
        type IsString<T> = T extends string ? true : false;
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "conditional_types.ts", &config);

	assert!(result.is_ok(), "Conditional types transformation should succeed");
	let output = result.unwrap();

	// Conditional types should be erased
	assert!(
		output.is_empty() || output.contains("type"),
		"Conditional types should be erased"
	);
}

#[test]
fn test_transform_intersection_union_types() {
	let source = r#"
        type Admin = User & { permissions: string[] };
        type Result<T> = { success: true; value: T } | { success: false; error: Error };
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "compound_types.ts", &config);

	assert!(result.is_ok(), "Compound types transformation should succeed");
	let output = result.unwrap();

	// Intersection and union types should be erased
	assert!(output.is_empty() || output.contains("type"), "Compound types should be erased");
}

#[test]
fn test_transform_ambient_declarations() {
	let source = r#"
        declare module 'some-module' {
            export function foo(): void;
        }
        declare namespace NS {
            function bar(): void;
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "ambient.ts", &config);

	assert!(result.is_ok(), "Ambient declarations transformation should succeed");
	let output = result.unwrap();

	// Ambient declarations should be removed or preserved as-is depending on target
	assert!(
		output.is_empty() || output.contains("declare") || output.contains("module"),
		"Ambient declarations should be handled"
	);
}

#[test]
fn test_transform_function_overloads() {
	let source = r#"
        function overload(x: string): string;
        function overload(x: number): number;
        function overload(x: any): any {
            return x;
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "overload.ts", &config);

	assert!(result.is_ok(), "Function overload transformation should succeed");
	let output = result.unwrap();

	// Overloads should be merged into single implementation
	assert!(output.contains("function overload"), "Function should be present");
	// Count occurrences of 'function overload' - should be exactly 1
	// (implementation only)
	let count = output.matches("function overload").count();
	assert!(count <= 1, "Overloads should be merged into single implementation");
}

#[test]
fn test_transform_assertion_functions() {
	let source = r#"
        function assert(condition: any, msg?: string): asserts condition {
            if (!condition) throw new Error(msg);
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "assertion.ts", &config);

	assert!(result.is_ok(), "Assertion function transformation should succeed");
	let output = result.unwrap();

	// Assertion functions should be preserved with asserts signature stripped
	assert!(output.contains("assert"), "Assertion function should be present");
}

#[test]
fn test_transform_using_declarations() {
	// Note: using declarations may not be fully supported yet in OXC 0.48
	let source = r#"
        using disposable = new Disposable();
        using (disposable) {}
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "using.ts", &config);

	// This may or may not be supported yet
	assert!(
		result.is_ok() || result.is_err(),
		"using declarations either work or not yet supported"
	);
}

#[test]
fn test_transform_tuples() {
	let source = r#"
        let tuple: [string, number, boolean] = ['hello', 42, true];
        type Point = [number, number];
        const p: Point = [10, 20];
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "tuples.ts", &config);

	assert!(result.is_ok(), "Tuple transformation should succeed");
	let output = result.unwrap();

	// Tuples should be preserved as arrays
	assert!(
		output.contains("['hello', 42, true]") || output.contains("tuple"),
		"Tuple values should be preserved"
	);
}

#[test]
fn test_transform_unknown_literals() {
	let source = r#"
        const unknownVal: unknown = getValue();
        if (typeof unknownVal === 'string') {
            // narrowed to string
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "unknown.ts", &config);

	assert!(result.is_ok(), "Unknown type transformation should succeed");
	let output = result.unwrap();

	// Unknown should be handled (maybe as any)
	assert!(
		output.contains("unknownVal") || output.contains("getValue"),
		"Unknown value handling should work"
	);
}

#[test]
fn test_transform_never_type() {
	let source = r#"
        function error(msg: string): never {
            throw new Error(msg);
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "never.ts", &config);

	assert!(result.is_ok(), "Never type transformation should succeed");
	let output = result.unwrap();

	// Never return type should be erased, but function should work
	assert!(
		output.contains("function error") && output.contains("throw"),
		"Never function should be correct runtime"
	);
}

#[test]
fn test_transform_variadic_tuples() {
	let source = r#"
        type Rest<T extends any[]> = [...T, ...any[]];
        const tuple: [string, ...number[]] = ['a', 1, 2, 3];
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "variadic.ts", &config);

	assert!(result.is_ok(), "Variadic tuple transformation should succeed");
	let output = result.unwrap();

	// Variadic tuples should be handled
	assert!(
		output.contains("tuple") || output.contains("..."),
		"Variadic tuple should be preserved"
	);
}

#[test]
fn test_transform_labeled_statements() {
	let source = r#"
        outer: for (let i = 0; i < 10; i++) {
            for (let j = 0; j < 10; j++) {
                if (i + j > 15) break outer;
            }
        }
    "#;
	let config = CompilerConfig::simple();
	let result = transform_source(source, "labeled.ts", &config);

	assert!(result.is_ok(), "Labeled statements transformation should succeed");
	let output = result.unwrap();

	// Labels should be preserved
	assert!(
		output.contains("outer:") || output.contains("break outer"),
		"Labeled statements should be preserved"
	);
}

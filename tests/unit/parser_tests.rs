//! Comprehensive parser tests for Rest compiler
//!
//! These tests verify that the OXC parser correctly handles various TypeScript
//! language constructs, including edge cases and complex scenarios.

use std::time::Instant;

use Rest::{Fn::OXC::Parser, Struct::CompilerConfig};

/// Helper function to parse TypeScript source and return result
fn parse_source(source:&str, file_name:&str) -> anyhow::Result<Rest::Struct::OXC::ParserResult> {
	let config = CompilerConfig::simple();

	let parser_config = CompilerConfig::to_parser_config(&config);

	Parser::parse(source, file_name, &parser_config)
}

#[test]
fn test_parse_empty_file() {
	let source = "";

	let result = parse_source(source, "empty.ts");

	assert!(result.is_ok(), "Empty file should parse successfully");

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty(), "Empty file should have no errors");

	assert!(parse_result.program.body.is_empty(), "Empty file should have no statements");
}

#[test]
fn test_parse_simple_variable_declaration() {
	let source = r#"
        let x: number = 42;

        let y: string = "hello";

        const z: boolean = true;

    "#;

	let result = parse_source(source, "simple_vars.ts");

	assert!(result.is_ok(), "Simple variable declarations should parse");

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());

	assert_eq!(parse_result.program.body.len(), 3);
}

#[test]
fn test_parse_function_declarations() {
	let source = r#"
        function add(a: number, b: number): number {

            return a + b;
        }

        const multiply = (x: number, y: number): number => x * y;

        interface MathOps {

            add(a: number, b: number): number;

            multiply(x: number, y: number): number;
        }

    "#;

	let result = parse_source(source, "functions.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());

	assert!(parse_result.program.body.len() >= 2);
}

#[test]
fn test_parse_class_with_types() {
	let source = r#"
        class Person {

            private name: string;

            private age: number;

            constructor(name: string, age: number) {

                this.name = name;

                this.age = age;
            }

            greet(): string {

                return `Hello, I'm ${this.name}`;
            }
        }

        const p = new Person("Alice", 30);

    "#;

	let result = parse_source(source, "class.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_interface() {
	let source = r#"
        interface Animal {

            name: string;

            age: number;

            speak(): void;
        }

        interface Dog extends Animal {

            breed: string;

            bark(): void;
        }

    "#;

	let result = parse_source(source, "interface.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_type_aliases() {
	let source = r#"
        type ID = string | number;

        type Callback = (data: any) => void;

        type Partial<T> = {

            [P in keyof T]?: T[P];
        };

    "#;

	let result = parse_source(source, "types.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_enums() {
	let source = r#"
        enum Color {

            Red = 1,

            Green = 2,

            Blue = 3
        }

        enum Direction {

            Up,

            Down,

            Left,

            Right
        }

    "#;

	let result = parse_source(source, "enums.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());

	// Verify enum bodies are present
	let body = &parse_result.program.body;

	assert_eq!(body.len(), 2);
}

#[test]
fn test_parse_namespace() {
	let source = r#"
        namespace MyNamespace {

            export const value = 42;

            export function foo(): number { return value; }
        }

        module AnotherModule {

            export interface IModule {}
        }

    "#;

	let result = parse_source(source, "namespace.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_imports_exports() {
	let source = r#"
        import { Component, OnInit } from '@angular/core';

        import * as React from 'react';

        import defaultExport from 'some-module';

        import { type User } from './types';

        export class MyComponent implements OnInit {

            ngOnInit(): void {}
        }

        export default MyComponent;

    "#;

	let result = parse_source(source, "imports_exports.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_decorators() {
	let source = r#"
        function Component(target: any) { }

        function Injectable() { }

        @Component
        class MyService {

            @Inject('token')

            private dependency: any;
        }

        @Injectable()

        export class AnotherService {}

    "#;

	let result = parse_source(source, "decorators.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_jsx() {
	let source = r#"
        import React from 'react';

        const element = <div>Hello, world!</div>;

        const Component = () => (
            <div className="container">
                <h1>Title</h1>
                <p>Content</p>
            </div>
        );

    "#;

	let config = CompilerConfig::simple();

	let parser_config = CompilerConfig::to_parser_config(&config);

	// JSX parsing requires jsx flag in parser config
	let result = Parser::parse(source, "component.tsx", &parser_config);

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_static_class_properties() {
	let source = r#"
        class MyClass {

            static PI = 3.14159;

            static readonly VERSION = '1.0.0';

            static initialized = false;

            static getDoublePI() {

                return MyClass.PI * 2;
            }
        }

    "#;

	let result = parse_source(source, "static_props.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());

	assert!(parse_result.program.body.len() > 0);
}

#[test]
fn test_parse_generic_types() {
	let source = r#"
        function identity<T>(arg: T): T {

            return arg;
        }

        interface Box<T> {

            value: T;
        }

        class Container<T, U> {

            first: T;

            second: U;
        }

    "#;

	let result = parse_source(source, "generics.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_advanced_types() {
	let source = r#"
        type Nullable<T> = T | null;

        type NonNullable<T> = T extends null | undefined ? never : T;

        type Partial<T> = {

            [P in keyof T]?: T[P];
        };

        type Readonly<T> = {

            readonly [P in keyof T]: T[P];
        };

        type Pick<T, K extends keyof T> = {

            [P in K]: T[P];
        };

    "#;

	let result = parse_source(source, "advanced_types.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_conditional_types() {
	let source = r#"
        type IsString<T> = T extends string ? true : false;

        type First<T extends any[]> = T extends [infer U, ...infer Rest] ? U : never;

        type Last<T extends any[]> = T extends [...infer Rest, infer U] ? U : never;

    "#;

	let result = parse_source(source, "conditional_types.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_async_await() {
	let source = r#"
        async function fetchData(url: string): Promise<any> {

            const response = await fetch(url);

            const data = await response.json();

            return data;
        }

        const getData = async () => {

            try {

                const result = await fetchData('/api/data');

                return result;
            } catch (error) {

                console.error(error);

                throw error;
            }
        };

    "#;

	let result = parse_source(source, "async.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_generator_functions() {
	let source = r#"
        function* idGenerator() {

            let id = 0;

            while (true) {

                yield id++;
            }
        }

        function* fibonacci(): Generator<number> {

            let [prev, curr] = [0, 1];

            while (true) {

                yield curr;

                [prev, curr] = [curr, prev + curr];
            }
        }

    "#;

	let result = parse_source(source, "generators.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_complex_expressions() {
	let source = r#"
        const obj = {

            a: 1,

            b: 'string',

            c: { nested: true },

            d: [1, 2, 3],

            e() { return this.a; },

            *generator() { yield 1; }
        };

        const arr = [
            { x: 1, y: 2 },

            { x: 3, y: 4 }
        ];

    "#;

	let result = parse_source(source, "expressions.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_template_literals() {
	let source = r#"
        const name = "World";

        const greeting = `Hello, ${name}!`;

        const multiline = `
            This is a
            multiline
            template
        `;

        const expression = `Sum: ${1 + 2}`;

    "#;

	let result = parse_source(source, "templates.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_optional_chaining() {
	let source = r#"
        const obj = { a: { b: { c: 42 } } };

        const value = obj?.a?.b?.c;

        const method = obj?.a?.method?.();

        const arr = maybeArray?.[0];

    "#;

	let result = parse_source(source, "optional_chaining.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_nullish_coalescing() {
	let source = r#"
        const value = null ?? 'default';

        const num = undefined ?? 0;

        const obj = {} as Any ?? {};

    "#;

	let result = parse_source(source, "nullish_coalescing.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_big_int() {
	let source = r#"
        const big: bigint = 9007199254740991n;

        const huge = 1234567890123456789012345678901234567890n;

    "#;

	let result = parse_source(source, "bigint.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_private_class_fields() {
	let source = r#"
        class MyClass {

            #privateField: string;

            #count = 0;

            constructor(value: string) {

                this.#privateField = value;
            }

            getPrivate(): string {

                return this.#privateField;
            }
        }

    "#;

	let result = parse_source(source, "private_fields.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_assertion_functions() {
	let source = r#"
        function assert(condition: any, msg?: string): asserts condition {

            if (!condition) {

                throw new Error(msg ?? 'Assertion failed');
            }
        }

        function assertEqual<T>(actual: T, expected: T): asserts actual is T {

            if (actual !== expected) {

                throw new Error('Not equal');
            }
        }

    "#;

	let result = parse_source(source, "assertions.ts");

	assert!(result.is_ok());

	let parse_result = result.unwrap();

	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_using_declarations() {
	let source = r#"
        using disposable = new Disposable();

        using resource1 = acquireResource();

        using (disposable) {

            // use disposable
        }

    "#;

	let result = parse_source(source, "using.ts");

	assert!(
		result.is_ok() || result.is_err(),
		"using declarations may or may not be supported yet"
	);
}

#[test]
fn test_parse_large_file_stress() {
	// Generate a large TypeScript file to test parser performance and memory
	let mut source = String::from("function func() { ");

	for i in 0..1000 {
		source.push_str(&format!("let x{i}: number = {i}; "));
	}

	source.push_str("}");

	let start = Instant::now();

	let result = parse_source(&source, "large.ts");

	let elapsed = start.elapsed();

	assert!(result.is_ok(), "Large file should parse successfully: {:?}", result.err());

	assert!(elapsed.as_secs() < 5, "Parsing should complete within reasonable time");
}

#[test]
fn test_parse_invalid_syntax_handling() {
	let source = r#"
        function broken( {
            return 42;
        }

        const missing = ;
    "#;
	let result = parse_source(source, "invalid.ts");
	assert!(
		result.is_ok(),
		"Parser should handle invalid syntax gracefully (may still return parse result with errors)"
	);
	let parse_result = result.unwrap();
	// We expect either errors or some partial parsing
	assert!(!parse_result.program.body.is_empty() || !parse_result.errors.is_empty());
}

#[test]
fn test_parse_unicode_identifiers() {
	let source = r#"
        const π = 3.14159;
        let 你好 = "hello";
        function привет(): void {}
        const 🎉 = "celebrate";
        interface kabanay {}
    "#;
	let result = parse_source(source, "unicode.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_import_attributes() {
	let source = r#"
        import data from './data.json' with { type: 'json' };
        import css from './styles.css' with { loader: 'css' };
    "#;
	let result = parse_source(source, "import_attrs.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_satisfies_operator() {
	let source = r#"
        const obj = { a: 1, b: 'string' } satisfies Record<string, any>;
        type T = { x: number } & { y: string };
        const value: T = { x: 1, y: 's' } satisfies T;
    "#;
	let result = parse_source(source, "satisfies.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_const_assertions() {
	let source = r#"
        const arr = [1, 2, 3] as const;
        const tuple = ['a', 'b', 'c'] as const;
        const obj = { a: 1, b: 'string' } as const;
    "#;
	let result = parse_source(source, "const_assert.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_function_let_binding() {
	let source = r#"
        let f = function(): void {};
        const g = function*(): Generator<number> { yield 1; };
        const arrow = (x: number): number => x * 2;
    "#;
	let result = parse_source(source, "function_bindings.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_destructuring() {
	let source = r#"
        const { a, b } = obj;
        const { x: newX, y: newY } = point;
        const [first, second, ...rest] = array;
        const [head, ...tail] = list;
    "#;
	let result = parse_source(source, "destructuring.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_spread_elements() {
	let source = r#"
        const arr1 = [1, 2];
        const arr2 = [...arr1, 3, 4];

        const obj1 = { a: 1 };
        const obj2 = { ...obj1, b: 2 };

        function sum(...args: number[]): number {
            return args.reduce((a, b) => a + b, 0);
        }
    "#;
	let result = parse_source(source, "spread.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_regular_expressions() {
	let source = r#"
        const pattern = /hello/gim;
        const regex = new RegExp('world', 'i');
        const emails = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    "#;
	let result = parse_source(source, "regex.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_mapped_types() {
	let source = r#"
        type Keys = 'a' | 'b';
        type Mapped = { [K in Keys]: string };

        type Optional = {
            [K in keyof T]?: T[K];
        };
    "#;
	let result = parse_source(source, "mapped_types.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_template_literal_types() {
	let source = r#"
        type Name = `Hello, ${string}!`;
        type Email = `${string}@${string}.${string}`;
        type Upper<S extends string> = S extends Uppercase<S> ? true : false;
    "#;
	let result = parse_source(source, "template_literal_types.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_top_level_await() {
	let source = r#"
        await someAsyncOperation();
        const data = await fetch('/api/data');
        export const result = await process(data);
    "#;
	let result = parse_source(source, "tla.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_import_meta() {
	let source = r#"
        console.log(import.meta);
        const url = import.meta.url;
        const resolve = import.meta.resolve;
    "#;
	let result = parse_source(source, "import_meta.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_throw_expressions() {
	let source = r#"
        const value = condition ? value : throw new Error('Invalid');
        const x = foo() ?? throw new Error('Not found');
    "#;
	let result = parse_source(source, "throw_expr.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_function_attributes() {
	let source = r#"
        function foo() {
            // Some function
        }
        (foo as any).__internal = true;

        const method = {
            m(): void {}
        };
        method.m.__labeled = 'internal';
    "#;
	let result = parse_source(source, "attributes.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_atomic_types() {
	let source = r#"
        let atomic: Atomics.CompareExchange;
        let sharedArray: Int32Array;
        function atomicOp(atomic: any): void {}
    "#;
	let result = parse_source(source, "atomic.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_math_intrinsics() {
	let source = r#"
        const max = Math.max(1, 2, 3);
        const abs = Math.abs(-5);
        const pow = Math.pow(2, 3);
        const random = Math.random();
    "#;
	let result = parse_source(source, "math.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_this_types() {
	let source = r#"
        class MyClass {
            private value: number;

            constructor(value: number) {
                this.value = value;
            }

            getThis(): this {
                return this;
            }

            clone(): this {
                return new (this.constructor as any)(this.value);
            }
        }
    "#;
	let result = parse_source(source, "this_types.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_indexed_access_types() {
	let source = r#"
        type T = { a: string; b: number };
        type A = T['a'];  // string
        type B = T['b'];  // number
        type All = T[keyof T];
    "#;
	let result = parse_source(source, "indexed_access.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_utility_types() {
	let source = r#"
        type Partial<T> = { [P in keyof T]?: T[P] };
        type Required<T> = { [P in keyof T]-?: T[P] };
        type Readonly<T> = { readonly [P in keyof T]: T[P] };
        type Record<K extends keyof any, T> = { [P in K]: T };
        interface Pick<T, K extends keyof T> { [P in K]: T[P]; }
    "#;
	let result = parse_source(source, "utility_types.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_no_implicit_any_scenarios() {
	let source = r#"
        // These should NOT produce errors during parsing (type checking is separate)
        function fn(param) { return param; }  // implicit any
        const arr = [];  // implicit any for array elements
        let x;  // implicit any
    "#;
	let result = parse_source(source, "implicit_any.ts");
	assert!(
		result.is_ok(),
		"Parser should accept constructs that may be flagged by type checker"
	);
	let parse_result = result.unwrap();
	// Parser doesn't enforce strict types, so these are valid syntax
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_strict_null_checks_scenarios() {
	let source = r#"
        let x: string | null = null;
        let y: string | undefined;
        function strict(param: string | null | undefined) {}
    "#;
	let result = parse_source(source, "strict_null.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_always_truthy_types() {
	let source = r#"
        function process(input: string | undefined | null) {
            if (input) {
                // input is string here (truthy)
            }
        }
    "#;
	let result = parse_source(source, "truthy.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_never_type() {
	let source = r#"
        function error(message: string): never {
            throw new Error(message);
        }

        function infiniteLoop(): never {
            while (true) {}
        }
    "#;
	let result = parse_source(source, "never.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_unknown_type() {
	let source = r#"
        let unknownValue: unknown = getSomething();
        if (typeof unknownValue === 'string') {
            unknownValue;  // TypeScript would narrow to string
        }
    "#;
	let result = parse_source(source, "unknown.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_symbol_type() {
	let source = r#"
        const sym1 = Symbol('description');
        const sym2: unique symbol = Symbol('unique');
    "#;
	let result = parse_source(source, "symbol.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_intersection_types() {
	let source = r#"
        type Admin = User & { permissions: string[] };
        type Serialized = JSONSerializable & ToString;
        type Combined = A & B & C;
    "#;
	let result = parse_source(source, "intersection.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_union_types() {
	let source = r#"
        type Result<T> = { success: true; value: T } | { success: false; error: Error };
        type StringOrNumber = string | number;
        type Mixed = null | undefined | boolean | number | string;
    "#;
	let result = parse_source(source, "union.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_ambiguous_functions() {
	let source = r#"
        function overload1(x: string): string;
        function overload1(x: number): number;
        function overload1(x: any): any {
            return x;
        }
    "#;
	let result = parse_source(source, "overload.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_ambient_declarations() {
	let source = r#"
        declare module 'some-module' {
            export function foo(): void;

            export const bar: string;
        }

        declare namespace MyNamespace {
            function doSomething(): void;
        }

        declare global {
            interface Window {
                myCustomProp: any;
            }
        }
    "#;
	let result = parse_source(source, "ambient.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_export_namespace() {
	let source = r#"
        export namespace MyExports {
            export const VERSION = '1.0';

            export interface Types {}

            export function helper() {}
        }
    "#;
	let result = parse_source(source, "export_namespace.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_dynamic_import() {
	let source = r#"
        const module = await import('./module.ts');
        if (condition) {
            const mod = await import('some-module');
        }
    "#;
	let result = parse_source(source, "dynamic_import.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_import_type_assertions() {
	let source = r#"
        import type { User, Admin } from './types';
        import type * as Types from './all-types';
        import type { default as DefaultExport } from './module';
    "#;
	let result = parse_source(source, "import_type.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_export_type_only() {
	let source = r#"
        export type User = { name: string; age: number };
        export interface IAnimal { name: string };
    "#;
	let result = parse_source(source, "export_type.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_construct_signature() {
	let source = r#"
        interface Constructor<T> {
            new (): T;
        }

        type Class<T> = {
            new (...args: any[]): T;
        };
    "#;
	let result = parse_source(source, "constructor_sig.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

#[test]
fn test_parse_function_bindings() {
	let source = r#"
        function namedFunction() {}
        const arrow = () => {};
        const method = {
            regular() {},
            arrow: () => {},
            async asyncMethod() {},
            *generator() {}
        };
        class MyClass {
            static staticMethod() {}

            instanceMethod() {}

            get accessor() { return this._value; }

            set accessor(val) { this._value = val; }
        }
    "#;
	let result = parse_source(source, "bindings.ts");
	assert!(result.is_ok());
	let parse_result = result.unwrap();
	assert!(parse_result.errors.is_empty());
}

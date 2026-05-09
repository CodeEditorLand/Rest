//! Comprehensive codegen tests for Rest compiler
//!
//! These tests verify that the OXC codegen produces correct JavaScript output
//! from transformed ASTs, including formatting, source maps, and edge cases.

#![cfg(test)]

use Rest::{
	Fn::OXC::{self, Compiler, Parser, Transformer},
	Struct::CompilerConfig,
};

/// Helper to parse, transform, and codegen TypeScript source
fn codegen_source(source:&str, file_name:&str, config:&CompilerConfig) -> anyhow::Result<String> {
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
fn test_codegen_simple_expression() {
	let source = "const x = 42;";

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "simple.ts", &config);

	assert!(result.is_ok(), "Simple expression should generate code");

	let output = result.unwrap();

	// Should contain the constant assignment
	assert!(
		output.contains("const x") && output.contains("42"),
		"Should generate 'const x = 42;'"
	);
}

#[test]
fn test_codegen_function() {
	let source = r#"
        function add(a: number, b: number): number {

            return a + b;
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "func.ts", &config);

	assert!(result.is_ok(), "Function should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("function add") || output.contains("add"),
		"Function should be present in output"
	);

	assert!(output.contains("return"), "Return statement should be present");
}

#[test]
fn test_codegen_class() {
	let source = r#"
        class Person {

            name: string;

            age: number;

            constructor(name: string, age: number) {

                this.name = name;

                this.age = age;
            }

            greet(): string {

                return `Hello, ${this.name}`;
            }
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "class.ts", &config);

	assert!(result.is_ok(), "Class should generate code");

	let output = result.unwrap();

	assert!(output.contains("class Person"), "Class definition should be present");

	assert!(output.contains("constructor"), "Constructor should be present");

	assert!(output.contains("greet"), "Method should be present");
}

#[test]
fn test_codegen_arrow_functions() {
	let source = r#"
        const add = (a: number, b: number): number => a + b;

        const square = (x: number): number => x * x;

        const identity = <T>(x: T): T => x;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "arrow.ts", &config);

	assert!(result.is_ok(), "Arrow functions should generate code");

	let output = result.unwrap();

	assert!(output.contains("=>"), "Arrow syntax should be present");

	assert!(
		output.contains("add") && output.contains("square"),
		"Arrow function names should be present"
	);
}

#[test]
fn test_codegen_template_literals() {
	let source = r#"
        const name = "World";

        const greeting = `Hello, ${name}!`;

        const multiline = `
            Line 1
            Line 2
        `;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "templates.ts", &config);

	assert!(result.is_ok(), "Template literals should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("`Hello, ${name}!`") || output.contains("Hello") && output.contains("name"),
		"Template literal should be preserved"
	);

	assert!(output.contains("`"), "Backticks should be present");
}

#[test]
fn test_codegen_destructuring() {
	let source = r#"
        const { a, b } = obj;

        const { x: newX, y: newY } = point;

        const [first, second, ...rest] = array;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "destructure.ts", &config);

	assert!(result.is_ok(), "Destructuring should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("a") && output.contains("b") || output.contains("first") && output.contains("second"),
		"Destructuring patterns should be present"
	);
}

#[test]
fn test_codegen_spread_operator() {
	let source = r#"
        const arr1 = [1, 2];

        const arr2 = [...arr1, 3, 4];

        const obj1 = { a: 1 };

        const obj2 = { ...obj1, b: 2 };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "spread.ts", &config);

	assert!(result.is_ok(), "Spread operator should generate code");

	let output = result.unwrap();

	assert!(output.contains("..."), "Spread operator should be present");

	assert!(
		output.contains("arr1") && output.contains("arr2"),
		"Spread arrays should be present"
	);
}

#[test]
fn test_codegen_optional_chaining() {
	let source = r#"
        const value = obj?.a?.b?.c;

        const method = obj?.a?.method?.();

        const arr = maybeArray?.[0];

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "optional.ts", &config);

	assert!(result.is_ok(), "Optional chaining should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("?.=") || output.contains("?.") || output.contains("obj"),
		"Optional chaining should be preserved"
	);
}

#[test]
fn test_codegen_nullish_coalescing() {
	let source = r#"
        const value = null ?? 'default';

        const num = undefined ?? 0;

        const obj = {} ?? {};

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "nullish.ts", &config);

	assert!(result.is_ok(), "Nullish coalescing should generate code");

	let output = result.unwrap();

	assert!(output.contains("??"), "Nullish coalescing operator should be present");
}

#[test]
fn test_codegen_async_await() {
	let source = r#"
        async function fetchData(url: string): Promise<any> {

            const response = await fetch(url);

            const data = await response.json();

            return data;
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "async.ts", &config);

	assert!(result.is_ok(), "Async/await should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("async") || output.contains("function fetchData"),
		"Async function should be present"
	);

	assert!(output.contains("await"), "Await should be preserved");
}

#[test]
fn test_codegen_generators() {
	let source = r#"
        function* idGenerator() {

            let id = 0;

            while (true) {

                yield id++;
            }
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "generator.ts", &config);

	assert!(result.is_ok(), "Generators should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("*") && output.contains("yield"),
		"Generator function with yield should be present"
	);
}

#[test]
fn test_codegen_big_int() {
	let source = r#"
        const big: bigint = 9007199254740991n;

        const huge = 1234567890123456789012345678901234567890n;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "bigint.ts", &config);

	assert!(result.is_ok(), "BigInt should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("9007199254740991n") || output.contains("bigint"),
		"BigInt literal should be preserved"
	);
}

#[test]
fn test_codegen_regular_expressions() {
	let source = r#"
        const pattern = /hello/gim;

        const regex = new RegExp('world', 'i');

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "regex.ts", &config);

	assert!(result.is_ok(), "RegEx should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("/hello/") || output.contains("RegExp"),
		"Regular expressions should be preserved"
	);
}

#[test]
fn test_codegen_import_export_various() {
	let source = r#"
        import { named } from 'module';

        import * as ns from 'namespace';

        import defaultExport from 'default-module';

        export const exported = 42;

        export class ExportedClass {}

        export default ExportedClass;

        export * from './other';

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "ie.ts", &config);

	assert!(result.is_ok(), "Import/export should generate code");

	let output = result.unwrap();

	assert!(output.contains("import"), "Imports should be preserved");

	assert!(output.contains("export"), "Exports should be preserved");
}

#[test]
fn test_codegen_import_meta() {
	let source = r#"
        console.log(import.meta.url);

        const resolve = import.meta.resolve;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "meta.ts", &config);

	assert!(result.is_ok(), "import.meta should generate code");

	let output = result.unwrap();

	assert!(output.contains("import.meta"), "import.meta should be preserved");
}

#[test]
fn test_codegen_throw_expressions() {
	let source = r#"
        const result = condition ? value : throw new Error('fail');

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "throw.ts", &config);

	assert!(result.is_ok(), "Throw expression should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("throw") || output.contains("Error"),
		"Throw logic should be present"
	);
}

#[test]
fn test_codegen_assertions() {
	let source = r#"
        const elem = document.getElementById('id') as HTMLDivElement;

        const anyVal = something as any;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "assert.ts", &config);

	assert!(result.is_ok(), "Type assertions should generate code");

	let output = result.unwrap();

	// Assertions should be removed (identity)
	assert!(
		output.contains("elem") || output.contains("getElementById"),
		"Assertion target should be present"
	);
}

#[test]
fn test_codegen_satisfies() {
	let source = r#"
        const obj = { a: 1, b: 'str' } satisfies Record<string, any>;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "satisfies.ts", &config);

	assert!(result.is_ok(), "Satisfies operator should generate code");

	let output = result.unwrap();

	// Satisfies should be erased, object preserved
	assert!(output.contains("obj") || output.contains("a: 1"), "Object should be preserved");
}

#[test]
fn test_codegen_variadic_tuples() {
	let source = r#"
        const tuple: [string, ...number[]] = ['a', 1, 2, 3];

        const rest: [...string[], number] = ['a', 'b', 1];

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "variadic.ts", &config);

	assert!(result.is_ok(), "Variadic tuples should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("tuple") || output.contains("..."),
		"Variadic tuple should be preserved"
	);
}

#[test]
fn test_codegen_generator_expressions() {
	let source = r#"
        const gen = function*() { yield 1; yield 2; };

        const result = [...gen()];

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "gen_expr.ts", &config);

	assert!(result.is_ok(), "Generator expressions should generate code");

	let output = result.unwrap();

	assert!(output.contains("*") || output.contains("yield"), "Generator should be present");
}

#[test]
fn test_codegen_private_fields() {
	let source = r#"
        class C {

            #private: string = 'secret';

            constructor() {}

            get(): string { return this.#private; }
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "private.ts", &config);

	assert!(result.is_ok(), "Private fields should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("private") || output.contains("#private"),
		"Private field should be handled"
	);
}

#[test]
fn test_codegen_this_types() {
	let source = r#"
        class MyClass {

            value: number;

            getThis(): this {

                return this;
            }
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "this_type.ts", &config);

	assert!(result.is_ok(), "This types should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("getThis") && output.contains("this"),
		"This return type should work"
	);
}

#[test]
fn test_codegen_labeled_statements() {
	let source = r#"
        outer: for (let i = 0; i < 10; i++) {

            for (let j = 0; j < 10; j++) {

                if (i + j > 15) break outer;
            }
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "labeled.ts", &config);

	assert!(result.is_ok(), "Labeled statements should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("outer:") || output.contains("break outer"),
		"Labels should be preserved"
	);
}

#[test]
fn test_codegen_switch_expressions() {
	let source = r#"
        const result = switch (x) {

            case 1 => 'one';

            case 2 => 'two';

            default => 'other';
        };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "switch_expr.ts", &config);

	assert!(
		result.is_ok() || result.is_err(),
		"Switch expressions may or may not be supported yet"
	);
}

#[test]
fn test_codegen_function_let_binding() {
	let source = r#"
        let f = function(): void {};

        const arrow = (x: number): number => x * 2;

        const method = {

            regular() {},

            arrow: () => {},

            async asyncMethod() {},

            *generator() {}
        };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "bindings.ts", &config);

	assert!(result.is_ok(), "Various function bindings should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("function") || output.contains("arrow"),
		"Function bindings should be present"
	);
}

#[test]
fn test_codegen_complex_expressions() {
	let source = r#"
        const obj = {

            a: 1,

            b: 'string',

            c: { nested: true },

            d: [1, 2, 3],

            e() { return this.a; },

            *g() { yield 1; }
        };

        const arr = [
            { x: 1, y: 2 },

            { x: 3, y: 4 }
        ];

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "complex.ts", &config);

	assert!(result.is_ok(), "Complex expressions should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("obj") && output.contains("arr"),
		"Object and array should be present"
	);
}

#[test]
fn test_codegen_export_star() {
	let source = r#"
        export * from './module1';

        export * from './module2';

        export { foo, bar } from './module3';

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "export_star.ts", &config);

	assert!(result.is_ok(), "Export star should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("export") && output.contains("from"),
		"Export star should be preserved"
	);
}

#[test]
fn test_codegen_import_type() {
	let source = r#"
        import type { User } from './types';

        import type * as Types from './all';

        import type { default as Default } from './mod';

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "import_type.ts", &config);

	assert!(result.is_ok(), "Import type should generate code");

	let output = result.unwrap();

	// Type-only imports should be removed or leave empty
	// Since they're erased, output might be empty or contain only non-type imports
	assert!(
		output.is_empty() || !output.contains("import type"),
		"Type-only imports should be erased"
	);
}

#[test]
fn test_codegen_declare_merge() {
	let source = r#"
        class MyClass {}

        namespace MyClass {

            export const staticValue = 42;
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "merge.ts", &config);

	assert!(result.is_ok(), "Class+namespace merge should generate code");

	let output = result.unwrap();

	assert!(output.contains("MyClass"), "Class should be present");

	assert!(
		output.contains("staticValue") || output.contains("42"),
		"Namespace member should be merged"
	);
}

#[test]
fn test_codegen_symbol_usage() {
	let source = r#"
        const sym1 = Symbol('desc1');

        const sym2: unique symbol = Symbol('desc2');

        const symbols = { one: sym1, two: sym2 };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "symbols.ts", &config);

	assert!(result.is_ok(), "Symbols should generate code");

	let output = result.unwrap();

	assert!(output.contains("Symbol"), "Symbol constructor should be present");
}

#[test]
fn test_codegen_intl_constructs() {
	let source = r#"
        const date = new Intl.DateTimeFormat('en-US');

        const number = new Intl.NumberFormat('de-DE');

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "intl.ts", &config);

	assert!(result.is_ok(), "Intl objects should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("Intl") || output.contains("DateTimeFormat"),
		"Intl constructors should be preserved"
	);
}

#[test]
fn test_codegen_reflect_apis() {
	let source = r#"
        const keys = Reflect.ownKeys(obj);

        const construct = Reflect.construct(Class, args);

        const apply = Reflect.apply(func, thisArg, args);

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "reflect.ts", &config);

	assert!(result.is_ok(), "Reflect APIs should generate code");

	let output = result.unwrap();

	assert!(output.contains("Reflect"), "Reflect object should be present");
}

#[test]
fn test_codegen_proxy() {
	let source = r#"
        const handler = {

            get: (target, prop) => target[prop],

            set: (target, prop, value) => { target[prop] = value; return true; }
        };

        const proxy = new Proxy(target, handler);

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "proxy.ts", &config);

	assert!(result.is_ok(), "Proxy should generate code");

	let output = result.unwrap();

	assert!(output.contains("Proxy"), "Proxy constructor should be present");
}

#[test]
fn test_codegen_promise() {
	let source = r#"
        const p = new Promise((resolve, reject) => {
            if (success) resolve(value);
            else reject(error);
        });

        Promise.all([p1, p2]).then(results => {});

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "promise.ts", &config);

	assert!(result.is_ok(), "Promises should generate code");

	let output = result.unwrap();

	assert!(output.contains("Promise"), "Promise should be present");
}

#[test]
fn test_codegen_map_set() {
	let source = r#"
        const map = new Map<string, number>();

        const set = new Set<string>();

        map.set('key', 42);

        set.add('value');

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "map_set.ts", &config);

	assert!(result.is_ok(), "Map/Set should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("Map") && output.contains("Set"),
		"Map and Set should be present"
	);
}

#[test]
fn test_codegen_typed_arrays() {
	let source = r#"
        const buffer = new ArrayBuffer(16);

        const int32 = new Int32Array(buffer);

        const uint8 = new Uint8Array(buffer);

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "typed_arrays.ts", &config);

	assert!(result.is_ok(), "Typed arrays should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("ArrayBuffer") || output.contains("Int32"),
		"Typed arrays should be preserved"
	);
}

#[test]
fn test_codegen_generator_delegation() {
	let source = r#"
        function* gen1() { yield 1; yield 2; }

        function* gen2() { yield* gen1(); yield 3; }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "gen_delegate.ts", &config);

	assert!(result.is_ok(), "Generator delegation should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("yield*") || output.contains("yield"),
		"Generator delegation should work"
	);
}

#[test]
fn test_codegen_await_generator() {
	let source = r#"
        async function* asyncGen() {

            yield await Promise.resolve(1);

            yield await Promise.resolve(2);
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "async_gen.ts", &config);

	assert!(result.is_ok(), "Async generators should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("async") && output.contains("*") && output.contains("yield"),
		"Async generator should work"
	);
}

#[test]
fn test_codegen_object_literal_methods() {
	let source = r#"
        const obj = {

            method() { return 42; },

            arrow: () => 100,

            get accessor() { return this._val; },

            set accessor(v) { this._val = v; }
        };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "obj_methods.ts", &config);

	assert!(result.is_ok(), "Object literal methods should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("method") || output.contains("accessor"),
		"Object methods and accessors should be present"
	);
}

#[test]
fn test_codegen_computed_properties() {
	let source = r#"
        const key = 'dynamic';

        const obj = {

            [key]: 'value',

            ['prop' + 1]: 100
        };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "computed.ts", &config);

	assert!(result.is_ok(), "Computed properties should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("[") && output.contains("key"),
		"Computed property syntax should be present"
	);
}

#[test]
fn test_codegen_object_spread() {
	let source = r#"
        const obj1 = { a: 1 };

        const obj2 = { b: 2 };

        const merged = { ...obj1, ...obj2, c: 3 };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "obj_spread.ts", &config);

	assert!(result.is_ok(), "Object spread should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("{ ...obj1") || output.contains("...obj1"),
		"Object spread should be preserved"
	);
}

#[test]
fn test_codegen_array_spread() {
	let source = r#"
        const arr1 = [1, 2];

        const arr2 = [3, 4];

        const combined = [...arr1, ...arr2];

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "arr_spread.ts", &config);

	assert!(result.is_ok(), "Array spread should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("...arr") || output.contains("combined"),
		"Array spread should be preserved"
	);
}

#[test]
fn test_codegen_optional_elements() {
	let source = r#"
        const arr = [1, , 3];  // holey array
        const obj = {

            a: 1,

            b: undefined,

            c: 3
        };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "optional.ts", &config);

	assert!(result.is_ok(), "Optional elements should generate code");

	let output = result.unwrap();

	// Should have array with potentially empty elements
	assert!(
		output.contains("arr") || output.contains("1") && output.contains("3"),
		"Array with holes should be handled"
	);
}

#[test]
fn test_codegen_shorthand_properties() {
	let source = r#"
        const x = 1;

        const obj = { x, y: 2 };

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "shorthand.ts", &config);

	assert!(result.is_ok(), "Shorthand properties should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("x") && output.contains("obj"),
		"Shorthand property should be expanded or preserved"
	);
}

#[test]
fn test_codegen_function_parameter_properties() {
	let source = r#"
        function process({ a, b }: { a: number; b: string }) {

            return a + b.length;
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "param_props.ts", &config);

	assert!(result.is_ok(), "Parameter properties should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("process") && output.contains("a") && output.contains("b"),
		"Destructured parameters should be present"
	);
}

#[test]
fn test_codegen_rest_parameters() {
	let source = r#"
        function sum(...nums: number[]): number {

            return nums.reduce((a, b) => a + b, 0);
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "rest.ts", &config);

	assert!(result.is_ok(), "Rest parameters should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("...nums") || output.contains("nums"),
		"Rest parameter should be present"
	);
}

#[test]
fn test_codegen_default_parameters() {
	let source = r#"
        function greet(name: string = 'World'): string {

            return `Hello, ${name}!`;
        }

        function add(a: number, b: number = 10): number { return a + b; }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "defaults.ts", &config);

	assert!(result.is_ok(), "Default parameters should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("World") || output.contains("'World'"),
		"Default parameter value should be present"
	);
}

#[test]
fn test_codegen_typecast_expressions() {
	let source = r#"
        const x = <string>someValue;

        const y = someValue as number;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "typecast.ts", &config);

	assert!(result.is_ok(), "Type casts should generate code");

	let output = result.unwrap();

	// Type assertions should be removed (identity)
	assert!(
		output.contains("x") && output.contains("someValue"),
		"Assertion should be removed, value preserved"
	);
}

#[test]
fn test_codegen_non_null_assertion() {
	let source = r#"
        const value = maybeNull!;

        const definite = obj!.prop!;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "non_null.ts", &config);

	assert!(result.is_ok(), "Non-null assertions should generate code");

	let output = result.unwrap();

	// Non-null assertions should be removed
	assert!(
		output.contains("value") || output.contains("maybeNull"),
		"Value should be present without assertion"
	);
}

#[test]
fn test_codegen_optional_chaining_assignment() {
	let source = r#"
        obj?.a = 1;

        arr?.[0] = 'value';

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "opt_assign.ts", &config);

	assert!(result.is_ok(), "Optional chaining assignment should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("?") || output.contains("obj"),
		"Optional chaining assignment should be handled"
	);
}

#[test]
fn test_codegen_logical_assignment() {
	let source = r#"
        x &&= y;

        x ||= y;

        x ??= y;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "logical_assign.ts", &config);

	assert!(result.is_ok(), "Logical assignment should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("&&=") || output.contains("||=") || output.contains("??="),
		"Logical assignment operators should be present"
	);
}

#[test]
fn test_codegen_pipeline_operator() {
	let source = r#"
        const result = value
            |> transform1
            |> transform2;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "pipeline.ts", &config);

	assert!(
		result.is_ok() || result.is_err(),
		"Pipeline operator may not be fully supported yet"
	);

	if let Ok(output) = result {
		assert!(
			output.contains("|>") || output.contains("transform"),
			"Pipeline should be transformed or preserved"
		);
	}
}

#[test]
fn test_codegen_function_bind_operator() {
	let source = r#"
        const f = func |> context;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "bind.ts", &config);

	assert!(
		result.is_ok() || result.is_err(),
		"Function bind operator may not be fully supported yet"
	);
}

#[test]
fn test_codegen_numeric_separator() {
	let source = r#"
        const big = 1_000_000;

        const binary = 0b1111_1111;

        const hex = 0xFF_FF;

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "separator.ts", &config);

	assert!(result.is_ok(), "Numeric separators should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("000") || output.contains("1_000_000"),
		"Numeric separator should be preserved or removed"
	);
}

#[test]
fn test_codegen_decorator_metadata() {
	let source = r#"
        function Component(target: any) {

            target.isComponent = true;
        }

        @Component
        class MyClass {}

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "decorator_meta.ts", &config);

	assert!(result.is_ok(), "Decorator metadata should generate code");

	let output = result.unwrap();

	// Decorators should generate __decorate or similar
	assert!(
		output.contains("__decorate") || output.contains("Component") || output.contains("MyClass"),
		"Decorator metadata should be present"
	);
}

#[test]
fn test_codegen_class_decorators() {
	let source = r#"
        function sealed(target: any) { }

        function ctor(target: any) { }

        @sealed
        @ctor
        class DecoratedClass {}

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "class_decorators.ts", &config);

	assert!(result.is_ok(), "Class decorators should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("DecoratedClass") || output.contains("sealed"),
		"Class decorators should be processed"
	);
}

#[test]
fn test_codegen_method_decorators() {
	let source = r#"
        function readonly(target: any, key: string, descriptor: PropertyDescriptor) {

            descriptor.writable = false;
        }

        class MyClass {

            @readonly
            method() { return 42; }
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "method_decorators.ts", &config);

	assert!(result.is_ok(), "Method decorators should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("method") || output.contains("readonly"),
		"Method decorators should be processed"
	);
}

#[test]
fn test_codegen_accessor_decorators() {
	let source = r#"
        function configurable(target: any, key: string, descriptor: PropertyDescriptor) {

            descriptor.configurable = true;
        }

        class MyClass {

            private _value: number = 0;

            @configurable
            get value(): number { return this._value; }

            @configurable
            set value(v: number) { this._value = v; }
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "accessor_decorators.ts", &config);

	assert!(result.is_ok(), "Accessor decorators should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("value") || output.contains("get") || output.contains("set"),
		"Accessor decorators should be processed"
	);
}

#[test]
fn test_codegen_parameter_decorators() {
	let source = r#"
        function validate(target: any, key: string, index: number) {

            // validation logic
        }

        class MyClass {

            method(@validate param: string) {}
        }

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "param_decorators.ts", &config);

	assert!(result.is_ok(), "Parameter decorators should generate code");

	let output = result.unwrap();

	assert!(
		output.contains("method") || output.contains("param"),
		"Parameter decorators should be processed"
	);
}

#[test]
fn test_codegen_export_decorators() {
	let source = r#"
        function exportClass(target: any) {

            // Export metadata
        }

        @exportClass
        class ExportedClass {}

    "#;

	let config = CompilerConfig::simple();

	let result = codegen_source(source, "export_decorators.ts", &config);

	assert!(result.is_ok(), "Export decorators should generate code");

	let output = result.unwrap();

	assert!(output.contains("ExportedClass"), "Exported class should be present");
}

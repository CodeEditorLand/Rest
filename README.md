<table>
<tr>
<td align="left" valign="middle">
<h3 align="left"> Rest</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">
⛱️
</h3>
</td>
<td align="left" valign="middle">
<h3 align="left"> + </h3>
</td>
<td align="left" valign="middle">
<h3 align="left">
<a href="https://Editor.Land" target="_blank">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://PlayForm.Cloud/Dark/Image/GitHub/Land.svg">
<source media="(prefers-color-scheme: light)" srcset="https://PlayForm.Cloud/Image/GitHub/Land.svg">
<img width="28" alt="Land Logo" src="https://PlayForm.Cloud/Image/GitHub/Land.svg">
</picture>
</a>
</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">
<a href="https://Editor.Land" target="_blank">
Land
</a>
</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">
🏞️
</h3>
</td>
</tr>
</table>

---

# **Rest** ⛱️ The High-Performance TypeScript Compiler for Land 🏞️

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://github.com/CodeEditorLand/Rest/tree/Current/LICENSE)
[![Rust Version](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![OXC Version](https://img.shields.io/badge/OXC-0.48-blue.svg)](https://oxc.rs/)

Welcome to **Rest**, a high-performance TypeScript compiler built with Rust and
OXC, designed for 100% compatibility with VSCode's build process. Rest is one of
the five Elements in the CodeEditorLand architecture, responsible for
**compilation and build tooling**. It replaces esbuild's TypeScript loader with
a Rust-powered compiler that produces VSCode-compatible output.

**Rest** is engineered to:

1. **Deliver High Performance**: Compile TypeScript 2-3x faster than esbuild
   using OXC.
2. **Ensure VSCode Compatibility**: Produce byte-for-byte identical output to
   VSCode's gulp build.
3. **Provide Memory Safety**: Leverage Rust's ownership model for deterministic
   performance without garbage collection.
4. **Support Modern Tooling**: Built on OXC 0.48, the latest TypeScript
   infrastructure.

---

## Key Features 🔐

- **Full TypeScript 5.x Support**: Complete compatibility with TypeScript 5.x
  syntax and features.
- **Decorator Handling**: Proper support for `emitDecoratorMetadata` and
  decorator transformations.
- **Class Fields Control**: Configurable `useDefineForClassFields` behavior
  (VSCode default: false).
- **Parallel Compilation**: Optional `--Parallel` flag for multi-core
  compilation.
- **Directory-Based Compilation**: Process entire directory structures with
  preserved layout.
- **Comprehensive Error Reporting**: Detailed error messages with source
  location information.
- **Compilation Metrics**: Built-in tracking of compilation count, elapsed time,
  and error counts.
- **Source Map Generation**: Planned support for source maps (in progress).

---

## Core Architecture Principles 🏗️

| Principle          | Description                                                              | Key Components Involved          |
| :----------------- | :----------------------------------------------------------------------- | :------------------------------- |
| **Performance**    | Rust + OXC delivers 2-3x faster compilation than esbuild.                | OXC Parser, Transformer, Codegen |
| **Compatibility**  | OXC is used by VSCode internally, ensuring 1:1 output compatibility.     | OXC 0.48, VSCode build process   |
| **Memory Safety**  | No garbage collection, deterministic performance through Rust ownership. | Rust lifetime management         |
| **Modern Tooling** | Built on the latest OXC infrastructure for TypeScript compilation.       | OXC 0.48+                        |

---

## Integration 🛠️

Rest integrates into the build system through environment variables:

```bash
# Set the compiler to Rest
export Compiler=Rest

# Optional: Configure Rest binary path
export REST_BINARY_PATH="path/to/Rest"

# Optional: Enable verbose logging
export REST_VERBOSE=true

# Optional: Enable source maps (when implemented)
export RestSourcemap=true
```

When `Compiler=Rest` is set, the build system intercepts TypeScript files and
delegates compilation to the Rest binary instead of using esbuild's built-in
TypeScript loader.

---

## CLI Usage 📖

```bash
Rest compile [OPTIONS]

Required:
  --input, -i <PATH>    Input directory containing TypeScript files
  --output, -o <PATH>   Output directory for compiled JavaScript

Optional:
  --target <ES2024>     ECMAScript target (default: es2024)
  --module <commonjs>   Module system: commonjs, esmodule (default: commonjs)
  --source-maps         Generate source maps (not yet implemented)
  --Parallel            Enable parallel compilation (default: false)
  --use-define-for-class-fields
                        Use defineForClassFields semantic (default: false)
  --help, -h            Show help
  --version, -V         Show version
```

---

## Configuration ⚙️

Rest supports two main configuration presets:

### Simple (Single-File)

```rust
use Rest::Struct::CompilerConfig;

let config = CompilerConfig::simple();
// - Target: es2024
// - Module: commonjs
// - Private fields: not converted
// - NLS: disabled
// - Workers: disabled
```

### VSCode (Full Pipeline)

```rust
let config = CompilerConfig::vscode();
// - Target: es2024
// - Module: esmodule
// - Private fields: converted to __<name>
// - NLS: enabled (localization processing)
// - Workers: enabled
// - Bundling: enabled
```

---

## Deep Dive & Component Breakdown 🔬

To understand how `Rest`'s internal components interact to provide
high-performance TypeScript compilation, see the following source files:

- **[`Source/Library.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Library.rs)** -
  Binary entry point
- **[`Source/Fn/OXC/Compiler.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Compiler.rs)** -
  Main compiler orchestration
- **[`Source/Fn/OXC/Parser.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Parser.rs)** -
  OXC parser wrapper
- **[`Source/Fn/OXC/Transformer.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Transformer.rs)** -
  AST transformation
- **[`Source/Fn/OXC/Codegen.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Codegen.rs)** -
  Code generation
- **[`Source/Struct/CompilerConfig.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Struct/CompilerConfig.rs)** -
  Advanced configuration

The source files explain the OXC-based compilation pipeline, decorator handling,
and VSCode compatibility transformations.

---

## Architecture 🏛️

```
┌──────────────────────────────────────────────────────────────┐
│ Rest Compiler                                                  │
├──────────────────────────────────────────────────────────────┤
│                                                                │
│ Input: TypeScript files (directory)                            │
│ Output: JavaScript files (directory)                           │
│                                                                │
│ Pipeline:                                                      │
│ 1. Parse (OXC Parser)                                         │
│    └─> AST with 'static lifetime                              │
│ 2. Transform (OXC Transformer)                                │
│    ├─> Strip TypeScript types                                │
│    ├─> Handle decorators                                     │
│    └─> Apply useDefineForClassFields                         │
│ 3. Codegen (OXC Codegen)                                     │
│    └─> Generate JavaScript                                   │
│ 4. Write (Filesystem)                                         │
│    └─> Output with preserved directory structure              │
│                                                                │
│ Configuration:                                                 │
│ - CompilerConfig::simple()                                    │
│ - CompilerConfig::vscode()                                    │
│                                                                │
│ Metrics:                                                       │
│ - Compilation count                                           │
│ - Total elapsed time                                          │
│ - Error count                                                 │
│                                                                │
└──────────────────────────────────────────────────────────────┘
```

---

## Performance 🚀

From benchmark results (first 100 files of VSCode):

- **Single file compile**: ~0.4ms
- **Batch of 100 files**: ~200-300ms
- **Throughput**: 300-500 files/second
- **Memory**: ~50-100MB for large compilations

Rest is significantly faster than esbuild for TypeScript compilation because:

1. OXC is purpose-built for TypeScript (no JS fallback)
2. Zero-copy operations with careful lifetime management
3. No type-checking overhead (like `tsc --noEmit`)
4. Parallel processing optional with `--Parallel` flag

---

## VSCode Compatibility ✅

Rest is designed to produce **byte-for-byte identical** output to VSCode's
gulp/tsb build:

### Verified Compatibilities

- ✅ Decorator transformation (`__decorate` helper)
- ✅ `useDefineForClassFields = false` (default)
- ✅ `emitDecoratorMetadata = true` (default)
- ✅ Target ES2024
- ✅ CommonJS and ESM module formats
- ✅ Private field conversion (with advanced config)
- ✅ Class field initialization patterns

### Not Yet Implemented

- ⏸️ Source map generation (in progress)

---

## Testing 🧪

### Automated Test Suite

Run the comprehensive test suite:

```bash
./Element/Rest/run_tests.sh
```

Tests cover:

- Binary existence and version
- Simple TypeScript compilation
- Class fields and methods
- Decorators with metadata
- Interfaces and types
- Async functions
- Multiple file batches
- VSCode compatibility mode
- ESM output format
- Parallel compilation
- RestPlugin integration

### Integration Testing

Compare Rest output with VSCode's build:

```bash
./Element/Rest/benchmark_vscode_compatibility.sh
```

This script:

- Compiles a sample of VSCode source files with Rest
- Compares output byte-for-byte with VSCode's gulp build
- Reports match percentage and any differences
- Measures performance metrics

---

## Troubleshooting 🔧

### Binary Not Found

```bash
# Build Rest
cargo build --release --package=Rest

# Set binary path
export REST_BINARY_PATH="Element/Rest/Target/release/Rest"
```

### Compilation Errors

Rest uses OXC which may have different error messages than `tsc`:

- Parse errors: Check syntax, especially decorators
- Transform errors: May indicate unsupported TypeScript features
- Enable `REST_VERBOSE=true` for detailed logs

### Segmentation Faults

Rest includes critical fixes for OXC lifetime management. If you encounter
segfaults:

1. Ensure using OXC 0.48+
2. Check that `parse_result` stays alive through transformation
3. Enable `RUST_LOG=debug` for detailed tracing

---

## Development 💻

### Building

```bash
# Debug build
cargo build --package=Rest

# Release build (optimized)
cargo build --release --package=Rest
```

### Running Tests

```bash
# Quick test suite
./Element/Rest/run_tests.sh

# Rust unit/integration tests
cargo test --package=Rest

# With verbose output
cargo test --package=Rest -- --nocapture

# Specific test
cargo test --package=Rest test_name
```

### Modifying Compiler

1. **Parser changes**: Edit
   [`Source/Fn/OXC/Parser.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Parser.rs)
2. **Transform changes**: Edit
   [`Source/Fn/OXC/Transformer.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Transformer.rs)
3. **Codegen changes**: Edit
   [`Source/Fn/OXC/Codegen.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Codegen.rs)
4. **CLI changes**: Edit
   [`Source/Fn/Binary/Command.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/Binary/Command.rs)
5. **Configuration**: Update
   [`Source/Struct/SWC.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Struct/SWC.rs)
   or
   [`Source/Struct/CompilerConfig.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Struct/CompilerConfig.rs)

---

## Changelog 📜

See [`CHANGELOG.md`](https://github.com/CodeEditorLand/Rest/tree/Current/) for a
history of changes to this component.

---

## License ⚖️

This project is released into the public domain under the **Creative Commons CC0
Universal** license. You are free to use, modify, distribute, and build upon
this work for any purpose, without any restrictions. For the full legal text,
see the [`LICENSE`](https://github.com/CodeEditorLand/Rest/tree/Current/) file.

---

## Related 📚

- [OXC Documentation](https://oxc.rs/)
- [VSCode Build Process](https://github.com/microsoft/vscode/tree/main/build)
- [CodeEditorLand Architecture](https://github.com/CodeEditorLand/Land/tree/Current/Documentation/Architecture/components/Rest.md)
- [Rest Compiler Detailed Docs](https://github.com/CodeEditorLand/Rest/tree/Current/COMPILER.md)
- [Verification Report](https://github.com/CodeEditorLand/Rest/tree/Current/VERIFICATION.md)

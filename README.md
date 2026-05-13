<table>
<tr>
<td align="left" valign="middle">
<h3 align="left">Rest</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">
⛱️
</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">+</h3>
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
<h3 align="left">🏞️</h3>
</td>
</tr>
</table>

---

# **Rest**&#x2001;⛱️

The High-Performance TypeScript Compiler for Land 🏞️

> **`VS Code`'s `TypeScript` build uses `tsc` with `Node.js` overhead on every
> incremental compile. Build times grow linearly with project size. Even
> alternatives like `esbuild` still run in a `Node.js` process.**

_"VS Code's TypeScript build step. 2-3x faster."_

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://github.com/CodeEditorLand/Rest/tree/Current/LICENSE)
[<img src="https://editor.land/Image/Rust.svg" width="14" alt="Rust" />](https://www.rust-lang.org/)&#x2001;[![Crates.io](https://img.shields.io/crates/v/Rest.svg)](https://crates.io/crates/Rest)
[<img src="https://editor.land/Image/Rust.svg" width="14" alt="Rust" />](https://www.rust-lang.org/)&#x2001;[![Rust Version](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![OXC Version](https://img.shields.io/badge/OXC-0.48-blue.svg)](https://oxc.rs/)

📖 **[Rust API Documentation](https://Rust.Documentation.Editor.Land/Rest/)**

Welcome to **Rest**, a high-performance `TypeScript` compiler built with `Rust` and
`OXC`, designed for 100% compatibility with `VSCode`'s build process. Rest is one of
the Elements in the `CodeEditorLand` architecture, responsible for **compilation
and build tooling**. It replaces `esbuild`'s `TypeScript` loader with a `Rust`-powered
compiler that produces `VSCode`-compatible output.

**Rest** is engineered to:

1. **Deliver High Performance**: Compile `TypeScript` 2-3x faster than `esbuild`
   using `OXC`.
2. **Ensure VSCode Compatibility**: Produce byte-for-byte identical output to
   VSCode's gulp build.
3. **Provide Memory Safety**: Leverage Rust's ownership model for deterministic
   performance without garbage collection.
4. **Support Modern Tooling**: Built on OXC 0.48, the latest TypeScript
   infrastructure.

### Why OXC over esbuild for TypeScript?

The `Rest` compiler was chosen over the existing esbuild pipeline for three
reasons that directly affect the VSCode build:

1. **OXC is used by VSCode internally.** This means Rest produces output that
   matches VSCode's own build pipeline - not an approximation.
2. **`emitDecoratorMetadata` support.** VSCode's codebase relies on decorator
   metadata emission. OXC handles this correctly; esbuild has limited support.
3. **`useDefineForClassFields = false`.** VSCode requires the legacy class
   fields behavior for ES5 compatibility. OXC's configurable codegen handles
   this exactly; esbuild's is implicit.

---

## Key Features&#x2001;🔐

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

## Core Architecture Principles&#x2001;🏗️

| Principle          | Description                                                              | Key Components Involved          |
| :----------------- | :----------------------------------------------------------------------- | :------------------------------- |
| **Performance**    | Rust + OXC delivers 2-3x faster compilation than esbuild.                | OXC Parser, Transformer, Codegen |
| **Compatibility**  | OXC is used by VSCode internally, ensuring 1:1 output compatibility.     | OXC 0.48, VSCode build process   |
| **Memory Safety**  | No garbage collection, deterministic performance through Rust ownership. | Rust lifetime management         |
| **Modern Tooling** | Built on the latest OXC infrastructure for TypeScript compilation.       | OXC 0.48+                        |

---

## Deep Dive & Component Breakdown&#x2001;🔬

To understand how `Rest`'s internal components interact to provide
high-performance TypeScript compilation, see the following source files:

- **[`Source/Library.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Library.rs)** -
  Binary entry point
- **[`Source/Fn/OXC/Compiler.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Compiler.rs)** -
  Main compiler orchestration
- **[`Source/Fn/OXC/Parser.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Parser.rs)** -
  OXC parser wrapper
- **[`Source/Fn/OXC/Transformer.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Transformer.rs)** -
  AST transformation (decorators, class fields, JSX)
- **[`Source/Fn/OXC/Codegen.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Fn/OXC/Codegen.rs)** -
  Code generation from transformed AST
- **[`Source/Struct/CompilerConfig.rs`](https://github.com/CodeEditorLand/Rest/tree/Current/Source/Struct/CompilerConfig.rs)** -
  Advanced configuration (decorators, class fields, target)

The source files explain the OXC-based compilation pipeline, decorator handling,
and VSCode compatibility transformations.

---

## `Rest` in the Land Ecosystem&#x2001;⛱️ + 🏞️

| Component         | Role & Key Responsibilities                                  |
| :---------------- | :----------------------------------------------------------- |
| **Rest Compiler** | High-performance TypeScript to JavaScript compilation.       |
| **RestPlugin**    | esbuild plugin that integrates Rest into the build pipeline. |
| **Build System**  | Environment-driven compiler selection (esbuild or Rest).     |

---

## Getting Started&#x2001;🚀

### Installation&#x2001;📥

```toml
[dependencies]
Rest = { git = "https://github.com/CodeEditorLand/Rest.git", branch = "Current" }
```

Or use via the `Output` element's `Compiler=Rest` environment variable.

### Usage&#x2001;🚀

The Rest compiler is invoked as a CLI binary:

```bash
# Compile a directory
rest --input ./Source --output ./Target

# With parallel compilation
rest --input ./Source --output ./Target --Parallel

# Check available options
rest --help
```

Via the Output element build pipeline:

```bash
# Use Rest compiler for TypeScript transpilation
export Compiler=Rest
npm run prepublishOnly

# Development mode with Rest
export NODE_ENV=development
export Compiler=Rest
npm run Run
```

---

## See Also

- [Rest Documentation](https://editor.land/Doc/rest)
- [Architecture Overview](https://editor.land/Doc/architecture)
- [Why Rust](https://editor.land/Doc/why-rust)
- [Output](https://github.com/CodeEditorLand/Output)
- [Cocoon](https://github.com/CodeEditorLand/Cocoon)

---

## License&#x2001;⚖️

This project is released into the public domain under the **Creative Commons CC0
Universal** license. You are free to use, modify, distribute, and build upon
this work for any purpose, without any restrictions. For the full legal text,
see the [`LICENSE`](https://github.com/CodeEditorLand/Rest/tree/Current/) file.

---

## Changelog&#x2001;📜

Stay updated with our progress! See
[`CHANGELOG.md`](https://github.com/CodeEditorLand/Rest/tree/Current/) for a
history of changes specific to **Rest**.

---

## Funding \& Acknowledgements&#x2001;🙏🏻

**Rest** is a core element of the **Land** ecosystem. This project is funded
through [NGI0 Commons Fund](https://NLnet.NL/commonsfund), a fund established by
[NLnet](https://NLnet.NL) with financial support from the European Commission's
[Next Generation Internet](https://ngi.eu) program. Learn more at the
[NLnet project page](https://NLnet.NL/project/Land).

The project is operated by PlayForm, based in Sofia, Bulgaria.

PlayForm acts as the open-source steward for Code Editor Land under the NGI0
Commons Fund grant.

<table>
<thead>
<tr>
<th align="left"><strong>Land</strong></th>
<th align="left"><strong>PlayForm</strong></th>
<th align="left"><strong>NLnet</strong></th>
<th align="left"><strong>NGI0 Commons Fund</strong></th>
</tr>
</thead>
<tbody>
<tr>
<td align="left" valign="middle">
<a href="https://Editor.Land">
<img width="60" src="https://raw.githubusercontent.com/CodeEditorLand/Asset/refs/heads/Current/Logo/Land.svg" alt="Land">
</a>
</td>
<td align="left" valign="middle">
<a href="https://PlayForm.Cloud">
<img width="76" src="https://raw.githubusercontent.com/PlayForm/Asset/refs/heads/Current/Logo/PlayForm.svg" alt="PlayForm">
</a>
</td>
<td align="left" valign="middle">
<a href="https://NLnet.NL">
<img width="240" src="https://NLnet.NL/logo/banner.svg" alt="NLnet">
</a>
</td>
<td align="left" valign="middle">
<a href="https://NLnet.NL/commonsfund">
<img width="240" src="https://NLnet.NL/image/logos/NGI0CommonsFund_tag_black_mono.svg" alt="NGI0 Commons Fund">
</a>
</td>
</tr>
</tbody>
</table>

---

**Project Maintainers**: Source Open
([Source/Open@Editor.Land](mailto:Source/Open@Editor.Land)) |
[GitHub Repository](https://github.com/CodeEditorLand/Rest) |
[Report an Issue](https://github.com/CodeEditorLand/Rest/issues) |
[Security Policy](https://github.com/CodeEditorLand/Rest/security/policy)

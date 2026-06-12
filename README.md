# **Rest** 🛠️

The High-Performance TypeScript Compiler for Land 🏞️

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://github.com/CodeEditorLand/Rest/blob/Current/LICENSE)
[<img src="https://editor.land/Image/Rust.svg" width="14" alt="Rust" />](https://www.rust-lang.org/) [![Crates.io](https://img.shields.io/crates/v/Rest.svg)](https://crates.io/crates/Rest)
[<img src="https://editor.land/Image/Rust.svg" width="14" alt="Rust" />](https://www.rust-lang.org/) [![Rust Version](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![OXC Version](https://img.shields.io/badge/OXC-0.48-blue.svg)](https://oxc.rs/)

**[Rust API Documentation](https://rust.documentation.editor.land/Rest/)**

---

## Overview

Rest is a high-performance TypeScript compiler built with Rust and OXC, designed for 100% compatibility with VSCode's build process. It replaces esbuild's TypeScript loader with a Rust-powered compiler that produces VSCode-compatible output. VS Code's TypeScript build uses `tsc` with Node.js overhead on every incremental compile, with build times growing linearly with project size. Even alternatives like esbuild still run in a Node.js process. Rest delivers 2-3x faster compilation.

**Rest is engineered to:**

1. **Deliver High Performance:** Compile TypeScript 2-3x faster than esbuild using OXC.
2. **Ensure VSCode Compatibility:** Produce byte-for-byte identical output to VSCode's gulp build.
3. **Provide Memory Safety:** Leverage Rust's ownership model for deterministic performance without garbage collection.
4. **Support Modern Tooling:** Built on OXC 0.48, the latest TypeScript infrastructure.

### Why OXC over esbuild for TypeScript?

1. **OXC is used by VSCode internally.** Rest produces output that matches VSCode's own build pipeline — not an approximation.
2. **`emitDecoratorMetadata` support.** VSCode's codebase relies on decorator metadata emission. OXC handles this correctly; esbuild has limited support.
3. **`useDefineForClassFields = false`.** VSCode requires the legacy class fields behavior for ES5 compatibility. OXC's configurable codegen handles this exactly; esbuild's is implicit.

## Architecture

```mermaid
graph LR
    classDef rest     fill:#ffe0cc,stroke:#e67e22,stroke-width:2px,color:#4a1500;
    classDef oxc      fill:#fff3c0,stroke:#f39c12,stroke-width:1px,color:#5a3e00;
    classDef output   fill:#d0d8ff,stroke:#4a6fa5,stroke-width:1px,color:#001050;
    classDef consumer fill:#f0d0ff,stroke:#9b59b6,stroke-width:1px,color:#2c0050;

    subgraph REST["Rest 🛠️ - Rust/OXC TypeScript Compiler"]
        direction TB
        subgraph PIPELINE["Fn/OXC/ - Compilation Pipeline"]
            Parser["Parser.rs\nOXC parser TypeScript 5.x"]:::oxc
            Transformer["Transformer.rs\nemitDecoratorMetadata · class fields · JSX"]:::oxc
            Codegen["Codegen.rs\nOXC code generation"]:::oxc
            Compiler["Compiler.rs\norchestrates pipeline"]:::rest
            Watch["Watch.rs\nnotify-based file watch"]:::rest
            Parser --> Transformer --> Codegen
            Compiler --> Parser
        end
        subgraph CONFIG["Struct/"]
            CompilerCfg["CompilerConfig.rs\nuseDefineForClassFields · target · decorators"]:::rest
        end
        subgraph MODES["Fn/Build · Bundle · NLS · Worker"]
            BuildMode["Build.rs - directory compilation"]:::rest
            BundleMode["Bundle/ - bundling mode"]:::rest
        end
        Compiler --> CompilerCfg
        Compiler --> BuildMode
    end

    subgraph OUTPUT["Output ⚫ - Build Pipeline"]
        ESBuild["ESBuild/Output.ts"]:::output
        RestPlugin["ESBuild/Rest/Plugin.ts\nintercepts .ts, delegates to Rest"]:::output
        ESBuild --> RestPlugin
    end

    subgraph CONSUMERS["Artifacts consumed by"]
        Sky["Sky 🌌"]:::consumer
        Cocoon["Cocoon 🦋"]:::consumer
    end

    RestPlugin -- spawns CLI --> Compiler
    Compiler -- emits .js --> ESBuild
    ESBuild --> Sky
    ESBuild --> Cocoon
```

## Key Components

| Component | Path | Description |
| --------- | ---- | ----------- |
| Library (Entry) | `Source/Library.rs` | Binary entry point |
| OXC Compiler | `Source/Fn/OXC/Compiler.rs` | Main compiler orchestration |
| OXC Parser | `Source/Fn/OXC/Parser.rs` | OXC parser wrapper for TypeScript 5.x |
| OXC Transformer | `Source/Fn/OXC/Transformer.rs` | AST transformation (decorators, class fields, JSX) |
| OXC Codegen | `Source/Fn/OXC/Codegen.rs` | Code generation from transformed AST |
| Compiler Config | `Source/Struct/CompilerConfig.rs` | Advanced configuration (decorators, class fields, target) |
| Build Mode | `Source/Fn/Build/Build.rs` | Directory compilation |
| Bundle Mode | `Source/Fn/Bundle/` | Bundling mode |
| Watch | `Source/Watch.rs` | notify-based file watch |

## In the Land Project

Rest operates as the TypeScript compilation engine within the Output build pipeline. The RestPlugin (esbuild plugin) intercepts `.ts` files and delegates to the Rest CLI binary, which spawns the OXC-based compiler. Output artifacts flow to Sky and Cocoon. When `Compiler=Rest` is set, the Output element uses Rest instead of esbuild for TypeScript transpilation.

**Architecture Principles:** Performance (Rust + OXC delivers 2-3x faster compilation than esbuild), Compatibility (OXC is used by VSCode internally, ensuring 1:1 output), Memory Safety (no garbage collection, deterministic performance through Rust ownership), Modern Tooling (built on OXC 0.48+).

## Getting Started

### Installation

```toml
[dependencies]
Rest = { git = "https://github.com/CodeEditorLand/Rest.git", branch = "Current" }
```

Or use via the `Output` element's `Compiler=Rest` environment variable.

### Usage

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

### Key Features

- **Full TypeScript 5.x Support:** Complete compatibility with TypeScript 5.x syntax and features.
- **Decorator Handling:** Proper support for `emitDecoratorMetadata` and decorator transformations.
- **Class Fields Control:** Configurable `useDefineForClassFields` behavior (VSCode default: false).
- **Parallel Compilation:** Optional `--Parallel` flag for multi-core compilation.
- **Directory-Based Compilation:** Process entire directory structures with preserved layout.
- **Comprehensive Error Reporting:** Detailed error messages with source location information.
- **Compilation Metrics:** Built-in tracking of compilation count, elapsed time, and error counts.
- **Source Map Generation:** Planned support for source maps (in progress).

## API Reference

- [Rust API Documentation](https://rust.documentation.editor.land/Rest/)

## Related Documentation

- [Architecture Overview](https://Editor.Land/Doc/architecture)
- [Why Rust](https://Editor.Land/Doc/why-rust)
- [Output](https://github.com/CodeEditorLand/Output) — Build artifact pipeline
- [Cocoon](https://github.com/CodeEditorLand/Cocoon) — Node.js extension host

---

## Funding

This project is funded through [NGI0 Commons Fund](https://NLnet.NL/commonsfund), a fund established by [NLnet](https://NLnet.NL) with financial support from the European Commission's Next Generation Internet program, under grant agreement No 101135429.

The project is operated by PlayForm, based in Sofia, Bulgaria. PlayForm acts as the open-source steward for Code Editor Land under the NGI0 Commons Fund grant.

| | |
| --- | --- |
| [![Land](https://raw.githubusercontent.com/CodeEditorLand/Asset/refs/heads/Current/Logo/Dual/Land.svg)](https://Editor.Land) | [![PlayForm](https://raw.githubusercontent.com/PlayForm/Asset/refs/heads/Current/Logo/PlayForm.svg)](https://PlayForm.Cloud) |
| [![NLnet](https://raw.githubusercontent.com/CodeEditorLand/Asset/refs/heads/Current/Logo/NLnet.svg)](https://NLnet.NL) | [![NGI0](https://raw.githubusercontent.com/CodeEditorLand/Asset/refs/heads/Current/Logo/NGI0.svg)](https://NLnet.NL/commonsfund) |

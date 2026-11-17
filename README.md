# **Rest**

<table>
	<tr>
		<td>
			<a href="https://GitHub.Com/CodeEditorLand/Rest" target="_blank">
				<picture>
					<source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/last-commit/CodeEditorLand/Rest?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0" />
					<source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/last-commit/CodeEditorLand/Rest?label=Update&color=white&labelColor=white&logoColor=black&logoWidth=0" />
					<img src="https://img.shields.io/github/last-commit/CodeEditorLand/Rest?label=Update&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Update" title="Update" />
				</picture>
			</a>
			<br />
			<a href="https://GitHub.Com/CodeEditorLand/Rest" target="_blank">
				<picture>
					<source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/issues/CodeEditorLand/Rest?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0" />
					<source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/issues/CodeEditorLand/Rest?label=Issue&color=white&labelColor=white&logoColor=black&logoWidth=0" />
					<img src="https://img.shields.io/github/issues/CodeEditorLand/Rest?label=Issue&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Issue" title="Issue" />
				</picture>
			</a>
		</td>
		<td>
			<a href="https://github.com/CodeEditorLand/Rest" target="_blank">
				<picture>
					<source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/stars/CodeEditorLand/Rest?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0" />
					<source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/stars/CodeEditorLand/Rest?style=flat&label=Star&logo=github&color=white&labelColor=white&logoColor=black&logoWidth=0" />
					<img src="https://img.shields.io/github/stars/CodeEditorLand/Rest?style=flat&label=Star&logo=github&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Star" title="Star" />
				</picture>
			</a>
			<br />
			<a href="https://GitHub.Com/CodeEditorLand/Rest" target="_blank">
				<picture>
					<source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/downloads/CodeEditorLand/Rest/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0" />
					<source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/downloads/CodeEditorLand/Rest/total?label=Download&color=white&labelColor=white&logoColor=black&logoWidth=0" />
					<img src="https://img.shields.io/github/downloads/CodeEditorLand/Rest/total?label=Download&color=black&labelColor=black&logoColor=white&logoWidth=0" alt="Download" title="Download" />
				</picture>
			</a>
		</td>
	</tr>
</table>

The JS Bundler Configuration for Land

> **Why a dedicated compilation pipeline matters:**
>
> - The Land editor needs to compile the `Dependency/Editor` VS Code source tree into self-contained `JavaScript` bundles.
> - `Cocoon` loads those bundles at runtime.
> - Without a dedicated compilation pipeline, every consumer would reinvent parsing, transformation, and codegen, leading to:
>   - Inconsistent output
>   - Duplicated work
>   - Poor iteration times

> **Design goal:**
>
> - One compilation pipeline
> - `OXC` at native speed
> - Zero `Node.js` processes in the hot path

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://github.com/CodeEditorLand/Rest/tree/Current/LICENSE)
[<img src="https://editor.land/Image/Rust.svg" width="14" alt="Rust" />](https://www.rust-lang.org/) [![Rust Version](https://img.shields.io/badge/Rust-1.95+-orange.svg)](https://www.rust-lang.org/)
[<img src="https://editor.land/Image/Rust.svg" width="14" alt="Rust" />](https://www.rust-lang.org/) Edition 2024

**[`Rust` API Documentation](https://rust.documentation.rest.editor.land/)**

---

## Overview 📋

- `Rest` is the JS bundler configuration for the Land Code Editor.
- It contains the build scripts and `Rust` compilation pipeline used to bundle the VS Code platform code from the `Dependency/Editor` submodule (workbench, services, platform layers).
- The result is self-contained `JavaScript` modules that `Cocoon` can `require()` at runtime.
- `Rest` is invoked as a CLI binary or build step, not a network service, during the Land build.

```mermaid
graph LR
    SRC["Dependency/Editor<br/>VS Code source"] --> REST["Rest<br/>CLI + library"]
    REST --> OXC["OXC pipeline"]
    OXC --> OUT["Output<br/>bundles"]
    OUT --> COCOON["Cocoon<br/>require at runtime"]
    MAINTAIN["Maintain"] --> REST
```

- `Rest`'s compilation backend is `OXC` (the Oxidation Compiler): a `Rust`-native `TypeScript` 5.x parser, AST transformer, and code generator.
  - It runs at native speed with no `Node.js` process in the hot path.
- The `SWC/` handler module is a historical name retained for its call sites.
  - Internally it now runs on the same `OXC`-backed compiler (`Source/Fn/SWC/Compile.rs`), so both module names converge on one implementation.
- An optional `ESBuild` wrapper (`Source/Fn/Bundle/ESBuild.rs`) shells out to the `esbuild` CLI, if present on the system.
  - It covers bundling scenarios `OXC` alone does not cover: AMD output and complex multi-file bundling.
- The build or CLI surface is organized into modules under `Fn/`:
  - One for builds
  - One for bundling
  - One for the native language service
  - One for each compiler pipeline
- Every configuration and CLI-argument type lives in `Struct/`, giving the entire pipeline compile-time type checking through `Rust`'s type system.

`Rest` is engineered to:

1. **Provide a Unified Compilation Pipeline** - Expose the `OXC`-backed compilation pipeline (still named `OXC` and `SWC` at the module level) as a `Rust` library and CLI with structured configuration and result types.
2. **Enable Fast `TypeScript` Transformation** - Leverage `OXC`'s parser, transformer, and codegen for `TypeScript` 5.x with decorator and JSX support at native speed.
3. **Support Build Workflows** - Handle directory compilation, optional `ESBuild` bundling, NLS (Native Language Service) extraction and replacement, and compilation worker management.
4. **Integrate with Land Architecture** - Serve as the build-time compilation step that produces the bundles `Output` ships and `Cocoon` loads, orchestrated by `Maintain`.

---

## Key Features 🔐

```mermaid
graph TB
    REST["Rest features"] --> OXC["OXC-backed compilation"]
    REST --> TYPES["Type-safe configuration"]
    REST --> MOD["Modular Fn/ architecture"]
    REST --> ESB["ESBuild bundling"]
    REST --> NLS["Native language service"]
    REST --> WORKER["Compilation workers"]
    REST --> WATCH["Watch mode"]
```

| Feature | Description | Source |
| -------- | -------- | -------- |
| **`OXC`-Backed Compilation** | `OXC` (`Source/Fn/OXC/`) is the compiler that does the parsing, transformation, and code generation. | `Source/Fn/OXC/`, `Source/Fn/SWC/` |
| **`SWC` Compatibility** | `SWC` (`Source/Fn/SWC/`) is a module retained for naming and call-site compatibility; its `Compile.rs` runs the same `OXC`-backed pipeline underneath rather than the `swc_core` crate. | `Source/Fn/SWC/Compile.rs` |
| **Type-Safe Configuration Layer** | Configuration and CLI-argument types defined in `Struct/` provide compile-time validation of the entire pipeline surface. Every entry point is schema-checked before it reaches production through `Rust`'s type system. | `Struct/` |
| **Modular Function Architecture** | Build logic is decomposed into `Fn/` modules (`Build`, `Bundle`, `NLS`, `Worker`, `OXC`, `SWC`, `Transform`, `Binary`), each responsible for a single compilation domain. Modules are composable, testable, and independently maintainable. | `Fn/` |
| **`ESBuild` Bundling** | Integration with `ESBuild` for fast production bundling with configurable build profiles. | `Source/Fn/Bundle/ESBuild.rs`, `Source/Fn/Bundle/Config.rs` |
| **Native Language Service** | NLS endpoints provide extraction, replacement, and bundling of native language strings for internationalization workflows. | `Source/Fn/NLS/` |
| **Compilation Workers** | Dedicated worker lifecycle management with bootstrap, compilation, and capability detection for parallel compilation across multiple cores. | `Source/Fn/Worker/` |
| **Watch Mode** | Both the `OXC` and `SWC` module namespaces support file-system watch mode for incremental recompilation on file changes. | `Source/Fn/OXC/Watch.rs`, `Source/Fn/SWC/Watch/` |

---

## Core Architecture Principles 🏗️

```mermaid
graph TB
    P["Core principles"] --> M["Function modularity"]
    P --> T["Type safety"]
    P --> O["OXC-backed pipeline"]
    P --> F["Performance first"]
```

| Principle | Description | Key Components |
| -------- | -------- | -------- |
| **Function Modularity** | Build logic is decomposed into focused `Fn/` modules, each responsible for a single compilation domain. Modules are composable and independently testable. | `Fn/Build`, `Fn/Bundle`, `Fn/NLS`, `Fn/Worker`, `Fn/OXC`, `Fn/SWC`, `Fn/Transform`, `Fn/Binary` |
| **Type Safety** | Configuration and CLI-argument schemas live in `Struct/` for compile-time validation. Every entry point is schema-checked through `Rust`'s type system. | `Struct/CompilerConfig`, `Struct/SWC`, `Struct/Binary` |
| **`OXC`-Backed Pipeline** | A single `OXC`-based compiler backs both the `OXC` and `SWC` module namespaces; `SWC` is retained for call-site compatibility, not a second compilation engine. | `Fn/OXC/Compiler`, `Fn/SWC/Compile` |
| **Performance First** | Native `Rust` compilation with zero-cost abstractions and parallel compilation via `rayon` for multi-core throughput - no `Node.js` process in the hot path. | `Source/Library.rs`, `Source/Main.rs`, `Fn/Binary/Command/Parallel` |

---

## System Architecture 🗺️

```mermaid
graph LR
    classDef rest fill:#ffe0cc,stroke:#e67e22,stroke-width:2px,color:#4a1500;
    classDef handler fill:#d0d8ff,stroke:#4a6fa5,stroke-width:2px,color:#001050;
    classDef struct fill:#d4f5d4,stroke:#27ae60,stroke-width:2px,color:#0a3a0a;
    classDef external fill:#f0d0ff,stroke:#9b59b6,stroke-width:1px,stroke-dasharray:5 5,color:#2c0050;

    subgraph REST["Rest - JS bundler / compilation pipeline (CLI + library)"]
        direction TB
        HANDLERS["Fn/ - Build/Bundle/NLS/Worker/OXC/SWC/Transform/Binary"]:::handler
        STRUCTS["Struct/ - CompilerConfig/SWC/Binary"]:::struct
        Entry["Library.rs - library entry"]:::rest
        BinaryEntry["Main.rs - binary entry"]:::rest
        Entry --> HANDLERS
        Entry --> STRUCTS
        BinaryEntry --> HANDLERS
    end

    subgraph ECO["Land ecosystem"]
        Editor["Dependency/Editor - VS Code source"]:::external
        OutputN["Output - bundled JS artifacts"]:::external
        MaintainN["Maintain - build orchestration"]:::external
        Cocoon["Cocoon - extension host"]:::external
    end

    Editor --> REST
    REST --> OutputN
    MaintainN --> REST
    OutputN --> Cocoon
```

Connection paths:

| Path | Protocol | Use Case |
| -------- | -------- | -------- |
| `Maintain` 💪🏻 -> `Rest` | CLI invocation | Triggers compilation as part of the Land build |
| `Dependency/Editor` -> `Rest` | File-system read | Source input - VS Code `TypeScript`/`JavaScript` tree |
| `Rest` -> `Output` ⚫ | File-system write | Writes bundled `JavaScript` artifacts |
| `Output` -> `Cocoon` 🦋 | `Node.js` `require()` | `Cocoon` loads the bundles `Rest` produced at runtime |
| `Rest` -> `OXC` Compiler | Direct call | `TypeScript` 5.x parsing, transformation, codegen |
| `Rest` -> `ESBuild` | Subprocess (optional) | Fallback bundling for AMD output / complex bundles |

---

## Key Components 🧩

```mermaid
graph TB
    ROOT["Source/"] --> LIB["Library.rs"]
    ROOT --> MAIN["Main.rs"]
    ROOT --> BINARY["Binary.rs"]
    ROOT --> FN["Fn/ modules"]
    ROOT --> STRUCT["Struct/ types"]
    FN --> OXC["OXC/"]
    FN --> SWC["SWC/"]
    FN --> BUNDLE["Bundle/"]
    FN --> NLS["NLS/"]
    FN --> WORKER["Worker/"]
    FN --> TRANSFORM["Transform/"]
    FN --> BIN["Binary/"]
```

| Component | Path | Description |
| -------- | -------- | -------- |
| Library (Entry) | `Source/Library.rs` | Library root - `rlib`, `cdylib`, and `staticlib` targets |
| Binary Entry | `Source/Main.rs` | CLI binary entry point |
| OXC Compiler | `Source/Fn/OXC/Compiler.rs` | Main `OXC`-based compiler orchestration |
| OXC Parser | `Source/Fn/OXC/Parser.rs` | `OXC` parser wrapper for `TypeScript` 5.x |
| OXC Transformer | `Source/Fn/OXC/Transformer.rs` | AST transformation (decorators, class fields, JSX) |
| OXC Codegen | `Source/Fn/OXC/Codegen.rs` | Code generation from transformed AST |
| OXC Compile | `Source/Fn/OXC/Compile.rs` | Full `OXC` compilation pipeline |
| OXC Watch | `Source/Fn/OXC/Watch.rs` | Watch mode for `OXC` compilation |
| SWC Compiler | `Source/Fn/SWC/Compile.rs` | `SWC`-based compilation pipeline |
| SWC Watch | `Source/Fn/SWC/Watch.rs` | Watch mode for `SWC` compilation |
| Build Mode | `Source/Fn/Build.rs` | Directory compilation handler |
| Bundle Builder | `Source/Fn/Bundle/Builder.rs` | Bundle builder orchestration |
| Bundle Config | `Source/Fn/Bundle/Config.rs` | Bundle configuration profiles |
| ESBuild | `Source/Fn/Bundle/ESBuild.rs` | `ESBuild` integration for fast bundling |
| NLS Extract | `Source/Fn/NLS/Extract.rs` | NLS string extraction |
| NLS Replace | `Source/Fn/NLS/Replace.rs` | NLS string replacement |
| NLS Bundle | `Source/Fn/NLS/Bundle.rs` | NLS bundling |
| Worker Bootstrap | `Source/Fn/Worker/Bootstrap.rs` | Worker initialization |
| Worker Compile | `Source/Fn/Worker/Compile.rs` | Worker compilation |
| Worker Detect | `Source/Fn/Worker/Detect.rs` | Worker capability detection |
| Transform PrivateField | `Source/Fn/Transform/PrivateField.rs` | Private field AST transforms |
| Binary Commands | `Source/Fn/Binary/Command/` | CLI command handlers (Sequential, Parallel, Entry) |
| Compiler Config | `Source/Struct/CompilerConfig.rs` | Compiler configuration types |
| SWC Types | `Source/Struct/SWC.rs` | `SWC`-related type definitions |
| Binary Command Types | `Source/Struct/Binary/Command/` | CLI argument and option types |

---

## Project Structure 🗺️

```mermaid
graph TB
    ROOT["Element/Rest/"] --> SRC["Source/"]
    ROOT --> DOC["Documentation/Rust/doc/"]
    SRC --> LIB["Library.rs / Main.rs / Binary.rs"]
    SRC --> FN["Fn/"]
    SRC --> STRUCT["Struct/"]
    FN --> SUB["Build / Bundle / NLS / Worker / OXC / SWC / Transform / Binary"]
```

```text
Element/Rest/
|-- Source/
|   |-- Library.rs              # Library root (rlib + cdylib + staticlib)
|   |-- Main.rs                 # Binary entry point (CLI)
|   |-- Binary.rs               # Binary initialization
|   |-- Fn/                     # API handler modules
|   |   |-- mod.rs              # Module re-exports
|   |   |-- Build.rs            # Directory compilation endpoint
|   |   |-- Bundle/             # Bundling API
|   |   |   |-- mod.rs
|   |   |   |-- Builder.rs      # Bundle builder
|   |   |   |-- Config.rs       # Bundle configuration
|   |   |   |-- ESBuild.rs      # ESBuild integration
|   |   |-- NLS/                # Native Language Service endpoints
|   |   |   |-- mod.rs
|   |   |   |-- Bundle.rs       # NLS bundling
|   |   |   |-- Extract.rs      # NLS extraction
|   |   |   |-- Replace.rs      # NLS replacement
|   |   |-- Worker/             # Compilation worker API
|   |   |   |-- mod.rs
|   |   |   |-- Bootstrap.rs    # Worker initialization
|   |   |   |-- Compile.rs      # Worker compilation
|   |   |   |-- Detect.rs       # Worker capability detection
|   |   |-- OXC/                # OXC compiler endpoints
|   |   |   |-- mod.rs
|   |   |   |-- Codegen.rs      # Code generation
|   |   |   |-- Compile.rs      # Compilation pipeline
|   |   |   |-- Compiler.rs     # Compiler orchestration
|   |   |   |-- Parser.rs       # OXC parser wrapper
|   |   |   |-- Transformer.rs  # AST transformation
|   |   |   |-- Watch.rs        # Watch mode
|   |   |-- SWC/                # SWC compiler endpoints
|   |   |   |-- mod.rs
|   |   |   |-- Compile.rs      # SWC compilation
|   |   |   |-- Watch/          # SWC watch mode
|   |   |   |   |-- Compile.rs
|   |   |-- Transform/          # AST transformation endpoints
|   |   |   |-- mod.rs
|   |   |   |-- PrivateField.rs # Private field transforms
|   |   |-- Binary/             # Binary command handlers
|   |   |   |-- mod.rs
|   |   |   |-- Command.rs      # Command dispatcher
|   |   |   |-- Command/        # Command implementations
|   |   |   |   |-- Entry.rs    # Entry command
|   |   |   |   |-- Parallel.rs # Parallel execution
|   |   |   |   |-- Sequential.rs # Sequential execution
|   |-- Struct/                 # Data type definitions
|   |   |-- mod.rs
|   |   |-- CompilerConfig.rs   # Compiler configuration schema
|   |   |-- SWC.rs              # SWC type definitions
|   |   |-- Binary/             # Binary command types
|   |   |   |-- mod.rs
|   |   |   |-- Command.rs      # Command argument types
|   |   |   |-- Command/        # Command option types
|   |   |   |   |-- Entry.rs
|   |   |   |   |-- Option.rs
|-- Documentation/
|   |-- Rust/
|   |   |-- doc/                # Cargo doc output
```

---

## In the Land Project 🔌

```mermaid
graph LR
    MAINTAIN["Maintain"] --> REST["Rest"]
    EDITOR["Dependency/Editor"] --> REST
    REST --> OUTPUT["Output"]
    OUTPUT --> COCOON["Cocoon"]
    COMMON["Common"] --> REST
```

- `Rest` serves as the build-time JS bundler for the Land ecosystem.
  - It compiles the `Dependency/Editor` VS Code source tree into the `JavaScript` bundles that `Cocoon` loads at runtime.
  - It runs as a CLI or library invoked during the Land build (orchestrated by `Maintain`).
  - It executes through `Fn/` build modules backed by the `OXC` compiler pipeline.
  - It configures itself through `Struct/` type-safe data definitions.
- `Rest` is the compilation step for the broader Land toolchain.

| Consumer | Role | Integration |
| -------- | -------- | -------- |
| **`Maintain`** 💪🏻 | Build system | Invokes the `Rest` CLI/binary as part of the Land build orchestration |
| **`Dependency/Editor`** 💻 | VS Code source | Input tree `Rest` reads and compiles |
| **`Output`** ⚫ | Bundled JS output | Destination directory for the bundles `Rest` produces |
| **`Cocoon`** 🦋 | Extension host | Loads (`require()`s) the bundles `Rest` produced, via `Output` |

`Rest` depends on `Common` 🧑🏻‍🏭 for shared type definitions and utility functions used across the Land `Rust` infrastructure.

---

## Getting Started 🚀

```mermaid
graph LR
    PREREQ["Rust 1.95.0+, edition 2024"] --> BUILD["cargo build --release"]
    BUILD --> LIB["library usage: Rest git dependency"]
    BUILD --> CLI["CLI binary"]
```

### Prerequisites 📦

- **`Rust`** 1.95.0 or later (edition 2024)

### Build 📦

```bash
cd Element/Rest
cargo build --release
```

### Key Dependencies 🧩

| Dependency | Purpose |
| -------- | -------- |
| `oxc_parser` | `TypeScript`/`JavaScript` parsing |
| `oxc_transformer` | AST transformation (decorators, JSX, class fields) |
| `oxc_codegen` | Code generation from transformed AST |
| `oxc_minifier` | Code minification |
| `oxc_semantic` | Semantic analysis and type checking |
| `oxc_ast` | AST type definitions |
| `rayon` | Parallel compilation across CPU cores |
| `tokio` | Async runtime for the CLI/build pipeline |
| `clap` | CLI argument parsing |
| `notify` | File-system watch notifications |
| `walkdir` | Recursive directory traversal for source discovery |
| `globset` | Glob-pattern matching for include/exclude filters |
| `Common` | Shared Land type definitions and utilities |

### As a Library 🔌

```toml
[dependencies]
Rest = { git = "https://github.com/CodeEditorLand/Rest.git", branch = "Current" }
```

---

## Security 🛡️

```mermaid
graph TB
    SEC["Security layers"] --> T["Type safety via Struct/"]
    SEC --> M["Memory safety via Rust ownership semantics"]
    SEC --> I["Input validation via serde and clap"]
    SEC --> C["Compiler sandboxing via in-memory ASTs"]
```

`Rest` enforces security at multiple layers:

| Layer | Mechanism |
| -------- | -------- |
| **Type safety** | `Rust`'s compile-time type system validates all configuration and CLI-argument schemas through `Struct/` data types - malformed input is rejected before reaching build logic |
| **Memory safety** | `Rust`'s ownership semantics eliminate buffer overflows, use-after-free, and data races without a garbage collector |
| **Input validation** | Structured deserialization via `serde` and `clap` ensures all configuration inputs and CLI arguments are validated at the boundary |
| **Compiler sandboxing** | The `OXC`-backed compiler pipeline operates on in-memory ASTs - no file-system access beyond the explicitly configured input/output directories |

---

## Compatibility 🔌

```mermaid
graph LR
    REST["Rest"] --> MAINTAIN["Maintain"]
    REST --> EDITOR["Dependency/Editor"]
    REST --> OUTPUT["Output"]
    OUTPUT --> COCOON["Cocoon"]
    REST --> COMMON["Common"]
```

`Rest` is designed to be compatible with:

| Target | Integration |
| -------- | -------- |
| **`Maintain`** 💪🏻 | Invoked as a CLI/build step for compilation and build operations |
| **`Dependency/Editor`** 💻 | Reads VS Code `TypeScript`/`JavaScript` source as build input |
| **`Output`** ⚫ | Writes bundled `JavaScript` artifacts consumed at runtime |
| **`Cocoon`** 🦋 | Indirect - loads the bundles `Rest` produced, via `Output` |
| **`Common`** 🧑🏻‍🏭 | Shared trait implementations and type definitions |

---

## API Reference 📖

```mermaid
graph LR
    DOC["API documentation"] --> RUSTDOC["rust.documentation.rest.editor.land"]
```

- **[`Rust` API Documentation](https://rust.documentation.rest.editor.land/)**

---

## License 📖

```mermaid
graph LR
    LICENSE["Creative Commons CC0 Universal"] --> FILE["LICENSE file"]
```

This project is released into the public domain under the **Creative Commons CC0 Universal** license. You are free to:

- Use this work for any purpose
- Modify this work
- Distribute this work
- Build upon this work

For the full legal text, see the [`LICENSE`](https://github.com/CodeEditorLand/Rest/tree/Current/LICENSE) file.

[Security Policy](https://github.com/CodeEditorLand/Rest/security/policy)

---

## Changelog 📖

```mermaid
graph LR
    CHANGELOG["CHANGELOG.md"] --> HISTORY["History of changes"]
```

Stay updated with our progress! See [`CHANGELOG.md`](https://github.com/CodeEditorLand/Rest/tree/Current/CHANGELOG.md) for a history of changes.

---

## Funding & Acknowledgements 📖

```mermaid
graph LR
    LAND["Land"] --> FUND["NGI0 Commons Fund"]
    FUND --> NLNET["NLnet"]
    NLNET --> EC["European Commission Next Generation Internet program"]
    FUND --> PARTNERS["Land / PlayForm / NLnet / NGI0 Commons Fund"]
```

- **Land** is proud to be an open-source endeavor.
- Our journey is significantly supported by the organizations and projects that believe in the future of open-source software.
- This project is funded through [NGI0 Commons Fund](https://NLnet.NL/commonsfund), a fund established by [NLnet](https://NLnet.NL) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program.
- Learn more at the [NLnet project page](https://NLnet.NL/project/Land).

<table>
	<thead>
		<tr>
			<th align="left">
				<strong>
					Land
				</strong>
			</th>
			<th align="left">
				<strong>
					PlayForm
				</strong>
			</th>
			<th align="left">
				<strong>
					NLnet
				</strong>
			</th>
			<th align="left">
				<strong>
					NGI0 Commons Fund
				</strong>
			</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="left" valign="middle">
				<a href="https://editor.land">
					<img width="60" src="https://raw.githubusercontent.com/CodeEditorLand/Asset/refs/heads/Current/Logo/Land.svg" alt="Land" />
				</a>
			</td>
			<td align="left" valign="middle">
				<a href="https://PlayForm.Cloud">
					<img width="76" src="https://raw.githubusercontent.com/PlayForm/Asset/refs/heads/Current/Logo/PlayForm.svg" alt="PlayForm" />
				</a>
			</td>
			<td align="left" valign="middle">
				<a href="https://NLnet.NL">
					<img width="240" src="https://NLnet.NL/logo/banner.svg" alt="NLnet" />
				</a>
			</td>
			<td align="left" valign="middle">
				<a href="https://NLnet.NL/commonsfund">
					<img width="240" src="https://NLnet.NL/image/logos/NGI0CommonsFund_tag_black_mono.svg" alt="NGI0 Commons Fund" />
				</a>
			</td>
		</tr>
	</tbody>
</table>

---

## Related Documentation 📖

- [Architecture Overview](https://Editor.Land/Doc/architecture) - Land system architecture
- [Why `Rust`](https://Editor.Land/Doc/why-rust) - Why `Rust` for Land infrastructure
- [`Maintain`](https://github.com/CodeEditorLand/Maintain) - Build system and development runner
- [`Cocoon`](https://github.com/CodeEditorLand/Cocoon) - `Node.js`/`Effect-TS` extension host
- [`Output`](https://github.com/CodeEditorLand/Output) - Bundled JS output consumed by `Cocoon`
- [`Common`](https://github.com/CodeEditorLand/Common) - Shared abstract foundation
- [Land Documentation Index](https://Editor.Land/Doc) - Full documentation index

---

**Project Maintainers:** Source Open
([Source/Open@Editor.Land](mailto:Source/Open@Editor.Land)) |
[GitHub Repository](https://github.com/CodeEditorLand/Rest) |
[Report an Issue](https://github.com/CodeEditorLand/Rest/issues)

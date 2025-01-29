# [Rest] ⛱️

[Rest]: HTTPS://crates.io/crates/BinaryRest

## Installation 🚀

```sh
cargo install BinaryRest
```

## 🛠️ Usage

```
Rest ⛱️

Usage: Rest [OPTIONS]

Options:
  -P, --Parallel           Parallel ⏩
  -R, --Root <ROOT>        Root 📂 [default: .]
  -E, --Exclude <EXCLUDE>  Exclude 🚫 [default: node_modules]
      --Pattern <PATTERN>  Pattern 🔍 [default: .]
  -h, --help               Print help
  -V, --version            Print version
```

## Options

The [Rest] tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories.

Default is:

```sh
Rest -P -E node_modules
```

#### --Parallel or -P:

Run processing in parallel.

Default is:

```sh
Rest
```

#### --Pattern:

Specify a custom pattern for matching.

Default is:

```sh
Rest -P --Pattern .
```

#### --Root or -R:

Set the current working directory to a different folder.

Default is:

```sh
Rest -P --Root .
```

## Examples

## Dependencies

[Rest] relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Rest]: HTTPS://crates.io/crates/psummary

## Changelog

See [`CHANGELOG.md`](CHANGELOG.md) for a history of changes to this CLI.

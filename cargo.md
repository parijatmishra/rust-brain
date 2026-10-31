# Cargo

Cargo is Rust's build system and package manager: it compiles crates, fetches
dependencies, runs tests, and orchestrates workspaces.

## Rust Crates

A *crate* is a unit of compilation. Every crate has a "root": a file from where
the compiler starts compilation; the compiler also compiles other files that are
*modules* of the crate. The final result is an *artifact*: a library or an
executable. One crate root turns into one artifact.

While we can compile crates manually using just `rustc`, it is very tedious for
anything more than single-file programs with no dependencies. Real Rust projects
using Cargo to build crates.

## Cargo Packages

A Cargo *package* is a directory with one or more source files and a *manifest*
(a file named `Cargo.toml`) that tells Cargo what to build.

### Targets

Cargo knows about five *kinds* of targets. In a package, there can be only one
target of kind "library". Other kinds can have more than one target.

| Kind             | Default crate root      | Crate name   |
| ---------------- | ----------------------- | ------------ |
| Library          | `src/lib.rs`            | package name |
| Binary           | `src/main.rs`           | package name |
|                  | `src/bin/NAME.rs`       | NAME         |
|                  | `src/bin/NAME/main.rs`  | NAME         |
| Integration Test | `tests/NAME.rs`         | NAME         |
|                  | `tests/NAME/main.rs`    | NAME         |
| Example          | `examples/NAME.rs`      | NAME         |
|                  | `examples/NAME/main.rs` | NAME         |
| Benchmark        | `benches/NAME.rs`       | NAME         |
|                  | `benches/NAME/main.rs`  | NAME         |

One can override names, locations and other options for a crate with explicit
target tables in the manifest, such as `[[bin]]`, `[[example]]`, and
`[[bench]]`.

Cargo can be taught about new target kinds, such as `custom-build` and
`proc-macro`. They appear as extra `[lib]` table entries. See [cargo
targets][cargo-targets].

### Dependencies

Declare dependencies in `[dependencies]` (available everywhere) and
`[dev-dependencies]` (only tests, examples, and benchmarks). Depencencies state
versions requirements via semver ranges (`"1"` accepts `1.x.y` but not `2.0`). A
*path* dependency names another package by directory (typically a workspace
member) instead of a name and version in some registry.

> **N.B.** If a dependency has hyphens in its name, when importing it in Rust
> code using `use ...`, the hyphens are converted to underscores.

E.g., in `Cargo.toml`:

```toml
[dependencies]
unicode-segmentation = "1"
```

In code:

```rust,ignore
use unicode_segmentation::UnicodeSegmentation;
```

### Common Commands

Run from inside a package directory, or anywhere in the workspace with
`--package NAME`:

| Command                      | Description                                                |
| ---------------------------- | ---------------------------------------------------------- |
| `cargo new DIR [--lib]`      | scaffold a binary (library) package in directory `DIR`     |
| `cargo build`                | compile library and binary targets using `debug` profile   |
| `cargo build [--release]`    | compile library and binary targets using `release` profile |
| `cargo test`                 | run unit, integration, doc tests                           |
| `cargo test --test NAME`     | run named integration test                                 |
| `cargo run [--bin NAME]`     | build and run main binary / named binary                   |
| `cargo run --example NAME`   | build and run named example                                |
| `cargo bench [--bench NAME]` | run benchmarks                                             |
| `cargo add NAME`             | add a dependency, then `cargo remove` / `cargo update`     |
| `cargo tree`                 | print the dependency graph                                 |
| `cargo doc [--open]`         | generate API documentation from doc comments               |

The kind of a target decides which command invokes it: `run` runs binaries
and examples, `test` runs unit and integration tests, `bench` runs
benchmarks. Nothing runs a library; other targets consume it.

## Workspaces

A *workspace* is a set of packages sharing one build directory, lockfile,
and dependency-resolution pass. The root manifest lists members and can
define shared metadata that members inherit with `NAME.workspace = true`.

File *Cargo.toml*.

```toml
{{#include Cargo.toml}}
```

File *cargo02/Cargo.toml* (excerpt):

```toml
{{#include ./cargo02/Cargo.toml:package}}
```

> **N.B.** A workspace groups packages for building and sharing settings; it is
not a module hierarchy. Crates still import each other only through declared
dependencies.

## Build Scripts

A *build script* is a `build.rs` file (or `build = "..."` manifest entry) that
Cargo compiles and runs before the package, typically to generate code or find
native libraries. See [build scripts][build-scripts].

## Further Reading

For publishing, cross-compilation, and detailed feature resolution are out of
scope, see [Cargo Reference][cargo-ref].

[build-scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[cargo-targets]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
[cargo-ref]: https://doc.rust-lang.org/cargo/reference/

## Examples

### `cargo01`

A very simple Cargo package, with just a library crate.

```text
cargo01
├── Cargo.toml
└── src
    └── lib.rs
```

File *./cargo01/Cargo.toml*:

```toml
{{#include ./cargo01/Cargo.toml}}
```

File *./cargo01/src/lib.rs*:

```rust
{{#include ./cargo01/src/lib.rs}}
```

The only useful command that can be run in this package is `cargo build`.

### `cargo02`

This package demonstrates:

- How to inherit settings from parent workspace
- How to declare dependencies to external crates, and path dependencies to other
  packages in the filesystem.
- How library, binary, integration test, exaple and benchmark crates are laid
  out in the package.
- How to override configuration for a crate.

#### Crates

```text
cargo02
├── benches
│   └── my_add.rs
├── Cargo.toml
├── examples
│   └── my_add.rs
├── src
│   ├── bin
│   │   ├── count_words
│   │   │   └── main.rs
│   │   └── greet.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    └── my_add.rs
```

Just following Cargo conventions, this package has the following crates:

- A **library crate** named "cargo02", rooted at `src/lib.rs`.
- A **binary crate** also named "cargo02", rooted at `src/main.rs`.
- A **binary crate** named "greet", rooted at `src/bin/greet.rs`.
- A **binary crate** named "count_words", rooted at
  `src/bin/count_words/main.rs`.
- An **integration test crate** named "my_add", rooted at `tests/my_add.rs`.
- A **benchmark binary crate** named "my_add", rooted at `benches/my_add.rs`.
- An **example binary crate** named "my_add", rooted at `examples/my_add.rs`.

#### Commands

- `cargo build`: compile `cargo02` library crate, and `cargo02`, `greet` and
  `count_words` binary crates.
- `cargo test`: compile and run unit, doc and integration tests.
- `cargo test --test my_add`: compile and run the `my_add` integration test.
- `cargo test --lib`: compile and run unit tests in the library crate.
- `cargo test --doc`: compile and run the library's doc tests.
- `cargo run --bin cargo02`: compile and run the `cargo02` (main) binary crate.
- `cargo run --bin greet`: compile and run the `greet` binary crate.
- `cargo run --bin count_words`: compile and run the `count_words` binary crate.
- `cargo run --example my_add`: compile and run the `my_add` example crate.
- `cargo bench`: compile and run all benchmark crates.
- `cargo bench --bench my_add`: compile and run only the `my_add` benchmark
  crate.

#### Customizing Crates

Dependending on the crate, various default setting values can be over-ridden in
the package manifest.

File *./cargo02/Cargo.toml* (excerpt):

```toml
{{#include ./cargo02/Cargo.toml:bench}}
```

#### Using The Library Crate

File *./cargo02/src/lib.rs*:

```rust
{{#include ./cargo02/src/lib.rs}}
```

File *./cargo02/src/main.rs*:

```rust
# extern crate cargo02;
{{#include ./cargo02/src/main.rs}}
```

#### Using Dependencies

File *./cargo02/Cargo.toml* (excerpt):

```toml
{{#include ./cargo02/Cargo.toml:deps}}
```

File *./cargo02/src/bin/greet.rs*:

```rust
# extern crate cargo01;
{{#include ./cargo02/src/bin/greet.rs}}
```

File *./cargo02/src/bin/count_words/main.rs*:

```rust,ignore
{{#include ./cargo02/src/bin/count_words/main.rs}}
```

> **N.B.** the dependency crate name is `unicode-segmentation` in `Cargo.toml`,
> but in Rust code, it is referred to as `unicode_segmentation` (underscores
> replacing hyphens).

# Cargo

Cargo is Rust's build system and package manager: it compiles crates, fetches
dependencies, runs tests, and orchestrates workspaces.

## Rust Crates

A *crate* is a unit of compilation. A crate has a "root" file, from where the
compiler starts compilation; the compiler compiles other files that are
*modules* of the crate. The final result is an *artifact*: a library or an
executable. One crate root turns into one artifact.

While we can compile crates manually using just `rustc`, it is very tedious for
anything other than single-file programs with no dependencies. Real Rust
projects using Cargo to manage dependencies and builds.

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
member) instead of a name and version in some registry. E.g.:

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

## Example: minimal

The directory `cargo01-greet` in the source repo contain a minimal Cargo package
with only a library crate. Cargo auto-detects the library crate and target and
builds it via `cargo build`.

```text
cargo01-greet
├── Cargo.toml
└── src
    └── lib.rs
```

File: *cargo01-greet/Cargo.toml*:

```toml
{{#include ./cargo01-greet/Cargo.toml}}
```

File: **cargo01-greet/src/lib.rs**

```rust
{{#include ./cargo01-greet/src/lib.rs}}
```

## Example: all targets

The directory `cargo01` in the source repo contains a Cargo package with all
five target kinds.

```text
tree cargo01
cargo01
├── benches
│   └── my_add.rs
├── Cargo.toml
├── examples
│   ├── my_add.rs
│   └── word_count.rs
├── src
│   ├── bin
│   │   └── cargo01-hello.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    └── my_add.rs
```

Its manifest shows how to use dependencies and how to customize a target.

File: *cargo01/Cargo.toml*:

```toml
{{#include ./cargo01/Cargo.toml}}
```

Some possible commands:

- `cargo build`
- `cargo test`
- `cargo test --lib`
- `cargo test --test my_add`
- `cargo run --bin cargo01`
- `cargo run --bin cargo01-hello`
- `cargo run --example my_add`
- `cargo bench`
- `cargo bench --bench my_add`

## Workspaces

A *workspace* is a set of packages sharing one build directory, lockfile,
and dependency-resolution pass. The root manifest lists members and can
define shared metadata that members inherit with `NAME.workspace = true`
(see `edition.workspace = true` in `cargo01/Cargo.toml`):

File: *Cargo.toml*.

```toml
{{#include Cargo.toml}}
```

A workspace groups packages for building and sharing settings; it is not a
module hierarchy. Crates still import each other only through declared
dependencies, as with the `cargo01-greet` path dependency above.

## Build Scripts

A *build script* is a `build.rs` file (or `build = "..."` manifest entry) that
Cargo compiles and runs before the package, typically to generate code or find
native libraries. See [build scripts][build-scripts].

## Deferred Details

Publishing, cross-compilation, and detailed feature resolution are out of
scope. See [Cargo Reference][cargo-ref].

[build-scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[cargo-targets]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
[cargo-ref]: https://doc.rust-lang.org/cargo/reference/

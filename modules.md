# Modules

A Rust crate can have many items. Often, we want to:

1. Group items in a logical hierarchy of *namespaces*;
2. Make some items in a namespace "private", i.e., not visible to code outside
   that namespace;
3. Split large crates into files.

The Rust module system addresses these needs.

> **N.B.** modules are a feature of the Rust compiler (`rustc`) and not Cargo.

## Example

Suppose we are creating a library with the following items:

```text
# Public API
arithmetic_addition_add()
arithmetic_double() // internally, calls arithmetic_addition_add()
report_summary()
# Internal
const labels_TOTALS: &str = "TOTAL"; // used everywhere inside the crate
arithmetic_combine() // used by arithmetic_addition_add()
arithmetic_sample_total() // used by summary()
report_line() // used by summary()
```

## Organization

We can organize the above into a logical hierarchy:

```rust
mod labels {
  const TOTAL: &str = "TOTAL";
}
mod arithmetic {
  mod addition {
    fn add() {}                   // PUBLIC API
  }
  fn double() {}                  // PUBLIC API
  fn combine() {}
  fn sample_total() {}
}
mod report {
  fn summary() {}                 // PUBLIC API
  fn line() {}
}
```

## Visibility: `pub` and `pub(crate)`

By default:

1. A module's items are private to it, and not visible to its parent or sibling
   modules.
2. A modules's items are visible to its child modules.

To make things visible:

1. We add `pub` to items we want to make visible outside the module.
2. We add `pub(crate)` to items we want to make visible within the crate.
3. To make an item visible to external clients, `pub` must be added to all its
   parent modules.

```rust
mod labels {
  pub(crate) const TOTAL: &str = "TOTAL";     // NOTE: pub(crate)
}
pub mod arithmetic {                          // NOTE: pub
  pub mod addition {                          // NOTE: pub
    pub fn add() {}                           // NOTE: pub
  }
  pub fn double() {}                          // NOTE: pub
  fn combine() {}
  pub(crate) fn sample_total() {}             // NOTE: pub(crate)
}
pub mod report {                              // NOTE: pub
  pub fn summary() {}                         // NOTE: pub
  fn line() {}
}
```

## Re-exporting: `use pub`

Currently, an external client can access `summary` as
`<crate>::report::summary`. However, for convenience, we want it to be available
as `<crate>::summary`. We can add `pub use report::summary` at the top level of
the crate to do so:

```rust
pub mod report {
  pub fn summary() {}                         // <crate>::report::summary
}
pub use report::summary;                      // <crate>::summary
```

## Paths

The various functions can now refer to each other, using `::` separared *paths*,
and the `self`, `super` and `crate` keywords.

File: *./modules01/src/lib.rs*:

```rust,ignore
{{#include ./modules01/src/lib.rs}}
```

## Importing: `use`, `use ... as`

A client of the crate can refer to its items using the fully qualified path,
starting from the crate name.

```rust,ignore
{{#include ./modules01/src/main.rs:qualified}}
```

The `use` keyword brings nested modules into scope and allows them to be
referred without qualification.

```rust,ignore
{{#include ./modules01/src/main.rs:module_import}}
```

The `use` keyword can bring items, not just modules, into scope.

```rust,ignore
{{#include ./modules01/src/main.rs:item_import}}
```

The `use ... as` statement can rename an item or module.

```rust,ignore
{{#include ./modules01/src/main.rs:use_as}}
```

For brevity, multiple `use` statements can be collapsed into a "grouped use"
statement:

```rust,ignore
{{#include ./modules01/src/main.rs:grouped_import}}
```

## Modules and Files

`mod xxx { ... }` defines a module inline, as in `modules01` above.  `mod xxx;`
loads the module from a file. By convention, the file is either `xxx.rs` OR
`xxx/mod.rs` in the parent module's directory (both files cannot exist for the
same declaration).

> **N.B.** A module file is not loaded just because it exists: it needs a `mod`
declaration.

> **N.B.** The above locations can be changed by the [`#[path]`
> attribute][ref-module-path] to a `mod` declaration.

The crate root's module directory is the directory containing its source file.
For each nested module, append its name as a directory, whether the module is
inline or loaded from a file. For example:

| Declaration                                                   | Default file locations                         |
| ------------------------------------------------------------- | ---------------------------------------------- |
| `mod alpha;` in `src/lib.rs`                                  | `src/alpha.rs` or `src/alpha/mod.rs`           |
| `mod beta;` in `src/alpha.rs`                                 | `src/alpha/beta.rs` or `src/alpha/beta/mod.rs` |
| `mod beta;` in `src/alpha/mod.rs`                             | `src/alpha/beta.rs` or `src/alpha/beta/mod.rs` |
| `mod beta;` inside inline `mod alpha { ... }` in `src/lib.rs` | `src/alpha/beta.rs` or `src/alpha/beta/mod.rs` |

In the last case, an inline module declares a child in a file. The inline
module's name still contributes a directory to the child's file path.

The `modules02` package reorganizes `modules01` across files:

```text
modules02/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    ├── labels.rs
    ├── arithmetic.rs
    ├── arithmetic/
    │   └── addition.rs
    └── report/
        └── mod.rs
```

Files `lib.rs` and `main.rs` are default Cargo crate root locations. The other
Rust files define modules loaded by the following declarations. Paths are
relative to `modules02/src/`.

| File                     | Declaration           | In File         |
| ------------------------ | --------------------- | --------------- |
| `labels.rs`              | `mod labels;`         | `lib.rs`        |
| `arithmetic.rs`          | `pub mod arithmetic;` | `lib.rs`        |
| `arithmetic/addition.rs` | `pub mod addition;`   | `arithmetic.rs` |
| `report/mod.rs`          | `pub mod report;`     | `lib.rs`        |

## References

- [Modules][ref-modules]
- [Paths][ref-paths]
- [Module Paths][ref-module-path]
- [Visibility and Privacy][ref-vis]
- [Use Declarations][ref-use]
- [Namespaces][ref-namespaces] and [Name Resolution][ref-resolution], including
  macro-specific rules.

[ref-modules]: https://doc.rust-lang.org/reference/items/modules.html
[ref-module-path]:
  <https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute>
[ref-paths]: https://doc.rust-lang.org/reference/paths.html
[ref-vis]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
[ref-use]: https://doc.rust-lang.org/reference/items/use-declarations.html
[ref-namespaces]: https://doc.rust-lang.org/reference/names/namespaces.html
[ref-resolution]: https://doc.rust-lang.org/reference/names/resolution.html

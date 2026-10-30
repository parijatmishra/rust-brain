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

When the Rust compiler sees a `mod XXX` declaration, it looks for the module
code in:

1. Between `{... }` immediately following the declaration, as in `modules01`
   above.
2. In a file `XXX.rs` relative to the file containing the parent module (or
   crate root).
3. In a file `XXX/mod.rs` relative to the file containing the parent module (or
   crate root).

## References

- [Modules][ref-modules]
- [Paths][ref-paths]
- [Visibility and Privacy][ref-vis]
- [Use Declarations][ref-use]
- [Namespaces][ref-namespaces] and [Name Resolution][ref-resolution], including
  macro-specific rules.

[ref-modules]: https://doc.rust-lang.org/reference/items/modules.html
[ref-paths]: https://doc.rust-lang.org/reference/paths.html
[ref-vis]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
[ref-use]: https://doc.rust-lang.org/reference/items/use-declarations.html
[ref-namespaces]: https://doc.rust-lang.org/reference/names/namespaces.html
[ref-resolution]: https://doc.rust-lang.org/reference/names/resolution.html

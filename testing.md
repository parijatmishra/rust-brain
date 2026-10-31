# Testing

## Integration Tests

Files directly under a package's `tests/` directory are separate test crates.
They import the package's library by its crate name and can access **only its
public API**, just like external users. Each file is compiled as a separate test
executable; it does not share the library's private items.

File: `testing/tests/public_api.rs` (excerpt).

```rust
# extern crate testing;
{{#include testing/tests/public_api.rs:integration_tests}}
```

`#[test]` marks a function for the test harness. Here the assertions check the
public function, without depending on its implementation.

## Unit Tests

Unit tests normally live alongside the code they test, in a `tests` child
module. A child module can access its parent's private items, so these tests
can inspect implementation details that integration tests cannot.
`#[cfg(test)]` includes the module when the crate is compiled for testing,
not in the normal library build.

File: *testing/src/lib.rs* (excerpts).

```rust
{{#include testing/src/lib.rs:helper}}

{{#include testing/src/lib.rs:unit_tests}}
# fn main() {}
```

A test passes if it returns normally; a failed `assert!`, `assert_eq!`, or
`assert_ne!` panics and fails the test. Assertions can include a diagnostic
message, as in the integration test. `#[should_panic]` marks a test that must
panic; `#[should_panic(expected = "message")]` also checks the panic message.

Tests may run concurrently and in an unspecified order. Do not depend on
execution order or unsynchronized shared mutable state; isolate external
resources such as files too.

## Doc Tests

Rust code blocks in documentation comments are examples and, by default, tests
run by `rustdoc`. This public function has a passing example and one that must
fail to compile:

File: *testing/src/lib.rs* (excerpt).

```rust
# fn is_odd(number: u32) -> bool { number % 2 == 1 }
{{#include testing/src/lib.rs:docs}}
```

- Each block is tested independently; imports and variables from another block
  do not carry over. Import the library by its crate name and use its public
  API. The library is not compiled with `cfg(test)` for these examples.
- In a documentation code block, prefix a line with `#` and a space to compile
  and run it while hiding it from rendered documentation. An explicit
  `fn main()` is allowed; otherwise `rustdoc` normally supplies one.
- `compile_fail` succeeds on **any** compilation error, not necessarily the
  intended one. Keep the example focused and review the error when needed.
  `no_run` compiles but does not execute the example; `ignore` skips it.
- A successful doc test must compile **and** run without panicking, unless its
  code block has an appropriate attribute such as `compile_fail` or `no_run`.

## Running Tests

From the workspace root:

```sh
cargo test -p testing                     # unit, integration, and doc tests
cargo test -p testing --test public_api   # one integration-test target
cargo test -p testing --lib               # library unit tests only
cargo test -p testing --doc               # library doc tests only
cargo test -p testing private_helper       # filter test names by substring
cargo test --workspace                    # test all workspace members
```

Cargo normally also builds eligible binaries and examples as part of testing;
`--lib`, `--test`, and `--doc` select particular targets. The name filter is
not necessarily unique. Test names can include module paths; use the harness's
`--exact` option for an exact match.

Arguments after `--` go to the test harness rather than Cargo:

```sh
cargo test -p testing --lib -- --list
cargo test -p testing --lib -- tests::private_helper --exact
cargo test -p testing -- --nocapture       # show test output as it happens
cargo test -p testing -- --test-threads=1  # no parallel tests within a harness
```

`--test-threads=1` does not promise an execution order or isolate shared
resources. Mark a test `#[ignore]` to skip it by default, and use
`cargo test -p testing -- --ignored` to run ignored tests. For further options,
run `cargo test --help` and `cargo test -p testing --lib -- --help`.

## Reference

- [Writing tests in The Rust Programming Language][writing-tests]
- [Test organization in The Rust Programming Language][test-organization]
- [`rustdoc` documentation tests][doc-tests]
- [The `test` attribute and test harness][test-attribute]
- [Cargo `test` command][cargo-test] and [Cargo targets][cargo-targets]
- [Assertions in the standard library][assertions]

[writing-tests]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[test-organization]:
  <https://doc.rust-lang.org/book/ch11-03-test-organization.html>
[doc-tests]: https://doc.rust-lang.org/rustdoc/documentation-tests.html
[test-attribute]: https://doc.rust-lang.org/reference/attributes/testing.html
[cargo-test]: https://doc.rust-lang.org/cargo/commands/cargo-test.html
[cargo-targets]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
[assertions]: https://doc.rust-lang.org/std/macro.assert.html

# Debugging

## Inspecting Values

`#[derive(Debug)]` enables debug formatting for the struct. `{:?}` prints a
compact representation; `{:#?}` formats it across multiple lines. Debug output
is for inspection, not a stable serialization format.

`println!` writes to standard output; `eprintln!` writes diagnostics to standard
error. Both add a newline.

File: *debugging/src/main.rs* (excerpt).

```rust
{{#include debugging/src/main.rs:output}}
```

For this value, the debug-formatted output on standard output is:

```text
job: Job { name: "import", completed: 2 }
job (pretty): Job {
    name: "import",
    completed: 2,
}
```

The diagnostic `checking import: 2 completed` goes to standard error.

`dbg!` prints the source location, expression, and its debug-formatted value to
standard error, then returns the value. It runs in release builds too. It can
move its argument: borrow with `dbg!(&value)` to inspect without moving it, or
use the returned value as below.

File: *debugging/src/main.rs* (excerpt).

```rust
{{#include debugging/src/main.rs:dbg}}
```

## Assertions

`assert!` checks a boolean assumption; `assert_eq!` checks that two values are
equal, and `assert_ne!` checks that they differ. All panic on failure and can
include a custom message. The equality assertions also report the compared
values using debug formatting. These assertions run in both debug and release
builds.

File: *debugging/src/main.rs* (excerpt).

```rust
{{#include debugging/src/main.rs:assertions}}
```

Run `cargo run -p debugging` for the passing example. To see a failed assertion,
run `cargo run -p debugging -- --fail`; the following excerpt then panics:

File: *debugging/src/main.rs* (excerpt).

```rust,no_run
{{#include debugging/src/main.rs:failed_assertion}}
```

The panic message includes:

```text
assertion `left == right` failed: completed count mismatch
  left: 2
 right: 3
```

Set `RUST_BACKTRACE=1` to include a backtrace with the panic:

```sh
RUST_BACKTRACE=1 cargo run -p debugging -- --fail
```

## Debug Assertions

`debug_assert!`, `debug_assert_eq!`, and `debug_assert_ne!` normally run in
debug builds but not release builds. Cargo profile settings can change this.
Do not rely on side effects inside their arguments: disabled checks do not
evaluate them.

File: *debugging/src/main.rs* (excerpt).

```rust
{{#include debugging/src/main.rs:debug_assertions}}
```

The following assertion fails when debug assertions are enabled:

File: *debugging/src/main.rs* (excerpt).

```rust,no_run
{{#include debugging/src/main.rs:failed_debug_assertion}}
```

With the default Cargo profiles, `cargo run -p debugging -- --debug-fail`
panics, while `cargo run -p debugging --release -- --debug-fail` completes
normally.

## References

- [`Debug` formatting](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
- [`println!`](https://doc.rust-lang.org/std/macro.println.html) and
  [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html)
- [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html)
- [`assert!`](https://doc.rust-lang.org/std/macro.assert.html),
  [`assert_eq!`](https://doc.rust-lang.org/std/macro.assert_eq.html), and
  [`assert_ne!`](https://doc.rust-lang.org/std/macro.assert_ne.html)
- [`debug_assert!`][debug-assert], [`debug_assert_eq!`][debug-assert-eq], and
  [`debug_assert_ne!`][debug-assert-ne]
- [Cargo's `debug-assertions` profile setting][profile-debug-assertions]
- [Panic backtraces][panic-backtrace]

[debug-assert]: https://doc.rust-lang.org/std/macro.debug_assert.html
[debug-assert-eq]: https://doc.rust-lang.org/std/macro.debug%5Fassert%5Feq.html
[debug-assert-ne]: https://doc.rust-lang.org/std/macro.debug%5Fassert%5Fne.html
[profile-debug-assertions]:
  <https://doc.rust-lang.org/cargo/reference/profiles.html#debug-assertions>
[panic-backtrace]:
  <https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html>

# Documentation

The Rust compiler understands two kinds of comments.

| #   | Syntax      | Meaning                    |
| --- | ----------- | -------------------------- |
| 1   | `// ....`   | A line-comment.            |
| 2   | `/* ... */` | A block-comment; can nest. |

The `rustdoc` tool understands four more comment styles.

| #   | Syntax       | Meaning                                                                                    |
| --- | ------------ | ------------------------------------------------------------------------------------------ |
| 3   | `///`        | OUTER-DOC-LINE: documents the immediately following item.                                  |
| 4   | `/** ... */` | OUTER-DOC-BLOCK: documents the immediately following item.                                 |
| 5   | `//!`        | INNER-DOC-LINE: documents the enclosing item. Usually used to document crates or modules.  |
| 6   | `/*! ... */` | INNER-DOC-BLOCK: documents the enclosing item. Usually used to document crates or modules. |

## Example

File: *./comments/src/lib.rs*:

```rust
{{#include comments/src/lib.rs}}
```

Run `cargo doc -p comments --open`: this will run `rustdoc` on the package
`comments`, put the generated files in `target/doc/comments/index.html`, and
open that file in your browser.

## References

- [The `rustdoc` book](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
- [Writing documentation][writing]
- [API documentation guidelines][API-docs]

[writing]: https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
[API-docs]: https://rust-lang.github.io/api-guidelines/documentation.html

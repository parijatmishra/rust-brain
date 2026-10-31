# Prelude

## Standard Prelude

By default, the Rust compiler includes the [standard library][rust-std] as a
dependency of all packages. (This can be changed: see [no_std][no-std].)

Normally, when a program wants to use an item from a dependency, it needs to
either refer to its fullly qualified path or bring the item name into scope
using `use`. However, some items are used in almost every Rust program. It would
be very tedious to have to bring them into scope in every piece of Rust code.

Hence, the Rust compiler effectively inserts a `use std::prelude::*` in every
file, making some names available to the code without explicitly importing them.
This `std::prelude` module is called the [standard prelude][std-prelude]. See
the link to find out which names it makes available.

```rust
// the standard prelude includes:
// - std::collections:Vec and the vec! macro
// - std::option::Option::{self, Some, None}
let numbers: Vec<i32> = vec![1, 2];
let first: Option<i32> = numbers.first().copied();
assert_eq!(first, Some(1));
```

```rust
# fn main() {
// std::collections::HashMap is not in the standard prelude
// It must be reffered to via its fully qualified path
let mut counts = std::collections::HashMap::new();
counts.insert("apples", 2);

// Or brought into scope via use
use std::collections::HashMap;
let mut other_counts = HashMap::new();
other_counts.insert("oranges", 3);
# }
```

## References

- [Rust Standard Library][rust-std]
- [Standard Prelude][std-prelude]

[rust-std]: https://doc.rust-lang.org/std/
[std-prelude]: https://doc.rust-lang.org/std/prelude/index.html
[no-std]: https://docs.rust-embedded.org/book/intro/no-std.html

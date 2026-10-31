# Values and Identifiers

## Identifiers

An identifier names a binding or item. Names are case-sensitive, may contain
Unicode letters, and cannot start with a digit. Use `snake_case` for local
bindings and `SCREAMING_SNAKE_CASE` for constants and statics.

Keywords normally cannot be identifiers. A [raw identifier][identifiers], such
as `r#type`, allows many keywords to serve as names; `r#` is not part of the
name. A leading underscore suppresses unused-binding warnings. `_` alone does
not create a binding; `_name` does. Destructuring and other pattern uses come
with patterns.

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:identifiers}}
```

A raw identifier can use a keyword as a name:

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:raw_identifier}}
```

Without `r#`, the keyword is rejected:

```rust,compile_fail
let type = 42;
```

## Local Bindings and Initialization

`let name = value;` introduces a local binding, immutable by default. A binding
must be initialized before use; Rust does not supply a default value for an
uninitialized local. Declaration and initialization may be separate, even for
an immutable binding. Its first assignment initializes it; later assignments
would require mutability.

The [binding syntax][let-statements] also accepts patterns. Start with one
identifier; destructuring and conditional bindings receive their own treatment
later.

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:local_initialization}}
```

Using a binding before initialization is rejected:

```rust,compile_fail
let value: i32;
let copy = value; // Error: `value` is not initialized.
```

## Type Annotations and Inference

`let name: Type = value;` states a binding's type explicitly. Without the
annotation, Rust infers a type from the initializer and surrounding uses, not
just from the declaration in isolation. An annotation constrains inference;
it does not request a conversion.

Every binding has a type fixed at compile time, whether inferred or annotated.
Inference is not dynamic typing, and `mut` does not allow a binding's type to
change. Primitive types and their literal syntax follow in the next topic;
generic inference comes with generics.

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:type_annotations}}
```

A later assignment can constrain the type inferred for an earlier binding:

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:later_type_constraint}}
```

An annotation does not convert an incompatible initializer:

```rust,compile_fail
let number: i32 = true; // Error: expected i32, found bool.
```

## Mutable Bindings

`let mut name = value;` permits reassignment and ordinary mutation through the
binding. Reassignment uses `name = value;`, without another `let`, and the new
value must have the same type.

Mutability belongs to the binding; it is not a different value type. An
immutable local binding is not a `const`: its initializer can run at runtime.
Ownership, mutation through references, and interior mutability refine these
rules later.

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:mutable_binding}}
```

An initialized immutable binding cannot be reassigned:

```rust,compile_fail
let score = 10;
score = 20; // Error: `score` is immutable.
```

A mutable binding still has a fixed type:

```rust,compile_fail
let mut score: i32 = 10;
score = true; // Error: expected i32, found bool.
```

## Scope and Shadowing

A local binding's scope begins after its `let` statement and extends to the
end of its enclosing block, subject to shadowing. A new `let` can introduce the
same name again. This is [shadowing][scopes], not reassignment: the new binding
has its own type and mutability, independent of the previous binding. Its
initializer can still use the previous binding.

Shadowing works in the same block or in a nested block. After a nested block
ends, the outer binding is visible again. Name visibility alone does not
explain when values are destroyed; ownership and drop scopes come later.

Shadowing can change the type of a name:

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:shadowing}}
```

It can also change mutability:

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:shadow_mutability}}
```

The new immutable binding cannot be reassigned:

```rust,compile_fail
let mut level = 1;
level += 1;
let level = level;
level = 3; // Error: the new binding is immutable.
```

A nested binding shadows the outer one only within its block:

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:nested_scope}}
```

A block-local binding is not visible outside the block:

```rust,compile_fail
{
    let inside = 42;
    assert_eq!(inside, 42);
}
let _ = inside; // Error: `inside` is out of scope.
```

## Constants

`const NAME: Type = value;` declares a [constant item][constants], not a local
variable. Its type and initializer are required, and the initializer must be a
constant expression evaluated at compile time. An immutable `let`, by
contrast, may be initialized from a runtime computation.

Constants can appear at module scope or within a block. They cannot be
reassigned or declared `mut`. A constant denotes a value, not a unique storage
location. Uses need not share an address; do not treat a constant as a shared
variable.  `const fn` and the restrictions on constant evaluation receive
further treatment with functions; associated constants come with traits.

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:constant_value}}
```

An immutable local can use a runtime parameter; a constant initializer cannot:

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:constant_and_runtime}}
```

A constant declaration requires a type:

```rust,compile_fail
const LIMIT = 10; // Error: missing type.
```

A constant cannot use a runtime local as its initializer:

```rust,compile_fail
let runtime = std::process::id();
const ID: u32 = runtime; // Error: a constant cannot use a local binding.
```

A constant cannot be reassigned:

```rust,compile_fail
const LIMIT: u32 = 10;
LIMIT = 20; // Error: a constant is not a mutable binding.
```

A constant in a nested scope can shadow an outer constant:

File: *values_ident/src/main.rs* (excerpt).

```rust
{{#include values_ident/src/main.rs:constant_shadowing}}
```

A `let` binding cannot shadow a visible constant. A bare name matching a
constant in a `let` pattern refers to that constant, rather than creating a
binding. The integer pattern below is refutable, so `let` rejects it:

```rust,compile_fail
const LIMIT: u32 = 10;
let LIMIT = 20; // Error: refutable pattern, not a new binding.
```

## Immutable Statics

`static NAME: Type = value;` declares a [static item][statics] with a type and
constant initializer. Unlike a constant, a static denotes a storage location
that lasts for the program's execution. A static declared inside a function is
still one static, not a fresh local on each call; its name has local visibility.

An immutable static can be read from safe code but cannot be reassigned. Sharing
it must be thread-safe: its type must satisfy `Sync`, explained with threads.
This does not mean every immutable static has immutable internals; interior
mutability and synchronization cover controlled shared mutation later. Runtime
lazy initialization comes in its own topic, not from an ordinary static
initializer.

TODO: Code samples:

- Declare a primitive constant and an immutable primitive static side by side;
  read both and explain value versus shared storage in comments.
- Read a function-local static on repeated calls, noting that it is the same
  static each time; leave reference and address comparisons for later.
- Reject reassignment of an immutable static and a runtime-dependent static
  initializer in separate `compile_fail` snippets.

## Recognizing Mutable Statics

`static mut NAME: Type = value;` declares a mutable static. Recognize the
syntax, but do not use it as an ordinary global variable. Reading or writing
its stored value requires unsafe code and obligations concerning aliasing and
concurrent access; `unsafe` does not supply synchronization or make invalid
access sound. Even single-threaded access can violate aliasing rules.

Rust 2024 [denies references to mutable statics][static-mut-refs] by default,
including implicit references. Safe shared-state APIs belong with interior
mutability, synchronization, and atomics; unsafe access techniques remain
outside this book's scope.

TODO: Code samples:

- Show only a primitive `static mut` declaration, annotated with its hazards;
  do not read it, write it, or create references to it.

[identifiers]: https://doc.rust-lang.org/reference/identifiers.html
[let-statements]:
  <https://doc.rust-lang.org/reference/statements.html#let-statements>
[scopes]:
  <https://doc.rust-lang.org/reference/names/scopes.html#pattern-binding-scopes>
[constants]: https://doc.rust-lang.org/reference/items/constant-items.html
[statics]: https://doc.rust-lang.org/reference/items/static-items.html
[static-mut-refs]:
  <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>

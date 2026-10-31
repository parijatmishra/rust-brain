# Introduction

{{#include README.md:purpose-scope}}

## Ordering of Topics

The "Bootstrap" section is intended to get someone who has used but forgotten
the details get started quickly on scaffoling a new project or understanding an
existing codebase's organization. To show working examples, its sample code uses
concepts introduced in later topics, of necessity.

The subsequent topics are ordered by concepts, with later concepts building on
earlier ones. They are intended to be read in order. This book is not a
reference: the full semantics of a keyword, syntactical element, type or trait
may be spread across several topics if that leads to better understanding.

Each topic has a status keyword: `DONE` means developed, `WIP` means being
developed, and `TODO` means planned. Use this entry format:

```text
- (STATUS) [Title](link-to-source.md) --- brief description.
  - Emphasize: rules and distinctions the reader should retain.
  - Defer: details covered later or outside this book's scope.
  - Examples: code examples that demonstrate the topic.
```

A `TODO` entry may omit its link or leave the destination empty. A `WIP` entry
must link to an existing file. Preserve links when changing an entry's status.

## Bootstrap

Use minimally complete examples, even where they require forward references.
Explain scaffolding only enough to build, run, inspect, and test the code;
resolve its underlying concepts in later topics.

- (DONE) [Cargo](./cargo.md)

- (DONE) [Modules](./modules.md)

- (DONE) [Comments](./comments.md)

- (DONE) [Debugging](./debugging.md)

- (DONE) [Testing](./testing.md)

- (DONE) [Prelude](./prelude.md)

## Basics

Focus on syntax and value-level behavior. Use owned primitives and small
examples; identify references, generics, and macros without developing them
before their dedicated sections.

- (WIP) [Values and Identifiers](./values_idents.md) --- `let`; `let mut`;
  `const`; `static`; type annotations; type inference; shadowing; recognition
  of `static mut`.
  - Emphasize: binding mutability and shadowing are different. A constant is
    not a mutable binding, and a static denotes a shared storage location.
    Type inference does not remove static type checking.
  - Defer: ownership, references, interior mutability, synchronization, and
    unsafe access to mutable statics. Show `static mut` only to recognize its
    syntax and hazards.
  - Examples: infer and annotate a value's type; mutate a binding; shadow it
    with a different type; contrast a constant with an immutable static.

- (TODO) [Primitive Types] --- booleans; integers; floats; `char`; `str`;
  tuples; arrays; unit; references; raw pointer and never (`!`) syntax.
  - Emphasize: arrays have a fixed length in their type; tuples can mix types;
    unit is a value, not an absent return type. A `char` is a Unicode scalar
    value, not a byte or necessarily a displayed character.
  - Defer: borrowing rules, string storage, dynamically sized types, and raw
    pointer dereferencing. Introduce `!` through a diverging expression.
  - Examples: annotate numeric literals; unpack a tuple; index an array;
    contrast `'x'` with `"x"`; show a function returning unit.

- (TODO) [Expressions vs Statements] --- expressions; statements; block
  expressions; trailing expressions; semicolons.
  - Emphasize: blocks can produce values. A semicolon often changes a block's
    result to unit; branches used as values must have compatible types.
  - Defer: the full expression grammar and macro-specific syntax. Revisit
    diverging branches with control flow and error handling.
  - Examples: initialize a variable from a block; compare blocks with and
    without a trailing semicolon; use an `if` expression as a value.

- (TODO) [Basic Operators] --- arithmetic; comparison; boolean short-circuiting;
  bitwise operations; assignment; precedence; integer overflow;
  checked, wrapping, and saturating arithmetic.
  - Emphasize: `&&` and `||` short-circuit; bitwise operators do not. Integer
    division truncates toward zero. Overflow behavior depends on the operation
    and overflow-check settings; choose explicit methods when it matters.
  - Defer: operator overloading to traits. Link to the precedence table rather
    than making examples depend on obscure precedence rules.
  - Examples: skip a right-hand operand through short-circuiting; build a bit
    mask; compare `checked_add`, `wrapping_add`, and `saturating_add`.

- (TODO) [Type Conversions] --- casts with `as`; numeric truncation and sign
  changes; explicit type conversion; introductory `From` calls.
  - Emphasize: `as` is not a general checked conversion. Different numeric
    casts have different rules; a cast may lose information without reporting
    failure. `From` describes infallible conversions, not arbitrary casts.
  - Defer: `From`, `Into`, `TryFrom`, and `TryInto` implementations to
    conversion traits. Avoid pointer casts and exhaustive numeric-cast tables.
  - Examples: widen an integer with `u32::from`; narrow an integer with `as`;
    contrast signed and unsigned casts; show float-to-integer truncation.

- (TODO) [Control Flow] --- `if` and `else`; `loop`; `while`; `break`;
  `continue`; labels; values returned from `loop`.
  - Emphasize: conditions must be boolean. `loop` can produce a value through
    `break value`; ordinary `while` loops cannot. Labels select which enclosing
    loop a `break` or `continue` targets.
  - Defer: `for` to the iteration-basics topic later in this section;
    pattern-based conditions to enums and patterns. Keep asynchronous
    control flow outside this section.
  - Examples: select a value with `if`; search with a value-producing `loop`;
    skip iterations; leave a nested loop through a label.

- (TODO) [Basic Functions] --- function definitions; parameter and return
  types; calls; trailing return expressions; early `return`; `const fn`.
  - Emphasize: function signatures state parameter types and non-unit return
    types. A `const fn` may run at runtime; its body must also satisfy the
    restrictions for use in constant evaluation.
  - Defer: references, generics, lifetimes, trait objects, and closures.
    Keep examples to owned primitive arguments and return values.
  - Examples: define and call an arithmetic function; return early from a
    branch; use a `const fn` both in a constant and with a runtime argument.

- (TODO) [Structs] --- named-field, tuple, and unit structs; inherent `impl`;
  receiver choices; associated functions; method calls; the newtype pattern.
  - Emphasize: methods have a receiver; associated functions need not.
    Introduce `self`, `&self`, and `&mut self` as consuming, shared, and mutable
    access. Method calls can insert borrowing and dereferencing.
  - Defer: ownership details, receiver lifetimes, and full method lookup.
    Treat reference receivers as a forward reference to borrowing.
  - Examples: construct each struct form; add a constructor and methods;
    distinguish a newtype from a type alias; call a mutating method.

- (TODO) [Enums] --- variants and payloads; inherent methods; `match`;
  `if let`; `while let`; `let ... else`; `let` chains in conditions.
  - Emphasize: a value contains one variant at a time. `match` must be
    exhaustive; `if let` handles a selected pattern. The `else` branch of
    `let ... else` must diverge rather than supply an alternative value.
  - Defer: full pattern syntax and move-versus-borrow behavior to later topics.
    Use Rust 2024 and a supporting stable compiler for `let` chains.
  - Examples: model a small state machine; extract a variant's payload;
    return early with `let ... else`; combine a match and guard-like condition.

- (TODO) [Patterns] --- destructuring bindings and parameters; wildcards;
  rest patterns; alternatives; guards; ranges; `@`; refutability;
  reference patterns; match ergonomics.
  - Emphasize: pattern position is not expression position. Distinguish
    irrefutable bindings from conditional matches, and a reference pattern
    from a binding that borrows with `ref` or `ref mut`.
  - Defer: detailed move and borrow consequences until ownership. Revisit
    match ergonomics there rather than memorizing desugaring rules here.
  - Examples: destructure a struct and tuple; match ranges and alternatives;
    bind with `@`; compare matching an owned value with matching a reference.

- (TODO) [`Option`] --- `Option<T>`; `Some` and `None`; matching;
  `if let`; `let ... else`; `unwrap`, `expect`, and eager fallbacks.
  - Emphasize: absence is represented in the type, and `None` is not a null
    pointer. `unwrap` and `expect` panic on absence, so pattern-based
    handling stays the default.
  - Defer: closure-taking combinators to closures and error handling, and
    borrowed inspection through `as_ref` to ownership. Treat panics as
    scaffolding until their own topic.
  - Examples: return an optional lookup result; match `Some` and `None`;
    return early on absence with `let ... else`; supply a fallback value.

- (TODO) [Iteration Basics] --- `for`; `IntoIterator`; ranges; iterating
  arrays and slices by reference or by value; `Iterator::next`; iteration
  ending through `Option`; `break` and `continue` inside `for`.
  - Emphasize: `for` consumes an iterator that `IntoIterator` produces.
    Iterating by reference borrows each element, while consuming an owned
    value moves its elements. `next` reports progress through `Option`.
  - Defer: lazy adapters, `collect`, fallible iteration, and returning
    iterators to the iterator-adapters topic after the standard collections.
  - Examples: iterate a range and an array; compare borrowing iteration with
    consuming it; drive `next` by hand; skip elements with `continue`.

## Ownership, References, Borrowing

Use integers, `String`, and small structs to make ownership visible. Pair
accepted code with focused borrow-checker failures; explain which value or
borrow each rule concerns.

- (TODO) [Ownership and Destruction] --- ownership; mutable bindings; stack
  and heap; moves versus copies; drop scopes and order; partial moves;
  temporary lifetimes.
  - Emphasize: moving transfers ownership, not necessarily a heap allocation.
    Mutation permissions and ownership are distinct. Scope and control flow
    determine destruction; partially moved values have restricted uses.
  - Defer: `Drop` implementations and derive mechanics to the traits
    section. Resolve `Copy` and `Clone` semantics in the next topic;
    introduce temporary lifetimes here and revisit them with lifetime
    annotations.
  - Examples: pass an integer and a `String` by value; clone a string;
    move one struct field; trace which bindings remain usable afterward.

- (TODO) [Copying and Cloning] --- moves versus copies; `Copy` types;
  `#[derive(Copy, Clone)]`; explicit `clone`; drop at scope exit; types
  that cannot be `Copy`.
  - Emphasize: copying duplicates a value implicitly, while cloning is an
    explicit call that may allocate or run code. Types needing custom
    destruction cannot be `Copy`, and not every value can be duplicated
    cheaply.
  - Defer: trait definitions, derive mechanics, and `Drop` implementations
    to the traits section; auto traits and variance to later topics.
  - Examples: pass an integer and a `String` by value and compare later
    uses; derive `Copy` and `Clone` for a small record; clone a string to
    keep the original usable.

- (TODO) [References and Borrowing] --- shared and mutable references;
  borrowing rules; reborrowing; non-lexical lifetimes; references versus
  ownership; array and slice borrows.
  - Emphasize: references do not transfer ownership. Shared access prevents
    ordinary mutation; mutable access must be exclusive while the borrow is
    active. A borrow can end before the binding's enclosing scope ends.
  - Defer: explicit lifetime annotations and interior mutability. Treat
    reborrowing as temporary access, not as copying an exclusive reference.
  - Examples: inspect and mutate a caller-owned value; reborrow `&mut T`;
    show a borrow ending at its last use; borrow an array as a slice.

- (TODO) [Strings and str] --- `String` versus `str`; string slices;
  UTF-8 bytes; Unicode scalar values; grapheme clusters; indexing and slicing.
  - Emphasize: `String` owns a growable UTF-8 buffer; `&str` borrows valid
    UTF-8 text. Byte offsets are not character positions, and slice boundaries
    must fall on UTF-8 boundaries. Integer string indexing is unavailable.
  - Defer: detailed iterator APIs and grapheme segmentation. Label any Unicode
    segmentation crate as an external dependency rather than a `std` feature.
  - Examples: pass both a literal and a `String` to a function taking `&str`;
    compare byte and scalar counts; contrast valid and invalid byte slices.

- (TODO) [Lifetimes] --- borrowed parameters and returns; explicit lifetime
  annotations; elision; borrowed fields; lifetime bounds; `'static`;
  temporary lifetimes and drop scopes.
  - Emphasize: annotations express relationships checked by the compiler;
    they do not extend a value's life. Distinguish `&'static T` from `T:
    'static`, and a reference's usable lifetime from its owner's scope.
  - Defer: higher-ranked bounds and advanced variance. Introduce bounds only
    far enough to understand borrowed types and later generic interfaces.
  - Examples: return one of two borrowed strings; define a struct borrowing
    text; explain a rejected reference to a local or temporary value.

## Generics

Introduce parameterized types before trait-based constraints. Reuse ownership
and borrowing examples so that generic syntax is the only new mechanism.

- (TODO) [Generic Structs, Enums and Functions] --- type parameters; inference;
  explicit arguments; const generics; default generic parameters where allowed.
  - Emphasize: each instantiation remains statically typed. Unconstrained
    parameters do not support arbitrary operations; const parameters can make
    values such as array lengths part of a type.
  - Defer: bounded generics to traits; detailed monomorphization costs and
    advanced const-evaluation restrictions. Distinguish defaults on types
    from inference at a function call.
  - Examples: define a generic wrapper and identity function; supply explicit
    arguments with turbofish syntax; wrap an array with a const length.

- (TODO) [`Result`] --- `Result<T, E>`; `Ok` and `Err`; variant definitions;
  pattern-based use; methods without closure parameters; conversions between
  `Option` and `Result`.
  - Emphasize: failure is represented in the type, and an `Err` is an
    ordinary value until the program chooses how to handle it. `unwrap` and
    `expect` can panic; keep exhaustive matching visible before introducing
    shorter handling patterns.
  - Defer: closure-taking combinators and `?` to error handling, and custom
    error types to the same section. Recall `Option` from the basics section
    when bridging absence and failure.
  - Examples: represent a failed conversion; match each variant; inspect a
    borrowed result with `as_ref`; turn an absent value into an error and an
    error into an absent value.

## Traits and Trait Objects

Build from shared interfaces to generic constraints and then runtime dispatch.
Separate the behavior a trait promises from how a caller chooses an
implementation.

- (TODO) [Traits] --- definitions and implementations; bounds; `where`;
  supertraits; default methods; associated functions; blanket implementations;
  coherence and orphan rules; method disambiguation; fully qualified syntax.
  - Emphasize: bounds state available behavior; implementations must satisfy
    coherence rules. A supertrait requires another trait implementation, not
    inherited fields. A local newtype can provide a distinct implementation.
  - Defer: associated types, trait objects, and detailed coherence edge cases.
    Explain blanket implementations through one constrained generic example.
  - Examples: define and implement a trait; constrain a generic function;
    resolve overlapping method names with `<Type as Trait>::method`.

- (TODO) [Associated Types] --- associated types and constants; associated
  types versus trait parameters; generic associated types.
  - Emphasize: an associated type is chosen by an implementation for a given
    trait instantiation. Generic associated types describe a family of types,
    including types borrowing from the implementor.
  - Defer: advanced higher-ranked constraints and dyn-compatibility rules.
    Explain required lifetime bounds only as the borrowing example needs them.
  - Examples: define a trait with an output type and constant; contrast it with
    a generic trait; return a borrowed view through a lifetime-parameterized
    associated type.

- (TODO) [Derivable Traits] --- `Debug`; `PartialEq` and `Eq`; `PartialOrd`
  and `Ord`; `Hash`; `Default`; `Copy` and `Clone` contracts recalled from
  the ownership section.
  - Emphasize: derives compose field behavior and impose relevant bounds.
    Recall `Copy` as implicit duplication and `Clone` as an explicit call
    that can run code. Equality, ordering, and hashing carry contracts
    beyond matching signatures.
  - Defer: collection-specific consequences to standard collections and
    custom derive implementation to macros. Introduce float comparison
    limits without treating all numeric types as totally ordered.
  - Examples: derive traits for a small record; contrast a copyable value
    with a cloned `String`; show why a float-containing type cannot derive
    `Eq`.

- (TODO) [Conversion Traits] --- `From` and `Into`; `TryFrom` and `TryInto`;
  infallible versus fallible conversions; target-type inference.
  - Emphasize: implementing `From` supplies the corresponding `Into` through
    a blanket implementation; the same relationship holds for the fallible
    pair. Use fallible conversion when valid inputs may be rejected.
  - Defer: error propagation with `?` to error handling. Distinguish these
    conversion contracts from numeric casts already introduced with `as`.
  - Examples: convert a source value into a newtype; check an integer narrowing
    with `try_from`; resolve an ambiguous `.into()` using a type annotation.

- (TODO) [Operator Traits] --- operator traits and implementations;
  associated output types; owned and borrowed operands; assignment variants.
  - Emphasize: operator syntax maps to specific traits, and receiver choices
    affect whether operands move or borrow. `Add` and `AddAssign` are separate
    contracts; not every language operator can be overloaded.
  - Defer: exhaustive operator coverage and elaborate numeric abstractions.
    Connect each example to the earlier basic-operator syntax.
  - Examples: implement `Add` for a small vector type; preserve operands with
    a borrowed implementation; add `AddAssign` for in-place updates.

- (TODO) [Borrowing Traits] --- `AsRef`; `AsMut`; `Borrow`; borrowed views;
  generic parameters accepting multiple representations.
  - Emphasize: these traits expose borrowed views rather than transfer
    ownership. `Borrow` additionally requires compatible equality, ordering,
    and hashing behavior between owned and borrowed forms.
  - Defer: heterogeneous map lookup until collections. Do not treat `AsRef`
    and `Borrow` as interchangeable simply because their methods return
    references.
  - Examples: accept both owned and borrowed text through `AsRef<str>`;
    expose a mutable slice through `AsMut`; contrast a view with a `Borrow`
    implementation that preserves identity.

- (TODO) [`Sized`, `?Sized`] --- sized and dynamically sized types;
  implicit bounds; slices; trait-object previews; pointers to unsized values.
  - Emphasize: generic type parameters usually have an implicit `Sized` bound;
    `?Sized` relaxes it rather than requiring an unsized type. A pointer can
    have a known size even when its pointee does not.
  - Defer: trait-object dispatch details and layout internals. Explain why
    slices and trait objects normally appear behind references or owning
    pointers without promising a stable ABI.
  - Examples: compare `[u8; 4]` with `[u8]`; accept `&T` with `T: ?Sized`;
    call the same function with both sized data and a string slice.

- (TODO) [Unique Owning Pointers] --- `Box<T>`; `Deref` and `DerefMut`;
  implementing `Drop`; method lookup; autoderef and autoref;
  destruction versus deallocation.
  - Emphasize: moving a box moves ownership of its allocation, not its
    pointee. `Drop` runs cleanup before fields are destroyed; deallocation
    is distinct from the cleanup behavior of the stored value.
  - Defer: a full coercion survey and unsafe pointer implementations. Preview
    boxed trait objects here and resolve their dispatch rules next.
  - Examples: allocate and mutate a boxed value; implement safe dereferencing
    for a wrapper; log drop order and use `std::mem::drop` for early cleanup.

- (TODO) [Trait Objects] --- `dyn Trait`; static versus dynamic dispatch;
  borrowed versus boxed objects; dyn compatibility; object lifetime bounds;
  associated-type constraints; restrictions on generic methods.
  - Emphasize: a trait object erases the concrete implementor while preserving
    access to a compatible interface. `Box<dyn Trait>` owns its value;
    `&dyn Trait` borrows it. Object lifetime bounds constrain borrowed data.
  - Defer: ABI details and async traits. Explain `where Self: Sized` as removing
    a method from object dispatch, not making that method callable on `dyn`.
  - Examples: call one interface through generic and dynamic dispatch; return
    a boxed implementation; specify an associated type on a trait object;
    explain a rejected generic method.

## Recursive Types

Combine enums, generics, ownership, and indirection in one small data
structure. Prefer clear ownership behavior over a production collection
implementation.

- (TODO) [Recursive Types] --- a `List<T>`; indirection with `Box`;
  construction; borrowed traversal; insertion; removal; destruction.
  - Emphasize: recursive values need indirection to have a finite layout.
    Owning links determine destruction; moving a link differs from borrowing
    its node. A teaching list is not a reason to replace `Vec` by default.
  - Defer: unsafe linked structures, allocator design, and production tuning.
    Flag that recursive destruction of a very deep list can exhaust the stack.
  - Examples: define an empty-or-node enum; implement push and pop by moving
    links; traverse borrowed nodes; provide a small borrowed iterator.

## Closures, `Fn*` Traits and Higher-Order Functions

Connect callable syntax to capture, ownership, and trait bounds. Use small
callbacks before introducing iterator adapters that rely on them.

- (TODO) [Closures] --- closure syntax; parameter and return inference;
  capture inference; borrowing versus moving captures; `move` closures.
  - Emphasize: captures depend on how the body uses its environment. `move`
    changes how values enter a closure; it does not mean every call consumes
    the closure or that a captured reference becomes owned data.
  - Defer: detailed call-trait bounds to the next topic; async closures to
    async Rust. Keep capture examples independent of threads.
  - Examples: borrow a local for reading; mutate captured state; move a
    `String` into a closure; contrast moving a string with moving `&str`.

- (TODO) [`Fn`, `FnMut`, and `FnOnce`] --- anonymous closure types;
  call traits; shared, mutable, and consuming calls; choosing callback bounds.
  - Emphasize: every closure expression has a distinct type that cannot be
    named directly. All closures implement `FnOnce`; their bodies determine
    whether they also implement `FnMut` and `Fn`. `move` alone does not decide.
  - Defer: higher-ranked bounds and type erasure. Use the least restrictive
    bound that still supports the caller's intended use.
  - Examples: accept a read-only callback, a stateful repeated callback, and
    one that consumes a capture; show which calls require a mutable binding.

- (TODO) [`fn` Pointers] --- function items; `fn` pointer types; coercions;
  noncapturing closures; differences from capturing closures and `dyn Fn`.
  - Emphasize: a function item has its own type and can coerce to a function
    pointer. A `fn` pointer stores no captured environment; a trait object
    can refer to a callable with captures.
  - Defer: foreign calling conventions and unsafe function pointers. Keep
    function-pointer lifetimes separate from captured closure lifetimes.
  - Examples: assign a named function to an annotated pointer; coerce a
    noncapturing closure; explain why a capturing closure cannot become `fn`.

- (TODO) [Higher-Order Functions] --- callable arguments and returns;
  argument-position versus return-position `impl Trait`; returning closures;
  generics versus `impl Trait` versus `dyn Trait`; `use<...>` capture bounds.
  - Emphasize: argument-position `impl Trait` accepts a caller-chosen type;
    return-position `impl Trait` hides one concrete type chosen by the
    function. Opaque-type capture bounds are not closure capture instructions.
  - Defer: advanced opaque-type rules and async return types. State the edition
    used when explaining default lifetime capture and explicit `use<...>`.
  - Examples: pass a generic callback; return a closure capturing an owned
    value; use a boxed callable for differing concrete return types;
    demonstrate a precise opaque-type capture bound.

- (TODO) [Higher-Ranked Trait Bounds] --- `for<'a>`; bounds valid for every
  lifetime; borrowed callback interfaces; elided callable lifetimes.
  - Emphasize: accepting a callback for any caller-supplied borrow differs
    from choosing one lifetime for the whole interface. The bound does not
    require the borrowed values to live forever.
  - Defer: advanced interactions with generic associated types and async
    callback traits. Connect those cases back to this borrowing model later.
  - Examples: call a `for<'a> Fn(&'a str)` callback with separate local strings;
    compare explicit higher-ranked syntax with its elided callable form.

## Input and Output

Introduce formatted output before error handling so custom error types can
implement `Display` with everything they need already in scope, and so
later sections that print diagnostics treat formatting as recall rather
than scaffolding. Keep `?`-based propagation brief here; the operator
receives its own treatment in the next section.

- (TODO) [I/O and Formatting] --- `format!`; width, precision, and alignment;
  `print!` and `println!`; `write!` and `writeln!`; `Display`;
  `std::io::Read` and `Write`; buffering.
  - Emphasize: formatting and I/O are related but distinct. `fmt::Write`
    writes formatted text, while `io::Write` writes bytes and can report I/O
    errors. Reads and writes can be partial; buffering changes when work occurs.
  - Defer: terminal control, async I/O internals, and full formatting grammar.
    Link to format specifications for uncommon options.
  - Examples: implement `Display`; format aligned fields; write to a string
    and a byte sink; use buffered I/O and propagate write or flush errors.

## Error Handling

Distinguish expected failures represented as values from panics. Build on
`Result`, conversions, and closures rather than introducing an error framework
before the standard interfaces are clear.

- (TODO) [Panics] --- `panic!`; assertion failures; unwinding versus aborting;
  panic messages and backtraces.
  - Emphasize: unwinding runs applicable destructors; aborting does not.
    Panics are not the normal return path for expected input or I/O failures.
    A panic is distinct from a compile-time rejection.
  - Defer: panic hooks, unwind safety, and FFI boundaries. Mention
    `catch_unwind` only with its limits, not as general exception handling.
  - Examples: trigger an assertion with a useful message; inspect a backtrace;
    contrast cleanup during unwinding with an abort-configured build.

- (TODO) [`Result<T, E>`] --- successful and error values; borrowed inspection;
  common `Result` methods; closure-taking combinators; fallback behavior.
  - Emphasize: `map`, `map_err`, and `and_then` transform different parts of
    the result. Eager fallback arguments are evaluated before the method call;
    closure-based alternatives can defer work until it is needed.
  - Defer: propagation syntax to `?` and custom error composition to the next
    topics. Keep `unwrap` and `expect` visibly separate from recovery.
  - Examples: parse and transform a value; attach context to an error;
    chain fallible steps; compare `unwrap_or` with `unwrap_or_else`.

- (TODO) [The `?` Operator] --- propagation with `Result` and `Option`;
  compatible return types; error conversion; conversion between absence
  and failure; brief recognition of other supported carriers.
  - Emphasize: `?` returns early from its enclosing function or closure.
    It does not automatically turn `Option` into `Result` or the reverse;
    `Result` propagation can convert compatible errors through `From`.
  - Defer: unstable custom `Try` implementations and residual machinery.
    Briefly identify standard carriers such as `ControlFlow` without treating
    `?` as an operator available to every enum.
  - Examples: propagate parsing failures; return early on `None`; use
    `ok_or_else` to turn missing data into an error and `.ok()` to discard one.

- (TODO) [Custom Errors] --- `std::error::Error`; `Display`; error enums;
  source errors; `From` conversions; propagation across error types.
  - Emphasize: error types should preserve distinctions callers need.
    `Display` provides a message; `source` exposes an underlying cause.
    Conversion should not silently discard useful failure information.
  - Defer: third-party derive and reporting crates until the explicit
    implementation is understood; label any later crate as external.
  - Examples: combine parsing and I/O failures in an enum; implement
    `Display`, `Error`, and `From`; propagate both failures with `?`.

## Standard Collections

For each collection, cover creation, lookup, insertion, update, removal, and
consumption. State type-argument bounds, operation costs, allocation behavior,
ordering guarantees, and restrictions while borrows are live. Explain where
`Eq`, `Hash`, and `Ord` must agree; defer iterator-based APIs until iterators.

- (TODO) [`Vec`] --- growable contiguous storage; indexing and `get`;
  push and pop; insertion and removal; length versus capacity; slices.
  - Emphasize: growth can reallocate, so mutations cannot invalidate live
    element references. Indexing can panic; `get` returns an option. End
    insertion is amortized constant time; shifting elements is linear.
  - Defer: raw allocation APIs and unsafe initialization. Revisit bulk
    processing and collection construction with iterators.
  - Examples: reserve capacity; update through a checked mutable lookup;
    compare `remove` with `swap_remove`; pass contents as a slice.

- (TODO) [`VecDeque`] --- a growable ring buffer; front and back operations;
  indexed access; queue behavior; split storage and `make_contiguous`.
  - Emphasize: efficient operations at both ends do not imply that all
    elements occupy one contiguous slice. Middle insertion or removal can
    move elements; borrowed views restrict mutation.
  - Defer: ring-buffer implementation details and concurrent queues.
    Compare with `Vec` through the workload rather than a universal ranking.
  - Examples: enqueue at the back and dequeue at the front; inspect the two
    slices; make storage contiguous before using a slice-based operation.

- (TODO) [`HashMap`] --- key-value lookup; insertion and replacement; removal;
  the entry API; capacity; borrowed-key lookup; `Eq` and `Hash` bounds.
  - Emphasize: equal keys must hash equally; changing a stored key's identity
    through interior mutability violates the map's contract. Iteration order
    is unspecified, and expected constant-time lookup is not a worst-case bound.
  - Defer: custom hashers and iterator-based traversal. Connect borrowed-key
    lookup to the earlier `Borrow` contract.
  - Examples: look up a `String` key using `&str`; update a count with `entry`;
    replace and remove values; show that insertion can change capacity.

- (TODO) [`BTreeMap`] --- ordered key-value storage; lookup; insertion;
  updates; removal; entry access; ordered and range-oriented queries.
  - Emphasize: keys use `Ord`, and iteration follows key order rather than
    insertion order. Core searches and updates are logarithmic; this tree
    does not expose a vector-like capacity reservation model.
  - Defer: node layout and iterator-based range traversal. Revisit range
    queries after the iterator interface is established.
  - Examples: insert keys out of order; retrieve the first and last entries;
    update through `entry`; compare an ordered-key use case with `HashMap`.

- (TODO) [`HashSet`] --- unique values; membership; insertion; removal;
  replacement; capacity; `Eq` and `Hash`; set operations.
  - Emphasize: uniqueness follows equality, not object identity. Membership
    has expected constant-time cost, not a worst-case guarantee; ordering is
    unspecified. Hashing and equality must remain consistent while stored.
  - Defer: iterator-returning union, intersection, and difference APIs until
    iterators. Contrast replacement with simply inserting an equal value.
  - Examples: reject duplicate values; check and remove membership;
    use borrowed text for lookup; retrieve a stored representative with `get`.

- (TODO) [`BTreeSet`] --- ordered unique values; membership; insertion;
  removal; endpoint access; `Ord`; range and set operations.
  - Emphasize: ordering determines equivalence for set membership and must
    agree with equality. Searches and updates are logarithmic; ordered
    storage is not insertion-order preservation.
  - Defer: iterator-based ranges and set algebra. Keep tree implementation
    details and mutable-key hazards separate from ordinary API use.
  - Examples: insert unsorted values; remove the smallest value; compare
    membership and endpoint access with the corresponding `HashSet` use case.

- (TODO) [`BinaryHeap`] --- priority queues; `Ord`; push; peek; pop;
  capacity; max-heaps; min-heaps with `Reverse`; guarded priority updates.
  - Emphasize: only the greatest element is guaranteed at the top; ordinary
    traversal is not sorted. Peek is constant time; pop is logarithmic;
    insertion cost also depends on sifting and possible allocation.
  - Defer: specialized scheduling policies and custom heap implementations.
    Explain `peek_mut` without implying arbitrary interior mutation is safe
    for heap ordering.
  - Examples: process priorities from greatest to least; use `Reverse` for
    smallest-first order; update the top through the supported guard API.

- (TODO) [Iterator Adapters] --- lazy adapters such as `map`, `filter`, and
  `zip`; `collect`; short-circuiting methods; fallible iteration; returning
  iterators; revisiting collection traversal APIs.
  - Emphasize: adapters generally do no work until consumed. Iterator item
    types determine whether elements are borrowed or moved; the collection
    and target type determine ordering and `collect` behavior.
  - Defer: custom optimization, streaming abstractions, and async streams.
    Revisit every preceding collection's traversal, range, and set APIs on
    top of the iteration-basics topic from the basics section.
  - Examples: compare `iter`, `iter_mut`, and `into_iter`; filter and
    collect; collect results with early failure; return `impl Iterator`
    borrowing data.

## Interior Mutability and Shared Ownership

Separate shared ownership from permission to mutate. Start with single-threaded
APIs and show where checks move from compile time to runtime.

- (TODO) [Interior Mutability] --- `Cell<T>`; `RefCell<T>`;
  mutation through shared references; runtime borrow guards;
  fallible and panicking borrow operations.
  - Emphasize: interior mutability does not remove access rules. `Cell`
    changes values without handing out ordinary interior references;
    `RefCell` enforces shared-versus-exclusive borrowing at runtime.
  - Defer: unsafe internals and thread-safe synchronization. Explain that
    keeping a guard alive keeps its runtime borrow active.
  - Examples: update a counter through `&self` with `Cell`; mutate a value
    through `RefCell`; compare a borrow panic with `try_borrow_mut` failure.

- (TODO) [Shared Ownership] --- `Rc<T>`; reference counting; cloning handles;
  shared access; strong counts; destruction of the last strong owner.
  - Emphasize: cloning an `Rc` shares its allocation rather than cloning `T`.
    Shared ownership does not itself permit mutation of the contents, and
    `Rc` is not a cross-thread ownership mechanism.
  - Defer: weak references, copy-on-write, and atomic reference counting.
    Use counts to explain examples, not as synchronization or access control.
  - Examples: give two owners the same value; inspect pointer identity;
    drop handles in sequence and observe when the contained value is dropped.

- (TODO) [Combining Ownership and Mutability] --- `Rc<Cell<T>>`;
  `Rc<RefCell<T>>`; shared handles; mutation; borrow-guard scopes.
  - Emphasize: the outer `Rc` shares ownership; the inner cell controls
    mutation. All clones reach the same contents, and conflicting `RefCell`
    borrows can fail even when they originate through different handles.
  - Defer: thread-safe equivalents and complicated graph designs. Keep guard
    lifetimes short and visible rather than hiding them in nested expressions.
  - Examples: share a counter with `Rc<Cell<_>>`; edit shared text through
    `Rc<RefCell<_>>`; show and fix a conflict between two cloned handles.

- (TODO) [Reference Cycles] --- strong `Rc` cycles; unreachable allocations;
  reference counting versus tracing garbage collection.
  - Emphasize: reference counts track strong links, not reachability from
    the rest of the program. A cycle can keep every count nonzero after all
    external owners disappear; memory safety does not imply freedom from leaks.
  - Defer: tracing collector implementation and general graph ownership
    frameworks. Introduce weak links as the next topic's remedy.
  - Examples: construct a two-node strong cycle; log missing destructor
    calls after external handles are dropped; contrast an acyclic chain.

- (TODO) [`Weak<T>`] --- `Rc::downgrade`; fallible `upgrade`; strong versus
  weak counts; value lifetime versus allocation lifetime; non-owning links.
  - Emphasize: a weak handle does not keep `T` alive. Upgrading can fail after
    the last strong owner is gone, while weak handles can keep allocation
    bookkeeping alive without retaining the destroyed value.
  - Defer: cross-thread weak pointers to `Arc`. Avoid treating a weak link as
    an always-valid reference or silently unwrapping upgrades.
  - Examples: use strong child links and a weak parent link; upgrade while
    the parent lives; drop the parent and handle a failed upgrade.

- (TODO) [Unique Access and Clone-on-Write] --- `Rc::get_mut`;
  `Rc::make_mut`; weak-pointer treatment; mutable handles versus mutable
  contents; uniqueness and cloning.
  - Emphasize: `get_mut` requires no other strong or xweak handles.  `make_mut`
    clones when other strong owners exist; with only weak handles, it can detach
    them without cloning the value. Neither is interior mutability.
  - Defer: `Arc` and atomics to threads; `Cow` to advanced topics. A mutable
    reference to one `Rc` handle does not make all shared contents uniquely
    owned.
  - Examples: mutate a unique allocation; trigger copy-on-write with a second
    strong handle; observe failed weak upgrades after weak handles are detached.

## Threads

Make ownership transfer and shared access explicit before adding
synchronization.  Use stable standard-library APIs first; distinguish
memory-safety guarantees from scheduling, progress, and application correctness.

- (TODO) [Threads] --- OS threads; `spawn`; `move` closures; joining;
  scoped threads; `Send`; `Sync`; auto traits.
  - Emphasize: `Send` permits transfer between threads; `Sync` permits shared
    references across threads. `move` does not make non-`Send` data transferable
    or turn a borrowed reference into `'static` data. Scoped threads can borrow.
  - Defer: manual unsafe auto-trait implementations and scheduler details.
    Contrast spawn's bounds with scoped lifetimes rather than prescribing
    `'static` for every threaded program.
  - Examples: move owned work into a thread; inspect a join result; borrow
    local data with `thread::scope`; explain a rejected `Rc` transfer.

- (TODO) [Synchronization] --- `Arc`; `Mutex`; `RwLock`; channels;
  sharing versus transferring ownership; lock guards; poisoning; deadlocks.
  - Emphasize: `Arc<T>` provides atomic shared ownership, not arbitrary
    thread-safe access to `T`. Lock scopes affect contention and deadlocks;
    poisoning is a warning about interrupted invariants, not memory protection.
  - Defer: async locks and channels to async Rust; elaborate lock-free
    structures. Explain each primitive's bounds and failure behavior separately.
  - Examples: share a counter with `Arc<Mutex<_>>`; read with `RwLock`;
    transfer owned messages through a channel; demonstrate a lock-order hazard.

- (TODO) [Atomics] --- atomic loads and stores; read-modify-write operations;
  `AtomicBool`; `AtomicUsize`; `Ordering`; acquire and release;
  relaxed operations; sequential consistency; compare-and-exchange.
  - Emphasize: an atomic operation protects its own value; it does not make
    a multi-step invariant atomic. Relaxed ordering does not publish unrelated
    data, and stronger ordering does not repair an incorrect algorithm.
  - Defer: lock-free data structures, memory reclamation, and a full
    memory-model treatment. Prefer locks for compound state unless an atomic
    design is clear.
  - Examples: count independent events with relaxed `fetch_add`; signal
    shutdown with an atomic flag; illustrate release/acquire publication using
    only atomic payload fields and safe APIs.

## Async Rust

Examples use stable Rust and safe APIs; no unsafe implementation is required.
`std` provides async building blocks, not a general-purpose task runtime.
Label Tokio and any other ecosystem crates as external dependencies, and show
the Cargo dependencies and features required by examples.

- (TODO) [Futures and Await] --- futures as values representing computations;
  `async fn`; `async { ... }`; `.await`; lazy execution; sequential awaits;
  runtime entry-point scaffolding.
  - Emphasize: calling an async function creates a future without executing
    its body; `.await` is a potential suspension point, not necessarily a
    yield. `async` does not automatically create a task or thread, or make
    blocking operations nonblocking.
  - Defer: polling mechanics, pinning, concurrent composition, and spawning.
    Initially explain an external runtime's entry-point macro only enough to
    run examples.
  - Examples: create a future, print a message, then await it to demonstrate
    execution order; await two operations sequentially.

- (TODO) [State Across Await] --- retained locals and borrows; borrowing across
  `.await`; async functions borrowing their arguments; async block capture;
  `async move`; ownership and destruction of captured values.
  - Emphasize: suspension retains the state needed to resume. Borrowing across
    `.await` is supported; moving a reference into a future does not make its
    referent owned or extend its lifetime.
  - Defer: spawn-specific `Send` and `'static` requirements. End by motivating
    pinning: a suspended future may retain references into its own stored
    state.
  - Examples: borrow a caller-owned `String` across an await; contrast an
    async block borrowing a string with one taking ownership through
    `async move`.

- (TODO) [Polling and Wakeups] --- `Future`; associated `Output`;
  `Future::poll`; `Poll::Ready` and `Poll::Pending`; `Context`; `Waker`;
  executors; futures versus runtime tasks versus OS threads.
  - Emphasize: an executor polls task futures; nested awaits drive child
    futures within the enclosing task. Pending operations arrange wakeups
    when progress may be possible. A wakeup requests another poll, not a
    guarantee of readiness; polling should return promptly.
  - Defer: executor implementation and low-level waker machinery. Show the
    real `poll` signature, treating `Pin<&mut Self>` as a forward reference
    resolved in the next topic.
  - Examples: trace poll, pending, wake, and repoll events; optionally wrap
    an existing future with safe `std::future::poll_fn` to log polling.

- (TODO) [Safe Pinning] --- address-sensitive future state; `Pin<P>`;
  `Unpin`; moving pointers versus moving pointees; `std::pin::pin!`;
  `Box::pin`; `Pin::as_mut`; `Pin::new` for `Unpin` pointees; allocation
  versus pinning.
  - Emphasize: motivate pinning through suspended futures, not a hand-written
    self-referential type. `Unpin` means a type does not require pinning's
    movement restriction, not that a particular value is currently unpinned.
    Local pinning needs no heap allocation; ordinary heap allocation does
    not itself impose pinning. Ordinary `.await` usually handles pinning
    without explicit user intervention.
  - Defer: unsafe pin constructors, structural pinning, and pin-projection
    implementations.
  - Examples: locally pin an async block; contrast with `Box::pin`; obtain
    another pinned mutable reborrow with `.as_mut()`. Keep all code safe.

- (TODO) [Concurrent Composition] --- sequential versus concurrent awaits;
  composing futures within one task; concurrency versus parallelism;
  external combinators such as Tokio's `join!`.
  - Emphasize: concurrency does not require spawning. Joined branches can
    borrow local data, but a blocking branch prevents the other branches
    in that task from progressing.
  - Defer: spawning to the next topic; races, timeouts, and early-exit
    combinators until cancellation.
  - Examples: compare sequential timers with joined timers; concurrently
    run operations borrowing separate local values.

- (TODO) [Spawning and Bounds] --- runtime task creation; join handles;
  task results and failures; common spawn APIs' `Send` and `'static`
  requirements; `async move`; local spawning of `!Send` futures.
  - Emphasize: a spawned task can progress without awaiting its handle.
    Inspect captured initial state and state retained across suspension
    when diagnosing `Send` failures. `'static` constrains borrowed
    dependencies, not how long a task must run. `async move` guarantees
    neither `Send` nor `'static`; common APIs also constrain task outputs.
  - Defer: detailed task cancellation. Briefly flag that dropping a Tokio
    join handle detaches rather than cancels its task. Contrast local
    spawning, which removes the `Send` requirement but not necessarily
    `'static`.
  - Examples: convert a borrowing composition example into spawned tasks
    using owned values; demonstrate why moving a borrowed reference is
    insufficient; compare `Rc`-based local work with suitable `Arc`-based
    shared state.

- (TODO) [Blocking Work] --- blocking I/O; CPU-heavy work; runtime worker
  threads; blocking pools; Tokio's external `spawn_blocking`; choosing
  OS threads versus async tasks.
  - Emphasize: an async function can still block its executing thread.
    Separate cooperative async waiting from blocking execution, and bound
    CPU-heavy concurrency rather than assuming a blocking pool supplies
    an appropriate CPU limit.
  - Defer: scheduler internals and performance tuning. Flag that already
    started `spawn_blocking` work generally cannot be aborted.
  - Examples: contrast an async timer with a blocking sleep; move a
    synchronous operation into `spawn_blocking` and await its result.

- (TODO) [Cancellation and Drop] --- dropping unfinished futures; races;
  timeouts; Tokio's external `select!`; cancellation safety; task abortion;
  cooperative shutdown; cleanup and external side effects.
  - Emphasize: dropping a future stops further polling of that future,
    not necessarily an external operation it initiated, and does not
    roll back effects. Distinguish dropping an owned future, dropping a
    borrow of a retained future, and dropping a task's join handle.
    Pinning does not prevent destruction.
  - Defer: complex shutdown protocols; introduce only enough cooperative
    cancellation to support the next topic.
  - Examples: race an operation against a timer; retain a pinned future
    across repeated selections; contrast dropping a Tokio join handle
    with aborting the task and awaiting its termination.

- (TODO) [Backpressure and Limits] --- bounded async channels; producer
  suspension; queue capacity; bounded in-flight work; worker limits;
  channel closure and draining; pipeline shutdown.
  - Emphasize: identify exactly where producers wait and what resource
    each bound limits. A bounded queue does not by itself bound all tasks
    or memory; spawning unlimited tasks that wait for capacity merely
    moves the backlog elsewhere.
  - Defer: elaborate streaming abstractions and application-specific
    overload policies.
  - Examples: build a bounded producer-consumer pipeline; introduce a
    concurrency limit before spawning additional work; close the input
    and await completion of accepted work.

- (TODO) [Async Closures] --- `async |x| { ... }`;
  `async move |x| { ... }`; ordinary closures returning async blocks;
  `AsyncFn`, `AsyncFnMut`, and `AsyncFnOnce`; reusable async callbacks.
  - Emphasize: distinguish the closure's captures from the state of each
    future it produces. Genuine async closures can support borrowing
    from their captures; they are not interchangeable with ordinary
    closures returning async blocks in every case.
  - Defer: advanced generic callback bounds beyond the examples' needs;
    connect borrowed callback interfaces to the earlier higher-ranked
    trait bounds topic.
  - Examples: accept and await an async callback; repeatedly invoke an
    async closure that mutably borrows captured state.

- (TODO) [Async Traits and Dyn] --- native async functions in traits;
  generic callers; returned-future `Send` bounds; dyn-compatibility
  limitations; explicitly boxed and erased futures.
  - Emphasize: a `Send` implementor does not imply a `Send` returned
    future. Async methods are not directly dyn-dispatchable; excluding
    a method with `where Self: Sized` makes it unavailable through the
    trait object. Boxing is an explicit design choice, not an inherent
    requirement of async trait methods.
  - Defer: procedural-macro solutions until the explicit representation
    is understood; label any such crate as an external dependency.
  - Examples: implement a trait with an async method and call it
    generically; contrast a dyn-compatible method returning
    `Pin<Box<dyn Future<Output = T> + Send + '_>>`. Explain allocation,
    pinning, type erasure, thread-transfer bounds, and borrowing lifetime
    separately.

## Macros

Explain macro use and small implementations after ordinary functions, traits,
and generics. Distinguish compile-time syntax generation from runtime function
calls; show crate requirements for procedural macros.

- (TODO) [Declarative Macros] --- `macro_rules!`; match arms; fragment
  specifiers; repetitions; separators; expansion; hygiene; visibility.
  - Emphasize: macros match and expand syntax rather than accept ordinary
    typed parameters. Expansion must still type-check; repeated use of an
    expression in the expansion can evaluate that expression more than once.
  - Defer: recursive token processing, advanced hygiene rules, and
    edition-specific matcher changes. Prefer a function when syntax generation
    is unnecessary.
  - Examples: write a macro with two forms; accept a comma-separated list;
    support a trailing comma; contrast single and repeated argument evaluation.

- (TODO) [Procedural Macros] --- derive, attribute, and function-like macros;
  token streams; `proc-macro` crates; generated code; diagnostics.
  - Emphasize: procedural macros run during compilation and produce Rust
    syntax; they are not ordinary runtime functions or a general interface
    for querying inferred types. Generated code remains subject to Rust's rules.
  - Defer: production parsers, complex derives, and compiler internals.
    Label `syn`, `quote`, and other helper crates as external dependencies
    and show their required Cargo configuration.
  - Examples: create a minimal derive in a separate crate; consume it from
    another crate; inspect the generated implementation; report invalid input
    with a useful compile error.

## Advanced Topics

Use focused examples for application-facing APIs and development tools.
Distinguish standard-library facilities from external crates, and show the
Cargo dependencies, features, or tool installation each example needs.

- (TODO) [Files] --- `File`; opening and creation; reading and writing;
  `Path` and `PathBuf`; `OsStr` and `OsString`; non-Unicode paths.
  - Emphasize: paths are not necessarily UTF-8 strings. Opening can fail,
    creation can truncate, and checking a path before using it does not remove
    filesystem races. Closing by drop is not the same as durable storage.
  - Defer: memory mapping, platform-specific handles, and atomic replacement
    protocols. Preserve path types rather than forcing lossy conversion.
  - Examples: join path components; read and write a file with `Result`;
    choose `OpenOptions` deliberately; display a non-Unicode path safely.

- (TODO) [Environment and CLI] --- command-line arguments; environment
  variables; `args_os`; `var_os`; missing values; Unicode conversion.
  - Emphasize: arguments and environment values may be non-Unicode.
    Distinguish missing input from invalid input; process-global environment
    mutation is not a safe configuration mechanism for running worker threads.
  - Defer: full command-line parsers and shell-specific quoting. Keep examples
    read-only with respect to the process environment and label parser crates
    as external if introduced.
  - Examples: accept a path through `args_os`; read an optional setting;
    validate a numeric value; print usage and return a failure exit status.

- (TODO) [Clone-on-Write and Lazy Initialization] --- `Cow`; `OnceCell`;
  `OnceLock`; `LazyCell`; `LazyLock`; borrowed versus owned data;
  one-time initialization; thread-safe versus single-threaded cells.
  - Emphasize: `Cow` delays owning or cloning data; it is not a one-time
    initialization primitive. Once cells accept initialization later, while
    lazy cells store an initializer. Synchronization guarantees differ by type.
  - Defer: custom lazy-state machines and detailed reentrant-initialization
    behavior. Explain failure and panic behavior for the specific API used
    rather than assuming all four cells recover identically.
  - Examples: return unchanged text as borrowed `Cow` and changed text as
    owned; initialize a field on first use; share a lazily initialized lookup
    table through `LazyLock`.

- (TODO) [Processes] --- `Command`; arguments; environment overrides;
  status and output; child handles; standard streams; waiting;
  graceful shutdown and forced termination.
  - Emphasize: command arguments do not pass through a shell unless one is
    explicitly launched. Dropping a child handle does not kill or wait for the
    process; undrained pipes can block progress. Shutdown is platform-specific.
  - Defer: process supervision, platform-specific signal handling, and async
    subprocess APIs. Distinguish requesting graceful exit from forcing a kill.
  - Examples: run a command with separate arguments; inspect exit status;
    capture output; request shutdown through an agreed input protocol and wait.

- (TODO) [Date and Time] --- `std::time`; `Duration`; `Instant`;
  `SystemTime`; external `time`, `chrono`, and `jiff` crates.
  - Emphasize: elapsed-time measurement and civil time are different problems.
    Wall clocks can change, and a UTC offset is not a time-zone rule set.
    Calendar arithmetic must account for ambiguous or nonexistent local times.
  - Defer: exhaustive calendar APIs and time-zone database internals. Choose
    one external crate for each example and state the features it needs.
  - Examples: measure elapsed work with `Instant`; handle `SystemTime`
    conversion errors; parse a timestamp; resolve a local time using explicit
    time-zone rules.

- (TODO) [Property Testing] --- external `proptest` and `quickcheck` crates;
  generated inputs; invariants; shrinking; reproducible failures.
  - Emphasize: properties describe behavior across inputs rather than a few
    expected outputs. A failing property may expose a bad property or generator,
    not only a bad implementation; shrinking helps isolate the cause.
  - Defer: complex state-machine models and custom shrinking strategies.
    Keep regression examples alongside generated tests.
  - Examples: test a serialization round trip; generate bounded inputs;
    inspect a shrunk counterexample; retain the failing case as a regression.

- (TODO) [Fuzz Testing] --- external `cargo-fuzz`, `bolero`, and `afl.rs`;
  fuzz targets; input corpora; coverage feedback; crash reproduction.
  - Emphasize: fuzz targets should accept arbitrary input without unexpected
    panics, hangs, or unsafe behavior. Toolchain requirements vary; `cargo-fuzz`
    normally requires nightly and remains an optional exception to stable
    examples.
  - Defer: sanitizer internals, nightly setup details, and distributed fuzzing.
    Distinguish byte-level fuzz inputs from structured property-test generation.
  - Examples: wrap a parser in a fuzz target; seed a small corpus; reproduce
    and minimize a failure; turn the discovered input into a normal stable test.

- (TODO) [Benchmarking] --- external `criterion` and `divan`; benchmark
  harnesses; release builds; warmup; repeated samples; `black_box`;
  setup costs and measurement boundaries.
  - Emphasize: a benchmark measures a workload under specific conditions,
    not a universal property of an API. Prevent optimized-away work and
    separate setup from the operation being compared; report variability.
  - Defer: advanced statistical analysis and cross-machine comparisons.
    Avoid interpreting a single timing or debug build as a performance result.
  - Examples: compare two implementations with identical inputs; keep
    allocation inside or outside the timed region deliberately; inspect a
    regression report across repeated runs.

- (TODO) [Profiling] --- external flamegraph tooling such as
  `cargo-flamegraph`; optimized builds with symbols; sampling;
  call stacks; identifying CPU hotspots.
  - Emphasize: profiles attribute observed work, not necessarily its cause.
    Wide flamegraph frames reflect sampled stack frequency, not a timeline;
    CPU samples can miss time spent blocked or waiting.
  - Defer: kernel tracing and platform-specific profiler internals. State
    installation, permissions, and operating-system requirements for the tool.
  - Examples: profile a representative workload; identify a hot call path;
    change one bottleneck and remeasure with the earlier benchmark.

- (TODO) [Allocation Cost Measurement] --- external `stats_alloc`, `dhat`,
  and `cap`; allocation counts; allocated bytes; live and peak usage;
  measurement scope; allocator instrumentation.
  - Emphasize: allocation count, total bytes, live heap, and process memory
    are different measurements. Instrumentation can perturb timing, and
    measurements must isolate setup and unrelated concurrent activity.
  - Defer: custom allocator implementations and platform heap internals.
    Show safe integration scaffolding and state each tool's capabilities
    rather than treating the tools as interchangeable.
  - Examples: measure repeated vector growth; compare reserved capacity;
    inspect retained allocations after a workload; rerun without instrumentation
    before drawing timing conclusions.

- (TODO) [`PhantomData`] --- unused type and lifetime parameters;
  zero-sized markers; auto traits; ownership and drop checking; variance.
  - Emphasize: `PhantomData` stores no `T`, but its type affects compiler
    reasoning. Different marker forms express different relationships;
    a marker does not create a borrow, allocation, or runtime safety check.
  - Defer: unsafe container implementation and a full variance treatment.
    Keep examples safe while explaining why marker choices matter to APIs.
  - Examples: distinguish units with `PhantomData<Unit>`; tie a marker to a
    borrowed lifetime through a safe constructor; compare auto-trait behavior
    of owning and non-owning marker forms.

## Projects

Combine earlier topics without introducing large frameworks. Keep the first
version small, state its limitations, and add tests before extending behavior.
Label external dependencies and distinguish teaching examples from production
implementations.

- (TODO) [Simple Grep] --- ownership; borrowing; iterators; error handling;
  command-line arguments; paths; buffered input and output.
  - Emphasize: stream input rather than requiring the whole file in memory.
    Decide whether matching operates on UTF-8 text or arbitrary bytes, and keep
    diagnostic output separate from matching lines on standard output.
  - Defer: regular expressions, recursive traversal, and platform-specific
    performance tuning. State the text and argument limitations of version one.
  - Examples: search one file for a literal; test matching independently of
    I/O; report invalid input and missing files; handle write failures.

- (TODO) [Single-Threaded HTTP Server] --- `std::net`; TCP listeners and
  streams; basic HTTP framing; request and response I/O; structured logging.
  - Emphasize: TCP supplies bytes, not message boundaries. Partial reads and
    writes are normal; bound request size and reading time. State the supported
    HTTP subset rather than implying a small parser implements the full
    protocol.
  - Defer: TLS, full HTTP compliance, persistent connections, and concurrency.
    Label any structured-logging crate as an external dependency.
  - Examples: handle one bounded request at a time; send a response with an
    explicit body length; close the connection; test fragmented input and log
    request outcomes without recording sensitive contents.

- (TODO) [Multi-Threaded HTTP Server] --- concurrency; synchronization;
  worker pools; bounded queues; shared state; overload handling; shutdown.
  - Emphasize: limit workers and queued connections rather than spawning
    without bounds. Keep lock scopes short and define what happens to accepted
    work during shutdown; thread safety alone does not prevent deadlocks.
  - Defer: production HTTP infrastructure, async rewrites, and advanced load
    balancing. Retain the first server's protocol limits explicitly.
  - Examples: hand connections to a fixed worker pool; reject or delay excess
    work; share a synchronized request counter; stop accepting, drain queued
    work, and join the workers.

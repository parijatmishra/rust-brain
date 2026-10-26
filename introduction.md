# Introduction

This book is a "Rust Refresher", written using `mdbook`. The text is intended to
be very brief, and supported by lots of sample code.

PROBLEM:

- Me: seasoned programmer, have learnt/used Rust, forgotten due to disuse. Need
  quick refresher before starting new project.
- Reading Rust books from scratch is time consuming: much text is spent on
  explaining concepts that I know but have fortted then details of.

GOALS:

- List of Rust concepts, with brief explanations.
- Lots of code examples to remind me of syntax
- Concepts ordered by least forward references.

NON-GOALS:

- Programming primer: assume knowledge of basic programming concepts, focus only
  on what differentiates Rust.
- Reference manual: cover the most useful cases, get me up to speed, and provide
  links to reference manuals.

This book covers the topics below. Topics are intended to be read in order.

Each list item below has a status code of `DONE` (topic has been developed),
`WIP` (topic is being developed), `TODO` (topic to be developed in future). The
format of a list item entry is:

```text
- (STATUS) [Title](linkto-source-md) --- brief description.
```

The link `(...)` may not be present or may be empty for `TODO` items. If
something is marked `WIP`, the link must be present and the linked file must
exist.

## Currently out of scope

This document will mention syntax related to the following, since I may
encounter it and puzzle over them, but not describe the concepts in detail.

- Rust edition 2024: differences from previous editions
- Unsafe Rust and FFI
- Cross-compilation
- Nightly features and experimental APIs.

## References

- [Standard library](https://doc.rust-lang.org/std/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Cargo Reference](https://doc.rust-lang.org/cargo/reference/)
- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)

## Bootstrap

NOTE: in this section, code examples intentionally contain forward references to
concepts introduced later, in order for the sample code to be minimally
complete.

- (TODO) [Cargo] --- crates; Cargo packages; Cargo workspaces.
- (TODO) [Modules] --- modules; `pub`; paths; `use` and its forms.
- (TODO) [Documentation] --- non-doc and rustdoc comment styles.
- (TODO) [Debugging] --- `assert!`/`assert_eq!`; `println!`/`eprintln!`;
  `#[derive(Debug)]`/`{:?}/{:#?}`.
- (TODO) [Testing] --- integration tests; unit tests; doctests.
- (TODO) [Prelude] --- identifiers that are imported automatically.

## Basics

- (TODO) [Values and Identifiers] --- `let`; `let mut`; `const`; `static`; type
  annotations; type inference; shadowing. Briefly describe `statuc mut` syntax.
- (TODO) [Primitive Types] --- bools; ints; floats; char; str; tuple; array;
  unit; reference. Briefly describe raw pointer and never (`!`) syntax.
- (TODO) [Expressions vs Statements] --- expressions versus statements; block
  expressions.
- (TODO) [Basics Operators] --- arithmetic; comparison; boolean
  short-circuiting; bitwise; assignment; precedence; integer overflow;
  checked/wrapping/saturating methods.
- (TODO) [Type Conversions] --- type casts with `as`; truncation; sign changes;
  safe conversions using `from`. Forward reference to "Conversion Traits".
- (TODO) [Control Flow] --- `if`/`else`; `loop`; `while`; `break`; `continue`;
  labels; loop values. Defer `for` to "Iterators".
- (TODO) [Basic Functions] --- function defintion; return values; early return;
  call syntax (no references, generics, lifetimes, trait objects or closures);
  `const fn`.
- (TODO) [Structs] --- Named-field; tuple; unit structs; inherent `impl`;
  method-call syntax and automatic borrowing/dereferencing; receiver choices;
  associated functions; the newtype pattern.
- (TODO) [Enums] --- enums; inherent methods; `match`; `if let`; `while let`;
  `let ... else`; `let` chains in `if`/`while` conditions.
- (TODO) [Patterns] --- destructuring bindings/parameters; wildcards; rest
  patterns; alternatives; guards; ranges; `@`; refutability; reference patterns;
  match ergonomics.

## Ownership, References, Borrowing

- (TODO) [Ownership and Destruction] --- ownership and mutable ownership; stack
  and heap; move vs copying; drop scopes/order; partial moves; temporary
  lifetimes. Examples using integers and `String`.  Forward reference to
  `Copy`/`Clone`/`Drop`.
- (TODO) [References and Borrowing] --- shared and mutable references; borrowing
  rules; reborrowing; non-lexical lifetimes; reference vs ownership; slices
  (arrays).
- (TODO) [Strings and str] --- `String` vs `str`; UTF-8: bytes versus Unicode
  scalar values versus grapheme clusters; indexing; slicing and why integer
  string indexing is absent.
- (TODO) [Lifetimes] --- borrowed function params/returns that need explicit
  lifetime annotations; elision; borrowed fields; lifetime bounds and `'static`
  lifetimes. Connect annotations to borrow checking rather than implying
  annotations extend the life of a value. Revist temporary lifetimes and drop
  scopes.

## Generics

- (TODO) [Generic Structs, Enums and Functions] --- syntax; inference; explicit
  arguments; const generics; default generic parameters. Defer bounded generics
  to "Traits".
- (TODO) [`Option` and `Result`] --- `Option<T>` and `Result<T>`: definitions,
  pattern-based use, and methods that do not require understanding closure
  parameters.

## Traits and Trait Objects

- (TODO) [Traits] --- trait definitions and implementation; generic bounds;
  `where`; supertraits; default methods; associated functions; method
  disambiguation; blanket implementations; coherence/orphan rules; newtypeas a
  coherence workaround; fully-qualified syntax.
- (TODO) [Associated Types] --- associated types and constants; associated types
  vs generic parameters. Generic assocaited types.
- (TODO) [Derivable Traits] --- `Debug`; `PartialEq` and `Eq`; `PartialOrd` and
  `Ord`; `Clone`; `Copy`; `Hash`; `Default`.
- (TODO) [Conversion Traits] --- `From` and `Into`; `TryFrom` and `TryInfo`.
- (TODO) [Operator Traits] --- operator traits and implementations. Connect to
  operators discussed already in "Basic Operators".
- (TODO) [Borrowing Traits] --- `AsRef`, `AsMut` and `Borrow`.
- (TODO) [`Sized`, `?Sized`] --- sized types and dynamically sized types; why
  slices/trait objects normally appear behind pointers.
- (TODO) [Unique Owning Pointers] --- `Box<T>`; `Deref` and `DerefMut`; defer
  coercions; implementing `Drop`. Revisit method lookup, autoderef/autoref, and
  destruction vs deallocation. Revisit trait objects.
- (TODO) [Trait Objects] --- `dyn Trait`; static vs dynamic dispath; borrowed vs
  boxed objects; `dyn` compatibility; object lifecycle bounds; associated-type
  constraints; restructions on generic methods and other non-dyn-compatible
  interfaces.

## Closures, `Fn*` Traits and Higher-Order Functions

- (TODO) [Closures] --- closure syntax; capture inference; borring vs moving
  captures; `move`.
- (TODO) [`Fn`, `FnMut`, and `FnOnce`] --- Explain that the type of a closure is
  unique and cannot be written explicitly. The `Fn`, `FnMut`, and `FnOnce`
  traits.
- (TODO) [`fn` pointers] --- function items and `fn` pointers; coercions;
  differences from closures and `dyn Fn`.
- (TODO) [Higher-Order Functions] --- argument-position vs return-position `impl
  Trait`; returning closures; choosing genericsbetween /`impl Trait`/`dyn
  Trait`; return-type capture rules and explicit `use<...>` capture bounds.
- (TODO) [Higher-Ranked Trait Bounds] --- higher-ranked trait bounds; borrowed
  callback interfaces as canonical examples.

## Error Handling

- (TODO) [Panics] --- `panic!`; unwinding vs aborting.
- (TODO) [`Result<T>`] --- `Result<T>`; common `Result` methods, including
  those taking closures.
- (TODO) [The `?` operator] -- Use of `?` with `Result`; compatibility; use of
  `?` with `Option`; conversion between `Option` and `Result`. Briefly mention
  other types it can be used with.
- (TODO) [Custom Errors] --- `std::error::Error` and custom error types;
  `Display`; conversion with `From` and `?` across error types.

## Standard Collections

For each type, describe: CRUD operations; requirements on type argument; costs;
capacity/allocation; invalidated borrows; ordering guarantees; consistency
contracts for `Eq`, `Hash`, and `Ord`.

- (TODO) [`Vec`]
- (TODO) [`VecDeque`]
- (TODO) [`Hashmap`]
- (TODO) [`BTreeMap`]
- (TODO) [`HashSet`]
- (TODO) [`BTreeSet`]
- (TODO) [`BinaryHeap`]
- (TODO) [Iterators] --- `Iterator`; `IntoIterator`; ranges; `for` loops;
  shared, mutable and consuming iterators; lazy adapters; `collect` inference;
  short-circuiting; fallible iteration; returning iterators from functions.
  Revist previous collection types and show iterator based APIs.

## Recursive Types

- (TODO) [Recursive Types] --- implementing a `List<T>` and some methods, using
  the concepts described above.

## Interior Mutability and Shared Ownership

- (TODO) [Interior Mutability] --- `Cell`, `RefCell`.
- (TODO) [Shared Owneship] --- `Rc<T>`.
- (TODO) [Combining Ownership and Mutability] --- `Rc<Cell<T>>` and
  `Rc<RefCell<T>>`.
- (TODO) [Reference Cycles] --- why strong `Rc` cycles leak and contrast
  reference counting with tracing garbage collection.
- (TODO) [`Weak<T>`] --- `Rc::downgrade`; fallible upgrade; value versus
  allocation lifetime; using weak links to avoid ownership cycles.
- (TODO) [Unique Access and Clone-on-Write] --- `Rc::get_mut` and
  `Rc::make_mut`; their treatment of weak pointers; distinguish `&mut Rc<T>`
  operations from interior mutability of the contents. Explain why none of these
  are thread-safe (forward reference to `Arc` and atomics).

## Threads

- (TODO) [Threads] --- threads, `move`, joining, scoped threads,
  `Send`/`Sync`, auto traits.
- (TODO) [Synchronization] --- `Arc`, `Mutex`, `RwLock`, channels, sharing
  vs transferring ownership, poisoning and deadlocks. Limintations of `Arc<T>`:
  `Arc<T>` alone does not make arbitrary `T` thread-safe.
- (TODO) [Atomics] --- ???

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

- (TODO) [Declarative Macros]
- (TODO) [Procedural Macros]

## Advanced Topics

- (TODO) [IO] --- `format!`: width/precision/alignment,
  `print!`/`println!`, `write`/`writeln!`, `Display`.
- (TODO) [Files] --- `File`, `Path`/`PathBuf`, non-Unicode input:
  `OsStr`/`OsString`.
- (TODO) [Env] --- CLI args, enviroments variables.
- (TODO) [One-time Lazy Computation] --- `Cow`, `OnceCell`/`OnceLock`,
  `LazyCell`/`LazyLock`.
- (TODO) [Processes] --- spawning, graceful shutdown.
- (TODO) [Date and Time] --- `std::time`, `time`, `chrono`, `jiff`.
- (TODO) [Property testing] --- `proptest`, `quickcheck`.
- (TODO) [Fuzz testing] --- `cargo-fuzz`, `bolero`, `afl.rs`.
- (TODO) [Benchmarking] --- `criterion`, `divan`.
- (TODO) [Profiling] --- `flamegraph`/`cargo-framegraph`.
- (TODO) [Allocation cost measurement] --- `stats_alloc`, `dhat`, `cap`.
- (TODO) [PhatomData] --- auto traits, ownership and drop checking, lifetime and
  variance.

## Projects

- (TODO) [simple grep] --- ownership, borrowing, iterators, errors, CLI,
  and buffered I/O.
- (TODO) [simple-threaded HTTP server] --- `std::net`, basic HTTP framing,
  request/response I/O. structured logging.
- (TODO) [multi-threaded HTTP server] --- concurrency and synchronization.

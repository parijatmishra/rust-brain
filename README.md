# rust-brain

This directory is the source code for personal "Rust Refresher" book, written
using [mdbook](https://rust-lang.github.io/mdBook/). This directory also is a
Cargo virtual workspace, containing code examples referred to in the book and
for quick reference.

PROBLEM:

- Me: seasoned programmer, have learnt/used Rust, forgotten due to disuse. Need
  quick refresher before starting new project.
- Reading Rust books from scratch is time consuming: much text is spent on
  explaining concepts that I know but have fortted then details of.

GOALS:

- List of Rust concepts, with brief explanations.
- Lots of code examples to remind me of syntax.
- Concepts ordered by least forward references.

NON-GOALS:

- Programming primer: assume knowledge of basic programming concepts, focus only
  on what differentiates Rust.
- Reference manual: cover the most useful cases, get me up to speed, and provide
  links to reference manuals.

<!-- ANCHOR: purpose-scope -->
## Purpose and Scope

This book is a Rust refresher built with `mdbook`, for programmers who have used
Rust and need to recall it before starting another project. It keeps
explanations brief, and relies heavily on code examples to remind the user of
syntax and Rust-specific rules without rereading a programming primer.

This book assumes that the reader is familiar with basic programming language
syntax, and has programmed in C, Java, and Python before. It assumes knowledge
of the concepts: types, values, variables and constants, conditionals and loops,
functions, parameters and arguments, return values, closures, concurrency, data
races, locks and deadlocks, testing, benchmarking, profiling, character set and
encoding, dependencies and verions, semver, package manger, build system, POSIX.
It focuses on what distinguishes Rust.

This book focuses on useful and common cases, rather than every language or tool
detail. Links to references are provided where appropriate for easy access to
more details. Here are some authoritative references. Individual topics will
link to appropriate chapters in them when eliding details.

- [The Rust Programming Language](https://doc.rust-lang.org/book/) ---
  affectionately called "The Book", this is a free and high-quality learning
  resource for the Rust language for newcomers; it discusses most the topics
  below in greater details than this refresher.
- [The Rust Standard Library Reference](https://doc.rust-lang.org/std/) ---
  types, constants, traits, macros, and their API contracts.
- [The Rust Language Reference](https://doc.rust-lang.org/reference/) ---
  language syntax and semantics.
- [Cargo Reference](https://doc.rust-lang.org/cargo/reference/) --- packages,
  dependencies, features, workspaces, and build commands.
- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)
  --- edition-specific changes and migration guidance.

The following topics are out of scope:

- Rust editions before 2024.
- Experimental and nightly Rust features.
- Unsafe Rust and FFI. The syntax for unsafe blocks, raw pointers, and foreign
  declarations are mentioned in the appropriate topics, without going into
  details, so the reader can recognize such syntax when reading code, and decide
  for themselves whether they want to learn the details.
- Cross compilation.
<!-- ANCHOR_END: purpose-scope -->

## Build

Pre-requisites:
- Install [Rust and Cargo](https://rust-lang.org/tools/install/).
- Install the other tools by running [`tools_install.sh`](./tools_install.sh).
- Check if all tools are present and working by running
  [`tools_check.sh`](./tools_check.sh).

Run [`code_build.sh`] to build and test only the code, and [`book_build.sh`] to
test and build both the code and the book. Generated book will be at
[`html/index.html`](html/index.html).

## Develop

The book's source code lives in `*.md` files, rooted at
[`SUMMARY.md`](./SUMMARY.md). If you add/modify the book or Rust source code,
run `book_build.sh`. Resolve any `mdbook-lint` warnings or errors, and any
issues with included code samples.  Note: the `mdbook-lint` plugin enforces
several markdown best practices, and they should be adhered to.

For convenience, [`watch.sh`] monitors the directory for file/directory changes
and runs `book_build.sh` automatically.

### Cargo Workspace Rules

- If you create a new Cargo package, ensure `edition = "2024"`.
- If you add/remove a binary to any cargo package, update `code_build.sh`,
  section `run binaries`. Similarly for `examples` and `benches`.
- AI agents: don't gratuituously add examples and benches to packages unless
  explicitly requested.

### Book '*.md' Rules

#### Code Samples

Code examples should live in the filesystem, as part of buildable Cargo
packages, which should be members of the Cargo workspace in the top-level
directory. Exceptions:

- A few short lines of code that are variations of a theme captured in a Cargo
  package's code and CANNOT easily be illustrated in that package itself.

- Code that would not compile, to show common "gotchas" with syntax or usage.
  Such code snippets should be illustrated in the topci's '*.md' file as:

    ````text
      ```rust,compile_fail
      let x: u32 = -1; // this won't compile
      ```
    ````

  or

    ````text
      ```rust,no_run
      panic!("oops!"); // compiles, but fails at runtime
      ```
    ````

For a topic, create one (or more, if absolutely required) member Cargo packages
and put illustrative code there. Identify parts of code that should be included
as illustrations in the topic's '*.md' file, and surround them with an mdbook
"anchor" comment with a file-unique "tag". In the '*.md' file, use code similar
to the following to include code snippets.

  ````text
  File: _relative/path/to/file.rs_.

  {{#include relative/path/to/file.rs:tag}}
  ````

Do not use line-numbers to include parts of files. Either use anchor tags, or
include the whole file. Include complete files (i.e., `\{\{#include
relative/path/to/file.rs}})` only if the point is to show the structure of the
entire file. In such cases, the file should be <20 lines long.

Do not share cargo packages between topics.

After creatign/modifying Rust code in the filesystem, run `./code_build.sh`.
Resolve any errors or warnings.

#### Topic Files

Topic file names should be all lower case, contain only ASCII alphanumeric
characters and the underscore ('_') character.

After creating/modifying a '*.md' file, run `./book_build.sh`.

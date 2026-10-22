# rust-brain

This project is a personal "Rust Refresher" book, written using [mdbook].

PROBLEM:
- I am seasoned programmer, have gone through "The Rust Programming Language"
  book, and written serious Rust programs. I've read other topic specific books
  on Rust.
- Due to infrequent use of Rust, I've forgotten much of what I learnt. Every now
  and then, I need/want to write/read new Rust code---at that point, I need a
  refresher. I don't want to read each book I've read before from scratch: they
  spend a lot of time on explaining the basics of concepts I understand but may
  have forgotten. I need something more concise than whole books.

ASSUMPTIONS about the reader:
- Don't need to be taught basic programming concepts. Deep experience in C, C++,
  Java, Python; conversant but not expert in functional languages like Haskell,
  OCaml.
- Has programmed Rust code before, but infrequently.

NON-GOALS:
- Be a programming tutorial---assume the user knows binaries, libraries,
  frameworks, build-systems, dependencies.
- Be an exhaustive reference manual describing every argument and option to
  every command/tool. Instead, describe the```` most common use cases, hint
  other interesting options exist and provide links to canonical documentation.

GOALS:
- Document Rust specific terminology, syntax, conventions, best practices,
  common stdlib types and most widely used crates for CLIs and HTTP APIs.
- Concise text, using code samples to illustrate ideas.

## Build

Pre-requisites:
- [Install Rust and Cargo](https://rust-lang.org/tools/install/).
- Install the other tools by running [`tools_install.sh`](./tools_install.sh).
- Check if all tools are present and working by running
  [`tools_verify.sh`](./tools_verify.sh).

[mdbook]: https://rust-lang.github.io/mdBook/

Then, run [`book_build.sh`](./book_build.sh). This command generate output in
the `html/` directory; the root of the book will be at `html/index.html`. (This
directory is excluded from git.)

## Develop

The book's source code lives in `src/*.md` files. If you add add/modify anything
in `src/`, run `book_build.sh` to regenerate the book's HTML.

The book relies heavily on code samples, in sub-directories under `code/`. If
you modify code in it, run `code_verify.sh` to ensure that the code compiles and
runs. If you add a new directory to `code/`, update `verify_code.sh` to ensure
it is compiled and tested.

For convenience, the following two scripts are available:
- [`code_verify_watch.sh`](./code_verify_watch.sh): monitors the `code/`
  directory for changes, and runs `code_verify.sh`.
- [`book_build_watch.sh`](./book_build_watch.sh): monitors the `src/` _and_
  `code/` directories for changes, and runs `book_build.sh`.

Leave one or both of these scripts running in a terminal window to get live
feedback as you modify code/book.

### Live-Reload

Run `mdbook serve` in the root directory: this will build the book and serve it
on on HTTP and print a URL (like [http://localhost:3000/]). You can open this
URL in a browser and the browser page will hot-reload as you make changes to
`src/*.md`.

> **WARNING**. This command will render `*.md` files, but _not_ check included
> Rust code, or check the `*.md` files for best practices or errors. If you use
> this command, you should run `build_book.sh` periodically and before checking
> code into git.

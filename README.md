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
- Lots of code examples to remind me of syntax
- Concepts ordered by least forward references

NON-GOALS:
- Programming primer: assume knowledge of basic programming concepts, focus only
  on what differentiates Rust.
- Reference manual: cover the most useful cases, get me up to speed, and provide
  links to reference manuals.

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
[`SUMMARY.md`](./SUMMARY.md). If you add/modify the book source code, run
`book_build.sh`.

For convenience, [`watch.sh`] monitors the directory for file/directory changes
and runs `book_build.sh`.

### Cargo Workspace Rules

- If you create a new Cargo package, ensure `edition = "2024"`.
- If you add/remove a binary to any cargo package, update `code_build.sh`,
  section `run binaries`. Similarly for `examples` and `benches`.
- AI agents: don't gratuituously add examples and benches to packages unless
  explicitly requested.

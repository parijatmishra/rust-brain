#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

echo
echo
echo "########################################################################"
echo "code build"
echo "########################################################################"
cargo build --workspace

echo
echo
echo "########################################################################"
echo "code test"
echo "########################################################################"
cargo test --workspace

echo
echo
echo "########################################################################"
echo "run binaries"
echo "########################################################################"
cargo run --package cargo02 --bin cargo02
cargo run --package cargo02 --bin greet
cargo run --package cargo02 --bin count_words
cargo run --package debugging --bin debugging
cargo run --package modules01 --bin modules01
cargo run --package modules02 --bin modules02
cargo run --package values_ident --bin values_ident

echo
echo
echo "########################################################################"
echo "run examples"
echo "########################################################################"
cargo run --package cargo02 --example my_add

echo
echo
echo "########################################################################"
echo "run benches"
echo "########################################################################"
cargo bench --workspace --benches

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
cargo run --package cargo01 --bin cargo01
cargo run --package cargo01 --bin cargo01-hello

echo
echo
echo "########################################################################"
echo "run examples"
echo "########################################################################"
cargo run --package cargo01 --example example_my_add

echo
echo
echo "########################################################################"
echo "run benches"
echo "########################################################################"
cargo bench workspace --benches

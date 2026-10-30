#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

./code_build.sh

echo
echo
echo "########################################################################"
echo "mdbook test"
echo "########################################################################"
# Use the top-level libraries, not competing metadata files in debug/deps.
target_dir="${CARGO_TARGET_DIR:-target}"
mdbook test --library-path "$target_dir/debug"

echo
echo
echo "########################################################################"
echo "mdbook build"
echo "########################################################################"
mdbook build

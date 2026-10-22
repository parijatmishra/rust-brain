#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
./code/build.sh
./code/test.sh
./code/run_examples.sh
./code/run_benches.sh
./code/run_binaries.sh

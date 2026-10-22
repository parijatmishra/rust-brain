#!/bin/bash
# Smoke-test all binaries of member packages.
# Keep this list updated when adding/removing binaries.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
# Tee all output (stdout + stderr, ours and cargo's) to run_binaries.log while keeping it on
# the terminal. `exec > >(...)` replaces the shell's stdout with a process substitution
# writing to tee; `2>&1` then merges stderr into that same stream. Since child processes
# inherit these fds, every cargo run and binary output below is captured.
exec > >(tee run_binaries.log) 2>&1
echo "=== package cargo01"
cargo run --package cargo01
echo "=== package cargo02"
echo "--- binary hello01"
cargo run --package cargo02 --bin hello01
echo "--- binary cargo02-hello02"
cargo run --package cargo02 --bin cargo02-hello02

#!/bin/bash
# Run all benchmark crates.
# Keep this list updated when adding/removing benchmarks.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
# Tee all output (stdout + stderr, ours and cargo's) to run_benches.log while keeping it on
# the terminal. `exec > >(...)` replaces the shell's stdout with a process substitution
# writing to tee; `2>&1` then merges stderr into that same stream. Since child processes
# inherit these fds, every cargo run and benchmark output below is captured.
exec > >(tee run_benches.log) 2>&1

cargo bench --workspace

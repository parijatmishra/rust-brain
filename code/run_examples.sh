#!/bin/bash
# Run all example crates.
# Keep this list updated when adding/removing examples.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
# Tee all output (stdout + stderr, ours and cargo's) to run_examples.log while keeping it on
# the terminal. `exec > >(...)` replaces the shell's stdout with a process substitution
# writing to tee; `2>&1` then merges stderr into that same stream. Since child processes
# inherit these fds, every cargo run and example output below is captured.
exec > >(tee run_examples.log) 2>&1
# None yet

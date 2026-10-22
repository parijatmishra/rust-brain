#!/bin/bash
# Run all unit and integration tests across all packages.
#
# --no-fail-fast: by default cargo stops at the first failing package.
# Keep going so the full test run is covered.
#
# Full output (stdout + stderr) is tee'd to test.log (see the exec line below);
# commands below inherit it, so nothing needs a per-command `| tee`.
#
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
# Tee all output (stdout + stderr, ours and cargo's) to test.log while keeping it on
# the terminal. `exec > >(...)` replaces the shell's stdout with a process substitution
# writing to tee; `2>&1` then merges stderr into that same stream. Since child processes
# inherit these fds, every test command below is captured.
exec > >(tee test.log) 2>&1

cargo test --workspace --no-fail-fast

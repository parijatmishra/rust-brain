#!/bin/bash
# Build all member packages in the workspace.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
# Tee all output (stdout + stderr, ours and cargo's) to build.log while keeping it on
# the terminal. `exec > >(...)` replaces the shell's stdout with a process substitution
# writing to tee; `2>&1` then merges stderr into that same stream. Since child processes
# inherit these fds, every build command below is captured.
exec > >(tee build.log) 2>&1

cargo build --workspace

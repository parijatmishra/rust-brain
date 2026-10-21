#!/usr/bin/env bash
# Watch code/ and src/; on change, run book_build.sh (which runs code_verify.sh
# first). Runs once at startup. Takes no arguments. Stop with Ctrl+C.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
source ./common.sh

reject_args "$@"
require_exec code_verify.sh   # called by book_build.sh
watch_and_run "Book build" ./book_build.sh code src

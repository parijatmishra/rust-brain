#!/usr/bin/env bash
# Watch code/; on change, run code_verify.sh. Runs once at startup.
# Takes no arguments. Stop with Ctrl+C.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
source ./common.sh

reject_args "$@"
watch_and_run "Code verification" ./code_verify.sh code

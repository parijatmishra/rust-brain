#!/bin/bash
# Clean all member packages in the workspace and delete the *.log files written
# by the other scripts in this directory.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

cargo clean --workspace --quiet
rm -f ./*.log

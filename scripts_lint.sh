#!/usr/bin/env bash
# Lint all *.sh scripts in the repo root and code/ with shellcheck. Optional: no other
# script depends on this. Takes no arguments.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

(( $# == 0 )) || { echo "usage: $0 (takes no arguments)" >&2; exit 2; }

if ! command -v shellcheck >/dev/null; then
  cat >&2 <<'EOF'
ERROR: shellcheck not found on PATH; cannot lint scripts.
Install it (see README.md, "Lint Shell Scripts"), e.g.:
  brew install shellcheck
or run it without installing:
  uvx --from shellcheck-py shellcheck -x ./*.sh code/*.sh
EOF
  exit 1
fi

shopt -s nullglob
scripts=(./*.sh code/*.sh)
(( ${#scripts[@]} > 0 )) || { echo "no *.sh files found" >&2; exit 1; }

# -x: follow `source ./common.sh` in the watch scripts.
shellcheck -x "${scripts[@]}"
echo "=== shellcheck passed (${#scripts[@]} scripts)"

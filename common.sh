# shellcheck shell=bash
# Shared helpers for the *_watch.sh scripts. This is a library: source it, do
# not execute it. Callers enable `set -euo pipefail` and cd to the repo root
# before sourcing.

if [[ ${BASH_SOURCE[0]} == "$0" ]]; then
  echo "${BASH_SOURCE[0]} is a library: source it, don't run it" >&2
  exit 1
fi

# die MESSAGE: print MESSAGE to stderr and exit 1.
die() { echo "$1" >&2; exit 1; }

# reject_args "$@": exit 2 if the calling script received any arguments.
reject_args() { (( $# == 0 )) || { echo "usage: $0 (takes no arguments)" >&2; exit 2; }; }

# require_exec FILE...: exit 1 unless every FILE is executable.
require_exec() { local f; for f; do [[ -x $f ]] || die "missing or not executable: $f"; done; }

# require_dir DIR...: exit 1 unless every DIR exists.
require_dir() { local d; for d; do [[ -d $d ]] || die "missing directory: $d"; done; }

# _run_once LABEL SCRIPT REASON: run SCRIPT and report the outcome.
# Inside `if`, a failure does not trip set -e and stop the watcher.
_run_once() {
  echo; echo "*** $(date +%T) $3"
  if "$2"; then
    echo "=== $1 passed"
  else
    echo "=== FAILED $1 (exit $?)"
  fi
}

# watch_and_run LABEL SCRIPT DIR...
# Run SCRIPT at startup, then once per burst of changes under the DIRs. Never
# returns normally; stop with Ctrl+C.
watch_and_run() {
  local label=$1 script=$2 n m
  shift 2
  command -v fswatch >/dev/null || die "fswatch not found (brew install fswatch)"
  require_exec "$script"
  require_dir "$@"

  # On Ctrl+C/TERM, kill the whole process group (fswatch and any running
  # SCRIPT). Caveats: a script that launches the watcher without job control
  # shares the group and is killed too; a TERM received while SCRIPT runs is
  # handled after it ends.
  trap 'trap - INT TERM; kill 0' INT TERM

  _run_once "$label" "$script" "startup"

  # fswatch -o prints one event count per batch (--latency = batch window, s).
  # The inner loop drains batches until 1s of quiet, so an edit burst, or edits
  # made while SCRIPT runs, yield a single re-run. Integer -t keeps bash 3.2
  # working. Process substitution keeps the loop in the main shell, so the trap
  # fires promptly while waiting for events.
  # Excludes (extended regex): Cargo target dirs (the dir itself and anything
  # below), vim swap/backup/write-test files.
  while read -r n; do
    while read -r -t 1 m; do n=$((n + m)); done
    _run_once "$label" "$script" "change ($n events)"
  done < <(fswatch -o --latency 0.5 --recursive --extended \
             --exclude '/target(/|$)' --exclude '\.sw[a-px]$' --exclude '~$' --exclude '/4913$' \
             --event Updated --event Created --event Removed --event Renamed \
             "$@")
}

#!/bin/bash
# json_lint_vm.sh — Example app: JSON lint (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2
preflight_plugins || exit 2

## json_lint_vm is now default-on (regression fixed on 2025-11)
## Leave this header for context; no opt-in guard anymore.

APP_DIR="$NYASH_ROOT/apps/examples/json_lint"
# Note: Temporary tolerance for Void arithmetic in builder-subpaths (TTL: remove when builder fix lands)
# This keeps quick green while we root-cause the Sub(Integer,Void) in Stage‑B/VM lowering.
# Keep tolerance off by default; flip on if needed by environment
export NYASH_VM_TOLERATE_VOID=${NYASH_VM_TOLERATE_VOID:-0}
output=$(run_nyash_vm "$APP_DIR/main.hako" --dev)

expected=$(cat << 'TXT'
OK
OK
OK
OK
OK
OK
OK
OK
OK
OK
ERROR
ERROR
ERROR
ERROR
ERROR
ERROR
TXT
)

compare_outputs "$expected" "$output" "json_lint_vm" || exit 1

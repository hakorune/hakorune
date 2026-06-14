#!/bin/bash
# co_task_scope_vm.sh — CONC-CO-MIR-001D VM guard

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

POS_APP="$ROOT/apps/tests/async-co-task-scope-positive/main.hako"
NEG_APP="$ROOT/apps/tests/async-co-task-scope-early-return-fail/main.hako"
TMP_DIR="$(mktemp -d /tmp/nyash_co_task_scope_vm_XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

run_case() {
  local app="$1"
  local stdout_log="$2"
  local stderr_log="$3"
  set +e
  "$NYASH_BIN" --backend vm "$app" >"$stdout_log" 2>"$stderr_log"
  local rc=$?
  set -e
  printf '%s\n' "$rc"
}

POS_STDOUT="$TMP_DIR/positive.stdout.log"
POS_STDERR="$TMP_DIR/positive.stderr.log"
POS_RC="$(run_case "$POS_APP" "$POS_STDOUT" "$POS_STDERR")"
if [ "$POS_RC" -ne 42 ]; then
  log_error "co_task_scope_vm: positive expected exit=42, got $POS_RC"
  echo "[INFO] stdout tail:" >&2
  tail -n 40 "$POS_STDOUT" >&2 || true
  echo "[INFO] stderr tail:" >&2
  tail -n 80 "$POS_STDERR" >&2 || true
  exit 1
fi

NEG_STDOUT="$TMP_DIR/negative.stdout.log"
NEG_STDERR="$TMP_DIR/negative.stderr.log"
NEG_RC="$(run_case "$NEG_APP" "$NEG_STDOUT" "$NEG_STDERR")"
if [ "$NEG_RC" -eq 0 ]; then
  log_error "co_task_scope_vm: negative unexpectedly succeeded"
  echo "[INFO] stdout tail:" >&2
  tail -n 40 "$NEG_STDOUT" >&2 || true
  echo "[INFO] stderr tail:" >&2
  tail -n 80 "$NEG_STDERR" >&2 || true
  exit 1
fi
if ! rg -q "\[freeze:contract\]\[co/early-exit-unsupported\]" "$NEG_STDERR"; then
  log_error "co_task_scope_vm: negative did not report co early-exit fail-fast"
  echo "[INFO] stdout tail:" >&2
  tail -n 40 "$NEG_STDOUT" >&2 || true
  echo "[INFO] stderr tail:" >&2
  tail -n 120 "$NEG_STDERR" >&2 || true
  exit 1
fi

test_pass "co_task_scope_vm: PASS"

#!/bin/bash
# async_min_vm.sh — async/await minimal (VM) → rc=42

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

APP="$ROOT/apps/tests/async-await-min/main.hako"
TMP_DIR="$(mktemp -d /tmp/nyash_async_min_vm_XXXXXX)"
STDOUT_LOG="$TMP_DIR/stdout.log"
STDERR_LOG="$TMP_DIR/stderr.log"

set +e
"$NYASH_BIN" --backend vm "$APP" >"$STDOUT_LOG" 2>"$STDERR_LOG"
RC=$?
set -e

if [ "$RC" -eq 42 ]; then
  test_pass "async_min_vm: PASS (exit=42)"
  rm -rf "$TMP_DIR"
  exit 0
fi

log_error "async_min_vm: expected exit=42, got $RC"
echo "[INFO] stdout tail:" >&2
tail -n 40 "$STDOUT_LOG" >&2 || true
echo "[INFO] stderr tail:" >&2
tail -n 40 "$STDERR_LOG" >&2 || true
rm -rf "$TMP_DIR"
exit 1

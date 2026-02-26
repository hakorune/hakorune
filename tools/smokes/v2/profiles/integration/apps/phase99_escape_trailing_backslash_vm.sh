#!/bin/bash
# Phase 99: json_loader escape with trailing backslash (VM)
#
# Verifies current behavior: trailing backslash is included in output (best-effort).

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase99_json_loader_escape_trailing_backslash_min.hako"

echo "[INFO] Phase 99: escape trailing backslash (VM) - $INPUT"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 0 ]; then
  # Current behavior: trailing backslash is included in output
  EXPECTED="hello\\"
  CLEAN=$(printf "%s\n" "$OUTPUT" | grep -v "^\[" | grep -v "^$" | head -n 1 | tr -d '\r')
  if [ "$CLEAN" = "$EXPECTED" ]; then
    echo "[PASS] Output verified: hello\\ (trailing backslash preserved)"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    echo "[FAIL] Unexpected output (expected: hello\\)"
    echo "[INFO] output (tail):"
    echo "$OUTPUT" | tail -n 50 || true
    FAIL_COUNT=$((FAIL_COUNT + 1))
  fi
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase99_escape_trailing_backslash_vm: Best-effort behavior verified"
  exit 0
else
  test_fail "phase99_escape_trailing_backslash_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

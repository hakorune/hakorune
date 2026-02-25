#!/bin/bash
# Phase 137 P0: loop(true) break-once with post-loop assigns + return add (VM)
#
# Verifies: x = 0 → loop(true) { x = 1; break } → x = x + 10 → return x + 2
# Expected: exit code 13 (0 → 1 → 11 → 13)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2
require_joinir_dev

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 137 P0: loop(true) break-once with post assigns + return x + 2 (VM)"

INPUT="$NYASH_ROOT/apps/tests/phase137_loop_true_break_once_post_return_add_min.hako"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 13 ]; then
  echo "[PASS] exit code verified: 13"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE (expected 13)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase137_loop_true_break_once_post_return_add_vm: All tests passed"
  exit 0
else
  test_fail "phase137_loop_true_break_once_post_return_add_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

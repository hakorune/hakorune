#!/bin/bash
# Phase 135 P0: loop(true) break-once + return (0 post assigns, Normalized shadow, VM)
#
# Verifies that loop(true) { <assign>* ; break } + return (0 assigns) works:
# - x = 0 → loop(true) { x = 1; break } → return x → 1
# - Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# Phase 135 is a dev-only Normalized shadow loop case.
require_joinir_dev

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 135 P0: loop(true) break-once + return (0 post assigns, Normalized shadow, VM)"

# Test 1: phase135_loop_true_break_once_post_empty_return_min.hako
echo "[INFO] Test 1: phase135_loop_true_break_once_post_empty_return_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase135_loop_true_break_once_post_empty_return_min.hako"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 1 ]; then
  # Phase 135: expected output is exit code 1 (return value)
  echo "[PASS] exit code verified: 1"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE (expected 1)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase135_loop_true_break_once_post_empty_return_vm: All tests passed"
  exit 0
else
  test_fail "phase135_loop_true_break_once_post_empty_return_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

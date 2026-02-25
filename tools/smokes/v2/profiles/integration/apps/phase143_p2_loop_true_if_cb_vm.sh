#!/bin/bash
# Phase 143 P2: loop(true) if continue-break (C-B, Normalized shadow, VM)
#
# Verifies Phase 143 P2 else symmetry:
# - loop(true) { if flag==1 {continue} else {break} } return 9 → exit code 9
# - Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 143 P2: loop(true) if continue-break (C-B, VM)"

echo "[INFO] Test 1: phase143_p2_loop_true_if_cb_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase143_p2_loop_true_if_cb_min.hako"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  NYASH_JOINIR_DEV=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 9 ]; then
  echo "[PASS] exit code verified: 9"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE (expected 9)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase143_p2_loop_true_if_cb_vm: All tests passed"
  exit 0
else
  test_fail "phase143_p2_loop_true_if_cb_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

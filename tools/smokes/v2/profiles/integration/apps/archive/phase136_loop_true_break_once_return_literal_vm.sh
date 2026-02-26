#!/bin/bash
# Phase 136 P0: loop(true) break-once + return literal (VM)
#
# Verifies that loop(true) { <assign>* ; break }; return <int_literal> works:
# - x = 0 → loop(true) { x = 1; break } → return 7 → exit code 7
# - Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# Phase 136 is a dev-only Normalized shadow loop case.
require_joinir_dev

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 136 P0: loop(true) break-once + return literal (VM)"

# Test: phase136_loop_true_break_once_return_literal_min.hako
echo "[INFO] Test: phase136_loop_true_break_once_return_literal_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase136_loop_true_break_once_return_literal_min.hako"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 7 ]; then
  # Phase 136: expected output is exit code 7 (return literal)
  echo "[PASS] exit code verified: 7"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE (expected 7)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase136_loop_true_break_once_return_literal_vm: All tests passed"
  exit 0
else
  test_fail "phase136_loop_true_break_once_return_literal_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

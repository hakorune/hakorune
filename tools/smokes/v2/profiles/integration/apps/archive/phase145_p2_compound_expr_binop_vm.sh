#!/bin/bash
# Phase 145 P2: Recursive ANF with nested BinaryOp + MethodCall (VM)
#
# Pattern: x + s.length() + z → exit code 18 (10 + 5 + 3)
# Dev-only: HAKO_ANF_DEV=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 145 P2: Recursive ANF - nested BinaryOp + MethodCall (VM)"

echo "[INFO] Test 1: phase145_p2_compound_expr_binop_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase145_p2_compound_expr_binop_min.hako"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_ANF_DEV=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 18 ]; then
  echo "[PASS] exit code verified: 18"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE (expected 18)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase145_p2_compound_expr_binop_vm: All tests passed"
  exit 0
else
  test_fail "phase145_p2_compound_expr_binop_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

#!/bin/bash
# Phase 284 P1: Return in loop test (VM)
# Expected: Exit code 7 from early return

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase284_p1_return_in_loop_min.hako"

echo "[INFO] Phase 284 P1: Return in loop test (VM) - $INPUT"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 7 ]; then
  echo "[PASS] Exit code verified: 7 (early return from loop)"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] hakorune exited with code $EXIT_CODE (expected 7)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase284_p1_return_in_loop_vm: Test passed"
  exit 0
else
  test_fail "phase284_p1_return_in_loop_vm: Test failed"
  exit 1
fi

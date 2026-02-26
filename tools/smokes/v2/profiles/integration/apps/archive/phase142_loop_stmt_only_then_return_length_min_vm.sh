#!/bin/bash
# Phase 142 P0: Loop normalization as single statement (VM)
#
# Verifies that loop(true) normalization returns consumed=1, allowing
# subsequent statements (return s.length()) to be processed normally.
# Expected: exit code 3 (s="abc" → s.length() → 3)
#
# Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# Phase 142 is a dev-only Normalized shadow loop case
require_joinir_dev

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 142 P0: Loop normalization as single statement (VM)"

# Test 1: phase142_loop_stmt_only_then_return_length_min.hako
echo "[INFO] Test 1: phase142_loop_stmt_only_then_return_length_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase142_loop_stmt_only_then_return_length_min.hako"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 3 ]; then
  # Phase 142: expected output is exit code 3 (s.length() where s="abc")
  echo "[PASS] exit code verified: 3"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE (expected 3)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase142_loop_stmt_only_then_return_length_min_vm: All tests passed"
  exit 0
else
  test_fail "phase142_loop_stmt_only_then_return_length_min_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

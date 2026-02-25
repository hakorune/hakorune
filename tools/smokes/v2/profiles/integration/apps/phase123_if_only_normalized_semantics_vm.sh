#!/bin/bash
# Phase 123: If-only Normalized Semantics (VM)
#
# Verifies that normalized semantics lowering works correctly in dev mode:
# - Return(Integer literal) generates Const + Ret
# - If(minimal compare) generates Compare + Ret structure

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 123: If-only Normalized Semantics (VM)"

# Test 1: phase123_if_only_return_literal_min.hako
echo "[INFO] Test 1: phase123_if_only_return_literal_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase123_if_only_return_literal_min.hako"

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
elif [ "$EXIT_CODE" -eq 0 ]; then
  EXPECTED="7"
  if validate_numeric_output 1 "$EXPECTED" "$OUTPUT"; then
    echo "[PASS] Output verified: 7"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
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
  test_pass "phase123_if_only_normalized_semantics_vm: All tests passed"
  exit 0
else
  test_fail "phase123_if_only_normalized_semantics_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

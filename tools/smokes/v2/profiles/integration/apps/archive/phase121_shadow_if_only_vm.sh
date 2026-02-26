#!/bin/bash
# Phase 121: StepTree→Normalized shadow parity check (VM)
#
# Verifies that shadow lowering for if-only patterns works correctly in dev mode
# and that strict mode does not fail (parity check passes).

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 121: StepTree→Normalized shadow parity (VM)"

# Test 1: phase103_if_only_merge_min.hako
echo "[INFO] Test 1: phase103_if_only_merge_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase103_if_only_merge_min.hako"

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
  EXPECTED="2"
  if validate_numeric_output 1 "$EXPECTED" "$OUTPUT"; then
    # Also check that shadow lowering was logged
    if echo "$OUTPUT" | grep -q "\[trace:dev\] phase121/shadow:"; then
      echo "[PASS] Output verified: 2, shadow logging present"
      PASS_COUNT=$((PASS_COUNT + 1))
    else
      echo "[WARN] Output correct but shadow logging missing"
      PASS_COUNT=$((PASS_COUNT + 1))
    fi
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

# Test 2: phase114_if_only_return_then_post_min.hako
echo "[INFO] Test 2: phase114_if_only_return_then_post_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase114_if_only_return_then_post_min.hako"

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
  EXPECTED=$'7\n2'
  if validate_numeric_output 2 "$EXPECTED" "$OUTPUT"; then
    echo "[PASS] Output verified: 7\\n2"
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

# Test 3: phase117_if_only_nested_if_call_merge_min.hako
echo "[INFO] Test 3: phase117_if_only_nested_if_call_merge_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase117_if_only_nested_if_call_merge_min.hako"

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
  EXPECTED=$'2\n3\n4'
  if validate_numeric_output 3 "$EXPECTED" "$OUTPUT"; then
    echo "[PASS] Output verified: 2\\n3\\n4"
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
  test_pass "phase121_shadow_if_only_vm: All tests passed"
  exit 0
else
  test_fail "phase121_shadow_if_only_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

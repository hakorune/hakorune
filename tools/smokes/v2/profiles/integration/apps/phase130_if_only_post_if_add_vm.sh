#!/bin/bash
# Phase 130: If-only Post-If Add (Normalized post_k with computation, VM)
#
# Verifies that post_k can execute computation statements after if:
# - Case A: flag=1 → x=2 → x=x+3 → 5
# - Case B: flag=0 → x=1 → x=x+3 → 4
# - Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 130: If-only Post-If Add (Normalized post_k with computation, VM)"

# Test 1: phase130_if_only_post_if_add_min.hako
echo "[INFO] Test 1: phase130_if_only_post_if_add_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase130_if_only_post_if_add_min.hako"

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
  # Phase 130: expect 5\n4 (Case A: 2+3=5, Case B: 1+3=4)
  EXPECTED=$'5\n4'
  if validate_numeric_output 2 "$EXPECTED" "$OUTPUT"; then
    echo "[PASS] Output verified: 5\\n4 (exit code: $EXIT_CODE)"
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
  test_pass "phase130_if_only_post_if_add_vm: All tests passed"
  exit 0
else
  test_fail "phase130_if_only_post_if_add_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

#!/bin/bash
# Phase 129: If-only Post-If Return Var (Normalized join_k, VM)
#
# Verifies that join_k continuation properly merges env after if:
# - x=1; flag=1; if flag==1 { x=2 }; return x
# - join_k receives env_phi (x=2 from then, x=1 from else)
# - Returns x=2 (flag=1 fixed)
# - Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

echo "[INFO] Phase 129: If-only Post-If Return Var (Normalized join_k, VM)"

# Test 1: phase129_if_only_post_if_return_var_min.hako
echo "[INFO] Test 1: phase129_if_only_post_if_return_var_min.hako"
INPUT="$NYASH_ROOT/apps/tests/phase129_if_only_post_if_return_var_min.hako"

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
  # Phase 129: expect x=2 (print(x) after if with x=2)
  EXPECTED="2"
  if validate_numeric_output 1 "$EXPECTED" "$OUTPUT"; then
    echo "[PASS] Output verified: 2 (exit code: $EXIT_CODE)"
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
  test_pass "phase129_if_only_post_if_return_var_vm: All tests passed"
  exit 0
else
  test_fail "phase129_if_only_post_if_return_var_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

#!/bin/bash
# Phase 118: Pattern3 carrier merge regression (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase118_pattern3_if_sum_min.hako"
EXPECTED="12"

echo "[INFO] Phase 118: Pattern3 if-sum carrier merge (VM) - $INPUT"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  test_fail "phase118_pattern3_if_sum_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase118_pattern3_if_sum_vm: execution failed"
  exit 1
fi

if ! echo "$OUTPUT" | grep -qF "[flowbox/adopt box_kind=Loop" \
  || ! echo "$OUTPUT" | grep -qF "via=shadow"; then
  echo "[FAIL] Missing FlowBox tag (box_kind=Loop via=shadow)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase118_pattern3_if_sum_vm: Missing FlowBox tag"
  exit 1
fi

if validate_numeric_output 1 "$EXPECTED" "$OUTPUT"; then
  test_pass "phase118_pattern3_if_sum_vm: output matches expected (12)"
  exit 0
fi

echo "[INFO] output (tail):"
echo "$OUTPUT" | tail -n 80 || true
test_fail "phase118_pattern3_if_sum_vm: output mismatch"
exit 1

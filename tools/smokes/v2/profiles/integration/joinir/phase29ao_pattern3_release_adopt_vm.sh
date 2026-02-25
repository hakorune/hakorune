#!/bin/bash
# phase29ao_pattern3_release_adopt_vm.sh - Pattern3 release adopt path smoke (VM)
#
# Expected:
# - Exit code 0
# - Output matches expected (12)
# - No shadow adopt tag

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase118_pattern3_if_sum_min.hako"
EXPECTED="12"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  -u HAKO_JOINIR_STRICT \
  -u NYASH_JOINIR_STRICT \
  -u HAKO_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEV \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  test_fail "phase29ao_pattern3_release_adopt_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29ao_pattern3_release_adopt_vm: execution failed"
  exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
  echo "[FAIL] Unexpected FlowBox tag in release output"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29ao_pattern3_release_adopt_vm: tag should not appear in release"
  exit 1
fi

if validate_numeric_output 1 "$EXPECTED" "$OUTPUT"; then
  test_pass "phase29ao_pattern3_release_adopt_vm: output matches expected (12)"
  exit 0
fi

echo "[INFO] output (tail):"
echo "$OUTPUT" | tail -n 80 || true
test_fail "phase29ao_pattern3_release_adopt_vm: output mismatch"
exit 1

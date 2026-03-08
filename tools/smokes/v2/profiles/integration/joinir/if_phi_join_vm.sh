#!/bin/bash
# current semantic wrapper; canonical entry for if_phi_join route smoke

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

SEMANTIC_STEM="if_phi_join_vm"
LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
INPUT="$NYASH_ROOT/apps/tests/if_phi_join_min.hako"
EXPECTED="12"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  test_fail "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "${LABEL_PREFIX}: execution failed"
  exit 1
fi

if ! echo "$OUTPUT" | grep -qF "[flowbox/adopt box_kind=Loop" \
  || ! echo "$OUTPUT" | grep -qF "via=shadow"; then
  echo "[FAIL] Missing FlowBox tag (box_kind=Loop via=shadow)"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "${LABEL_PREFIX}: Missing FlowBox tag"
  exit 1
fi

if validate_numeric_output 1 "$EXPECTED" "$OUTPUT"; then
  test_pass "${LABEL_PREFIX}: output matches expected (12)"
  exit 0
fi

echo "[INFO] output (tail):"
echo "$OUTPUT" | tail -n 80 || true
test_fail "${LABEL_PREFIX}: output mismatch"
exit 1

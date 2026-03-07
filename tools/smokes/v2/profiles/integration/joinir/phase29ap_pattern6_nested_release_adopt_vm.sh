#!/bin/bash
# phase29ap_pattern6_nested_release_adopt_vm.sh - nested_loop_minimal release adopt (VM)
# legacy compat stem; current semantic entry = nested_loop_minimal_release_adopt_vm.sh
#
# Expected:
# - Exit code 9
# - No shadow-adopt tags

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LEGACY_STEM="phase29ap_pattern6_nested_release_adopt_vm"
SEMANTIC_STEM="nested_loop_minimal_release_adopt_vm"
LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"

FIXTURE="$NYASH_ROOT/apps/tests/phase1883_nested_minimal.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  -u HAKO_JOINIR_STRICT \
  -u NYASH_JOINIR_STRICT \
  -u HAKO_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEV \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Release adopt must not print FlowBox tags"
    echo "$OUTPUT" | tail -n 60 || true
    test_fail "${LABEL_PREFIX}: Unexpected tag"
    exit 1
fi

if [ "$EXIT_CODE" -ne 9 ]; then
    echo "[FAIL] Expected exit 9, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "${LABEL_PREFIX}: Unexpected RC"
    exit 1
fi

test_pass "${LABEL_PREFIX}: PASS (exit=9)"
exit 0

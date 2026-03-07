#!/bin/bash
# phase29ao_pattern6_release_adopt_vm.sh - scan_with_init release adopt path smoke (VM)
# legacy compat stem; current semantic entry = scan_with_init_release_adopt_vm.sh
#
# Expected:
# - Exit code 1

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LEGACY_STEM="phase29ao_pattern6_release_adopt_vm"
SEMANTIC_STEM="scan_with_init_release_adopt_vm"
LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"

FIXTURE="$NYASH_ROOT/apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  -u HAKO_JOINIR_STRICT \
  -u NYASH_JOINIR_STRICT \
  -u HAKO_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEV \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" "$FIXTURE" 2>&1)
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

if [ "$EXIT_CODE" -ne 1 ]; then
    echo "[FAIL] Expected exit 1, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "${LABEL_PREFIX}: Unexpected RC"
    exit 1
fi

test_pass "${LABEL_PREFIX}: PASS (exit=1)"
exit 0

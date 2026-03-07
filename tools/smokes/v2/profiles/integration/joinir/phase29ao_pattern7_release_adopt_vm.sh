#!/bin/bash
# phase29ao_pattern7_release_adopt_vm.sh - split_scan release adopt path smoke (VM)
# legacy compat stem; current semantic entry = split_scan_release_adopt_vm.sh
#
# Expected:
# - Exit code 3

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ab_pattern7_splitscan_ok_min.hako"
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
    test_fail "phase29ao_pattern7_release_adopt_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Release adopt must not print FlowBox tags"
    echo "$OUTPUT" | tail -n 60 || true
    test_fail "phase29ao_pattern7_release_adopt_vm: Unexpected tag"
    exit 1
fi

if [ "$EXIT_CODE" -ne 3 ]; then
    echo "[FAIL] Expected exit 3, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ao_pattern7_release_adopt_vm: Unexpected RC"
    exit 1
fi

test_pass "phase29ao_pattern7_release_adopt_vm: PASS (exit=3)"
exit 0

#!/bin/bash
# generic_loop_continue_release_adopt_vm.sh - generic loop v0.1 continue (release adopt)
# Expected:
# - Exit code 4
# - No FlowBox tags

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ca_generic_loop_continue_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_JOINIR_DEV=0 \
    NYASH_JOINIR_STRICT=0 \
    HAKO_JOINIR_STRICT=0 \
    NYASH_JOINIR_DEBUG=0 \
    HAKO_JOINIR_DEBUG=0 \
    "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "generic_loop_continue_release_adopt_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 4 ]; then
    echo "[FAIL] Expected exit 4, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "generic_loop_continue_release_adopt_vm: Unexpected RC"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected FlowBox tag in release"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "generic_loop_continue_release_adopt_vm: Unexpected tag"
    exit 1
fi

test_pass "generic_loop_continue_release_adopt_vm: PASS (exit=4)"
exit 0

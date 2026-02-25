#!/bin/bash
# phase29cb_generic_loop_in_body_step_release_adopt_vm.sh - generic loop v0.2 in-body step (release adopt)
# Expected:
# - Exit code 3
# - No FlowBox tags

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29cb_generic_loop_in_body_step_min.hako"
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
    test_fail "phase29cb_generic_loop_in_body_step_release_adopt_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 3 ]; then
    echo "[FAIL] Expected exit 3, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29cb_generic_loop_in_body_step_release_adopt_vm: Unexpected RC"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected FlowBox tag in release"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29cb_generic_loop_in_body_step_release_adopt_vm: Unexpected tag"
    exit 1
fi

test_pass "phase29cb_generic_loop_in_body_step_release_adopt_vm: PASS (exit=9)"
exit 0

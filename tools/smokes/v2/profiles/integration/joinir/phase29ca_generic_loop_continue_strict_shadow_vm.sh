#!/bin/bash
# phase29ca_generic_loop_continue_strict_shadow_vm.sh - generic loop v0.1 continue (strict/dev)
# Expected:
# - Exit code 4
# - Generic loop v0 continue is selected, via either:
#   - FlowBox shadow adopt tag, or
#   - planner-first tag (the plan line may bypass shadow adopt tagging).

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ca_generic_loop_continue_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 HAKO_JOINIR_PLANNER_REQUIRED=1 "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ca_generic_loop_continue_strict_shadow_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 4 ]; then
    echo "[FAIL] Expected exit 4, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ca_generic_loop_continue_strict_shadow_vm: Unexpected RC"
    exit 1
fi

if grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
    && grep -qF "features=continue" <<<"$OUTPUT" \
    && grep -qF "via=shadow" <<<"$OUTPUT"; then
    test_pass "phase29ca_generic_loop_continue_strict_shadow_vm: PASS (exit=4, flowbox tag)"
    exit 0
fi

if grep -qF "[joinir/planner_first rule=Pattern1]" <<<"$OUTPUT"; then
    test_pass "phase29ca_generic_loop_continue_strict_shadow_vm: PASS (exit=4, planner-first tag)"
    exit 0
fi

echo "[FAIL] Missing tag (flowbox shadow adopt OR planner-first)"
echo "$OUTPUT" | tail -n 40 || true
test_fail "phase29ca_generic_loop_continue_strict_shadow_vm: Missing tag"
exit 1

exit 0

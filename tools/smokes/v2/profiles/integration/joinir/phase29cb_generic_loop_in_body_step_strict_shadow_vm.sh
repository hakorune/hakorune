#!/bin/bash
# phase29cb_generic_loop_in_body_step_strict_shadow_vm.sh - generic loop v0.2 in-body step (strict/dev)
# Expected:
# - Exit code 3
# - Generic loop v0.2 in-body step is selected, via either:
#   - FlowBox shadow adopt tag, or
#   - planner-first tag (the plan line may bypass shadow adopt tagging).

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29cb_generic_loop_in_body_step_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 HAKO_JOINIR_PLANNER_REQUIRED=1 "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29cb_generic_loop_in_body_step_strict_shadow_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 3 ]; then
    echo "[FAIL] Expected exit 3, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29cb_generic_loop_in_body_step_strict_shadow_vm: Unexpected RC"
    exit 1
fi

if grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
    && grep -qF "via=shadow" <<<"$OUTPUT"; then
    test_pass "phase29cb_generic_loop_in_body_step_strict_shadow_vm: PASS (exit=3, flowbox tag)"
    exit 0
fi

if planner_first_tag_matches "$OUTPUT" "[joinir/planner_first rule=Pattern1]"; then
    test_pass "phase29cb_generic_loop_in_body_step_strict_shadow_vm: PASS (exit=3, planner-first tag)"
    exit 0
fi

echo "[FAIL] Missing tag (flowbox shadow adopt OR planner-first)"
echo "$OUTPUT" | tail -n 40 || true
test_fail "phase29cb_generic_loop_in_body_step_strict_shadow_vm: Missing tag"
exit 1

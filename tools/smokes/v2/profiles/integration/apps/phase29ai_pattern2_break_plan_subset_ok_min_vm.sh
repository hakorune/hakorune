#!/bin/bash
# Phase 29ai P11: Pattern2 break plan subset OK minimal (VM backend)
# Tests: exit -> 15

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29ai_pattern2_break_plan_subset_ok_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ai_pattern2_break_plan_subset_ok_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -eq 15 ]; then
    if ! echo "$OUTPUT" | grep -qF "[flowbox/adopt box_kind=Loop" \
        || ! echo "$OUTPUT" | grep -qF "features=break" \
        || ! echo "$OUTPUT" | grep -qF "via=shadow"; then
        echo "[FAIL] Missing FlowBox tag (box_kind=Loop features=break via=shadow)"
        echo "$OUTPUT" | tail -n 40 || true
        test_fail "phase29ai_pattern2_break_plan_subset_ok_min_vm: Missing FlowBox tag"
        exit 1
    fi
    test_pass "phase29ai_pattern2_break_plan_subset_ok_min_vm: RC=15 (expected)"
    exit 0
fi

echo "[FAIL] Expected exit 15, got $EXIT_CODE"
echo "$OUTPUT" | tail -n 40 || true
test_fail "phase29ai_pattern2_break_plan_subset_ok_min_vm: Unexpected RC"
exit 1

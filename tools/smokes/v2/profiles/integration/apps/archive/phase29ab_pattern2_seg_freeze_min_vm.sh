#!/bin/bash
# Phase 29ab P3: Pattern2 LoopBodyLocal seg Freeze (VM backend)
# Tests: read-only violation must fail-fast with plan/normalize freeze tag

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29ab_pattern2_seg_freeze_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ab_pattern2_seg_freeze_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -eq 0 ]; then
    echo "[FAIL] Expected JoinIR freeze error, got exit 0"
    echo "[INFO] Output:"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ab_pattern2_seg_freeze_min_vm: Unexpected success"
    exit 1
fi

if echo "$OUTPUT" | grep -qF "[flowbox/adopt "; then
    echo "[FAIL] FlowBox adopt tag must not appear for Freeze case"
    echo "[INFO] Output:"
    echo "$OUTPUT" | tail -n 60 || true
    test_fail "phase29ab_pattern2_seg_freeze_min_vm: Unexpected FlowBox adopt"
    exit 1
fi

if expect_plan_freeze "phase29ab_pattern2_seg_freeze_min_vm" "$OUTPUT" "$EXIT_CODE"; then
    exit 0
else
    exit 1
fi

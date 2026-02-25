#!/bin/bash
# Phase 29ab P3: Pattern2 promotion NotApplicable (VM backend)
# Tests: no LoopBodyLocal in condition -> continue, output should be 2

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29ab_pattern2_seg_notapplicable_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ab_pattern2_seg_notapplicable_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

OUTPUT_CLEAN=$(echo "$OUTPUT" | filter_noise)

if echo "$OUTPUT" | grep -qF "[flowbox/adopt "; then
    echo "[FAIL] FlowBox adopt tag must not appear for NotApplicable case"
    echo "[INFO] Output (raw):"
    echo "$OUTPUT" | tail -n 60 || true
    test_fail "phase29ab_pattern2_seg_notapplicable_min_vm: Unexpected FlowBox adopt"
    exit 1
fi

if echo "$OUTPUT_CLEAN" | grep -q "^2$" || echo "$OUTPUT" | grep -q "^RC: 2$"; then
    test_pass "phase29ab_pattern2_seg_notapplicable_min_vm: promotion not applicable (output: 2)"
    exit 0
else
    echo "[FAIL] Unexpected output (expected: 2)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output (clean):"
    echo "$OUTPUT_CLEAN" | tail -n 20 || true
    test_fail "phase29ab_pattern2_seg_notapplicable_min_vm: Unexpected output"
    exit 1
fi

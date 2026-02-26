#!/bin/bash
# Phase 29ab P4: Pattern2 seg real-world minimal repro (VM backend)
# Expectation: Derived slot path succeeds (output: 4)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase263_pattern2_seg_realworld_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase263_pattern2_seg_realworld_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

OUTPUT_CLEAN=$(echo "$OUTPUT" | filter_noise)

if ! echo "$OUTPUT" | grep -qF "[flowbox/adopt box_kind=Loop" \
    || ! echo "$OUTPUT" | grep -qF "features=break" \
    || ! echo "$OUTPUT" | grep -qF "via=shadow"; then
    test_fail "phase263_pattern2_seg_realworld_min_vm: missing FlowBox tag"
    exit 1
fi

if echo "$OUTPUT_CLEAN" | grep -q "^4$" || echo "$OUTPUT" | grep -q "^RC: 4$"; then
    test_pass "phase263_pattern2_seg_realworld_min_vm: Derived slot promotion succeeded (output: 4)"
    exit 0
else
    echo "[FAIL] Unexpected output (expected: 4)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output (clean):"
    echo "$OUTPUT_CLEAN" | tail -n 20 || true
    test_fail "phase263_pattern2_seg_realworld_min_vm: Unexpected output"
    exit 1
fi

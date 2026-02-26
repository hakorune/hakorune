#!/bin/bash
# Phase 263 P0 - Pattern2 LoopBodyLocal seg fallback (VM backend)
# Tests: Pattern2 rejects "seg" (reassigned body-local) → Pattern1 fallback

HAKO_FILE="apps/tests/phase263_p0_pattern2_seg_min.hako"
BACKEND="vm"
EXPECTED_OUTPUT="0"

ACTUAL_OUTPUT=$(HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | tail -1 | grep -E '^[0-9]+$')

if [ "$ACTUAL_OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "✅ PASS: phase263_p0_pattern2_seg_vm"
    exit 0
else
    echo "❌ FAIL: phase263_p0_pattern2_seg_vm"
    echo "Expected:"
    echo "$EXPECTED_OUTPUT"
    echo "Got:"
    echo "$ACTUAL_OUTPUT"
    echo "---Full output (last 80 lines):---"
    HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | tail -n 80
    exit 1
fi

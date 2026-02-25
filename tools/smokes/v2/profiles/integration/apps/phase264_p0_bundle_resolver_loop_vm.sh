#!/bin/bash
# Phase 264 P0 - BundleResolver loop pattern (VM backend)
# Tests: Non-unit increment + conditional assignment loop

HAKO_FILE="apps/tests/phase264_p0_bundle_resolver_loop_min.hako"
BACKEND="vm"
EXPECTED_OUTPUT="RC: 0"

ACTUAL_OUTPUT=$(HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | tail -1)

if [ "$ACTUAL_OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "✅ PASS: phase264_p0_bundle_resolver_loop_vm"
    exit 0
else
    echo "❌ FAIL: phase264_p0_bundle_resolver_loop_vm"
    echo "Expected:"
    echo "$EXPECTED_OUTPUT"
    echo "Got:"
    echo "$ACTUAL_OUTPUT"
    echo "---Full output (last 80 lines):---"
    HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | tail -n 80
    exit 1
fi

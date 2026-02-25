#!/bin/bash
# Phase 283 P0 - Pattern3 loop_if_phi fix (VM backend)
# Tests: loop(i <= 5) with if-else phi (sum += odd numbers)

HAKO_FILE="apps/tests/loop_if_phi.hako"
BACKEND="vm"

# Expected: [Console LOG] sum=9
EXPECTED_OUTPUT="sum=9"

ACTUAL_OUTPUT=$(./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | grep -o 'sum=[0-9]*')

if [ "$ACTUAL_OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "✅ PASS: phase283_p0_loop_if_phi_vm"
    exit 0
else
    echo "❌ FAIL: phase283_p0_loop_if_phi_vm"
    echo "Expected:"
    echo "$EXPECTED_OUTPUT"
    echo "Got:"
    echo "$ACTUAL_OUTPUT"
    echo "---Full output (last 80 lines):---"
    ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | tail -n 80
    exit 1
fi

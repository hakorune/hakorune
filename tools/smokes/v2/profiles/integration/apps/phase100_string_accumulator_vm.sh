#!/bin/bash
# Phase 100 P3 - String Accumulator (VM backend)

HAKO_FILE="apps/tests/phase100_string_accumulator_min.hako"
BACKEND="vm"
EXPECTED_OUTPUT="3"

ACTUAL_OUTPUT=$(HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | grep -v '^\[' | grep -v 'Net plugin' | grep -v 'RC:' | grep -E '^[0-9]+$')

if [ "$ACTUAL_OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "✅ PASS: phase100_string_accumulator_vm"
    exit 0
else
    echo "❌ FAIL: phase100_string_accumulator_vm"
    echo "Expected:"
    echo "$EXPECTED_OUTPUT"
    echo "Got:"
    echo "$ACTUAL_OUTPUT"
    echo "---Full output (last 80 lines):---"
    HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend "$BACKEND" "$HAKO_FILE" 2>&1 | tail -n 80
    exit 1
fi

#!/bin/bash
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"
set +e
$HAKORUNE_BIN apps/tests/phase259_p0_is_integer_min.hako > /tmp/phase259_out.txt 2>&1
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 7 ]; then
    echo "[PASS] phase259_p0_is_integer_vm"
    exit 0
else
    echo "[FAIL] phase259_p0_is_integer_vm: expected exit 7, got $EXIT_CODE"
    cat /tmp/phase259_out.txt
    exit 1
fi

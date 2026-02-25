#!/bin/bash
# Phase 275 P0: Test A1 - Void in boolean context → TypeError (VM)
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

set +e
$HAKORUNE_BIN --backend vm apps/tests/phase275_p0_truthiness_void_error_min.hako > /tmp/phase275_truthiness_vm.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "[PASS] phase275_p0_truthiness_vm"
    exit 0
else
    echo "[FAIL] expected error, got exit $EXIT_CODE"
    cat /tmp/phase275_truthiness_vm.txt
    exit 1
fi

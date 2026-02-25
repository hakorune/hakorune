#!/bin/bash
# Phase 275 P0: Test C2 - Plus number-only promotion (VM)
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

set +e
$HAKORUNE_BIN --backend vm apps/tests/phase275_p0_plus_number_only_min.hako > /tmp/phase275_plus_vm.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 3 ]; then
    echo "[PASS] phase275_p0_plus_number_only_vm"
    exit 0
else
    echo "[FAIL] expected exit=3, got $EXIT_CODE"
    cat /tmp/phase275_plus_vm.txt
    exit 1
fi

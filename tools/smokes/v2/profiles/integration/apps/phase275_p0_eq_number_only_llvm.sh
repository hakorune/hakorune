#!/bin/bash
# Phase 275 P0: Test B2 - Number-only equality (LLVM)
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

set +e
NYASH_LLVM_USE_HARNESS=1 $HAKORUNE_BIN --backend llvm apps/tests/phase275_p0_eq_number_only_min.hako > /tmp/phase275_eq_llvm.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 3 ]; then
    echo "[PASS] phase275_p0_eq_number_only_llvm"
    exit 0
else
    echo "[FAIL] expected exit=3, got $EXIT_CODE"
    cat /tmp/phase275_eq_llvm.txt
    exit 1
fi

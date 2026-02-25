#!/bin/bash
# Phase 275 P0: Test A1 - Void in boolean context → TypeError (LLVM)
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

set +e
NYASH_LLVM_USE_HARNESS=1 $HAKORUNE_BIN --backend llvm apps/tests/phase275_p0_truthiness_void_error_min.hako > /tmp/phase275_truthiness_llvm.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "[PASS] phase275_p0_truthiness_llvm"
    exit 0
else
    echo "[FAIL] expected error, got exit $EXIT_CODE"
    cat /tmp/phase275_truthiness_llvm.txt
    exit 1
fi

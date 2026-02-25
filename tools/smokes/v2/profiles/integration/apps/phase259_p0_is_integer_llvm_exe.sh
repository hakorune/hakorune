#!/bin/bash
# Phase 259 P0: is_integer pattern (boolean predicate scan) - LLVM EXE
# LLVM execution via tools/build_llvm.sh (emit + link + run)
# Note: This uses the full LLVM toolchain, not the Python harness
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"
set +e
NYASH_LLVM_USE_HARNESS=1 $HAKORUNE_BIN --backend llvm apps/tests/phase259_p0_is_integer_min.hako > /tmp/phase259_llvm.txt 2>&1
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 7 ]; then
    echo "[PASS] phase259_p0_is_integer_llvm_exe"
    exit 0
else
    echo "[FAIL] phase259_p0_is_integer_llvm_exe: expected exit 7, got $EXIT_CODE"
    cat /tmp/phase259_llvm.txt
    exit 1
fi

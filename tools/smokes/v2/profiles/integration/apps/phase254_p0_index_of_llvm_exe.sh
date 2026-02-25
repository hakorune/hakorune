#!/bin/bash
# Phase 254 P0: index_of 形 loop (LLVM backend)
set -euo pipefail

HAKO_PATH="apps/tests/phase254_p0_index_of_min.hako"

# Test: "abc".index_of("b") → 1
EXPECTED_EXIT=1

NYASH_LLVM_USE_HARNESS=1 $HAKORUNE_BIN --backend llvm "$HAKO_PATH"
actual_exit=$?

if [[ $actual_exit -eq $EXPECTED_EXIT ]]; then
    echo "✅ phase254_p0_index_of_llvm_exe: PASS (exit=$actual_exit)"
    exit 0
else
    echo "❌ phase254_p0_index_of_llvm_exe: FAIL (expected=$EXPECTED_EXIT, got=$actual_exit)"
    exit 1
fi

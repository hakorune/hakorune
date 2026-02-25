#!/bin/bash
# Phase 283 P0: Pattern3 loop_if_phi fix (LLVM harness backend)
# Tests: loop(i <= 5) with if-else phi (sum += odd numbers)
set -euo pipefail

HAKO_PATH="apps/tests/loop_if_phi.hako"

# LLVM harnessはstdoutを抑制するため、exit=0 (Result: 0) で判定
EXPECTED_EXIT=0

NYASH_LLVM_USE_HARNESS=1 $HAKORUNE_BIN --backend llvm "$HAKO_PATH"
actual_exit=$?

if [[ $actual_exit -eq $EXPECTED_EXIT ]]; then
    echo "✅ phase283_p0_loop_if_phi_llvm: PASS (exit=$actual_exit)"
    exit 0
else
    echo "❌ phase283_p0_loop_if_phi_llvm: FAIL (expected=$EXPECTED_EXIT, got=$actual_exit)"
    exit 1
fi

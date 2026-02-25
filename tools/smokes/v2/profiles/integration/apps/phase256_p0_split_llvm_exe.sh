#!/bin/bash
set -euo pipefail

HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"
HAKO_PATH="apps/tests/phase256_p0_split_min.hako"
EXPECTED_EXIT=3

NYASH_LLVM_USE_HARNESS=1 $HAKORUNE_BIN --backend llvm "$HAKO_PATH" >/dev/null 2>&1
actual_exit=$?

if [[ $actual_exit -eq $EXPECTED_EXIT ]]; then
    echo "✅ phase256_p0_split_llvm_exe: PASS (exit=$actual_exit)"
    exit 0
else
    echo "❌ phase256_p0_split_llvm_exe: FAIL (expected=$EXPECTED_EXIT, got=$actual_exit)"
    exit 1
fi

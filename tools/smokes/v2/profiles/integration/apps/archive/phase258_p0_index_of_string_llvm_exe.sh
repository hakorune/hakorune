#!/bin/bash
# Phase 258 P0: index_of_string (dynamic needle) - LLVM backend
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
HAKORUNE_BIN="${HAKORUNE_BIN:-$PROJECT_ROOT/target/release/hakorune}"

HAKO_PATH="apps/tests/phase258_p0_index_of_string_min.hako"

# Test: "hello world".index_of_string("world") → 6
EXPECTED_EXIT=6

set +e
NYASH_LLVM_USE_HARNESS=1 "$HAKORUNE_BIN" --backend llvm "$PROJECT_ROOT/$HAKO_PATH"
actual_exit=$?
set -e

if [[ $actual_exit -eq $EXPECTED_EXIT ]]; then
    echo "✅ phase258_p0_index_of_string_llvm_exe: PASS (exit=$actual_exit)"
    exit 0
else
    echo "❌ phase258_p0_index_of_string_llvm_exe: FAIL (expected=$EXPECTED_EXIT, got=$actual_exit)"
    echo "    Hint: build LLVM harness via: cargo build --release --features llvm"
    exit 1
fi

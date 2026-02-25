#!/bin/bash
# Phase 274 P2: LLVM TypeOp is/as smoke test
# Tests SSOT alignment with Rust VM for TypeOp semantics
set -e

cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

set +e
NYASH_LLVM_USE_HARNESS=1 $HAKORUNE_BIN --backend llvm \
  apps/tests/phase274_p2_typeop_primitives_only.hako > /tmp/phase274_p2_llvm.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 3 ]; then
    echo "[PASS] phase274_p2_typeop_is_as_llvm"
    exit 0
else
    echo "[FAIL] phase274_p2_typeop_is_as_llvm: expected exit 3, got $EXIT_CODE"
    cat /tmp/phase274_p2_llvm.txt
    exit 1
fi

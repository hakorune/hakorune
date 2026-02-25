#!/bin/bash
# Phase 278 P0: Verify deprecated env vars are rejected

set -e

# Build
cargo build --release --features llvm

# Test 1: NYASH_LLVM_PHI_DEBUG should fail
echo "Test 1: Deprecated NYASH_LLVM_PHI_DEBUG should fail..."
if NYASH_LLVM_PHI_DEBUG=1 NYASH_LLVM_USE_HARNESS=1 \
   ./target/release/hakorune --backend llvm \
   apps/tests/phase275_p0_plus_number_only_min.hako 2>&1 | grep -q "was removed in Phase 278"; then
    echo "✅ Test 1 passed: Deprecated var rejected"
else
    echo "❌ Test 1 failed: Expected error message"
    exit 1
fi

# Test 2: NYASH_LLVM_TRACE_PHI should fail
echo "Test 2: Deprecated NYASH_LLVM_TRACE_PHI should fail..."
if NYASH_LLVM_TRACE_PHI=1 NYASH_LLVM_USE_HARNESS=1 \
   ./target/release/hakorune --backend llvm \
   apps/tests/phase275_p0_plus_number_only_min.hako 2>&1 | grep -q "was removed in Phase 278"; then
    echo "✅ Test 2 passed: Deprecated var rejected"
else
    echo "❌ Test 2 failed: Expected error message"
    exit 1
fi

# Test 3: SSOT vars should work
echo "Test 3: SSOT vars should work..."
if NYASH_LLVM_DEBUG_PHI=1 NYASH_LLVM_USE_HARNESS=1 \
   ./target/release/hakorune --backend llvm \
   apps/tests/phase275_p0_plus_number_only_min.hako > /dev/null 2>&1; then
    echo "✅ Test 3 passed: SSOT var works"
else
    echo "❌ Test 3 failed: SSOT var should work"
    exit 1
fi

echo "✅ All Phase 278 P0 smoke tests passed"

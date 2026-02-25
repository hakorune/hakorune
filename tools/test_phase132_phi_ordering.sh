#!/bin/bash
# Phase 132: Test PHI ordering fix
# This script tests representative cases that had PHI ordering issues

set -e

cd "$(dirname "$0")/.."

echo "=== Phase 132: PHI Ordering Test ==="
echo ""

# Enable debug mode for PHI ordering
export NYASH_PHI_ORDERING_DEBUG=1
export NYASH_CLI_VERBOSE=0

# Test cases
TEST_CASES=(
    "local_tests/phase123_simple_if.hako"
    "local_tests/phase123_while_loop.hako"
    "apps/tests/loop_min_while.hako"
    "apps/tests/joinir_if_select_simple.hako"
)

PASS=0
FAIL=0
RESULTS=()

for test_case in "${TEST_CASES[@]}"; do
    if [ ! -f "$test_case" ]; then
        echo "⚠️  Skipping $test_case (not found)"
        continue
    fi

    echo "---"
    echo "Testing: $test_case"
    echo ""

    # Test with VM backend first (baseline)
    echo "  VM backend..."
    if ./target/release/hakorune --backend vm "$test_case" > /tmp/vm_out.txt 2>&1; then
        VM_EXIT=$?
        VM_OUTPUT=$(cat /tmp/vm_out.txt | grep -E "RC:|Exit code:" | tail -1)
        echo "    ✅ VM: $VM_OUTPUT"
    else
        VM_EXIT=$?
        echo "    ❌ VM failed with exit code $VM_EXIT"
    fi

    # Test with LLVM backend
    echo "  LLVM backend..."
    if NYASH_LLVM_USE_HARNESS=1 NYASH_LLVM_OBJ_OUT=/tmp/test_$$.o \
       ./target/release/hakorune --backend llvm "$test_case" > /tmp/llvm_out.txt 2>&1; then
        LLVM_EXIT=$?
        LLVM_OUTPUT=$(cat /tmp/llvm_out.txt | grep -E "RC:|Exit code:" | tail -1)
        echo "    ✅ LLVM: $LLVM_OUTPUT"

        # Check for PHI ordering warnings
        if grep -q "WARNING.*terminator" /tmp/llvm_out.txt; then
            echo "    ⚠️  PHI ordering warning detected!"
            RESULTS+=("⚠️  $test_case: PHI ordering warning")
        else
            RESULTS+=("✅ $test_case: PASS")
            ((PASS++))
        fi
    else
        LLVM_EXIT=$?
        echo "    ❌ LLVM failed with exit code $LLVM_EXIT"

        # Check error type
        if grep -q "PHI" /tmp/llvm_out.txt; then
            echo "    💥 PHI-related error!"
        fi
        RESULTS+=("❌ $test_case: FAIL (exit $LLVM_EXIT)")
        ((FAIL++))
    fi

    echo ""
done

echo "==================================="
echo "Phase 132 Test Results"
echo "==================================="
for result in "${RESULTS[@]}"; do
    echo "$result"
done
echo ""
echo "Summary: $PASS passed, $FAIL failed out of $((PASS + FAIL)) tests"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "🎉 All tests passed!"
    exit 0
else
    echo "❌ Some tests failed"
    exit 1
fi

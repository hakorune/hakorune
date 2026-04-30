#!/bin/bash
# Phase 188 Task 188-1: Collect [joinir/freeze] error inventory
# Run loop tests with JoinIR-only configuration (no LoopBuilder fallback)

set -e

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

HAKORUNE_BIN="./target/release/hakorune"

# Check if hakorune binary exists
if [ ! -f "$HAKORUNE_BIN" ]; then
    echo "Error: $HAKORUNE_BIN not found. Run 'cargo build --release' first."
    exit 1
fi

# JoinIR-only configuration (JoinIR は常時 ON、NYASH_JOINIR_CORE は deprecated/no-op)
export NYASH_LEGACY_LOOPBUILDER=0
export NYASH_DISABLE_PLUGINS=1

# Test files
TEST_FILES=(
    "apps/tests/joinir_if_merge_multiple.hako"
    "apps/tests/joinir_if_merge_simple.hako"
    "apps/tests/joinir_if_select_local.hako"
    "apps/tests/joinir_if_select_simple.hako"
    "apps/tests/joinir_min_loop.hako"
    "apps/tests/loop_if_phi.hako"
    "apps/tests/loop_if_phi_continue.hako"
    "apps/tests/loop_min_while.hako"
    "apps/tests/loop_phi_one_sided.hako"
)

echo "=========================================="
echo "Phase 188 Task 188-1: JoinIR Error Inventory"
echo "Configuration:"
echo "  NYASH_LEGACY_LOOPBUILDER=0"
echo "  NYASH_DISABLE_PLUGINS=1"
echo "=========================================="
echo ""

TOTAL=0
FAILED=0
PASSED=0

for test_file in "${TEST_FILES[@]}"; do
    TOTAL=$((TOTAL + 1))
    echo "----------------------------------------"
    echo "[$TOTAL] Testing: $test_file"
    echo "----------------------------------------"

    if [ ! -f "$test_file" ]; then
        echo "SKIP: File not found"
        echo ""
        continue
    fi

    # Run test and capture both stdout and stderr
    if "$HAKORUNE_BIN" "$test_file" 2>&1; then
        echo "PASS"
        PASSED=$((PASSED + 1))
    else
        echo "FAIL (exit code: $?)"
        FAILED=$((FAILED + 1))
    fi
    echo ""
done

echo "=========================================="
echo "Summary:"
echo "  Total:  $TOTAL"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo "=========================================="

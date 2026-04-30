#!/bin/bash
# FileBox Fallback Smoke Test
# Tests all three NYASH_FILEBOX_MODE modes: auto, core-ro, plugin-only

set -e

cd "$(dirname "$0")/../../.."

# Build first
echo "=== Building hakorune ==="
cargo build --release

HAKO="./target/release/hakorune"
TEST_FILE="local_tests/test_filebox_fallback.hako"

# Test 1: Auto mode (default) - should fallback to builtin
echo ""
echo "=== Test 1: Auto mode (default, plugins disabled) ==="
if NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=auto "$HAKO" "$TEST_FILE" 2>&1 | grep -E "FileBox.*builtin.*core-ro.*fallback|created successfully"; then
    echo "✓ Test 1 passed: Auto mode uses builtin fallback"
else
    echo "✗ Test 1 failed"
    exit 1
fi

# Test 2: Core-ro mode - should use builtin directly
echo ""
echo "=== Test 2: Core-ro mode ==="
if NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=core-ro "$HAKO" "$TEST_FILE" 2>&1 | grep -E "FileBox.*builtin.*core-ro|created successfully"; then
    echo "✓ Test 2 passed: Core-ro mode uses builtin"
else
    echo "✗ Test 2 failed"
    exit 1
fi

# Test 3: Plugin-only mode - should fail when plugins disabled
echo ""
echo "=== Test 3: Plugin-only mode (should fail) ==="
if ! NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=plugin-only "$HAKO" "$TEST_FILE" 2>&1; then
    echo "✓ Test 3 passed: Plugin-only mode correctly fails when plugins disabled"
else
    echo "✗ Test 3 failed: Plugin-only mode should have failed"
    exit 1
fi

echo ""
echo "=== All fallback tests passed! ==="

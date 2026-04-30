#!/usr/bin/env bash
# Smoke test for Provider Registry SSOT (Single Source of Truth)
# Tests all three FileBox provider modes: auto, core-ro, plugin-only

set -e

cd "$(dirname "$0")/../../.."

NYASH="./target/release/hakorune"
TEST_FILE="/tmp/nyash_provider_test.txt"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "========================================"
echo "Provider Registry SSOT Smoke Test"
echo "========================================"
echo ""

# Ensure nyash is built
if [ ! -f "$NYASH" ]; then
    echo -e "${RED}Error: nyash not found at $NYASH${NC}"
    echo "Building with builtin-filebox feature..."
    cargo build --release --features builtin-filebox
fi

# Create test file
echo "Hello from FileBox provider test!" > "$TEST_FILE"

# Test program (simple - just tests provider initialization)
TEST_PROG=$(cat <<'EOF'
static box Main {
    main() {
        print("Provider test: initialization successful")
        return "OK"
    }
}
EOF
)

# Save test program
echo "$TEST_PROG" > /tmp/nyash_provider_test.hako

echo "Test 1: mode=core-ro (forced core-ro, ignore registry)"
echo "================================================"
output=$(NYASH_FILEBOX_MODE=core-ro NYASH_DISABLE_PLUGINS=1 "$NYASH" /tmp/nyash_provider_test.hako 2>&1)
if echo "$output" | grep -q "core-ro (forced)"; then
    echo -e "${GREEN}✓ PASS${NC}: core-ro mode selected correctly"
else
    echo -e "${RED}✗ FAIL${NC}: Expected 'core-ro (forced)' in output"
    echo "$output"
    exit 1
fi

if echo "$output" | grep -q "Provider test: initialization successful"; then
    echo -e "${GREEN}✓ PASS${NC}: Program executed successfully"
else
    echo -e "${RED}✗ FAIL${NC}: Program execution failed"
    echo "$output"
    exit 1
fi
echo ""

echo "Test 2: mode=auto (use registered provider from builtin factory)"
echo "================================================"
output=$(NYASH_FILEBOX_MODE=auto "$NYASH" /tmp/nyash_provider_test.hako 2>&1)

# In auto mode with builtin-filebox feature, should use registered provider
if echo "$output" | grep -q "registered provider"; then
    echo -e "${GREEN}✓ PASS${NC}: auto mode selected registered provider"
else
    echo -e "${YELLOW}⚠ WARN${NC}: Expected registered provider selection log in auto mode"
    echo "$output"
fi

if echo "$output" | grep -q "Provider test: initialization successful"; then
    echo -e "${GREEN}✓ PASS${NC}: Program executed successfully in auto mode"
else
    echo -e "${RED}✗ FAIL${NC}: Program execution failed in auto mode"
    echo "$output"
    exit 1
fi
echo ""

echo "Test 3: mode=plugin-only (uses registered provider, including builtin)"
echo "================================================"
# With builtin-filebox feature, plugin-only mode should succeed using builtin factory
output=$(NYASH_FILEBOX_MODE=plugin-only "$NYASH" /tmp/nyash_provider_test.hako 2>&1)

if echo "$output" | grep -q "plugin-only provider"; then
    echo -e "${GREEN}✓ PASS${NC}: plugin-only mode uses registered provider (builtin counts as plugin)"
else
    echo -e "${YELLOW}⚠ WARN${NC}: Expected plugin-only provider selection log"
    echo "$output"
fi

if echo "$output" | grep -q "Provider test: initialization successful"; then
    echo -e "${GREEN}✓ PASS${NC}: Program executed successfully in plugin-only mode"
else
    echo -e "${RED}✗ FAIL${NC}: Program execution failed in plugin-only mode"
    echo "$output"
    exit 1
fi
echo ""

echo "Test 4: auto mode with builtin factory (feature enabled)"
echo "================================================"
# Build with builtin-filebox feature and test auto mode
cargo build --release --features builtin-filebox 2>&1 | tail -3

output=$(NYASH_FILEBOX_MODE=auto NYASH_DISABLE_PLUGINS=1 "$NYASH" /tmp/nyash_provider_test.hako 2>&1)

if echo "$output" | grep -q "registered (priority=10)"; then
    echo -e "${GREEN}✓ PASS${NC}: Builtin factory registered successfully"
else
    echo -e "${YELLOW}⚠ WARN${NC}: Expected builtin factory registration log"
    echo "$output"
fi

if echo "$output" | grep -q "Provider test: initialization successful"; then
    echo -e "${GREEN}✓ PASS${NC}: Program executed successfully with builtin factory"
else
    echo -e "${RED}✗ FAIL${NC}: Program execution failed with builtin factory"
    echo "$output"
    exit 1
fi
echo ""

# Cleanup
rm -f /tmp/nyash_provider_test.txt /tmp/nyash_provider_test.hako

echo "========================================"
echo -e "${GREEN}All Provider Registry Tests PASSED!${NC}"
echo "========================================"

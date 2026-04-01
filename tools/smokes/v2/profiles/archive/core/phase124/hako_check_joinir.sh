#!/bin/bash
# Phase 124: archived hako_check JoinIR-only pin (legacy path removed)

set -e

PROFILE_NAME="hako_check_joinir"
DESCRIPTION="Archived hako_check JoinIR-only pin (Phase 124 consolidation)"

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Test cases
test_cases=(
    "phase123_simple_if.hako"
    "phase123_nested_if.hako"
    "phase123_while_loop.hako"
    "phase123_if_in_loop.hako"
)

# Root directory
ROOT="$(cd "$(dirname "$0")/../../../../.." && pwd)"
BIN="${ROOT}/target/release/hakorune"

echo "=========================================="
echo " Phase 124: hako_check JoinIR-Only Test"
echo "=========================================="
echo ""

# Check if binary exists
if [ ! -f "$BIN" ]; then
    echo -e "${RED}Error: Binary not found at $BIN${NC}"
    echo "Please run: cargo build --release"
    exit 1
fi

# Test counters
joinir_pass=0
joinir_fail=0

echo "=== Testing JoinIR-Only Path (Phase 124: No environment variables) ==="
echo ""

for case in "${test_cases[@]}"; do
    test_file="${ROOT}/local_tests/${case}"

    if [ ! -f "$test_file" ]; then
        echo -e "${YELLOW}Warning: Test file not found: $test_file${NC}"
        continue
    fi

    echo -n "Testing $case (joinir-only)... "

    # Phase 124: No environment variables - JoinIR is the only path
    if timeout 10 "$BIN" --backend vm "$test_file" >/dev/null 2>&1; then
        echo -e "${GREEN}PASS${NC}"
        joinir_pass=$((joinir_pass + 1))
    else
        echo -e "${RED}FAIL${NC}"
        joinir_fail=$((joinir_fail + 1))
    fi
done

echo ""
echo "=========================================="
echo " Archived Pin Summary"
echo "=========================================="
echo ""
echo "JoinIR-Only Path (Phase 124):"
echo "  PASS: $joinir_pass"
echo "  FAIL: $joinir_fail"
echo ""

# Determine overall result
if [ $joinir_fail -eq 0 ]; then
    echo -e "${GREEN}✓ All tests PASSED (JoinIR-only)${NC}"
    echo "Archived Phase 124: hako_check JoinIR-only pin replay completed"
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi

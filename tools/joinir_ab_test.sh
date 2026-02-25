#!/bin/bash
# Phase 33-8: A/B test automation for JoinIR lowering
#
# Usage:
#   tools/joinir_ab_test.sh <test_file.hako>
#
# Example:
#   tools/joinir_ab_test.sh apps/tests/joinir_if_merge_simple.hako

set -euo pipefail

test_case=$1  # e.g., "apps/tests/joinir_if_merge_simple.hako"

if [ ! -f "$test_case" ]; then
    echo "❌ Test file not found: $test_case"
    exit 1
fi

echo "🧪 Testing: $test_case"
echo ""

# Route A: Traditional if_phi
echo "=== Route A (if_phi) ==="
HAKO_JOINIR_IF_SELECT=0 \
NYASH_FEATURES=stage3 \
NYASH_FEATURES=stage3 \
    ./target/release/hakorune "$test_case" \
    > /tmp/route_a.out 2>&1
route_a_rc=$?
echo "Route A RC: $route_a_rc"
echo ""

# Route B: JoinIR Select/IfMerge
echo "=== Route B (JoinIR) ==="
HAKO_JOINIR_IF_SELECT=1 \
HAKO_JOINIR_STAGE1=1 \
HAKO_JOINIR_DEBUG=1 \
NYASH_FEATURES=stage3 \
NYASH_FEATURES=stage3 \
    ./target/release/hakorune "$test_case" \
    > /tmp/route_b.out 2>&1
route_b_rc=$?
echo "Route B RC: $route_b_rc"
echo ""

# Comparison
echo "=== 📊 Comparison ==="

# RC check
if [ $route_a_rc -eq $route_b_rc ]; then
    echo "✅ RC matched: $route_a_rc"
else
    echo "❌ RC mismatch: A=$route_a_rc, B=$route_b_rc"
    exit 1
fi

# Output check (ignore debug logs starting with '[')
if diff <(grep -v '^\[' /tmp/route_a.out) <(grep -v '^\[' /tmp/route_b.out); then
    echo "✅ Output matched"
else
    echo "❌ Output differs:"
    diff <(grep -v '^\[' /tmp/route_a.out) <(grep -v '^\[' /tmp/route_b.out) || true
    exit 1
fi

# Extract lowering info from Route B
echo ""
echo "=== 🔍 Lowering Info ==="
grep -E "IfMerge|IfSelect|if_phi" /tmp/route_b.out || echo "⚠️ No lowering info found"

echo ""
echo "🎉 A/B test PASSED for $test_case"

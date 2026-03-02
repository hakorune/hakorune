#!/bin/bash
set -e
ROOT="$(cd "$(dirname "$0")" && pwd)"
EMIT_ROUTE="$ROOT/tools/smokes/v2/lib/emit_mir_route.sh"

echo "=== Phase 25 MVP: Testing PHI Type Propagation Fix ==="
echo ""

# Test 1: Simple case (should still work)
echo "Test 1: Simple matmul case"
echo "------------------------"
cat > /tmp/test_matmul_simple.hako << 'EOF'
using numeric.core as nc

static box Main {
  main() {
    local a = nc.MatI64.new(2, 2)
    local b = nc.MatI64.new(2, 2)
    local c = a.mul_naive(b)
    return c
  }
}
EOF

# Build with AOT prep
if [ ! -x "$EMIT_ROUTE" ]; then
  echo "❌ emit route helper missing: $EMIT_ROUTE"
  exit 1
fi

HAKO_APPLY_AOT_PREP=1 NYASH_AOT_NUMERIC_CORE=1 NYASH_AOT_NUMERIC_CORE_TRACE=1 \
  "$EMIT_ROUTE" --route hako-helper --timeout-secs 60 --out /tmp/test_simple_result.json --input /tmp/test_matmul_simple.hako

# Check for transformation
if grep -q '"name":"NyNumericMatI64.mul_naive"' /tmp/test_simple_result.json; then
  echo "✅ Simple case: transformation SUCCESS"
else
  echo "❌ Simple case: transformation FAILED"
  exit 1
fi

echo ""
echo "Test 2: Complex matmul_core case"
echo "--------------------------------"

# Test 2: Complex case (matmul_core with PHI chains)
NYASH_AOT_NUMERIC_CORE=1 NYASH_AOT_NUMERIC_CORE_TRACE=1 \
  tools/perf/microbench.sh --case matmul_core --backend llvm --exe --runs 1 --n 4 2>&1 | tee /tmp/matmul_core_test.log

# Check trace output for successful transformation
if grep -q "MatI64 vids:" /tmp/matmul_core_test.log; then
  echo "✅ Complex case: MatI64 vids detected"
else
  echo "⚠️  Complex case: MatI64 vids not shown (might be OK if trace is off)"
fi

if grep -q "transformed.*BoxCall.*Call" /tmp/matmul_core_test.log; then
  echo "✅ Complex case: transformation SUCCESS"
else
  echo "❌ Complex case: transformation FAILED"
  echo "Check /tmp/matmul_core_test.log for details"
  exit 1
fi

# Check that we don't see untransformed boxcalls
if grep -q '\[mir_json/numeric_core\].*boxcall("mul_naive")' /tmp/matmul_core_test.log; then
  echo "⚠️  Complex case: still seeing untransformed boxcalls"
  exit 1
else
  echo "✅ Complex case: no untransformed boxcalls detected"
fi

echo ""
echo "=== All tests PASSED ==="
echo ""
echo "Diagnostic logs:"
grep -E '\[aot/numeric_core\]' /tmp/matmul_core_test.log | head -20

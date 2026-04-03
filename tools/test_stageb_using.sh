#!/usr/bin/env bash
# Compatibility / regression test for Stage-B using resolution
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Build if needed
if [ ! -f "$ROOT/target/release/nyash" ]; then
    echo "Building nyash..."
    cd "$ROOT"
    cargo build --release
fi

NYASH_BIN="$ROOT/target/release/nyash"
EMIT_ROUTE="$ROOT/tools/smokes/v2/lib/emit_mir_route.sh"

# Test 1: Basic using resolution (currently should work without HAKO_STAGEB_USING_RESOLVE)
echo "=== Test 1: Basic Stage-B compilation (compat path) ==="
cat > /tmp/test_stageb_basic.hako <<'EOF'
local x = 42
return x
EOF

if "$NYASH_BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "local x = 42; return x" 2>&1 | grep -q '"kind":"Program"'; then
    echo "✓ Test 1 PASSED: Basic Stage-B works"
else
    echo "✗ Test 1 FAILED: Basic Stage-B broken"
    exit 1
fi

# Test 2: Using resolution enabled (opt-in)
echo ""
echo "=== Test 2: Using resolution with opt-in ==="
cat > /tmp/test_stageb_using.hako <<'EOF'
using selfhost.shared.common.string_helpers as StringHelpers
local result = StringHelpers.skip_ws("  hello")
return result
EOF

# First test: with using resolution disabled (default)
echo "Testing with using resolution DISABLED (should fail or ignore)..."
if HAKO_STAGEB_USING_RESOLVE=0 "$NYASH_BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat /tmp/test_stageb_using.hako)" 2>&1 | grep -q '"kind":"Program"'; then
    echo "✓ Test 2a PASSED: Stage-B works without using resolution"
else
    echo "Note: Stage-B without using resolution may fail (expected for using statements)"
fi

# Second test: with using resolution enabled
echo ""
echo "Testing with using resolution ENABLED..."

# Create modules JSON from nyash.toml
MODULES_JSON='{"selfhost.shared.common.string_helpers":"lang/src/shared/common/string_helpers.hako"}'

if HAKO_STAGEB_USING_RESOLVE=1 HAKO_MODULES_JSON="$MODULES_JSON" "$NYASH_BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat /tmp/test_stageb_using.hako)" 2>&1 | grep -q '"kind":"Program"'; then
    echo "✓ Test 2b PASSED: Stage-B with using resolution works"
else
    echo "✗ Test 2b FAILED: Stage-B with using resolution broken"
    echo "Debugging output:"
    HAKO_STAGEB_USING_RESOLVE=1 HAKO_MODULES_JSON="$MODULES_JSON" "$NYASH_BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat /tmp/test_stageb_using.hako)" 2>&1 | tail -20
fi

# Test 3: Verify shared emit route wrapper still works
echo ""
echo "=== Test 3: Verify emit route wrapper compatibility ==="
cat > /tmp/test_emit_simple.hako <<'EOF'
local i = 0
loop(i < 10) {
  i = i + 1
}
return i
EOF

if [ ! -x "$EMIT_ROUTE" ]; then
    echo "✗ Test 3 FAILED: emit route helper missing: $EMIT_ROUTE"
    exit 1
fi

if "$EMIT_ROUTE" --route hako-helper --timeout-secs 30 --out /tmp/test_emit_output.json --input /tmp/test_emit_simple.hako 2>&1 | grep -q "OK"; then
    echo "✓ Test 3 PASSED: emit route wrapper still works"
else
    echo "✗ Test 3 FAILED: emit route wrapper broken"
fi

echo ""
echo "=== All tests completed ==="

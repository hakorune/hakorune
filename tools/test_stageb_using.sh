#!/usr/bin/env bash
# Test Stage-B using resolution functionality
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

# Test 1: Basic using resolution (currently should work without HAKO_STAGEB_USING_RESOLVE)
echo "=== Test 1: Basic Stage-B compilation (existing path) ==="
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

# Test 3: Verify existing emit_mir.sh path still works
echo ""
echo "=== Test 3: Verify hakorune_emit_mir.sh compatibility ==="
cat > /tmp/test_emit_simple.hako <<'EOF'
local i = 0
loop(i < 10) {
  i = i + 1
}
return i
EOF

if bash "$ROOT/tools/hakorune_emit_mir.sh" /tmp/test_emit_simple.hako /tmp/test_emit_output.json 2>&1 | grep -q "OK"; then
    echo "✓ Test 3 PASSED: emit_mir.sh still works"
else
    echo "✗ Test 3 FAILED: emit_mir.sh broken"
fi

echo ""
echo "=== All tests completed ==="

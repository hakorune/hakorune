#!/bin/bash
# Stage-B minimal harness test script
# Tests stageb_min_sample.hako through Stage-B compilation pipeline

set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"

NYASH_BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
TEST_FILE="$ROOT/lang/src/compiler/tests/stageb_min_sample.hako"

# Stage‑B / LoopSSA related env (can be overridden by caller)
# - HAKO_LOOPSSA_EXIT_PHI: 1=LoopSSA ON (dev検証), 0=OFF（既定は0: 安全側）
# - HAKO_COMPILER_BUILDER_TRACE: 1=LoopSSA/Builder周辺の詳細トレース
# - NYASH_VM_TRACE: 1=Rust VM の binop/SSA トレース（重いので手動ON推奨）
HAKO_LOOPSSA_EXIT_PHI="${HAKO_LOOPSSA_EXIT_PHI:-0}"
HAKO_COMPILER_BUILDER_TRACE="${HAKO_COMPILER_BUILDER_TRACE:-0}"
NYASH_VM_TRACE="${NYASH_VM_TRACE:-0}"

echo "=== Stage-B Minimal Harness Test ==="
echo "Test file: $TEST_FILE"
echo "Binary: $NYASH_BIN"
echo ""

# Test 1: Direct VM execution (should work)
echo "--- Test 1: Direct VM execution ---"
NYASH_DISABLE_PLUGINS=1 NYASH_FEATURES=stage3 \
  "$NYASH_BIN" --backend vm "$TEST_FILE" 2>&1 | tail -10
echo ""

# Test 2: Stage-B compilation (may have ValueId errors)
echo "--- Test 2: Stage-B compilation ---"
NYASH_JSON_ONLY=1 \
NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
HAKO_STAGEB_FUNC_SCAN=1 \
NYASH_FEATURES=stage3 \
NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
HAKO_LOOPSSA_EXIT_PHI="$HAKO_LOOPSSA_EXIT_PHI" \
HAKO_COMPILER_BUILDER_TRACE="$HAKO_COMPILER_BUILDER_TRACE" \
NYASH_VM_TRACE="$NYASH_VM_TRACE" \
  "$NYASH_BIN" --backend vm lang/src/compiler/entry/compiler_stageb.hako \
    -- --source "$(cat "$TEST_FILE")" 2>&1 | tail -20
echo ""

# Test 3: MIR verification (check for SSA errors)
echo "--- Test 3: MIR verification ---"
NYASH_VM_VERIFY_MIR=1 \
NYASH_DISABLE_PLUGINS=1 NYASH_FEATURES=stage3 \
  "$NYASH_BIN" --backend vm "$TEST_FILE" 2>&1 | \
  grep -E "(Undefined|verification|✅)" || echo "No verification errors (good!)"
echo ""

echo "=== Test complete ==="

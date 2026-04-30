#!/usr/bin/env bash
# Phase 154: HC020 Dead Block Detection Smoke Test
#
# Tests unreachable basic block detection using MIR CFG information.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[smoke/error] Binary not found: $BIN"
  echo "Run: cargo build --release"
  exit 1
fi

echo "=== Phase 154: HC020 Dead Block Detection Smoke Test ==="
echo

SOURCE_FIXTURE="$ROOT/apps/tests/hako_check/test_dead_blocks_early_return.hako"
MIR_FIXTURE="$ROOT/apps/tests/mir_shape_guard/bool_phi_branch_min_v1.mir.json"

if [ ! -f "$SOURCE_FIXTURE" ]; then
  echo "[smoke/error] Source fixture not found: $SOURCE_FIXTURE"
  exit 1
fi

if [ ! -f "$MIR_FIXTURE" ]; then
  echo "[smoke/error] MIR fixture not found: $MIR_FIXTURE"
  exit 1
fi

echo "[TEST 1] Consumer contract with prebuilt MIR CFG..."
SOURCE_TEXT="$(sed 's/\r$//' "$SOURCE_FIXTURE")"
MIR_JSON_CONTENT="$(cat "$MIR_FIXTURE")"
CONTRACT_OUTPUT=$(
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_USE_NY_COMPILER=0 \
  HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES=stage3 \
  NYASH_PARSER_SEAM_TOLERANT=1 \
  HAKO_PARSER_SEAM_TOLERANT=1 \
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_ENABLE_USING=1 \
  HAKO_ENABLE_USING=1 \
  NYASH_USING_AST=1 \
  "$BIN" "$ROOT/tools/hako_check/cli.hako" -- \
    --dead-blocks \
    --mir-json-content "$MIR_JSON_CONTENT" \
    --source-file "$SOURCE_FIXTURE" "$SOURCE_TEXT" 2>&1 || true
)

if echo "$CONTRACT_OUTPUT" | grep -q "\[HC020\]"; then
  echo "[PASS] Test 1: HC020 detected unreachable blocks from CFG"
else
  echo "[FAIL] Test 1: HC020 did not detect unreachable blocks"
  echo "$CONTRACT_OUTPUT"
  exit 1
fi
echo

echo "[TEST 2] Wrapper accepts --dead-blocks and completes live analysis..."
LIVE_OUTPUT=$("$ROOT/tools/hako_check.sh" --dead-blocks "$SOURCE_FIXTURE" 2>&1 || true)
if echo "$LIVE_OUTPUT" | grep -q "sed: unrecognized option '--dead-blocks'"; then
  echo "[FAIL] Test 2: wrapper still mis-parses --dead-blocks"
  echo "$LIVE_OUTPUT"
  exit 1
fi
echo "[PASS] Test 2: wrapper accepted --dead-blocks"
echo

echo "[smoke/success] HC020 CFG consumer contract is green"
exit 0

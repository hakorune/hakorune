#!/usr/bin/env bash
# Phase 153: Dead Code Detection Smoke Test
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[ERROR] hakorune binary not found: $BIN" >&2
  echo "Run: cargo build --release" >&2
  exit 2
fi

echo "=== Phase 153: Dead Code Detection Smoke Test ===" >&2

# Test 1: Basic dead code detection with --dead-code flag
echo "[TEST 1] Testing --dead-code flag with dead method..." >&2
TEST1_FILE="$ROOT/tools/hako_check/tests/HC019_dead_code/ng.hako"
TEST1_OUTPUT=$(NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_FEATURES=stage3 \
  "$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- --dead-code --source-file "$TEST1_FILE" "$(cat "$TEST1_FILE")" 2>&1 || true)

if echo "$TEST1_OUTPUT" | grep -q "HC019.*unreachable method"; then
  echo "[PASS] Test 1: Dead method detected" >&2
else
  echo "[FAIL] Test 1: Dead method NOT detected" >&2
  echo "Output: $TEST1_OUTPUT" >&2
  exit 1
fi

if echo "$TEST1_OUTPUT" | grep -q "HC019.*dead static box"; then
  echo "[PASS] Test 1: Dead static box detected" >&2
else
  echo "[FAIL] Test 1: Dead static box NOT detected" >&2
  echo "Output: $TEST1_OUTPUT" >&2
  exit 1
fi

# Test 2: Clean code (no dead code)
echo "[TEST 2] Testing --dead-code flag with clean code..." >&2
TEST2_FILE="$ROOT/tools/hako_check/tests/HC019_dead_code/ok.hako"
TEST2_OUTPUT=$(NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_FEATURES=stage3 \
  "$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- --dead-code --source-file "$TEST2_FILE" "$(cat "$TEST2_FILE")" 2>&1 || true)

if echo "$TEST2_OUTPUT" | grep -q "HC019"; then
  echo "[FAIL] Test 2: False positive detected" >&2
  echo "Output: $TEST2_OUTPUT" >&2
  exit 1
else
  echo "[PASS] Test 2: No false positives" >&2
fi

# Test 3: --rules dead_code flag
echo "[TEST 3] Testing --rules dead_code..." >&2
TEST3_OUTPUT=$(NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_FEATURES=stage3 \
  "$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- --rules dead_code --source-file "$TEST1_FILE" "$(cat "$TEST1_FILE")" 2>&1 || true)

if echo "$TEST3_OUTPUT" | grep -q "HC019"; then
  echo "[PASS] Test 3: --rules dead_code works" >&2
else
  echo "[FAIL] Test 3: --rules dead_code did not work" >&2
  echo "Output: $TEST3_OUTPUT" >&2
  exit 1
fi

# Test 4: JSON-LSP format with --dead-code
echo "[TEST 4] Testing JSON-LSP format with --dead-code..." >&2
TEST4_OUTPUT=$(NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_FEATURES=stage3 \
  "$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- --format json-lsp --dead-code --source-file "$TEST1_FILE" "$(cat "$TEST1_FILE")" || true)

if echo "$TEST4_OUTPUT" | grep -q '"rule":"HC019"'; then
  echo "[PASS] Test 4: JSON-LSP format works" >&2
else
  echo "[FAIL] Test 4: JSON-LSP format did not work" >&2
  echo "Output: $TEST4_OUTPUT" >&2
  exit 1
fi

echo "" >&2
echo "=== All Dead Code Detection Tests Passed ===" >&2
exit 0

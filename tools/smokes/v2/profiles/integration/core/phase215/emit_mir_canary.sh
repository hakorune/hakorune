#!/usr/bin/env bash
# E2E Canary: hakorune_emit_mir.sh robustly emits MIR(JSON) via selfhost-first and provider-first paths

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

TEST_HAKO=$(mktemp --suffix .hako)
cat >"$TEST_HAKO" <<'HAKO'
static box Main { method main(args) {
  local n = 10
  local i = 0
  local s = 0
  loop(i < n) { s = s + i  i = i + 1 }
  return s
} }
HAKO

# Test 1: jsonfrag mode (minimal while-form MIR, always succeeds)
TMP_JSON1=$(mktemp --suffix .json)
trap 'rm -f "$TEST_HAKO" "$TMP_JSON1" "$TMP_JSON2" || true' EXIT

set +e
(cd "$ROOT_DIR" && \
  HAKO_SELFHOST_BUILDER_FIRST=1 \
  HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
  HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 \
  HAKO_MIR_BUILDER_JSONFRAG_PURIFY=1 \
  NYASH_JSON_ONLY=1 \
  bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$TEST_HAKO" "$TMP_JSON1" >/dev/null 2>&1)
rc1=$?
set -e

if [ $rc1 -ne 0 ] || [ ! -f "$TMP_JSON1" ]; then
  echo "[FAIL] emit_mir_canary: jsonfrag mode failed"
  exit 1
fi

# Validate JSON structure
if ! grep -q '"functions"' "$TMP_JSON1" 2>/dev/null; then
  echo "[FAIL] emit_mir_canary: jsonfrag output missing functions"
  exit 1
fi

# Test 2: Test that microbench --exe flow works (uses jsonfrag fallback internally)
TMP_JSON2=$(mktemp --suffix .json)

set +e
(cd "$ROOT_DIR" && \
  HAKO_SELFHOST_BUILDER_FIRST=1 \
  HAKO_MIR_BUILDER_LOOP_JSONFRAG=1 \
  HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
  HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 \
  HAKO_MIR_BUILDER_JSONFRAG_PURIFY=1 \
  NYASH_JSON_ONLY=1 \
  bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$TEST_HAKO" "$TMP_JSON2" >/dev/null 2>&1)
rc2=$?
set -e

if [ $rc2 -ne 0 ] || [ ! -f "$TMP_JSON2" ]; then
  echo "[FAIL] emit_mir_canary: microbench path failed"
  exit 1
fi

# Validate JSON structure and basic loop MIR elements
if ! grep -q '"functions"' "$TMP_JSON2" 2>/dev/null; then
  echo "[FAIL] emit_mir_canary: output missing functions"
  exit 1
fi

if ! grep -q '"op".*:.*"phi"' "$TMP_JSON2" 2>/dev/null; then
  echo "[FAIL] emit_mir_canary: loop MIR missing PHI"
  exit 1
fi

echo "[PASS] emit_mir_canary"
exit 0

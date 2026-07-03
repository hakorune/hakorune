#!/bin/bash
# hako_min_compile_return_vm.sh — Selfhost Compiler minimal v0 compile→run canary

set -euo pipefail

# Opt-in guard: skip unless explicitly enabled
if [ "${SMOKES_ENABLE_HAKO_MIN:-0}" != "1" ]; then
  echo "[WARN] SKIP hako_min_compile_return_vm (enable with SMOKES_ENABLE_HAKO_MIN=1)" >&2
  exit 0
fi

ROOT="$(cd "$(dirname "$0")/../../../../../.." && pwd)"
HAKO_BIN_DEFAULT="$ROOT/tools/bin/hako"
HAKO_BIN="${HAKO_BIN:-$HAKO_BIN_DEFAULT}"

fail() { echo "[FAIL] $*" >&2; exit 1; }
pass() { echo "[PASS] $*" >&2; }

if [ ! -x "$HAKO_BIN" ]; then
  echo "[SKIP] Hako binary not found: $HAKO_BIN (set HAKO_BIN to override)" >&2
  exit 0
fi

TMP_SRC="/tmp/hako_min_src_$$.hako"
TMP_JSON="/tmp/hako_min_out_$$.json"
trap 'rm -f "$TMP_SRC" "$TMP_JSON"' EXIT

cat > "$TMP_SRC" <<'HK'
// dummy; not used in Stage-A (args only)
box Main { static method main() { return 0 } }
HK

# Compile to JSON v0 via selfhost compiler (Stage‑A: args only)
RAW="/tmp/hako_min_out_raw_$$.txt"
trap 'rm -f "$TMP_SRC" "$TMP_JSON" "$RAW"' EXIT
NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_SYNTAX_SUGAR_LEVEL=full \
  HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
  "$HAKO_BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler.hako" -- --min-json --return-int 42 > "$RAW" 2>/dev/null || true

# Extract first JSON v0 Program line
awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$RAW" > "$TMP_JSON"
if [ ! -s "$TMP_JSON" ]; then
  echo "[FAIL] JSON v0 not produced" >&2
  exit 1
fi

# Execute JSON v0
OUT=$("$HAKO_BIN" --json-file "$TMP_JSON" 2>&1)
ACTUAL=$(printf '%s\n' "$OUT" | awk -F': ' '/^Result:/ { val=$2 } END { print val }')
if [ -z "$ACTUAL" ]; then
  echo "[FAIL] could not parse Result line" >&2
  printf '%s\n' "$OUT" >&2
  fail "hako_min_compile_return_vm"
fi
if [ "$ACTUAL" != "42" ]; then
  echo "Expected: 42" >&2
  echo "Actual:   $ACTUAL" >&2
  printf '%s\n' "$OUT" >&2
  fail "hako_min_compile_return_vm"
fi

pass "hako_min_compile_return_vm"

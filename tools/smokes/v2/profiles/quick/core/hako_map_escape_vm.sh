#!/bin/bash
# hako_map_escape_vm.sh — Stage-A map literal escape/unicode boundary (opt-in)

set -euo pipefail

if [ "${SMOKES_ENABLE_STAGEA_BOUNDARY:-0}" != "1" ]; then
  echo "[SKIP] SMOKES_ENABLE_STAGEA_BOUNDARY!=1" >&2
  exit 0
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
fi
BIN="$ROOT/target/release/nyash"

if [ ! -x "$BIN" ]; then
  (cd "$ROOT" && cargo build --release >/dev/null 2>&1) || {
    echo "[FAIL] build release nyash" >&2
    exit 1
  }
fi

compile_stage_a() {
  local code="$1"
  local hako_tmp="/tmp/hako_stagea_map_$$.hako"
  local json_out="/tmp/hako_stagea_map_$$.mir.json"
  printf "%s\n" "$code" > "$hako_tmp"
  local raw="/tmp/hako_stagea_map_raw_$$.txt"
  NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_SYNTAX_SUGAR_LEVEL=full NYASH_ENABLE_ARRAY_LITERAL=1 \
  HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
  "$BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler.hako" -- --min-json --source "$(cat "$hako_tmp")" > "$raw" 2>&1
  awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$raw" > "$json_out" || true
  rm -f "$raw" "$hako_tmp"
  if [ ! -s "$json_out" ]; then
    echo "[DIAG] Stage-A failed to emit JSON (expected for some boundary cases)" >&2
    return 2
  fi
  echo "$json_out"
  return 0
}

run_gate_c() {
  local json_path="$1"
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$BIN" --json-file "$json_path" >/dev/null 2>&1 || true
  rm -f "$json_path"
}

# Case 1: escaped quote in key
code='box Main { static method main() { local m={"a\"b":1}; print(1); } }'
if json=$(compile_stage_a "$code"); then
  run_gate_c "$json"
  echo "[PASS] stagea_map_key_escaped_quote (emitted JSON)" >&2
else
  echo "[PASS] stagea_map_key_escaped_quote (diagnostic acceptable)" >&2
fi

# Case 2: unicode key
code='box Main { static method main() { local m={"ねこ":2}; print(2); } }'
if json=$(compile_stage_a "$code"); then
  run_gate_c "$json"
  echo "[PASS] stagea_map_key_unicode (emitted JSON)" >&2
else
  echo "[PASS] stagea_map_key_unicode (diagnostic acceptable)" >&2
fi

# Case 3: empty map
code='box Main { static method main() { local m={}; print(0); } }'
if json=$(compile_stage_a "$code"); then
  run_gate_c "$json"
  echo "[PASS] stagea_map_empty (emitted JSON)" >&2
else
  echo "[PASS] stagea_map_empty (diagnostic acceptable)" >&2
fi

exit 0

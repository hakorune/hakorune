#!/bin/bash
# hako_min_binop_vm.sh — Hako minimum binary operators canary (opt-in)

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# Try to detect repo root via git; fallback by climbing to tools directory
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi
HAKO_BIN_DEFAULT="$ROOT/tools/bin/hako"
HAKO_BIN="${HAKO_BIN:-$HAKO_BIN_DEFAULT}"

if [ "${SMOKES_ENABLE_HAKO_BINOP:-0}" != "1" ]; then
  echo "[SKIP] SMOKES_ENABLE_HAKO_BINOP!=1; skipping Hako binop canaries" >&2
  exit 0
fi

warn() { echo -e "[WARN] $*" >&2; }
info() { echo -e "[INFO] $*" >&2; }
fail() { echo -e "[FAIL] $*" >&2; return 1; }
pass() { echo -e "[PASS] $*" >&2; }

require_hako() {
  if [ ! -x "$HAKO_BIN" ]; then
    warn "Hako binary not found: $HAKO_BIN (set HAKO_BIN to override)"
    warn "Skipping Hako binop canaries"
    exit 0
  fi
}

# Compile Hako code to Program(JSON v0) via Selfhost Compiler
hako_compile_to_program_json() {
  local code="$1"
  local hako_tmp="/tmp/hako_binop_$$.hako"
  local json_out="/tmp/hako_binop_$$.mir.json"

  printf "%s\n" "$code" > "$hako_tmp"

  # Selfhost Compiler: Hako → JSON v0 (capture noise then extract JSON line)
  local raw="/tmp/hako_binop_raw_$$.txt"
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_SYNTAX_SUGAR_LEVEL=full \
  NYASH_ENABLE_ARRAY_LITERAL=1 \
  HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
  "$ROOT/target/release/nyash" --backend vm \
    "$ROOT/lang/src/compiler/entry/compiler.hako" -- --min-json --source "$(cat "$hako_tmp")" > "$raw" 2>&1
  awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$raw" > "$json_out"
  rm -f "$raw"

  local rc=$?
  rm -f "$hako_tmp"

  if [ $rc -ne 0 ] || [ ! -f "$json_out" ]; then
    warn "Compilation failed (rc=$rc)"
    rm -f "$json_out"
    return 1
  fi

  echo "$json_out"
  return 0
}

# Execute Program(JSON v0) via the compat umbrella intake (`--json-file`)
run_program_json_via_compat() {
  local json_path="$1"

  if [ ! -f "$json_path" ]; then
    warn "JSON file not found: $json_path"
    return 1
  fi

  # Compat execution (Program(JSON v0) -> lower -> execute)
  # Suppress noise for clean output
  NYASH_QUIET=1 \
  HAKO_QUIET=1 \
  NYASH_CLI_VERBOSE=0 \
  NYASH_NYRT_SILENT_RESULT=1 \
  out="$("$ROOT/target/release/nyash" --json-file "$json_path" 2>&1)"

  # Filter: drop interpreter headers and Result lines; print the last meaningful line
  printf '%s\n' "$out" | awk '/^(✅|ResultType|Result:)/{next} NF{last=$0} END{ if(last) print last }'

  local rc=$?
  rm -f "$json_path"
  return $rc
}

# Unified 2-stage execution: compile → run
run_hako() {
  local code="$1"

  local json_path
  json_path=$(hako_compile_to_program_json "$code") || return 1

  run_program_json_via_compat "$json_path"
  return $?
}

check_exact() {
  local expect="$1"; shift
  local got="$1"; shift
  local name="$1"; shift
  if [ "$got" = "$expect" ]; then pass "$name"; return 0; fi
  printf "Expected: %s\nActual:   %s\n" "$expect" "$got" >&2
  fail "$name"
}

require_hako

info "Hako binop canary: simple addition"
out=$(run_hako 'box Main { static method main() { print(1+2); } }')
check_exact "3" "$out" "hako_binop_add" || exit 1

info "Hako binop canary: precedence (multiply before add)"
out=$(run_hako 'box Main { static method main() { print(1+2*3); } }')
check_exact "7" "$out" "hako_binop_precedence" || exit 1

info "Hako binop canary: subtraction"
out=$(run_hako 'box Main { static method main() { print(10-4); } }')
check_exact "6" "$out" "hako_binop_sub" || exit 1

info "Hako binop canary: division"
out=$(run_hako 'box Main { static method main() { print(8/2); } }')
check_exact "4" "$out" "hako_binop_div" || exit 1

exit 0

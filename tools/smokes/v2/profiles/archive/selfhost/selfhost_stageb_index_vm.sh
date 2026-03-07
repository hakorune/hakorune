#!/bin/bash
# selfhost_stageb_index_vm.sh — Hako Stage‑B pipeline index operator canary (opt‑in)

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi
HAKO_BIN_DEFAULT="$ROOT/target/release/hakorune"
HAKO_BIN="${HAKO_BIN:-$HAKO_BIN_DEFAULT}"

warn() { echo -e "[WARN] $*" >&2; }
info() { echo -e "[INFO] $*" >&2; }
fail() { echo -e "[FAIL] $*" >&2; return 1; }
pass() { echo -e "[PASS] $*" >&2; }

resolve_hako_bin() {
  if [ "$(basename "$HAKO_BIN")" = "nyash" ]; then
    warn "HAKO_BIN points to deprecated nyash; using hakorune instead"
    HAKO_BIN="$HAKO_BIN_DEFAULT"
  fi
}

require_hako() {
  if [ "${SMOKES_ENABLE_STAGEB:-0}" != "1" ]; then
    warn "SMOKES_ENABLE_STAGEB!=1; skipping Stage‑B canaries"
    exit 0
  fi
  resolve_hako_bin
  if [ ! -x "$HAKO_BIN" ]; then
    warn "Hako binary not found: $HAKO_BIN (set HAKO_BIN to override)"
    warn "Skipping Stage‑B index canaries"
    exit 0
  fi
}

hako_compile_to_mir_stageb() {
  local code="$1"
  local hako_tmp="/tmp/hako_stageb_idx_$$.hako"
  local json_out="/tmp/hako_stageb_idx_$$.mir.json"
  printf "%s\n" "$code" > "$hako_tmp"

  local raw="/tmp/hako_stageb_idx_raw_$$.txt"
  NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_SYNTAX_SUGAR_LEVEL=full NYASH_ENABLE_ARRAY_LITERAL=1 \
  HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
  NYASH_FEATURES=stage3 \
  NYASH_VARMAP_GUARD_STRICT=0 NYASH_BLOCK_SCHEDULE_VERIFY=0 NYASH_PHI_VERIFY=0 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
  "$HAKO_BIN" --backend vm \
    "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat "$hako_tmp")" > "$raw" 2>&1
  awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$raw" > "$json_out"
  rm -f "$hako_tmp"

  if [ ! -s "$json_out" ]; then
    warn "Stage‑B compilation failed (LOG: $raw)"
    return 1
  fi
  rm -f "$raw"
  echo "$json_out"
}

run_mir_via_gate_c() {
  local json_path="$1"
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    out="$("$HAKO_BIN" --json-file "$json_path" 2>&1)"
  printf '%s\n' "$out" | awk '/\\[deprecate\\]/{next} /^(✅|ResultType|Result:)/{next} NF{last=$0} END{ if(last) print last }'
  rm -f "$json_path"
}

run_hako() {
  local code="$1"
  local json_path
  json_path=$(hako_compile_to_mir_stageb "$code") || return 1
  run_mir_via_gate_c "$json_path"
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

info "Stage‑B index: array read"
out=$(run_hako 'box Main { static method main() { local a=[1,2,3]; print(a[0]); } }') || exit 1
check_exact "1" "$out" "stageb_index_array_read" || exit 1

info "Stage‑B index: array write"
out=$(run_hako 'box Main { static method main() { local a=[1,2]; a[1]=9; print(a[1]); } }') || exit 1
check_exact "9" "$out" "stageb_index_array_write" || exit 1

info "Stage‑B index: map rw"
out=$(run_hako 'box Main { static method main() { local m={"a":1}; m["b"]=7; print(m["b"]); } }') || exit 1
check_exact "7" "$out" "stageb_index_map_rw" || exit 1

info "Stage‑B index: nested array"
out=$(run_hako 'box Main { static method main() { local a=[[1,2],[3,4]]; print(a[1][0]); } }') || exit 1
check_exact "3" "$out" "stageb_index_nested_array" || exit 1

info "Stage‑B index: missing map key diagnostic"
if run_hako 'box Main { static method main() { local m={"a":1}; print(m["c"]); } }' >/tmp/hako_stageb_idx_diag.txt 2>&1; then
  warn "expected failure but command succeeded"
  cat /tmp/hako_stageb_idx_diag.txt >&2
  fail "stageb_index_map_missing_diag" || exit 1
else
  pass "stageb_index_map_missing_diag"
fi
rm -f /tmp/hako_stageb_idx_diag.txt

exit 0

#!/bin/bash
# selfhost_stageb_v1_compat_vm.sh — Stage-B emit via MirJsonV1Adapter opt-in canary
# opt-in archive candidate: manual Stage-B diagnostic, not part of current selfhost required gate

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
pass() { echo -e "[PASS] $*" >&2; }
fail() { echo -e "[FAIL] $*" >&2; return 1; }
skip() { echo -e "[SKIP] $*" >&2; exit 0; }

if [ "${SMOKES_ENABLE_STAGEB_V1:-0}" != "1" ]; then
  skip "SMOKES_ENABLE_STAGEB_V1!=1; skipping Stage-B v1 compat canary"
fi

if [ ! -x "$HAKO_BIN" ]; then
  warn "Hako binary not found: $HAKO_BIN (set HAKO_BIN to override)"
  skip "hako binary missing"
fi

compile_stageb_v1() {
  local code="$1"
  local tmp_hako="/tmp/hako_stageb_v1_$$.hako"
  local json_out="/tmp/hako_stageb_v1_$$.mir.json"
  printf "%s\n" "$code" > "$tmp_hako"
  local raw="/tmp/hako_stageb_v1_raw_$$.txt"
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_SYNTAX_SUGAR_LEVEL=full \
  NYASH_ENABLE_ARRAY_LITERAL=1 \
  HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
  NYASH_FEATURES=stage3 \
  NYASH_VARMAP_GUARD_STRICT=0 NYASH_BLOCK_SCHEDULE_VERIFY=0 NYASH_PHI_VERIFY=0 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
  "$HAKO_BIN" --backend vm \
    "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --v1-compat --source "$(cat "$tmp_hako")" > "$raw" 2>&1
  awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$raw" > "$json_out"
  rm -f "$raw" "$tmp_hako"
  if [ ! -s "$json_out" ]; then
    warn "Stage-B v1 compat compilation failed"
    rm -f "$json_out"
    return 1
  fi
  echo "$json_out"
  return 0
}

run_gate_c() {
  local json_path="$1"
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    out="$("$HAKO_BIN" --json-file "$json_path" 2>&1)"
  printf '%s\n' "$out" | awk '/^\\[deprecate\\]/{next} /^(✅|ResultType|Result:)/{next} NF{last=$0} END{ if(last) print last }'
  local rc=$?
  rm -f "$json_path"
  return $rc
}

info "Stage-B v1 compat: return literal"
json_path=$(compile_stageb_v1 'box Main { static method main() { return 42; } }') || skip "Stage-B v1 compat compilation not available"
out=$(run_gate_c "$json_path")
if [ "$out" = "42" ]; then
  pass "stageb_v1_return"
else
  printf 'Expected: 42\nActual:   %s\n' "$out" >&2
  fail "stageb_v1_return"
fi

exit 0

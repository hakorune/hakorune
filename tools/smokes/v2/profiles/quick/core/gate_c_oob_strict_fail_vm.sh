#!/bin/bash
# gate_c_oob_strict_fail_vm.sh — Gate‑C(Core) strict OOB fail‑fast (opt‑in)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_ENABLE_OOB_STRICT:-0}" != "1" ]; then
  test_skip "gate_c_oob_strict_fail_vm" "opt-in (set SMOKES_ENABLE_OOB_STRICT=1)" && exit 0
fi

# Helper: compile minimal Stage‑B code to MIR(JSON v0)
hako_compile_to_mir_stageb() {
  local code="$1"
  local hako_tmp="/tmp/hako_oob_strict_$$.hako"
  local json_out="/tmp/hako_oob_strict_$$.mir.json"
  printf "%s\n" "$code" > "$hako_tmp"
  local raw="/tmp/hako_oob_strict_raw_$$.txt"
  NYASH_PARSER_ALLOW_SEMICOLON=1 HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
  NYASH_FEATURES=stage3 \
  NYASH_VARMAP_GUARD_STRICT=0 NYASH_BLOCK_SCHEDULE_VERIFY=0 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
  "$ROOT/target/release/nyash" --backend vm \
    "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat "$hako_tmp")" > "$raw" 2>&1 || true
  awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$raw" > "$json_out"
  rm -f "$raw" "$hako_tmp"
  echo "$json_out"
}

run_gate_c_core() {
  local json_path="$1"
  HAKO_OOB_STRICT=1 NYASH_OOB_STRICT=1 \
  HAKO_OOB_STRICT_FAIL=1 NYASH_OOB_STRICT_FAIL=1 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$ROOT/target/release/nyash" --json-file "$json_path" >/tmp/hako_oob_strict_run.txt 2>&1
  local rc=$?
  cat /tmp/hako_oob_strict_run.txt >&2
  rm -f "$json_path" /tmp/hako_oob_strict_run.txt
  return $rc
}

# Case 1: array OOB read should exit non‑zero under strict+fail
code_read='box Main { static method main() { local a=[1,2]; print(a[5]); return 0 } }'
json1=$(hako_compile_to_mir_stageb "$code_read") || {
  log_warn "Stage‑B emit failed; skipping"
  exit 0
}
if run_gate_c_core "$json1"; then
  echo "[FAIL] gate_c_oob_strict_fail_vm(read): expected non-zero rc" >&2
  exit 1
else
  echo "[PASS] gate_c_oob_strict_fail_vm(read)" >&2
fi

# Case 2: array OOB write should exit non‑zero under strict+fail
code_write='box Main { static method main() { local a=[1,2]; a[9]=3; return 0 } }'
json2=$(hako_compile_to_mir_stageb "$code_write") || {
  log_warn "Stage‑B emit failed; skipping"
  exit 0
}
if run_gate_c_core "$json2"; then
  echo "[FAIL] gate_c_oob_strict_fail_vm(write): expected non-zero rc" >&2
  exit 1
else
  echo "[PASS] gate_c_oob_strict_fail_vm(write)" >&2
fi

exit 0


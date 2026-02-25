#!/bin/bash
# phase29bq_selfhost_stage1_contract_smoke_vm.sh
# Contract smoke for Stage1 emit routes:
# - emit-program (canonical + alias): positive payload contract
# - emit-mir: fail-fast contract on invalid Program(JSON) path
# - D3-min12 guard: block helper-call mode normalization regression signature
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2
source "$NYASH_ROOT/tools/selfhost/lib/stage1_contract.sh"

BIN="${1:-$NYASH_ROOT/target/selfhost/hakorune.stage1_cli}"
ENTRY="${2:-$NYASH_ROOT/apps/tests/hello_simple_llvm.hako}"

if [[ "$BIN" != /* ]]; then
  BIN="$NYASH_ROOT/$BIN"
fi
if [[ "$ENTRY" != /* ]]; then
  ENTRY="$NYASH_ROOT/$ENTRY"
fi

if [ ! -x "$BIN" ]; then
  log_error "stage1 contract smoke binary not found/executable: $BIN"
  exit 2
fi
if [ ! -f "$ENTRY" ]; then
  log_error "stage1 contract smoke entry not found: $ENTRY"
  exit 2
fi
if [ -f "${BIN}.artifact_kind" ]; then
  if ! rg -q '^artifact_kind=stage1-cli$' "${BIN}.artifact_kind"; then
    log_error "stage1 contract smoke requires stage1-cli artifact: ${BIN}.artifact_kind"
    exit 2
  fi
fi

tmp_prog_raw="$(mktemp /tmp/phase29bq_stage1_contract_prog_raw.XXXXXX.log)"
tmp_prog_json="$(mktemp /tmp/phase29bq_stage1_contract_prog.XXXXXX.json)"
tmp_prog_alias_raw="$(mktemp /tmp/phase29bq_stage1_contract_prog_alias_raw.XXXXXX.log)"
tmp_mir_fail="$(mktemp /tmp/phase29bq_stage1_contract_mir_fail.XXXXXX.log)"
tmp_mir_fail_err="$(mktemp /tmp/phase29bq_stage1_contract_mir_fail_err.XXXXXX.log)"
cleanup() {
  rm -f "$tmp_prog_raw" "$tmp_prog_json" "$tmp_prog_alias_raw" "$tmp_mir_fail" "$tmp_mir_fail_err"
}
trap cleanup EXIT

source_text="$(stage1_contract_source_text "$ENTRY")"
stage1_entry_src="$NYASH_ROOT/lang/src/runner/stage1_cli_env.hako"
if [ -f "${BIN}.artifact_kind" ]; then
  meta_entry="$(awk -F= '$1=="entry"{print substr($0, index($0, "=")+1)}' "${BIN}.artifact_kind" | tail -n 1)"
  if [ -n "$meta_entry" ]; then
    stage1_entry_src="$meta_entry"
  fi
fi
if [ -f "$stage1_entry_src" ]; then
  if rg -q -e 'mode[[:space:]]*=[[:space:]]*me\._normalize_mode_' -e 'mode[[:space:]]*=[[:space:]]*Stage1CliEnvModeAlias\.normalize\(' "$stage1_entry_src"; then
    log_error "stage1 contract smoke: D3-min12 guard hit (helper-call mode normalization regression signature)"
    log_error "stage1 contract smoke: keep mode alias normalization inline in stage1_cli_env.hako"
    exit 1
  fi
fi

# Case-1: emit-program should produce Program(JSON v0)
set +e
stage1_contract_exec_mode "$BIN" "emit-program" "$ENTRY" "$source_text" >"$tmp_prog_raw" 2>/dev/null
rc_prog=$?
set -e
if [ "$rc_prog" -ne 0 ]; then
  log_error "stage1 contract smoke: emit-program failed (rc=$rc_prog)"
  exit 1
fi
if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' "$tmp_prog_raw" >"$tmp_prog_json"; then
  log_error "stage1 contract smoke: emit-program missing Program(JSON v0) payload"
  exit 1
fi

# Case-1b: emit-program aliases should keep the same contract.
for mode_alias in emit_program_json emit-program-json; do
  set +e
  stage1_contract_exec_mode "$BIN" "$mode_alias" "$ENTRY" "$source_text" >"$tmp_prog_alias_raw" 2>/dev/null
  rc_prog_alias=$?
  set -e
  if [ "$rc_prog_alias" -ne 0 ]; then
    log_error "stage1 contract smoke: emit-program alias failed (mode=$mode_alias rc=$rc_prog_alias)"
    exit 1
  fi
  if ! rg -q '"kind":"Program"' "$tmp_prog_alias_raw"; then
    log_error "stage1 contract smoke: emit-program alias missing Program(JSON v0) payload (mode=$mode_alias)"
    exit 1
  fi
done

# Case-2: emit-mir should fail-fast when Program(JSON) path is invalid.
set +e
stage1_contract_exec_mode "$BIN" "emit-mir" "$ENTRY" "$source_text" "/tmp/__stage1_contract_missing_program__.json" >"$tmp_mir_fail" 2>"$tmp_mir_fail_err"
rc_mir=$?
set -e
if [ "$rc_mir" -eq 0 ]; then
  log_error "stage1 contract smoke: emit-mir unexpectedly succeeded for invalid Program(JSON) path"
  exit 1
fi
if rg -q '"functions"[[:space:]]*:' "$tmp_mir_fail"; then
  log_error "stage1 contract smoke: emit-mir returned MIR payload unexpectedly on invalid input"
  exit 1
fi
if rg -q 'Unknown Box type: Main' "$tmp_mir_fail_err"; then
  log_error "stage1 contract smoke: stage1-cli entry not available in this artifact (Unknown Box type: Main)"
  echo "LOG: $tmp_mir_fail"
  echo "ERR: $tmp_mir_fail_err"
  exit 1
fi
if ! rg -q '\[stage1-cli\] emit mir-json:|\[freeze:contract\]\[stage1-cli/emit-mir\]|Result:[[:space:]]*(96|97|98)' "$tmp_mir_fail" "$tmp_mir_fail_err"; then
  log_error "stage1 contract smoke: emit-mir did not expose fail-fast marker"
  echo "LOG: $tmp_mir_fail"
  echo "ERR: $tmp_mir_fail_err"
  exit 1
fi

log_success "phase29bq_selfhost_stage1_contract_smoke_vm: PASS ($(basename "$BIN"))"

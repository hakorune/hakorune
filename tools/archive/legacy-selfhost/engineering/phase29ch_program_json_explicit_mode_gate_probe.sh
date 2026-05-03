#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../../../.." && pwd)}"
source "${ROOT}/tools/selfhost/lib/identity_routes.sh"
source "${ROOT}/tools/selfhost/lib/stage1_contract.sh"

STAGE1_BIN="${STAGE1_BIN:-${ROOT}/target/selfhost/hakorune.stage1_cli}"
STAGE2_BIN="${STAGE2_BIN:-${ROOT}/target/selfhost/hakorune.stage1_cli.stage2}"
ENTRY="${ENTRY:-${ROOT}/apps/tests/hello_simple_llvm.hako}"

for bin in "$STAGE1_BIN" "$STAGE2_BIN"; do
  if [[ ! -x "$bin" ]]; then
    echo "[FAIL] missing selfhost bin: $bin" >&2
    exit 2
  fi
done

if [[ ! -f "$ENTRY" ]]; then
  echo "[FAIL] missing entry: $ENTRY" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

program_json="$tmp_dir/program.json"
if ! run_stage1_env_route "$STAGE1_BIN" "program-json" "$ENTRY" "$program_json"; then
  echo "[FAIL] failed to materialize Program(JSON) via env route" >&2
  exit 1
fi
program_json_text="$(cat "$program_json")"

probe_bin() {
  local bin="$1"
  local label="$2"
  local plain_out="$tmp_dir/${label}.plain.out"
  local plain_err="$tmp_dir/${label}.plain.err"
  local compat_out="$tmp_dir/${label}.compat.out"
  local compat_err="$tmp_dir/${label}.compat.err"
  local alias_out="$tmp_dir/${label}.alias.out"
  local alias_err="$tmp_dir/${label}.alias.err"
  local rc=0

  stage1_contract_export_runner_defaults

  set +e
  env \
    "NYASH_NYRT_SILENT_RESULT=${NYASH_NYRT_SILENT_RESULT:-1}" \
    "NYASH_DISABLE_PLUGINS=1" \
    "NYASH_FILEBOX_MODE=core-ro" \
    "HAKO_SELFHOST_NO_DELEGATE=${HAKO_SELFHOST_NO_DELEGATE:-1}" \
    "HAKO_MIR_BUILDER_DELEGATE=${HAKO_MIR_BUILDER_DELEGATE:-0}" \
    "NYASH_USE_STAGE1_CLI=1" \
    "NYASH_STAGE1_MODE=emit-mir" \
    "HAKO_STAGE1_MODE=emit-mir" \
    "STAGE1_EMIT_PROGRAM_JSON=0" \
    "STAGE1_EMIT_MIR_JSON=1" \
    "HAKO_STAGE1_INPUT=${ENTRY}" \
    "NYASH_STAGE1_INPUT=${ENTRY}" \
    "STAGE1_SOURCE=${ENTRY}" \
    "STAGE1_INPUT=${ENTRY}" \
    "STAGE1_SOURCE_TEXT=${program_json_text}" \
    "STAGE1_PROGRAM_JSON_TEXT=${program_json_text}" \
    "$bin" >"$plain_out" 2>"$plain_err"
  rc=$?
  set -e

  echo "[program-json-explicit-gate] ${label}.plain_rc=${rc}"
  if [[ "$rc" -eq 0 ]]; then
    echo "[FAIL] ${label}: plain emit-mir unexpectedly accepted Program(JSON) text" >&2
    cat "$plain_out" >&2
    cat "$plain_err" >&2
    exit 1
  fi
  if ! rg -q 'emit-mir-program mode' "$plain_out" "$plain_err"; then
    echo "[FAIL] ${label}: missing explicit-mode fail-fast marker" >&2
    cat "$plain_out" >&2
    cat "$plain_err" >&2
    exit 1
  fi

  set +e
  stage1_contract_exec_mode \
    "$bin" \
    "$ENTRY" \
    "emit_mir_program" \
    "$program_json_text" >"$alias_out" 2>"$alias_err"
  rc=$?
  set -e

  echo "[program-json-explicit-gate] ${label}.legacy_alias_rc=${rc}"
  if [[ "$rc" -eq 0 ]]; then
    echo "[FAIL] ${label}: legacy explicit alias unexpectedly accepted" >&2
    cat "$alias_out" >&2
    cat "$alias_err" >&2
    exit 1
  fi

  if ! stage1_contract_exec_program_json_compat \
    "$bin" \
    "$program_json_text" >"$compat_out" 2>"$compat_err"; then
    echo "[FAIL] ${label}: explicit compat mode failed" >&2
    cat "$compat_out" >&2
    cat "$compat_err" >&2
    exit 1
  fi

  if ! rg -q '"functions"[[:space:]]*:' "$compat_out"; then
    echo "[FAIL] ${label}: explicit compat mode did not emit MIR JSON" >&2
    cat "$compat_out" >&2
    cat "$compat_err" >&2
    exit 1
  fi
}

probe_bin "$STAGE1_BIN" "stage1"
probe_bin "$STAGE2_BIN" "stage2"

echo "[program-json-explicit-gate] result=PASS"

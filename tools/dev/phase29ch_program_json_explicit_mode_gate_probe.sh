#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
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
bash "${ROOT}/tools/selfhost/run_stage1_cli.sh" --bin "$STAGE1_BIN" emit program-json "$ENTRY" >"$program_json"
program_json_text="$(cat "$program_json")"

probe_bin() {
  local bin="$1"
  local label="$2"
  local plain_out="$tmp_dir/${label}.plain.out"
  local plain_err="$tmp_dir/${label}.plain.err"
  local compat_out="$tmp_dir/${label}.compat.out"
  local compat_err="$tmp_dir/${label}.compat.err"
  local rc=0

  stage1_contract_export_runner_defaults

  set +e
  stage1_contract_run_bin_with_env \
    "$bin" \
    "emit-mir" \
    "$ENTRY" \
    "$program_json_text" \
    0 \
    1 \
    "" \
    "$program_json_text" \
    "$plain_out" \
    "$plain_err"
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

  if ! stage1_contract_exec_program_json_text \
    "$bin" \
    "$ENTRY" \
    "$program_json_text" \
    "emit-mir-program" >"$compat_out" 2>"$compat_err"; then
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

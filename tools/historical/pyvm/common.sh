#!/usr/bin/env bash
set -euo pipefail

PYVM_SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
PYVM_ROOT_DIR=$(CDPATH= cd -- "$PYVM_SCRIPT_DIR/../../.." && pwd)
PYVM_RUNNER="$PYVM_ROOT_DIR/tools/historical/pyvm/pyvm_runner.py"
PYVM_TMP_DIR="$PYVM_ROOT_DIR/tmp/pyvm"

pyvm_fail() {
  echo "❌ $*" >&2
  return 1
}

pyvm_resolve_bin() {
  if [[ -n "${NYASH_BIN:-}" ]]; then
    echo "$NYASH_BIN"
    return 0
  fi
  if [[ -x "$PYVM_ROOT_DIR/target/release/hakorune" ]]; then
    echo "$PYVM_ROOT_DIR/target/release/hakorune"
    return 0
  fi
  if [[ -x "$PYVM_ROOT_DIR/target/release/nyash" ]]; then
    echo "$PYVM_ROOT_DIR/target/release/nyash"
    return 0
  fi
  (cd "$PYVM_ROOT_DIR" && cargo build --release --bin hakorune >/dev/null)
  echo "$PYVM_ROOT_DIR/target/release/hakorune"
}

pyvm_require_tools() {
  command -v python3 >/dev/null 2>&1 || pyvm_fail "python3 not found"
  [[ -f "$PYVM_RUNNER" ]] || pyvm_fail "runner not found: $PYVM_RUNNER"
  mkdir -p "$PYVM_TMP_DIR"
}

pyvm_run_mir_capture() {
  local mir_json="$1"
  shift
  local out status=0
  out=$(python3 "$PYVM_RUNNER" --in "$mir_json" "$@" 2>&1) || status=$?
  printf '%s' "$out"
  return $status
}

pyvm_emit_mir_from_source() {
  local source_file="$1"
  local out_mir="$2"
  shift 2
  local -a env_kv=("$@")
  local bin
  bin=$(pyvm_resolve_bin)
  env "${env_kv[@]}" "$bin" --emit-mir-json "$out_mir" "$source_file"
}

pyvm_emit_mir_from_program_json() {
  local program_json="$1"
  local out_mir="$2"
  shift 2
  local -a env_kv=("$@")
  local bin
  bin=$(pyvm_resolve_bin)
  env "${env_kv[@]}" "$bin" --program-json-to-mir "$out_mir" --json-file "$program_json"
}

pyvm_run_source_capture() {
  local source_file="$1"
  shift
  local -a env_kv=("$@")
  pyvm_require_tools
  local mir_json="$PYVM_TMP_DIR/src_$(basename "$source_file").$$.$RANDOM.json"
  local emit_out emit_status=0 run_out run_status=0
  emit_out=$(pyvm_emit_mir_from_source "$source_file" "$mir_json" "${env_kv[@]}" 2>&1) || emit_status=$?
  if [[ $emit_status -ne 0 ]]; then
    printf '%s' "$emit_out"
    rm -f "$mir_json"
    return $emit_status
  fi
  run_out=$(pyvm_run_mir_capture "$mir_json") || run_status=$?
  rm -f "$mir_json"
  printf '%s' "$run_out"
  return $run_status
}

pyvm_run_program_json_capture() {
  local program_json="$1"
  shift
  local -a env_kv=("$@")
  pyvm_require_tools
  local mir_json="$PYVM_TMP_DIR/prog_$(basename "$program_json").$$.$RANDOM.json"
  local emit_out emit_status=0 run_out run_status=0
  emit_out=$(pyvm_emit_mir_from_program_json "$program_json" "$mir_json" "${env_kv[@]}" 2>&1) || emit_status=$?
  if [[ $emit_status -ne 0 ]]; then
    printf '%s' "$emit_out"
    rm -f "$mir_json"
    return $emit_status
  fi
  run_out=$(pyvm_run_mir_capture "$mir_json") || run_status=$?
  rm -f "$mir_json"
  printf '%s' "$run_out"
  return $run_status
}

pyvm_run_inline_capture() {
  local source="$1"
  shift
  local inline_file="$PYVM_TMP_DIR/inline_$$.$RANDOM.hako"
  printf '%s\n' "$source" > "$inline_file"
  local out status=0
  out=$(pyvm_run_source_capture "$inline_file" "$@" 2>&1) || status=$?
  rm -f "$inline_file"
  printf '%s' "$out"
  return $status
}

#!/bin/bash
# stageb_helpers.sh — Helpers to compile Hako(Stage‑B) source to Program(JSON v0)

_STAGEB_HELPERS_TOOLS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
source "${_STAGEB_HELPERS_TOOLS_DIR}/lib/program_json_v0_compat.sh"

stageb_export_vm_compile_env() {
  export NYASH_PARSER_ALLOW_SEMICOLON=1
  export NYASH_ALLOW_USING_FILE=0
  export HAKO_ALLOW_USING_FILE=0
  export NYASH_USING_AST=1
  export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
  export NYASH_DISABLE_NY_COMPILER=1
  export HAKO_DISABLE_NY_COMPILER=1
  export HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0
  export NYASH_VARMAP_GUARD_STRICT=0
  export NYASH_BLOCK_SCHEDULE_VERIFY=0
  # Stage-B entry includes nested compat loops; these smokes check CLI/JSON
  # contracts, not planner-required JoinIR acceptance.
  export NYASH_JOINIR_DEV=0
  export HAKO_JOINIR_STRICT=0
  export NYASH_QUIET=0
  export HAKO_QUIET=0
  export NYASH_CLI_VERBOSE=0
  unset NYASH_MODULES
}

stageb_compile_to_json_with_args() {
  # Args: HAKO_CODE [STAGEB_ARGS...]
  local code="$1"; shift || true
  local debug_on_failure="${STAGEB_COMPILE_DEBUG_ON_FAILURE:-0}"
  local hako_tmp="/tmp/hako_stageb_$$.hako"
  local json_out="/tmp/hako_stageb_$$.mir.json"
  printf "%s\n" "$code" > "$hako_tmp"
  local raw="/tmp/hako_stageb_raw_$$.txt"
  (
    stageb_export_vm_compile_env
    cd "$NYASH_ROOT" && \
      "$NYASH_BIN" --backend vm \
        "$NYASH_ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- \
        "$@" --source "$(cat "$hako_tmp")"
  ) > "$raw" 2>&1 || true
  if awk '(/"version":0/ && /"kind":"Program"/) {print; found=1; exit} END{exit(found?0:1)}' "$raw" > "$json_out"; then
    rm -f "$raw" "$hako_tmp"
    echo "$json_out"
    return 0
  fi

  if [ "$debug_on_failure" = "1" ]; then
    echo "[stageB/emit-debug] failed to extract v0 JSON; raw tail:" >&2
    tail -n 120 "$raw" >&2 || true
  fi
  # Give up; return an empty path (caller treats as failure)
  rm -f "$raw" "$hako_tmp" "$json_out"
  return 1
}

stageb_compile_to_json() {
  # Args: HAKO_CODE
  local code="$1"
  STAGEB_COMPILE_DEBUG_ON_FAILURE=1 stageb_compile_to_json_with_args "$code"
}

stageb_compile_to_json_with_bundles() {
  # Args: MAIN_CODE [BUNDLE1] [BUNDLE2] ...
  local code="$1"; shift || true
  local extra_args=()
  while [ "$#" -gt 0 ]; do
    extra_args+=("--bundle-src" "$1")
    shift
  done
  stageb_compile_to_json_with_args "$code" "${extra_args[@]}"
}

stageb_compile_to_json_with_require() {
  # Args: MAIN_CODE REQUIRES_CSV (e.g., "U1,U2")
  local code="$1"; shift || true
  local requires_csv="$1"; shift || true
  local extra_args=()
  IFS=',' read -r -a REQS <<< "$requires_csv"
  for r in "${REQS[@]}"; do
    [ -n "$r" ] && extra_args+=("--require-mod" "$r")
  done
  stageb_compile_to_json_with_args "$code" "${extra_args[@]}"
}

stageb_json_nonempty() {
  local path="$1"
  [ -s "$path" ]
}

# Thin shared wrapper for the phase29bq Program(JSON v0) fixture producers.
# Keep the exact raw CLI contract in one place without changing downstream pins.
stageb_emit_program_json_v0_fixture() {
  local out_path="$1"
  local fixture="$2"
  program_json_v0_compat_emit_to_file "$NYASH_BIN" "$out_path" "$fixture" >/dev/null
}

# Execute a compiled Stage‑B Program(JSON v0) via the compat umbrella intake and expect specific rc
# Args: JSON_PATH EXPECTED_RC
stageb_gatec_expect_rc() {
  local json="$1"; shift
  local expected_rc="$1"; shift
  NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$NYASH_BIN" --json-file "$json" >/dev/null 2>&1
  local rc=$?
  if [ "$rc" = "$expected_rc" ]; then
    return 0
  else
    echo "[FAIL] Gate‑C(Core) rc=$rc, expected=$expected_rc" >&2
    return 1
  fi
}

# Fallback: compile Ny source to MIR(JSON v1) via Rust MIR path (backend=mir)
# Returns a path to JSON file (v1 schema). Caller may set NYASH_NYVM_V1_DOWNCONVERT=1 for execution.
stageb_compile_via_rust_mir() {
  local code="$1"
  local ny_tmp="/tmp/hako_stageb_src_$$.hako"
  local json_out="/tmp/hako_stageb_rust_$$.mir.json"
  printf "%s\n" "$code" > "$ny_tmp"
  if NYASH_FEATURES="${NYASH_FEATURES:-stage3}" NYASH_PARSER_ALLOW_SEMICOLON=1 \
     "$NYASH_BIN" --backend mir --emit-mir-json "$json_out" "$ny_tmp" >/dev/null 2>&1; then
    rm -f "$ny_tmp"
    echo "$json_out"
    return 0
  fi
  rm -f "$ny_tmp" "$json_out"
  return 1
}

#!/usr/bin/env bash
# selfhost_build_exe.sh — EXE artifact owner helpers
#
# Purpose:
# - Own the Program(JSON v0) → MIR(JSON) → EXE artifact lane.
# - Keep this helper separate from direct MIR / core-direct and dispatcher logic.

resolve_emit_exe_nyllvm() {
  local nyll="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}"
  if [ ! -x "$nyll" ] && [ ! -f "$nyll" ]; then
    echo "[selfhost] ny-llvmc not found: $nyll (Set NYASH_NY_LLVM_COMPILER or build ny-llvmc)" >&2
    return 2
  fi
  printf '%s' "$nyll"
}

apply_emit_exe_env() {
  local nyll="$1" nyrt_dir="$2"
  export NYASH_LLVM_USE_HARNESS=1
  export NYASH_NY_LLVM_COMPILER="$nyll"
  export NYASH_EMIT_EXE_NYRT="$nyrt_dir"
}

resolve_emit_exe_context() {
  local nyll
  nyll="$(resolve_emit_exe_nyllvm)" || return $?
  local nyrt_dir="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}"
  apply_emit_exe_env "$nyll" "$nyrt_dir"

  local mir_tmp
  mir_tmp="$(select_emit_exe_mir_tmp_path)"

  printf '%s\n%s\n%s\n' "$nyll" "$nyrt_dir" "$mir_tmp"
}

select_emit_exe_mir_tmp_path() {
  if [ -n "${MIR_OUT:-}" ]; then
    printf '%s' "$MIR_OUT"
  else
    printf '%s' "/tmp/hako_stageb_mir_$$.json"
  fi
}

cleanup_emit_exe_temp_outputs() {
  local json_path="$1" mir_tmp="$2"
  if [ "$KEEP_TMP" != "1" ]; then
    if [ -z "$JSON_OUT" ]; then rm -f "$json_path" 2>/dev/null || true; fi
    if [ -z "$MIR_OUT" ]; then rm -f "$mir_tmp" 2>/dev/null || true; fi
  fi
}

emit_mir_json_from_program_json_v0() {
  local json_path="$1" mir_out_path="$2"
  echo "[selfhost] converting Program(JSON v0) → MIR(JSON)" >&2
  "$BIN" --json-file "$json_path" --program-json-to-mir "$mir_out_path"
}

emit_exe_from_mir_json() {
  local nyll="$1" mir_path="$2" nyrt_dir="$3" exe_out_path="$4"
  echo "[selfhost] converting MIR(JSON) → EXE" >&2
  "$nyll" --in "$mir_path" --emit exe --nyrt "$nyrt_dir" --out "$exe_out_path"
}

emit_exe_from_program_json_v0_with_context() {
  local json_path="$1" exe_out_path="$2" nyll="$3" nyrt_dir="$4" mir_tmp="$5"
  local rc=0
  emit_mir_json_from_program_json_v0 "$json_path" "$mir_tmp" || rc=$?
  if [ "$rc" -eq 0 ]; then
    emit_exe_from_mir_json "$nyll" "$mir_tmp" "$nyrt_dir" "$exe_out_path" || rc=$?
  fi
  cleanup_emit_exe_temp_outputs "$json_path" "$mir_tmp"
  return $rc
}

emit_exe_from_program_json_v0() {
  local json_path="$1" exe_out_path="$2"
  local exe_ctx nyll nyrt_dir mir_tmp
  exe_ctx="$(resolve_emit_exe_context)" || return $?
  nyll="$(printf '%s\n' "$exe_ctx" | sed -n '1p')"
  nyrt_dir="$(printf '%s\n' "$exe_ctx" | sed -n '2p')"
  mir_tmp="$(printf '%s\n' "$exe_ctx" | sed -n '3p')"

  emit_exe_from_program_json_v0_with_context "$json_path" "$exe_out_path" "$nyll" "$nyrt_dir" "$mir_tmp"
}

exe_output_requested() {
  [ -n "$EXE_OUT" ]
}

emit_requested_exe_output() {
  local json_path="$1"
  emit_exe_from_program_json_v0 "$json_path" "$EXE_OUT"
}

# Compat keep for older helper-local probes; W7.1 promotes the context form above.
emit_exe_from_program_json_v0_with_mir_tmp() {
  local json_path="$1" exe_out_path="$2" mir_tmp="$3" nyll="$4" nyrt_dir="$5"
  emit_exe_from_program_json_v0_with_context "$json_path" "$exe_out_path" "$nyll" "$nyrt_dir" "$mir_tmp"
}

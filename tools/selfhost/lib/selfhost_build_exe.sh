#!/usr/bin/env bash
# selfhost_build_exe.sh — EXE artifact owner helpers
#
# Purpose:
# - Own the direct source MIR -> EXE artifact lane.
# - Keep this helper separate from direct MIR / run keeper / dispatcher logic.

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
    printf '%s' "/tmp/hako_selfhost_mir_$$.json"
  fi
}

emit_exe_from_mir_json() {
  local nyll="$1" mir_path="$2" nyrt_dir="$3" exe_out_path="$4"
  echo "[selfhost] converting MIR(JSON) → EXE" >&2
  "$nyll" --in "$mir_path" --emit exe --nyrt "$nyrt_dir" --out "$exe_out_path"
}

cleanup_direct_exe_temp_outputs() {
  local mir_tmp="$1"
  if [ "$KEEP_TMP" != "1" ] && [ -z "${MIR_OUT:-}" ]; then
    rm -f "$mir_tmp" 2>/dev/null || true
  fi
}

emit_exe_from_source_mir_direct() {
  local exe_out_path="$1"
  local exe_ctx nyll nyrt_dir mir_tmp
  local rc=0
  if ! declare -F emit_mir_json_from_source >/dev/null 2>&1; then
    echo "[selfhost] direct MIR source helper is not loaded" >&2
    return 2
  fi
  exe_ctx="$(resolve_emit_exe_context)" || return $?
  nyll="$(printf '%s\n' "$exe_ctx" | sed -n '1p')"
  nyrt_dir="$(printf '%s\n' "$exe_ctx" | sed -n '2p')"
  mir_tmp="$(printf '%s\n' "$exe_ctx" | sed -n '3p')"

  emit_mir_json_from_source "$mir_tmp" || rc=$?
  if [ "$rc" -eq 0 ]; then
    emit_exe_from_mir_json "$nyll" "$mir_tmp" "$nyrt_dir" "$exe_out_path" || rc=$?
  fi
  cleanup_direct_exe_temp_outputs "$mir_tmp"
  return $rc
}

emit_requested_direct_exe_output() {
  emit_exe_from_source_mir_direct "$EXE_OUT"
}

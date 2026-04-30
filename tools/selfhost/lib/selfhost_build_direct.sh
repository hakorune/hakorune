#!/usr/bin/env bash
# selfhost_build_direct.sh — Direct MIR / core-direct owner helpers
#
# Purpose:
# - Own the direct MIR output path and the in-proc core-direct execution path.
# - Keep these helpers separate from exe-artifact / dispatcher logic.

emit_mir_json_from_source() {
  local mir_out_path="$1"
  echo "[selfhost] emitting MIR JSON → $mir_out_path" >&2
  "$BIN" --backend mir --emit-mir-json "$mir_out_path" "$IN" >/dev/null
}

run_program_json_v0_via_core_direct() {
  local json_path="$1"
  set +e
  NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 HAKO_CORE_DIRECT=1 HAKO_CORE_DIRECT_INPROC=1 \
    NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$BIN" --json-file "$json_path" >/dev/null 2>&1
  local rc=$?
  set -e
  return $rc
}

cleanup_program_json_tmp_if_needed() {
  local json_path="$1"
  if [ "$KEEP_TMP" != "1" ] && [ -z "$JSON_OUT" ]; then
    rm -f "$json_path" 2>/dev/null || true
  fi
}

announce_program_json_output_if_requested() {
  local json_path="$1"
  if [ -n "$JSON_OUT" ]; then
    echo "[selfhost] JSON v0 written: $json_path" >&2
  fi
}

emit_requested_mir_output_if_needed() {
  if [ -z "$MIR_OUT" ]; then
    return 0
  fi
  emit_mir_json_from_source "$MIR_OUT"
}

emit_direct_mir_only_output() {
  emit_requested_mir_output_if_needed || return $?
  echo "$MIR_OUT"
}

run_program_json_requested() {
  [ "$DO_RUN" = "1" ]
}

run_requested_program_json() {
  local json_path="$1"
  local rc=0
  run_program_json_v0_via_core_direct "$json_path" || rc=$?
  cleanup_program_json_tmp_if_needed "$json_path"
  return $rc
}

# Compat keep for older helper-local probes; W7.1 promotes the context form in selfhost_build.sh.
emit_exe_from_program_json_v0_with_mir_tmp() {
  local json_path="$1" exe_out_path="$2" mir_tmp="$3" nyll="$4" nyrt_dir="$5"
  emit_exe_from_program_json_v0_with_context "$json_path" "$exe_out_path" "$nyll" "$nyrt_dir" "$mir_tmp"
}

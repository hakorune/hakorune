#!/usr/bin/env bash
# selfhost_build_direct.sh — Direct MIR owner helpers
#
# Purpose:
# - Own the direct MIR output path.
# - Keep these helpers separate from run keeper / exe artifact / dispatcher logic.

emit_mir_json_from_source() {
  local mir_out_path="$1"
  echo "[selfhost] emitting MIR JSON → $mir_out_path" >&2
  "$BIN" --backend mir --emit-mir-json "$mir_out_path" "$IN" >/dev/null
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

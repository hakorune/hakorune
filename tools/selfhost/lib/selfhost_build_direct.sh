#!/usr/bin/env bash
# selfhost_build_direct.sh — Direct MIR owner helpers
#
# Purpose:
# - Own the direct MIR output path.
# - Keep these helpers separate from run keeper / exe artifact / dispatcher logic.

emit_mir_json_from_source() {
  local mir_out_path="$1"
  local emit_route="$ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
  local rc=0
  if [ -z "${IN:-}" ]; then
    echo "[selfhost] source input is required for MIR emission" >&2
    return 2
  fi
  if [ ! -x "$emit_route" ]; then
    echo "[selfhost] canonical MIR emit route missing: $emit_route" >&2
    return 2
  fi
  echo "[selfhost] emitting MIR JSON → $mir_out_path" >&2
  selfhost_phase_start "selfhost.emit_mir"
  NYASH_BIN="$BIN" "$emit_route" --route direct --out "$mir_out_path" --input "$IN" >/dev/null || rc=$?
  if [ "$rc" -eq 0 ]; then
    selfhost_phase_done "selfhost.emit_mir"
  else
    selfhost_phase_fail "selfhost.emit_mir" "$rc"
  fi
  return "$rc"
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

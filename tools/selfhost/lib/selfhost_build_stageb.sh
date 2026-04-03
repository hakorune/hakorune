#!/usr/bin/env bash
# selfhost_build_stageb.sh — Stage-B producer owner helpers
#
# Purpose:
# - Own Stage-B Program(JSON v0) production and raw snapshot handling.
# - Keep the shell producer path separate from direct-run / exe artifact / final dispatcher logic.
# - Prefer direct/source route defaults; keep VM compiler route explicit-only.
# - BuildBox emit-only is retired from the default caller path.

timestamp_now() { date +%Y%m%d_%H%M%S; }

emit_program_json_v0_via_direct_source() {
  local raw_path="$1" json_path="$2"
  (
    "$BIN" --emit-program-json-v0 "$json_path" "$IN"
  ) > "$raw_path" 2>&1
  local rc=$?
  if [ "$rc" -eq 0 ] && [ -s "$json_path" ]; then
    {
      printf '\n[selfhost/raw] --- emitted-json ---\n'
      cat "$json_path"
      printf '\n'
    } >> "$raw_path" 2>/dev/null || true
  fi
  return $rc
}

emit_stageb_program_json_raw() {
  local raw_path="$1" json_path="$2"
  stageb_cmd_desc="stage1 bridge emit-program-json-v0 (direct/core-first)"
  emit_program_json_v0_via_direct_source "$raw_path" "$json_path"
}

extract_program_json_v0_from_raw() {
  local raw_path="$1" json_path="$2"
  awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' "$raw_path" > "$json_path"
}

persist_stageb_raw_snapshot() {
  local raw_path="$1" json_path="$2" extract_ok="$3"
  if [ "$RAW_KEEP" != "1" ]; then
    return 0
  fi

  local ts raw_log_path
  ts="$(timestamp_now)"
  raw_log_path="$RAW_DIR/stageb_${ts}_$$.log"
  {
    echo "[selfhost/raw] cmd: ${stageb_cmd_desc:-unknown}"
    echo "[selfhost/raw] rc_stageb=${stageb_rc} extract_ok=${extract_ok}"
    echo "[selfhost/raw] src=${IN}"
    echo "[selfhost/raw] --- stdout+stderr ---"
    cat "$raw_path"
  } > "$raw_log_path" 2>/dev/null || true
  if [ "$extract_ok" = "1" ] && [ -s "$json_path" ]; then
    cp "$json_path" "$RAW_DIR/stageb_${ts}_$$.json" 2>/dev/null || true
  fi
  printf '%s' "$raw_log_path"
}

exit_after_stageb_emit_failure() {
  local raw_path="$1" raw_log_path="${2-}"
  echo "[selfhost] Stage‑B emit failed" >&2
  tail -n 120 "$raw_path" >&2 || true
  if [ -n "$raw_log_path" ]; then
    echo "[selfhost/debug] RAW log: $raw_log_path" >&2
  fi
  if [ "$KEEP_TMP" != "1" ]; then
    rm -f "$raw_path" 2>/dev/null || true
  fi
  exit 1
}

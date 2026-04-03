#!/usr/bin/env bash
# selfhost_build_stageb.sh — Stage-B producer owner helpers
#
# Purpose:
# - Own Stage-B Program(JSON v0) production and raw snapshot handling.
# - Keep the shell producer path separate from direct-run / exe artifact / final dispatcher logic.

timestamp_now() { date +%Y%m%d_%H%M%S; }

write_buildbox_emit_program_runner_hako() {
  local wrap_path="$1"
  cat > "$wrap_path" <<'HAKO'
using lang.compiler.build.build_box as BuildBox
static box Main {
  method _emit_program_json_checked(src) {
    if src == null { print("[selfhost/buildbox:no-src]"); return null; }
    local j = BuildBox.emit_program_json_v0(src, null);
    if j == null { print("[selfhost/buildbox:builder-null]"); return null; }
    return j;
  }

  method main(args) {
    local j = me._emit_program_json_checked(env.get("HAKO_SRC"));
    if j == null { return 1; }
    print(j);
    return 0;
  }
}
HAKO
}

buildbox_emit_only_keep_requested() {
  [ "${HAKO_USE_BUILDBOX:-0}" = "1" ] && [ "$DO_RUN" = "0" ] && [ -z "$EXE_OUT" ]
}

ensure_stageb_module_roots_list() {
  if [ -n "${HAKO_STAGEB_MODULE_ROOTS_LIST:-}" ]; then
    return 0
  fi
  roots_list="$(collect_stageb_module_roots_list "$ROOT" || true)"
  if [ -n "${roots_list:-}" ]; then
    export HAKO_STAGEB_MODULE_ROOTS_LIST="$roots_list"
  fi
}

emit_program_json_v0_via_buildbox() {
  local raw_path="$1"
  local wrap_path="/tmp/hako_buildbox_wrap_$$.hako"
  write_buildbox_emit_program_runner_hako "$wrap_path"
  (
    export HAKO_SRC="$SRC_CONTENT"
    cd "$ROOT" && "$BIN" --backend vm "$wrap_path"
  ) > "$raw_path" 2>&1
  local rc=$?
  rm -f "$wrap_path" 2>/dev/null || true
  return $rc
}

emit_program_json_v0_via_stageb_compiler() {
  local raw_path="$1"
  (
    export HAKO_SRC="$SRC_CONTENT"
    cd "$ROOT" && \
      "$BIN" --backend vm \
        "$ROOT/lang/src/compiler/entry/compiler.hako" -- \
        --stage-b --stage3
  ) > "$raw_path" 2>&1
}

emit_stageb_program_json_raw() {
  local raw_path="$1"
  stageb_cmd_desc=""
  if buildbox_emit_only_keep_requested; then
    stageb_cmd_desc="BuildBox.emit_program_json_v0 via compiler build_box"
    ensure_stageb_module_roots_list
    emit_program_json_v0_via_buildbox "$raw_path"
    return $?
  fi

  stageb_cmd_desc="compiler.hako --stage-b --stage3"
  emit_program_json_v0_via_stageb_compiler "$raw_path"
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

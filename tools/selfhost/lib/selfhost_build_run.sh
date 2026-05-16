#!/usr/bin/env bash
# selfhost_build_run.sh — run route helpers
#
# Purpose:
# - Own direct MIR(JSON) execution for normal `--run`.
# - Keep run logic separate from direct MIR output and EXE artifact helpers.

run_mir_json_via_direct_loader() {
  local mir_path="$1"
  selfhost_phase_start "selfhost.run"
  set +e
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$BIN" --mir-json-file "$mir_path" >/dev/null 2>&1
  local rc=$?
  set -e
  if [ "$rc" -eq 0 ]; then
    selfhost_phase_done "selfhost.run"
  else
    selfhost_phase_fail "selfhost.run" "$rc"
  fi
  return $rc
}

cleanup_run_mir_tmp_if_needed() {
  local mir_path="$1"
  if [ -n "${MIR_IN:-}" ]; then
    return 0
  fi
  if [ -z "$MIR_OUT" ]; then
    rm -f "$mir_path" 2>/dev/null || true
  fi
}

run_requested_direct_mir() {
  local mir_path="${MIR_IN:-${MIR_OUT:-/tmp/hako_run_mir_$$.json}}"
  local rc=0

  if [ -n "${MIR_IN:-}" ]; then
    echo "[selfhost] using MIR JSON input → $mir_path" >&2
  else
    emit_mir_json_from_source "$mir_path" || rc=$?
  fi
  if [ "$rc" -eq 0 ]; then
    run_mir_json_via_direct_loader "$mir_path" || rc=$?
  fi
  cleanup_run_mir_tmp_if_needed "$mir_path"
  return $rc
}

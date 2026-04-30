#!/usr/bin/env bash
# selfhost_build_run.sh — run route helpers
#
# Purpose:
# - Own direct MIR(JSON) execution for normal `--run`.
# - Keep run logic separate from direct MIR output and EXE artifact helpers.

run_mir_json_via_direct_loader() {
  local mir_path="$1"
  set +e
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$BIN" --mir-json-file "$mir_path" >/dev/null 2>&1
  local rc=$?
  set -e
  return $rc
}

cleanup_run_mir_tmp_if_needed() {
  local mir_path="$1"
  if [ -z "$MIR_OUT" ]; then
    rm -f "$mir_path" 2>/dev/null || true
  fi
}

run_requested_direct_mir() {
  local mir_path="${MIR_OUT:-/tmp/hako_run_mir_$$.json}"
  local rc=0

  emit_mir_json_from_source "$mir_path" || rc=$?
  if [ "$rc" -eq 0 ]; then
    run_mir_json_via_direct_loader "$mir_path" || rc=$?
  fi
  cleanup_run_mir_tmp_if_needed "$mir_path"
  return $rc
}

#!/usr/bin/env bash
# selfhost_build_run.sh — Stage-B run keeper helpers
#
# Purpose:
# - Own the remaining Program(JSON v0) -> core-direct run path.
# - Keep this keeper separate from direct MIR output and EXE artifact helpers.

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


#!/bin/bash
# Shared helpers for JoinIR port smokes.

joinir_port_check_selfhost_first_log() {
  local smoke_name="$1"
  local log_path="$2"

  if ! grep -Fq "[OK] MIR JSON written (selfhost-first):" "$log_path"; then
    log_error "$smoke_name: hako route did not use selfhost-first path"
    return 1
  fi
  if grep -Fq "[OK] MIR JSON written (direct-emit):" "$log_path"; then
    log_error "$smoke_name: direct-emit fallback detected"
    return 1
  fi
  if grep -Fq "[OK] MIR JSON written (delegate:" "$log_path"; then
    log_error "$smoke_name: delegate fallback detected"
    return 1
  fi

  return 0
}

joinir_port_require_main_ops() {
  local smoke_name="$1"
  local mir_json="$2"
  shift 2

  if ! jq -e '(.functions | map(select(.name=="main")) | length) == 1' "$mir_json" >/dev/null; then
    log_error "$smoke_name: main function missing in MIR"
    return 1
  fi

  local op
  for op in "$@"; do
    if ! jq -e --arg op "$op" '
      (.functions | map(select(.name=="main")) | .[0]) as $m
      | (($m.blocks[].instructions[] | select(.op == $op)) | length) >= 1
    ' "$mir_json" >/dev/null; then
      log_error "$smoke_name: required op missing in main: $op"
      return 1
    fi
  done

  return 0
}

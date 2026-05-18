#!/usr/bin/env bash

dev_gate_group_fail() {
  echo "[dev-gate-group] ERROR: $1" >&2
  return 2
}

dev_gate_group_require_steps_file() {
  local steps_file="$1"
  [[ -f "$steps_file" ]] || dev_gate_group_fail "missing steps file: ${steps_file}"
}

dev_gate_group_require_script() {
  local script="$1"
  [[ -f "$script" ]] || dev_gate_group_fail "missing script: ${script}"
}

dev_gate_script_step() {
  local label="$1"
  local script="$2"

  dev_gate_group_require_script "$script" || return $?
  if [[ "${DEV_GATE_GROUP_MODE:-}" == "list" ]]; then
    echo "    - ${script}"
    return 0
  fi

  echo "[${DEV_GATE_GROUP_TAG}] >>> ${label}"
  bash "$script"
}

dev_gate_cmd_step() {
  local label="$1"
  local display="$2"
  shift 2

  if [[ $# -eq 0 ]]; then
    dev_gate_group_fail "command step '${label}' is missing argv"
    return $?
  fi
  if [[ "${DEV_GATE_GROUP_MODE:-}" == "list" ]]; then
    echo "    - ${display}"
    return 0
  fi

  echo "[${DEV_GATE_GROUP_TAG}] >>> ${label}"
  "$@"
}

dev_gate_group_source() {
  local steps_file="$1"
  dev_gate_group_require_steps_file "$steps_file" || return $?
  # shellcheck source=/dev/null
  source "$steps_file"
}

dev_gate_group_list() {
  local steps_file="$1"
  DEV_GATE_GROUP_MODE="list" DEV_GATE_GROUP_TAG="" dev_gate_group_source "$steps_file"
}

dev_gate_group_run() {
  local tag="$1"
  local steps_file="$2"
  DEV_GATE_GROUP_MODE="run" DEV_GATE_GROUP_TAG="$tag" dev_gate_group_source "$steps_file"
}

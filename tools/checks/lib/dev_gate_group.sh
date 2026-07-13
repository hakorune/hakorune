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

dev_gate_group_verbose() {
  [[ "${DEV_GATE_VERBOSE:-0}" == "1" ]]
}

dev_gate_group_latched_status() {
  printf '%s' "${DEV_GATE_GROUP_FAILED_STATUS:-0}"
}

dev_gate_group_latch_failure() {
  local status="$1"
  DEV_GATE_GROUP_FAILED_STATUS="$status"
  return "$status"
}

dev_gate_group_execute() {
  local label="$1"
  shift

  local latched
  latched="$(dev_gate_group_latched_status)"
  if [[ "$latched" != "0" ]]; then
    return "$latched"
  fi

  DEV_GATE_GROUP_STEP_COUNT=$((DEV_GATE_GROUP_STEP_COUNT + 1))
  local step_index="$DEV_GATE_GROUP_STEP_COUNT"
  local log_file
  printf -v log_file '%s/%03d.log' "$DEV_GATE_GROUP_LOG_DIR" "$step_index"
  local started="$SECONDS"
  local status=0

  if dev_gate_group_verbose; then
    echo "[${DEV_GATE_GROUP_TAG}] >>> ${label}"
    if "$@" 2>&1 | tee "$log_file"; then
      status=${PIPESTATUS[0]}
    else
      status=${PIPESTATUS[0]}
    fi
  elif "$@" >"$log_file" 2>&1; then
    :
  else
    status=$?
  fi

  local elapsed=$((SECONDS - started))
  if [[ "$status" == "0" ]]; then
    DEV_GATE_GROUP_PASS_COUNT=$((DEV_GATE_GROUP_PASS_COUNT + 1))
    rm -f "$log_file"
    return 0
  fi

  echo "[${DEV_GATE_GROUP_TAG}] FAIL ${label} (exit=${status}, ${elapsed}s)" >&2
  if dev_gate_group_verbose; then
    echo "[${DEV_GATE_GROUP_TAG}] full_log=${log_file}" >&2
  else
    local tail_lines="${DEV_GATE_FAILURE_TAIL_LINES:-80}"
    if [[ ! "$tail_lines" =~ ^[1-9][0-9]*$ ]]; then
      tail_lines=80
    fi
    echo "[${DEV_GATE_GROUP_TAG}] --- failing output (last ${tail_lines} lines) ---" >&2
    tail -n "$tail_lines" "$log_file" >&2
    echo "[${DEV_GATE_GROUP_TAG}] full_log=${log_file}" >&2
  fi
  dev_gate_group_latch_failure "$status"
}

dev_gate_script_step() {
  local label="$1"
  local script="$2"

  local latched
  latched="$(dev_gate_group_latched_status)"
  if [[ "$latched" != "0" ]]; then
    return "$latched"
  fi
  if ! dev_gate_group_require_script "$script"; then
    dev_gate_group_latch_failure 2
    return 2
  fi
  if [[ "${DEV_GATE_GROUP_MODE:-}" == "list" ]]; then
    echo "    - ${script}"
    return 0
  fi

  dev_gate_group_execute "$label" bash "$script"
}

dev_gate_cmd_step() {
  local label="$1"
  local display="$2"
  shift 2

  if [[ $# -eq 0 ]]; then
    dev_gate_group_fail "command step '${label}' is missing argv"
    if [[ "${DEV_GATE_GROUP_MODE:-}" != "list" ]]; then
      DEV_GATE_GROUP_FAILED_STATUS=2
    fi
    return 2
  fi
  if [[ "${DEV_GATE_GROUP_MODE:-}" == "list" ]]; then
    echo "    - ${display}"
    return 0
  fi

  dev_gate_group_execute "$label" "$@"
}

dev_gate_group_source() {
  local steps_file="$1"
  dev_gate_group_require_steps_file "$steps_file" || return $?
  # shellcheck source=/dev/null
  source "$steps_file"
  local source_status=$?
  local latched
  latched="$(dev_gate_group_latched_status)"
  if [[ "$latched" != "0" ]]; then
    return "$latched"
  fi
  return "$source_status"
}

dev_gate_group_list() {
  local steps_file="$1"
  DEV_GATE_GROUP_MODE="list" DEV_GATE_GROUP_TAG="" dev_gate_group_source "$steps_file"
}

dev_gate_group_run() {
  local tag="$1"
  local steps_file="$2"
  local safe_tag="${tag//[^a-zA-Z0-9._-]/-}"
  local log_dir
  log_dir="$(mktemp -d "${TMPDIR:-/tmp}/hakorune-${safe_tag}.XXXXXX")"
  local started="$SECONDS"

  DEV_GATE_GROUP_MODE="run"
  DEV_GATE_GROUP_TAG="$tag"
  DEV_GATE_GROUP_LOG_DIR="$log_dir"
  DEV_GATE_GROUP_FAILED_STATUS=0
  DEV_GATE_GROUP_STEP_COUNT=0
  DEV_GATE_GROUP_PASS_COUNT=0

  local status=0
  if dev_gate_group_source "$steps_file"; then
    :
  else
    status=$?
  fi
  if [[ "$status" != "0" ]]; then
    return "$status"
  fi

  rm -rf "$log_dir"
  echo "[${tag}] PASS ${DEV_GATE_GROUP_PASS_COUNT}/${DEV_GATE_GROUP_STEP_COUNT} ($((SECONDS - started))s)"
}

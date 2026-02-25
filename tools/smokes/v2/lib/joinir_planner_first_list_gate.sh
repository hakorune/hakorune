#!/bin/bash
# joinir_planner_first_list_gate.sh - list runner for planner-first gate cases
# Use __EMPTY__ in the expected column to represent an empty stdout.

run_planner_first_list_gate() {
  local list_file="$1"
  local gate_name="$2"
  local timeout_secs="${3:-${RUN_TIMEOUT_SECS:-10}}"

  if [ -z "$list_file" ]; then
    log_error "planner_first_list_gate: list_file is required"
    return 1
  fi

  if [ -z "$gate_name" ]; then
    gate_name="$(basename "$list_file")"
  fi

  if [ ! -f "$list_file" ]; then
    log_error "$gate_name: list not found: $list_file"
    return 1
  fi

  local fail=0
  local fixture expected allowed_rc planner_tag

  while IFS=$'\t' read -r fixture expected allowed_rc planner_tag _rest; do
    if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
      continue
    fi

    fixture=${fixture//$'\r'/}
    expected=${expected//$'\r'/}
    allowed_rc=${allowed_rc//$'\r'/}
    planner_tag=${planner_tag//$'\r'/}

    if [ "$expected" = "__EMPTY__" ]; then
      expected=""
    fi

    if [ -z "$planner_tag" ]; then
      log_error "$gate_name: planner_tag missing for $fixture"
      return 1
    fi

    if [ -z "$allowed_rc" ]; then
      allowed_rc="0"
    fi

    if [[ "$fixture" != /* ]]; then
      fixture="$NYASH_ROOT/$fixture"
    fi

    local case_name
    case_name=$(basename "$fixture")

    if ! run_planner_first_gate \
      "$gate_name:$case_name" \
      "$fixture" \
      "$expected" \
      "$planner_tag" \
      "$allowed_rc" \
      "$timeout_secs"; then
      fail=1
      break
    fi
  done < "$list_file"

  if [ "$fail" -ne 0 ]; then
    return 1
  fi

  log_success "$gate_name: PASS"
  return 0
}

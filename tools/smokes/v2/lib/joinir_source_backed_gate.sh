#!/bin/bash
# joinir_source_backed_gate.sh - list runner for source-backed (--backend mir)
# acceptance of portable-owner corpus rows.
#
# Contract per case row: fixture + expected output + allowed exit code on the
# source-backed lane. There is no planner-tag column: planner_first/flowbox
# tags were retired-machinery evidence and the sole source-backed route emits
# none — the lane's acceptance proof is the preserved run itself.
#
# TSV columns: fixture  expected  allowed_rc  case_id  reason
# Use __EMPTY__ in the expected column to represent an empty stdout.
#
# NOTE: do not pin HAKO_JOINIR_STRICT / HAKO_JOINIR_PLANNER_REQUIRED here —
# combined they force the retired planner admission and freeze the
# source-backed route.

set -e

LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$LIB_DIR/env.sh" ]; then
  source "$LIB_DIR/env.sh"
fi

# Hermetic source-backed runner: strips developer-local debug/trace envs and
# explicitly neutralizes the retired planner pins so a caller-side export
# cannot leak them into the mir lane.
run_hermetic_source_backed() {
  env \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_CLI_VERBOSE=0 \
    HAKO_JOINIR_STRICT=0 \
    HAKO_JOINIR_PLANNER_REQUIRED=0 \
    HAKO_JOINIR_DEBUG=0 \
    HAKO_DEBUG=0 \
    HAKO_SHOW_CALL_LOGS=0 \
    HAKO_SILENT_TAGS=1 \
    "$@"
}

source_backed_exit_code_allowed() {
  local exit_code="$1"
  local allowed_codes="$2"
  local code

  for code in $allowed_codes; do
    if [ "$exit_code" -eq "$code" ]; then
      return 0
    fi
  done

  return 1
}

run_source_backed_gate() {
  local test_name="$1"
  local fixture="$2"
  local expected="$3"
  local allowed_codes="${4:-0}"
  local timeout_secs="${5:-10}"

  if [ -z "$test_name" ] || [ -z "$fixture" ]; then
    log_error "source_backed: missing required arguments"
    return 1
  fi

  export NYASH_ALLOW_USING_FILE=1

  set +e
  local output
  output=$(run_hermetic_source_backed \
    timeout "$timeout_secs" \
    "$NYASH_BIN" --backend mir "$fixture" 2>&1)
  local exit_code=$?
  set -e

  if [ "$exit_code" -eq 124 ]; then
    log_error "$test_name: hakorune timed out (> ${timeout_secs}s)"
    return 1
  fi

  if ! source_backed_exit_code_allowed "$exit_code" "$allowed_codes"; then
    log_error "$test_name: expected exit code(s) $allowed_codes, got $exit_code"
    echo "$output"
    return 1
  fi

  local output_clean
  output_clean=$(echo "$output" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]' || true)

  compare_outputs "$expected" "$output_clean" "$test_name" || return 1

  log_success "$test_name: PASS (exit=$exit_code)"
  return 0
}

run_source_backed_list_gate() {
  local list_file="$1"
  local gate_name="$2"
  local timeout_secs="${3:-${RUN_TIMEOUT_SECS:-10}}"

  if [ -z "$list_file" ]; then
    log_error "source_backed_list_gate: list_file is required"
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
  local fixture expected allowed_rc case_id

  while IFS=$'\t' read -r fixture expected allowed_rc case_id _rest; do
    if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
      continue
    fi

    fixture=${fixture//$'\r'/}
    expected=${expected//$'\r'/}
    allowed_rc=${allowed_rc//$'\r'/}
    case_id=${case_id//$'\r'/}

    if [ "$expected" = "__EMPTY__" ]; then
      expected=""
    fi

    if [ -z "$allowed_rc" ]; then
      allowed_rc="0"
    fi

    if [[ "$fixture" != /* ]]; then
      fixture="$NYASH_ROOT/$fixture"
    fi

    local case_name="${case_id:-$(basename "$fixture")}"

    local case_timeout_secs="$timeout_secs"
    if [[ "$_rest" =~ (^|[[:space:]])timeout=([0-9]+)([[:space:]]|$) ]]; then
      case_timeout_secs="${BASH_REMATCH[2]}"
    fi

    if ! run_source_backed_gate \
      "$gate_name:$case_name" \
      "$fixture" \
      "$expected" \
      "$allowed_rc" \
      "$case_timeout_secs"; then
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

# --- typed-terminal reject contract -----------------------------------------
#
# run_source_backed_terminal_gate verifies that a row's canonical lane
# outcome is a typed terminal rejection: nonzero exit AND the pinned
# `[authority/terminal]` marker substring present in combined output.
# Rows gated here were dispositioned `accepted-typed-reject` or
# `nonproduction-future-evidence` in the legacy-disposition manifest: the VM
# lane's acceptance came from retired machinery, so the source-backed lane
# must reject with the recorded typed terminal rather than silently accept.
#
# TSV columns: fixture  terminal_marker  case_id  reason

run_source_backed_terminal_gate() {
  local test_name="$1"
  local fixture="$2"
  local marker="$3"
  local timeout_secs="${4:-10}"

  if [ -z "$test_name" ] || [ -z "$fixture" ] || [ -z "$marker" ]; then
    log_error "source_backed_terminal: missing required arguments"
    return 1
  fi

  export NYASH_ALLOW_USING_FILE=1

  set +e
  local output
  output=$(run_hermetic_source_backed \
    timeout "$timeout_secs" \
    "$NYASH_BIN" --backend mir "$fixture" 2>&1)
  local exit_code=$?
  set -e

  if [ "$exit_code" -eq 124 ]; then
    log_error "$test_name: hakorune timed out (> ${timeout_secs}s)"
    return 1
  fi

  if [ "$exit_code" -eq 0 ]; then
    log_error "$test_name: expected typed terminal reject, got exit 0"
    echo "$output"
    return 1
  fi

  if ! echo "$output" | grep -qF -- "$marker"; then
    log_error "$test_name: terminal marker '$marker' not found in output"
    echo "$output" | tail -n 15
    return 1
  fi

  log_success "$test_name: PASS (typed terminal: $marker)"
  return 0
}

run_source_backed_terminal_list_gate() {
  local list_file="$1"
  local gate_name="$2"
  local timeout_secs="${3:-${RUN_TIMEOUT_SECS:-10}}"

  if [ -z "$list_file" ] || [ ! -f "$list_file" ]; then
    log_error "${gate_name:-terminal_list_gate}: list not found: $list_file"
    return 1
  fi
  if [ -z "$gate_name" ]; then
    gate_name="$(basename "$list_file")"
  fi

  local fail=0
  local fixture marker case_id

  while IFS=$'\t' read -r fixture marker case_id _rest; do
    if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
      continue
    fi

    fixture=${fixture//$'\r'/}
    marker=${marker//$'\r'/}
    case_id=${case_id//$'\r'/}

    if [[ "$fixture" != /* ]]; then
      fixture="$NYASH_ROOT/$fixture"
    fi

    local case_name="${case_id:-$(basename "$fixture")}"

    local case_timeout_secs="$timeout_secs"
    if [[ "$_rest" =~ (^|[[:space:]])timeout=([0-9]+)([[:space:]]|$) ]]; then
      case_timeout_secs="${BASH_REMATCH[2]}"
    fi

    if ! run_source_backed_terminal_gate \
      "$gate_name:$case_name" \
      "$fixture" \
      "$marker" \
      "$case_timeout_secs"; then
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

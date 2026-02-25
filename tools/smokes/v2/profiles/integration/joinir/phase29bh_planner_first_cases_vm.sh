#!/bin/bash
# phase29bh_planner_first_cases_vm.sh - planner-first cases (data-driven)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
require_env || exit 2

LIST_FILE="$(dirname "$0")/planner_first_cases.tsv"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

if [ ! -f "$LIST_FILE" ]; then
  log_error "phase29bh_planner_first_cases_vm: list not found: $LIST_FILE"
  exit 1
fi

fail=0
while IFS=$'\t' read -r fixture expected allow_rc planner_tag reason; do
  if [ -z "$fixture" ] || [[ "$fixture" == \#* ]]; then
    continue
  fi

  fixture=$(echo "$fixture" | tr -d '\r')
  expected=$(echo "$expected" | tr -d '\r')
  allow_rc=$(echo "$allow_rc" | tr -d '\r')
  planner_tag=$(echo "$planner_tag" | tr -d '\r')

  if [ -z "$allow_rc" ]; then
    allow_rc="0"
  fi

  if [[ "$fixture" != /* ]]; then
    fixture="$NYASH_ROOT/$fixture"
  fi

  case_name=$(basename "$fixture")
  if ! run_planner_first_gate \
    "phase29bh_planner_first_cases_vm:$case_name" \
    "$fixture" \
    "$expected" \
    "$planner_tag" \
    "$allow_rc" \
    "$RUN_TIMEOUT_SECS"; then
    fail=1
  fi

  if [ -n "$reason" ]; then
    log_info "phase29bh_planner_first_cases_vm: reason=$reason"
  fi

  if [ "$fail" -ne 0 ]; then
    break
  fi
done < "$LIST_FILE"

if [ "$fail" -ne 0 ]; then
  exit 1
fi

log_success "phase29bh_planner_first_cases_vm: PASS"
exit 0

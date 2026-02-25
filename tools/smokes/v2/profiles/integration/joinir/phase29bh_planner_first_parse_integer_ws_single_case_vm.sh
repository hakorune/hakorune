#!/bin/bash
# phase29bh_planner_first_parse_integer_ws_single_case_vm.sh - planner-first parse_integer ws gate (strict/dev)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29aq_string_parse_integer_ws_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
expected=$(cat << 'TXT'
0
TXT
)
run_planner_first_gate \
    "phase29bh_planner_first_parse_integer_ws_single_case_vm" \
    "$FIXTURE" \
    "$expected" \
    "[joinir/planner_first rule=Pattern2]" \
    "0" \
    "$RUN_TIMEOUT_SECS" || exit 1

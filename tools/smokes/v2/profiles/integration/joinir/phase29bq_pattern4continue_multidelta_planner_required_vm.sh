#!/bin/bash
# phase29bq_pattern4continue_multidelta_planner_required_vm.sh - legacy compat wrapper target for loop_continue_only multidelta gate
# Current semantic entry: loop_continue_only_multidelta_planner_required_vm.sh

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_pattern4continue_multidelta_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
expected=$(cat << 'TXT'
31
TXT
)
run_planner_first_gate \
    "phase29bq_pattern4continue_multidelta_planner_required_vm" \
    "$FIXTURE" \
    "$expected" \
    "[joinir/planner_first rule=LoopCondBreak]" \
    "0" \
    "$RUN_TIMEOUT_SECS" || exit 1

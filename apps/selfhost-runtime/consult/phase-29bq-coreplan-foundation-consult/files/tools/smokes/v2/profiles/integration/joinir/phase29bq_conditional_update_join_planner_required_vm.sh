#!/bin/bash
# phase29bq_conditional_update_join_planner_required_vm.sh - conditional update/join gate (strict/dev)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_conditional_update_join_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
expected=$(cat << 'TXT'
0
TXT
)
run_planner_first_gate \
    "phase29bq_conditional_update_join_planner_required_vm" \
    "$FIXTURE" \
    "$expected" \
    "[joinir/planner_first rule=Pattern1]" \
    "0" \
    "$RUN_TIMEOUT_SECS" || exit 1

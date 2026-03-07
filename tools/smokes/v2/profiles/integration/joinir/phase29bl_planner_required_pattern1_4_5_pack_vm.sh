#!/bin/bash
# phase29bl_planner_required_pattern1_4_5_pack_vm.sh - legacy pack stem for core loop routes planner-required gate
# Current semantic entry: core_loop_routes_planner_required_pack_vm.sh

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_list_gate.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

LIST_FILE="$(dirname "$0")/phase29bl_planner_required_pattern1_4_5_cases.tsv"
run_planner_first_list_gate \
  "$LIST_FILE" \
  "phase29bl_planner_required_pattern1_4_5_pack_vm" \
  "$RUN_TIMEOUT_SECS" || exit 1

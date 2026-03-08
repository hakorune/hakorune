#!/bin/bash
# loop_break_planner_required_pack_vm.sh - legacy pack stem for loop_break planner-required small set
# current semantic wrapper; canonical entry for loop_break_planner_required_pack_vm

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_list_gate.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
SCRIPT_STEM="${LEGACY_STEM_OVERRIDE:-loop_break_planner_required_pack_vm}"


LIST_FILE="$(dirname "$0")/phase29bi_planner_required_pattern2_cases.tsv"
run_planner_first_list_gate \
  "$LIST_FILE" \
  "loop_break_planner_required_pack_vm" \
  "$RUN_TIMEOUT_SECS" || exit 1

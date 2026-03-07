#!/bin/bash
# phase29bn_planner_required_pattern3_pack_vm.sh - legacy pack stem for if_phi_join planner-required gate
# Current semantic entry: if_phi_join_planner_required_pack_vm.sh

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_list_gate.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
LIST_FILE="$(dirname "$0")/phase29bn_planner_required_pattern3_cases.tsv"

run_planner_first_list_gate \
  "$LIST_FILE" \
  "phase29bn_planner_required_pattern3_pack_vm" \
  "$RUN_TIMEOUT_SECS" || exit 1

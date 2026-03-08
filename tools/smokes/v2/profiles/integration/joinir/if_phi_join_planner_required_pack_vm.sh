#!/bin/bash
# if_phi_join_planner_required_pack_vm.sh - legacy pack stem for if_phi_join planner-required gate
# current semantic wrapper; canonical entry for if_phi_join_planner_required_pack_vm

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_list_gate.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
SCRIPT_STEM="${LEGACY_STEM_OVERRIDE:-if_phi_join_planner_required_pack_vm}"

LIST_FILE="$(dirname "$0")/if_phi_join_planner_required_cases.tsv"

run_planner_first_list_gate \
  "$LIST_FILE" \
  "if_phi_join_planner_required_pack_vm" \
  "$RUN_TIMEOUT_SECS" || exit 1

#!/bin/bash
# scan_split_planner_required_pack_vm.sh - planner-required scan/split small pack (strict/dev)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_gate.sh"
source "$(dirname "$0")/../../../lib/joinir_planner_first_list_gate.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
SCRIPT_STEM="${LEGACY_STEM_OVERRIDE:-scan_split_planner_required_pack_vm}"


LIST_FILE="$(dirname "$0")/phase29bj_planner_required_scan_split_cases.tsv"
run_planner_first_list_gate \
  "$LIST_FILE" \
  "scan_split_planner_required_pack_vm" \
  "$RUN_TIMEOUT_SECS" || exit 1

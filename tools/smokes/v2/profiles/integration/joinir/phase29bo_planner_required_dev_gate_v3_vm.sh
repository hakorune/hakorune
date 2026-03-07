#!/bin/bash
# phase29bo_planner_required_dev_gate_v3_vm.sh - planner-required dev gate v3 aggregator

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
LOG_DIR="/tmp"

run_gate() {
  local gate="$1"
  local log_path="$2"

  if ! bash "$gate" 2>&1 | tee "$log_path"; then
    echo "[FAIL] gate failed: $gate"
    echo "LOG: $log_path"
    return 1
  fi

  return 0
}

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/loop_break_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_loop_break_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/scan_split_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_scan_split_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/core_loop_routes_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_loop_simple_while_loop_continue_only_loop_true_early_exit_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/if_phi_join_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_if_phi_join_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/bool_predicate_accum_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_bool_predicate_scan_accum_const_loop_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_regression_pack.log"

echo "[PASS] phase29bo_planner_required_dev_gate_v3_vm: all gates passed"

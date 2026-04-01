#!/bin/bash
# phase29bn_planner_required_dev_gate_v2_vm.sh - planner-required dev gate v2 aggregator

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
  "$LOG_DIR/phase29bn_v2_loop_break_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/scan_split_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bn_v2_scan_split_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/core_loop_routes_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bn_v2_loop_simple_while_loop_continue_only_loop_true_early_exit_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/if_phi_join_planner_required_pack_vm.sh" \
  "$LOG_DIR/phase29bn_v2_if_phi_join_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh" \
  "$LOG_DIR/phase29bn_v2_regression_pack.log"

echo "[PASS] phase29bn_planner_required_dev_gate_v2_vm: all gates passed"

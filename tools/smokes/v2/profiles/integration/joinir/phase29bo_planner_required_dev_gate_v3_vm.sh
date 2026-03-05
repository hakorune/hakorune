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

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_pattern2_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_scan_split_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_scan_split_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bl_planner_required_pattern1_4_5_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_pattern1_4_5_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bn_planner_required_pattern3_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_pattern3_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_pattern8_9_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh" \
  "$LOG_DIR/phase29bo_v3_regression_pack.log"

echo "[PASS] phase29bo_planner_required_dev_gate_v3_vm: all gates passed"

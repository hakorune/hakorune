#!/bin/bash
# phase29bk_planner_required_dev_gate_vm.sh - planner-required dev gate aggregator

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
LOG_DIR="/tmp"

run_gate() {
  local gate="$1"
  local log_path="$2"

  if ! "$gate" 2>&1 | tee "$log_path"; then
    echo "[FAIL] gate failed: $gate"
    echo "LOG: $log_path"
    return 1
  fi

  return 0
}

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh" \
  "$LOG_DIR/phase29bk_pattern2_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh" \
  "$LOG_DIR/phase29bk_pattern6_7_pack.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh" \
  "$LOG_DIR/phase29bk_regression_pack.log"

echo "[PASS] phase29bk_planner_required_dev_gate_vm: all gates passed"

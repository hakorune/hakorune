#!/bin/bash
# phase29bp_planner_required_dev_gate_v4_vm.sh - planner-required dev gate v4 (master list + regression)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
LOG_DIR="/tmp"

source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_planner_first_gate.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_planner_first_list_gate.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
LIST_FILE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv"

run_list_gate() {
  local log_path="$1"

  if ! (run_planner_first_list_gate \
    "$LIST_FILE" \
    "phase29bp_planner_required_dev_gate_v4_vm" \
    "$RUN_TIMEOUT_SECS") 2>&1 | tee "$log_path"; then
    echo "[FAIL] gate failed: planner_required_cases.tsv"
    echo "LOG: $log_path"
    return 1
  fi

  return 0
}

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

run_list_gate "$LOG_DIR/phase29bp_master_list.log"

run_gate "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh" \
  "$LOG_DIR/phase29bp_regression_pack.log"

echo "[PASS] phase29bp_planner_required_dev_gate_v4_vm: all gates passed"

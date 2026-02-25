#!/bin/bash
# phase29bq_continue_target_header_planner_required_vm.sh
#
# Strict/dev + planner_required gate for continue_target=header minimal case.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
LOG_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
LOG_PATH="$LOG_DIR/phase29bq_continue_target_header_planner_required.log"

GATE_FAST="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh"

if ! "$GATE_FAST" --only continue_target_header 2>&1 | tee "$LOG_PATH"; then
  echo "LOG: $LOG_PATH"
  exit 1
fi

echo "[PASS] phase29bq_continue_target_header_planner_required_vm: PASS"

#!/bin/bash
# phase29bq_fast_gate_vm.sh - fast iteration gate for Phase 29bq (JoinIR/CorePlan)
#
# Default: run only the Phase 29bq lightweight gates (fast).
# Options:
#   --full            Run Phase 29bq lightweight gates, then 29bp dev gate (includes 29ae regression).
#   --only {bq|loop_true|cond_update|loop_cond|29bp|29ae}
#
# Logs:
# - Writes per-step logs to /tmp (or PHASE29BQ_FAST_LOG_DIR)
# - On failure prints "LOG: <path>" as the last line.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
LOG_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"

source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

usage() {
  cat >&2 <<'EOF'
Usage:
  phase29bq_fast_gate_vm.sh [--full] [--only {bq|loop_true|cond_update|loop_cond|29bp|29ae}]
EOF
}

MODE="bq"
if [ "${1:-}" = "--full" ]; then
  MODE="full"
  shift
elif [ "${1:-}" = "--only" ]; then
  MODE="${2:-}"
  shift 2 || true
elif [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
  usage
  exit 0
elif [ -n "${1:-}" ]; then
  echo "[FAIL] Unknown arg: $1" >&2
  usage
  exit 2
fi

case "$MODE" in
  bq|loop_true|cond_update|loop_cond|29bp|29ae|full) ;;
  *)
    echo "[FAIL] Invalid --only value: $MODE" >&2
    usage
    exit 2
    ;;
esac

GATE_LOOP_TRUE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_loop_true_multi_break_planner_required_vm.sh"
GATE_COND_UPDATE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_conditional_update_join_planner_required_vm.sh"
GATE_LOOP_COND="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_loop_cond_multi_exit_planner_required_vm.sh"

GATE_29BP="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh"
GATE_29AE="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh"

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

RUN_ID="phase29bq_fast_gate_${$}"
LOG_LOOP_TRUE="$LOG_DIR/${RUN_ID}_loop_true.log"
LOG_COND_UPDATE="$LOG_DIR/${RUN_ID}_cond_update.log"
LOG_LOOP_COND="$LOG_DIR/${RUN_ID}_loop_cond.log"
LOG_29BP="$LOG_DIR/${RUN_ID}_29bp.log"
LOG_29AE="$LOG_DIR/${RUN_ID}_29ae.log"

run_bq_gates() {
  run_gate "$GATE_LOOP_TRUE" "$LOG_LOOP_TRUE"
  run_gate "$GATE_COND_UPDATE" "$LOG_COND_UPDATE"
  run_gate "$GATE_LOOP_COND" "$LOG_LOOP_COND"
}

case "$MODE" in
  bq)
    run_bq_gates
    ;;
  loop_true)
    run_gate "$GATE_LOOP_TRUE" "$LOG_LOOP_TRUE"
    ;;
  cond_update)
    run_gate "$GATE_COND_UPDATE" "$LOG_COND_UPDATE"
    ;;
  loop_cond)
    run_gate "$GATE_LOOP_COND" "$LOG_LOOP_COND"
    ;;
  29bp)
    run_gate "$GATE_29BP" "$LOG_29BP"
    ;;
  29ae)
    run_gate "$GATE_29AE" "$LOG_29AE"
    ;;
  full)
    run_bq_gates
    # NOTE: 29bp dev gate already runs 29ae regression pack (SSOT).
    run_gate "$GATE_29BP" "$LOG_29BP"
    ;;
esac

echo "[PASS] phase29bq_fast_gate_vm: PASS (mode=$MODE)"


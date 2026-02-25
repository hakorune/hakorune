#!/bin/bash
# phase29bs_fast_gate_vm.sh - fast iteration gate for JoinIR/CorePlan work (Phase 29bs)
#
# Default: run only the Phase 29bs nested-loop gate (fast).
# Options:
#   --full            Run 29bs gate, then 29bp dev gate (which includes 29ae regression).
#   --only {29bs|29bp|29ae}
#
# Logs:
# - Writes per-step logs to /tmp (or PHASE29BS_FAST_LOG_DIR)
# - On failure prints "LOG: <path>" as the last line.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
LOG_DIR="${PHASE29BS_FAST_LOG_DIR:-/tmp}"

source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

usage() {
  cat >&2 <<'EOF'
Usage:
  phase29bs_fast_gate_vm.sh [--full] [--only {29bs|29bp|29ae}]
EOF
}

MODE="29bs"
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
  29bs|29bp|29ae|full) ;;
  *)
    echo "[FAIL] Invalid --only value: $MODE" >&2
    usage
    exit 2
    ;;
esac

GATE_29BS="$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bs_loopframe_v1_nested_loop_strict_gate_vm.sh"
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

RUN_ID="phase29bs_fast_gate_${$}"
LOG_29BS="$LOG_DIR/${RUN_ID}_29bs.log"
LOG_29BP="$LOG_DIR/${RUN_ID}_29bp.log"
LOG_29AE="$LOG_DIR/${RUN_ID}_29ae.log"

case "$MODE" in
  29bs)
    run_gate "$GATE_29BS" "$LOG_29BS"
    ;;
  29bp)
    run_gate "$GATE_29BP" "$LOG_29BP"
    ;;
  29ae)
    run_gate "$GATE_29AE" "$LOG_29AE"
    ;;
  full)
    run_gate "$GATE_29BS" "$LOG_29BS"
    # NOTE: 29bp dev gate already runs 29ae regression pack (SSOT).
    run_gate "$GATE_29BP" "$LOG_29BP"
    ;;
esac

echo "[PASS] phase29bs_fast_gate_vm: PASS (mode=$MODE)"


#!/bin/bash
# phase29bq_collect_planner_required_blocker_vm.sh
#
# Purpose:
# - Run a fixture under planner-required mode and save the earliest freeze/reject line
#   plus the nearest StepTree root to /tmp (SSOT-friendly blocker capture for BoxCount).
#
# Output:
# - /tmp/phase29bq_joinir_blocker_<label>_<pid>.log
# - /tmp/phase29bq_joinir_blocker_<label>_<pid>.summary

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

FIXTURE="${1:-}"
LABEL="${2:-blocker}"

if [ -z "$FIXTURE" ]; then
  echo "[usage] $0 <fixture.hako> [label]" >&2
  exit 2
fi

if [ ! -f "$FIXTURE" ]; then
  echo "[error] fixture not found: $FIXTURE" >&2
  exit 2
fi

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_joinir_blocker_${LABEL}_${$}"
LOG="$TMP_DIR/${RUN_ID}.log"
SUMMARY="$TMP_DIR/${RUN_ID}.summary"

rm -f "$LOG" "$SUMMARY"

export NYASH_DISABLE_PLUGINS=1
export HAKO_JOINIR_STRICT=1
export HAKO_JOINIR_PLANNER_REQUIRED=1

set +e
"$NYASH_BIN" --backend vm "$FIXTURE" >"$LOG" 2>&1
RC=$?
set -e

MARKER="$(rg -n "\\[(plan/freeze:|plan/reject|joinir/freeze|freeze:)" "$LOG" | head -n 1 || true)"
if [ -n "$MARKER" ]; then
  MARKER_LINE="${MARKER%%:*}"
else
  MARKER_LINE="99999999"
fi

STEPTREE="$(awk -v stop="$MARKER_LINE" 'NR < stop && $0 ~ /StepTree root for/ { last = NR ":" $0 } END { print last }' "$LOG")"

{
  echo "fixture=$FIXTURE"
  echo "label=$LABEL"
  echo "rc=$RC"
  echo "log=$LOG"
  echo
  echo "first_freeze_or_reject="
  echo "$MARKER"
  echo
  echo "step_tree_root_nearby="
  echo "$STEPTREE"
} >"$SUMMARY"

echo "[ok] wrote $LOG"
echo "[ok] wrote $SUMMARY"
exit 0

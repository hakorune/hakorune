#!/bin/bash
# record_phase29x_x22_evidence.sh
# Run Phase 29x X22 gate set and print a Markdown row for evidence table.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DAY="${1:-}"
DATE_OVERRIDE="${2:-}"

if [ -z "$DAY" ]; then
  echo "Usage: $0 <day-number> [YYYY-MM-DD]" >&2
  echo "Example: $0 2 2026-02-14" >&2
  exit 2
fi

if ! [[ "$DAY" =~ ^[0-9]+$ ]]; then
  echo "day-number must be integer: $DAY" >&2
  exit 2
fi

DATE_ISO="${DATE_OVERRIDE:-$(date +%F)}"
if ! [[ "$DATE_ISO" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
  echo "date must be YYYY-MM-DD: $DATE_ISO" >&2
  exit 2
fi

LOG_PATH="/tmp/phase29x_x22_day${DAY}_${DATE_ISO}.log"
rm -f "$LOG_PATH"

run_step() {
  local label="$1"
  shift
  echo "== $label ==" | tee -a "$LOG_PATH"
  set +e
  "$@" 2>&1 | tee -a "$LOG_PATH"
  local rc=${PIPESTATUS[0]}
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "[x22-evidence] FAIL: $label (rc=$rc)" | tee -a "$LOG_PATH"
    echo "| $DAY | $DATE_ISO | FAIL | $label failed (rc=$rc); log=$LOG_PATH |"
    exit "$rc"
  fi
}

run_step "cargo-check" cargo check -q --bin hakorune
run_step "vm-route-observability" bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh
run_step "vm-route-strict-dev-priority" bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh
run_step "vm-route-non-strict-compat-boundary" bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh
run_step "selfhost-gate-5cases" ./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4

SUMMARY_LINE="$(rg -n "\[diag/selfhost\] gate_summary status=PASS" "$LOG_PATH" | tail -n 1 | sed -E 's/^[0-9]+://')"
STAGEB_TOTAL="$(echo "$SUMMARY_LINE" | sed -n 's/.*stageb_total_secs=\([0-9]\+\).*/\1/p')"
AVG_CASE="$(echo "$SUMMARY_LINE" | sed -n 's/.*avg_case_secs=\([0-9.]\+\).*/\1/p')"

if [ -z "$STAGEB_TOTAL" ] || [ -z "$AVG_CASE" ]; then
  echo "| $DAY | $DATE_ISO | PASS | route 3 smoke + 5-case selfhost gate PASS (log=$LOG_PATH) |"
else
  echo "| $DAY | $DATE_ISO | PASS | route 3 smoke + 5-case selfhost gate PASS (\`stageb_total_secs=$STAGEB_TOTAL\`, \`avg_case_secs=$AVG_CASE\`) |"
fi

echo "[x22-evidence] PASS log=$LOG_PATH"

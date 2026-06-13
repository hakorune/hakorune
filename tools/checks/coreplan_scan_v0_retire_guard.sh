#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-scan-v0-retire"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1013-COREPLAN-E1-005-SCAN-V0-RETIRE.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/coreplan_scan_v0_retire_guard.sh"
ACTIVE_GUARD="tools/checks/coreplan_active_v0_inventory_guard.sh"
REGISTRY_MD="src/mir/builder/control_flow/plan/REGISTRY.md"
LEGACY_MD="src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md"
SMOKE_CASES="tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv"

echo "[$TAG] checking scan_v0 retire"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$ACTIVE_GUARD" \
  "$REGISTRY_MD" \
  "$LEGACY_MD" \
  "$SMOKE_CASES"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$ACTIVE_GUARD"

guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-E1-005" \
  "$TASKBOARD" \
  "taskboard must record E1-005 scan_v0 retire"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"
guard_expect_fixed_in_file "$TAG" \
  "scan_loop_v0_comma_close_min" \
  "$SMOKE_CASES" \
  "focused comma/close scan smoke case must remain available"
guard_expect_fixed_in_file "$TAG" \
  "scan_loop_v0_lte_n_minus1_min" \
  "$SMOKE_CASES" \
  "focused range scan smoke case must remain available"
guard_expect_fixed_in_file "$TAG" \
  "loop_scan_v0" \
  "$REGISTRY_MD" \
  "registry must keep retired history for loop_scan_v0"
guard_expect_fixed_in_file "$TAG" \
  "loop_scan_v0" \
  "$LEGACY_MD" \
  "legacy boundary must keep retired history for loop_scan_v0"

if rg -n \
  "loop_scan_v0|LoopScanV0|SCAN_V0|scan_v0" \
  src/mir/builder/control_flow \
  -g '*.rs' >/tmp/coreplan-scan-v0-retire.refs 2>&1; then
  echo "[$TAG] ERROR: retired loop_scan_v0 code reference found" >&2
  cat /tmp/coreplan-scan-v0-retire.refs >&2
  rm -f /tmp/coreplan-scan-v0-retire.refs
  exit 1
fi
rm -f /tmp/coreplan-scan-v0-retire.refs

bash "$ACTIVE_GUARD"

echo "[$TAG] retired_box=loop_scan_v0"
echo "[$TAG] replacement_owner=loop_cond_break_continue"
echo "[$TAG] ok"

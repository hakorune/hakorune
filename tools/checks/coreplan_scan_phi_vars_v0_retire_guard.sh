#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-scan-phi-vars-v0-retire"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1015-COREPLAN-E1-007-SCAN-PHI-VARS-V0-RETIRE.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/coreplan_scan_phi_vars_v0_retire_guard.sh"
ACTIVE_GUARD="tools/checks/coreplan_active_v0_inventory_guard.sh"
REGISTRY_MD="src/mir/builder/control_flow/plan/REGISTRY.md"
LEGACY_MD="src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md"
SMOKE_CASES="tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv"

echo "[$TAG] checking scan_phi_vars_v0 retire"

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
  "COREPLAN-E1-007" \
  "$TASKBOARD" \
  "taskboard must record E1-007 scan_phi_vars_v0 retire"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"

for case_id in \
  "phi_injector_len_loop" \
  "phi_injector_var_step_len_loop" \
  "selfhost_phi_injector_k_loop_no_exit_min" \
  "selfhost_phi_collect_outer_loop_min" \
  "phi_injector_nested_loop_no_exit_var_step_min" \
  "selfhost_phi_injector_nested_loop_count_min"; do
  guard_expect_fixed_in_file "$TAG" \
    "$case_id" \
    "$SMOKE_CASES" \
    "focused scan-phi-vars smoke case must remain available: $case_id"
done

guard_expect_fixed_in_file "$TAG" \
  "loop_scan_phi_vars_v0" \
  "$REGISTRY_MD" \
  "registry must keep retired history for loop_scan_phi_vars_v0"
guard_expect_fixed_in_file "$TAG" \
  "loop_scan_phi_vars_v0" \
  "$LEGACY_MD" \
  "legacy boundary must keep retired history for loop_scan_phi_vars_v0"

if rg -n \
  "loop_scan_phi_vars_v0|LoopScanPhiVars|SCAN_PHI_VARS|scan_phi_vars_v0" \
  src/mir/builder/control_flow \
  -g '*.rs' >/tmp/coreplan-scan-phi-vars-v0-retire.refs 2>&1; then
  echo "[$TAG] ERROR: retired loop_scan_phi_vars_v0 code reference found" >&2
  cat /tmp/coreplan-scan-phi-vars-v0-retire.refs >&2
  rm -f /tmp/coreplan-scan-phi-vars-v0-retire.refs
  exit 1
fi
rm -f /tmp/coreplan-scan-phi-vars-v0-retire.refs

bash "$ACTIVE_GUARD"

echo "[$TAG] retired_box=loop_scan_phi_vars_v0"
echo "[$TAG] replacement_owner=loop_simple_while_or_loop_cond_break_continue"
echo "[$TAG] ok"

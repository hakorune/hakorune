#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-first-v0-retire"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/coreplan_first_v0_retire_guard.sh"
ACTIVE_GUARD="tools/checks/coreplan_active_v0_inventory_guard.sh"
FACTS="src/mir/builder/control_flow/facts/loop_scan_methods_v0.rs"
RECIPE="src/mir/builder/control_flow/recipes/loop_scan_methods_v0.rs"
LOWER="src/mir/builder/control_flow/plan/loop_scan_methods_v0/segment_linear.rs"
ROUTER="src/mir/builder/control_flow/joinir/route_entry/router.rs"
REGISTRY_MD="src/mir/builder/control_flow/plan/REGISTRY.md"
LEGACY_MD="src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md"

echo "[$TAG] checking first legacy-v0 retire pilot"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$ACTIVE_GUARD" \
  "$FACTS" \
  "$RECIPE" \
  "$LOWER" \
  "$ROUTER" \
  "$REGISTRY_MD" \
  "$LEGACY_MD"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$ACTIVE_GUARD"

guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-E1-002" \
  "$TASKBOARD" \
  "taskboard must record the first v0 retire pilot"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"

if rg -n \
  "loop_scan_methods_block_v0|LoopScanMethodsBlock|SCAN_METHODS_BLOCK|scan_methods_block" \
  src/mir/builder/control_flow \
  -g '*.rs' >/tmp/coreplan-first-v0-retire.refs 2>&1; then
  echo "[$TAG] ERROR: retired loop_scan_methods_block_v0 code reference found" >&2
  cat /tmp/coreplan-first-v0-retire.refs >&2
  rm -f /tmp/coreplan-first-v0-retire.refs
  exit 1
fi
rm -f /tmp/coreplan-first-v0-retire.refs

guard_expect_fixed_in_file "$TAG" \
  "LinearBlockRecipe::ExitAllowed" \
  "$FACTS" \
  "loop_scan_methods_v0 facts must publish ExitAllowed linear segments"
guard_expect_fixed_in_file "$TAG" \
  "LinearBlockRecipe::ExitAllowed" \
  "$LOWER" \
  "loop_scan_methods_v0 lowering must handle ExitAllowed linear segments"
guard_expect_fixed_in_file "$TAG" \
  "lower_exit_allowed_block_verified" \
  "$LOWER" \
  "loop_scan_methods_v0 lowering must lower ExitAllowed blocks directly"
guard_expect_fixed_in_file "$TAG" \
  "ExitAllowedBlockRecipe" \
  "$RECIPE" \
  "loop_scan_methods_v0 recipe vocabulary must own the folded block path"
guard_expect_fixed_in_file "$TAG" \
  "block_wrapped_scan_methods" \
  "$ROUTER" \
  "router test/helper naming must document folded block-wrapped scan_methods"
guard_expect_fixed_in_file "$TAG" \
  "loop_scan_methods_block_v0" \
  "$REGISTRY_MD" \
  "registry must keep retired history for loop_scan_methods_block_v0"
guard_expect_fixed_in_file "$TAG" \
  "loop_scan_methods_block_v0" \
  "$LEGACY_MD" \
  "legacy boundary must keep retired history for loop_scan_methods_block_v0"

bash "$ACTIVE_GUARD"

echo "[$TAG] retired_box=loop_scan_methods_block_v0"
echo "[$TAG] replacement_owner=loop_scan_methods_v0"
echo "[$TAG] ok"

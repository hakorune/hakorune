#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-active-v0-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1009-COREPLAN-E1-001-ACTIVE-V0-INVENTORY.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/coreplan_active_v0_inventory_guard.sh"
REGISTRY_MD="src/mir/builder/control_flow/plan/REGISTRY.md"
LEGACY_MD="src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md"
PLAN_MOD="src/mir/builder/control_flow/plan/mod.rs"
LOOP_TYPES="src/mir/builder/control_flow/plan/facts/loop_types.rs"
LOOP_BUILDER="src/mir/builder/control_flow/plan/facts/loop_builder.rs"
ROUTER="src/mir/builder/control_flow/joinir/route_entry/router.rs"
ENTRY_REGISTRY="src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs"
PREDICATES="src/mir/builder/control_flow/joinir/route_entry/registry/predicates.rs"
ROUTES="src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs"
COMPOSER="src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs"
MATCHER="src/mir/builder/control_flow/plan/recipe_tree/matcher/mod.rs"

echo "[$TAG] checking active routed loop_*_v0 inventory"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$REGISTRY_MD" \
  "$LEGACY_MD" \
  "$PLAN_MOD" \
  "$LOOP_TYPES" \
  "$LOOP_BUILDER" \
  "$ROUTER" \
  "$ENTRY_REGISTRY" \
  "$PREDICATES" \
  "$ROUTES" \
  "$COMPOSER" \
  "$MATCHER"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-E1-001: active-v0 inventory guard" \
  "$TASKBOARD" \
  "taskboard must keep E1 inventory row"
guard_expect_fixed_in_file "$TAG" \
  "active_v0_inventory_guard=1" \
  "$TASKBOARD" \
  "taskboard must record E1 inventory acceptance"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$CARD" \
  "E1 inventory card must name this guard"
guard_expect_fixed_in_file "$TAG" \
  "legacy_normalizer_empty_and_active_v0_empty_are_separate=1" \
  "$CARD" \
  "E1 inventory card must separate legacy normalizer and active-v0 closeout"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"

active_v0=(
  "loop_scan_v0"
  "loop_scan_methods_v0"
  "loop_scan_phi_vars_v0"
  "loop_bundle_resolver_v0"
)

for name in "${active_v0[@]}"; do
  guard_expect_fixed_in_file "$TAG" "$name" "$REGISTRY_MD" "REGISTRY.md must list $name"
  guard_expect_fixed_in_file "$TAG" "$name" "$LEGACY_MD" "LEGACY_V0_BOUNDARY.md must list $name"
  guard_expect_fixed_in_file "$TAG" "$name" "$LOOP_TYPES" "LoopFacts must expose $name while active"
  guard_expect_fixed_in_file "$TAG" "$name" "$LOOP_BUILDER" "loop_builder must build $name while active"
  guard_expect_fixed_in_file "$TAG" "$name" "$ENTRY_REGISTRY" "route registry must route $name while active"
done

if rg -n \
  "loop_scan_methods_block_v0|LoopScanMethodsBlock|SCAN_METHODS_BLOCK|scan_methods_block|loop_collect_using_entries_v0|LoopCollectUsingEntries|LOOP_COLLECT_USING_ENTRIES|collect_using_entries_v0" \
  src/mir/builder/control_flow \
  -g '*.rs' >/tmp/coreplan-active-v0-retired.refs 2>&1; then
  echo "[$TAG] ERROR: retired v0 code reference found" >&2
  cat /tmp/coreplan-active-v0-retired.refs >&2
  rm -f /tmp/coreplan-active-v0-retired.refs
  exit 1
fi
rm -f /tmp/coreplan-active-v0-retired.refs

echo "[$TAG] active_v0_box_count=${#active_v0[@]}"
echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-planner-required-route-exhaustion"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1007-COREPLAN-C1-001-PLANNER-REQUIRED-ROUTE-EXHAUSTION.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/coreplan_planner_required_route_exhaustion_guard.sh"
ROUTER="src/mir/builder/control_flow/joinir/route_entry/router.rs"
REGISTRY="src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs"
GENERIC_HANDLER="src/mir/builder/control_flow/joinir/route_entry/registry/handlers/generic.rs"
SINGLE_PLANNER="src/mir/builder/control_flow/plan/single_planner/rules.rs"

echo "[$TAG] checking planner_required route-exhaustion inventory"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$ROUTER" \
  "$REGISTRY" \
  "$GENERIC_HANDLER" \
  "$SINGLE_PLANNER"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-C1-001: planner_required route-exhaustion inventory guard" \
  "$TASKBOARD" \
  "taskboard must keep C1 row"
guard_expect_fixed_in_file "$TAG" \
  "planner_required_target_like_route_exhaustion_classified=1" \
  "$TASKBOARD" \
  "taskboard must record C1 target-like route exhaustion acceptance"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$CARD" \
  "C1 card must name this guard"
guard_expect_fixed_in_file "$TAG" \
  "non-candidate probe vs silent fallback" \
  "$CARD" \
  "C1 card must preserve classification boundary"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"

guard_expect_fixed_in_file "$TAG" \
  "let candidates = registry::collect_candidates(outcome.facts.as_ref());" \
  "$ROUTER" \
  "router must collect candidates under planner_required"
guard_expect_fixed_in_file "$TAG" \
  "if candidates.len() > 1" \
  "$ROUTER" \
  "router must keep ambiguity fail-fast"
guard_expect_fixed_in_file "$TAG" \
  "entry_ambiguous: candidates=" \
  "$ROUTER" \
  "router must keep entry_ambiguous diagnostic"
guard_expect_fixed_in_file "$TAG" \
  "route_exhausted func=" \
  "$ROUTER" \
  "router must keep route_exhausted detail"
guard_expect_fixed_in_file "$TAG" \
  "facts_present={}" \
  "$ROUTER" \
  "route_exhausted detail must report facts_present"
guard_expect_fixed_in_file "$TAG" \
  "candidates={}" \
  "$ROUTER" \
  "route_exhausted detail must report candidates"
guard_expect_fixed_in_file "$TAG" \
  "planner_none" \
  "$ROUTER" \
  "router must keep planner_none expected-plan freeze path"

guard_expect_fixed_in_file "$TAG" \
  "pub(crate) fn collect_candidates" \
  "$REGISTRY" \
  "registry must own candidate collection"
guard_expect_fixed_in_file "$TAG" \
  "pub(crate) fn try_route_recipe_first" \
  "$REGISTRY" \
  "registry must own recipe-first route iteration"

guard_expect_fixed_in_file "$TAG" \
  "Err(_err) if !env.strict_or_dev => return Ok(None)" \
  "$GENERIC_HANDLER" \
  "generic release compose fallback must remain explicit for inventory"
guard_expect_fixed_in_file "$TAG" \
  "Err(_) => Ok(None)" \
  "$GENERIC_HANDLER" \
  "generic release lower fallback must remain explicit for inventory"

guard_expect_fixed_in_file "$TAG" \
  "if gate.planner_required && !planner_present && outcome.facts.is_none()" \
  "$SINGLE_PLANNER" \
  "single_planner planner_required boundary must remain visible"

router_ok_none_count="$(rg -F -n 'Ok(None)' "$ROUTER" "$REGISTRY" "$GENERIC_HANDLER" | wc -l | tr -d ' ')"
echo "[$TAG] route_ok_none_site_count=$router_ok_none_count"
echo "[$TAG] ok"

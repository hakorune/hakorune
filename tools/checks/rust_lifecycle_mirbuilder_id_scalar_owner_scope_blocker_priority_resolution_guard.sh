#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-owner-scope-blocker-priority-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_owner_scope_blocker_priority_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2029-MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BLOCKER-PRIORITY-RESOLUTION-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BLOCKER-PRIORITY-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001"

need(fixture.get("kind") == "MirBuilderIdScalarOwnerScopeBlockerPriorityResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("reason_token") == "IdScalarOwnerScopeBoundednessNotProven", "previous reason drift")
need(previous.get("input_candidate_count") == 4, "input candidate drift")
need(previous.get("owner_scope_bounded_count") == 0, "owner scope count drift")
need(previous.get("state_targets_enumerated_count") == 0, "state target count drift")
need(previous.get("native_seed_file_boundary_derivable_count") == 0, "native seed file boundary drift")
need(previous.get("cross_owner_recipe_required_count") == 2, "cross-owner count drift")
need(previous.get("selection_eligible_for_source_plan_count") == 0, "source plan eligibility drift")

components = fixture.get("blocker_components") or []
by_id = {row.get("component_id"): row for row in components}
for component in ["StateTargetEnumeration", "NativeSeedFileBoundary", "CrossOwnerRecipeAuthority"]:
    need(component in by_id, f"missing component {component}")

state_targets = by_id["StateTargetEnumeration"]
need(state_targets.get("blocked_by") == "StateTargetsNotEnumerated", "state-target blocked-by drift")
need(state_targets.get("affected_candidate_count") == 4, "state-target affected count drift")
need(state_targets.get("dependency_rank") == 0, "state-target rank drift")
need(state_targets.get("selection_eligible") is True, "state-target eligibility drift")
need(state_targets.get("selected_next_card") == next_card, "state-target next drift")

native_boundary = by_id["NativeSeedFileBoundary"]
need(native_boundary.get("affected_candidate_count") == 4, "native boundary affected count drift")
need(native_boundary.get("dependency_rank") == 2, "native boundary rank drift")
need("StateTargetEnumeration" in (native_boundary.get("requires") or []), "native boundary must require state targets")
need(native_boundary.get("selection_eligible") is False, "native boundary must not be directly selectable")

cross_owner = by_id["CrossOwnerRecipeAuthority"]
need(cross_owner.get("affected_candidate_count") == 2, "cross-owner affected count drift")
need(cross_owner.get("dependency_rank") == 1, "cross-owner rank drift")
need("StateTargetEnumeration" in (cross_owner.get("requires") or []), "cross-owner must require state targets")
need(cross_owner.get("selection_eligible") is False, "cross-owner must not be directly selectable")

rule = fixture.get("selection_rule") or {}
need(rule.get("manual_axis_selection") is False, "manual axis selection drift")
need(rule.get("select_lowest_dependency_rank") is True, "dependency rank rule drift")
need(rule.get("prefer_common_root_blocker") is True, "root blocker rule drift")
for key in [
    "surface_count_as_proof",
    "cluster_size_as_proof",
    "source_file_path_as_authority",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"selection proof drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectOwnerScopeBlockerComponent", "decision kind drift")
need(decision.get("selected_component_id") == "StateTargetEnumeration", "selected component drift")
need(decision.get("reason_token") == "StateTargetEnumerationSelectedAsOwnerScopeRootBlocker", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
    "manual_axis_selection",
    "surface_count_as_proof",
    "cluster_size_as_proof",
    "route_membership_alone_as_proof",
    "source_file_path_as_authority",
    "source_plan_materialization",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    "selected_component_id = StateTargetEnumeration",
    "selected_next_card = MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-owner-scope-blocker-priority-resolution")
print("selected_component_id=StateTargetEnumeration")
print("selected_next_card=MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

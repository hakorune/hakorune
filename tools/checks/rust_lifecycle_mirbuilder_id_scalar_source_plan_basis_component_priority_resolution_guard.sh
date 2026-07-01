#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_source_plan_basis_component_priority_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2027-MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-001.md"
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


token = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001"

need(fixture.get("kind") == "MirBuilderIdScalarSourcePlanBasisComponentPriorityResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("reason_token") == "IdScalarSourcePlanDerivabilityRequiresScopeAndRecipeBasis", "previous reason drift")
need(previous.get("required_source_surfaces_complete_count") == 4, "source surface count drift")
need(previous.get("operation_vocabulary_complete_count") == 4, "operation count drift")
need(previous.get("nominal_id_domain_preserved_count") == 4, "nominal ID count drift")
need(previous.get("source_plan_derivable_count") == 0, "source plan derivable drift")

components = fixture.get("unresolved_components") or []
by_id = {row.get("component_id"): row for row in components}
for component in [
    "OwnerScopeBoundedness",
    "BehaviorRecipeEffectCoverage",
    "IdDomainBoundary",
    "StateMutationFrame",
    "ErrorSemantics",
    "DeterministicOrder",
    "VerifierInputContract",
]:
    need(component in by_id, f"missing component {component}")

owner_scope = by_id["OwnerScopeBoundedness"]
need(owner_scope.get("dependency_rank") == 0, "owner scope rank drift")
need(owner_scope.get("selection_eligible") is True, "owner scope eligibility drift")
need(owner_scope.get("next_card") == next_card, "owner scope next drift")

for component in ["StateMutationFrame", "BehaviorRecipeEffectCoverage", "VerifierInputContract"]:
    need(by_id[component].get("selection_eligible") is False, f"{component} must not be directly selectable")

rule = fixture.get("selection_rule") or {}
need(rule.get("manual_component_selection") is False, "manual component selection drift")
need(rule.get("select_lowest_dependency_rank") is True, "dependency rank rule drift")
need(rule.get("prefer_components_that_define_owner_subject") is True, "owner subject rule drift")
for key in ["cluster_size_as_proof", "surface_count_as_proof", "lexical_order_as_proof"]:
    need(rule.get(key) is False, f"selection proof drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectBasisComponent", "decision kind drift")
need(decision.get("selected_component_id") == "OwnerScopeBoundedness", "selected component drift")
need(decision.get("reason_token") == "OwnerScopeBoundednessSelectedAsSourcePlanRootComponent", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
    "manual_component_selection",
    "cluster_size_as_proof",
    "surface_count_as_proof",
    "directable_row_count_as_proof",
    "route_membership_alone_as_proof",
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
    "selected_component_id = OwnerScopeBoundedness",
    "selected_next_card = MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-source-plan-basis-component-priority-resolution")
print("selected_component_id=OwnerScopeBoundedness")
print("selected_next_card=MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

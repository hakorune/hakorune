#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_source_plan_basis_component_priority_resolution_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2033-MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002.md"
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

token = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002"
next_card = "MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001"

need(fixture.get("kind") == "MirBuilderIdScalarSourcePlanBasisComponentPriorityResolutionV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

previous = fixture.get("previous_state") or {}
need(previous.get("bounded_owner_count") == 2, "bounded owner count drift")
need(previous.get("native_seed_file_boundary_derivable_count") == 2, "file boundary count drift")
need(previous.get("state_target_count") == 22, "state target count drift")

components = {row["component_id"]: row for row in fixture.get("unresolved_components") or []}
need(components["IdDomainBoundary"]["dependency_rank"] == 0, "ID domain rank drift")
need(components["IdDomainBoundary"]["selection_eligible"] is True, "ID domain eligibility drift")
for component in ["StateMutationFrame", "ErrorSemantics", "DeterministicOrder", "BehaviorRecipeEffectCoverage", "VerifierInputContract"]:
    need(components[component]["selection_eligible"] is False, f"{component} must not be selectable")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectBasisComponent", "decision kind drift")
need(decision.get("selected_component_id") == "IdDomainBoundary", "selected component drift")
need(decision.get("reason_token") == "IdDomainBoundarySelectedAfterOwnerScopeAndFileBoundary", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_component_selection",
    "manual_owner_selection",
    "cluster_size_as_proof",
    "surface_count_as_proof",
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
for needle in [
    token,
    "selected_component_id = IdDomainBoundary",
    "selected_next_card = MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-002")
print("selected_component_id=IdDomainBoundary")
print("selected_next_card=MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

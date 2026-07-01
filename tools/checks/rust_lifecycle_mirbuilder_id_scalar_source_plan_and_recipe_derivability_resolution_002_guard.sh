#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_source_plan_and_recipe_derivability_resolution_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2026-MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002.md"
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


token = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarSourcePlanAndRecipeDerivabilityResolutionV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("basis_token") == "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-DERIVATION-BASIS-001", "basis token drift")
need(previous.get("surface_inventory_token") == "MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001", "surface token drift")
need(previous.get("operation_vocabulary_token") == "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001", "operation token drift")
need(previous.get("operation_vocabulary_reason_token") == "IdScalarOperationVocabularyInventoried", "operation reason drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "input candidate count drift")
need(pool.get("required_source_surfaces_complete_count") == 4, "surface complete count drift")
need(pool.get("operation_vocabulary_complete_count") == 4, "operation complete count drift")
need(pool.get("nominal_id_domain_preserved_count") == 4, "nominal ID count drift")
need(pool.get("owner_scope_bounded_count") == 0, "owner scope count drift")
need(pool.get("behavior_recipe_effect_coverage_complete_count") == 0, "behavior recipe count drift")
need(pool.get("source_plan_derivable_count") == 0, "source plan derivable count drift")
need(pool.get("behavior_recipe_derivable_count") == 0, "behavior recipe derivable count drift")
need(pool.get("selection_eligible_count") == 0, "selection eligible count drift")

required_blockers = {
    "OwnerScopeBoundedNotProven",
    "BehaviorRecipeEffectCoverageNotProven",
    "IdDomainBoundaryNotDeclared",
    "StateMutationFrameNotDeclared",
    "ErrorSemanticsNotDeclared",
    "DeterministicOrderNotDeclared",
    "VerifierInputContractNotDeclared",
}
rows = fixture.get("candidates") or []
need(len(rows) == 4, "candidate row count drift")
for row in rows:
    need(row.get("required_source_surfaces_complete") is True, "source surfaces not complete")
    need(row.get("operation_vocabulary_complete") is True, "operation vocabulary not complete")
    need(row.get("nominal_id_domain_isolation_preserved") is True, "nominal ID not preserved")
    need(row.get("source_plan_derivable") is False, "source plan must not be derivable")
    need(row.get("selection_eligible") is False, "selection must remain false")
    need(required_blockers.issubset(set(row.get("blocked_by") or [])), "missing blockers")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "IdScalarSourcePlanDerivabilityRequiresScopeAndRecipeBasis", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("selected_owner_edge_id") is None, "owner must not be selected")

claims = fixture.get("claims") or {}
need(claims.get("source_plan_derivability_rerun_completed") == 1, "rerun claim drift")
for key in [
    "source_plan_materialization",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "manual_owner_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
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
    "source_plan_derivable_count = 0",
    "IdScalarSourcePlanDerivabilityRequiresScopeAndRecipeBasis",
    "selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-002")
print("source_plan_derivable_count=0")
print("reason_token=IdScalarSourcePlanDerivabilityRequiresScopeAndRecipeBasis")
print("selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

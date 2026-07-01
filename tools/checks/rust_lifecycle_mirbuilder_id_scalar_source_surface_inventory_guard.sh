#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-source-surface-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_source_surface_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2024-MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001.md"
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


token = "MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001"

need(fixture.get("kind") == "MirBuilderIdScalarSourceSurfaceInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_token") == "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-DERIVATION-BASIS-001", "previous token drift")
need(previous.get("previous_reason_token") == "IdScalarSourcePlanDerivationBasisDefined", "previous reason drift")
need(previous.get("previous_selected_next_card") == token, "previous next drift")

policy = fixture.get("inventory_policy") or {}
need(policy.get("derivation_authority") == "projection_policy_fixture_source_surfaces", "authority drift")
need(policy.get("manual_surface_selection") is False, "manual surface selection drift")
need(policy.get("source_surface_inventory_only") is True, "inventory-only drift")
need(policy.get("source_plan_materialization") is False, "source plan materialization drift")
need(policy.get("operation_vocabulary_evaluated") is False, "operation vocabulary drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "input candidate count drift")
need(pool.get("required_source_surface_count") == 102, "source surface count drift")
need(pool.get("surface_complete_candidate_count") == 4, "complete candidate count drift")
need(pool.get("surface_incomplete_candidate_count") == 0, "incomplete candidate count drift")
need(pool.get("selection_eligible_for_source_plan_count") == 0, "source plan eligibility drift")

rows = fixture.get("candidates") or []
need(len(rows) == 4, "candidate row count drift")
for row in rows:
    need(row.get("required_source_surfaces_complete") is True, "incomplete source surface row")
    need(row.get("source_surface_confidence") == "FixtureJoinedSourceSurfaces", "source surface confidence drift")
    need(row.get("owner_scope") == "NotEvaluatedAtThisStage", "owner scope must not be inferred")
    need(row.get("next_card") == next_card, "candidate next card drift")
    surfaces = row.get("surfaces") or []
    need(surfaces, "missing surfaces")
    for surface in surfaces:
        for field in ["source_id", "source_path", "symbol", "evidence_ref"]:
            need(surface.get(field), f"surface missing {field}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectOperationVocabularyInventory", "decision kind drift")
need(decision.get("reason_token") == "IdScalarRequiredSourceSurfacesInventoried", "reason drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("source_surface_inventory_defined") == 1, "inventory claim drift")
for key in [
    "manual_surface_selection",
    "source_plan_materialization",
    "operation_vocabulary_evaluated",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
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
    "required_source_surface_count = 102",
    "surface_complete_candidate_count = 4",
    "selected_next_card = MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-source-surface-inventory")
print("required_source_surface_count=102")
print("surface_complete_candidate_count=4")
print("selected_next_card=MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

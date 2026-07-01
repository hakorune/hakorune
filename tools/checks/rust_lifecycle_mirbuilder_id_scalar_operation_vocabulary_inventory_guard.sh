#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_operation_vocabulary_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2025-MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001.md"
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


token = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002"

need(fixture.get("kind") == "MirBuilderIdScalarOperationVocabularyInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_token") == "MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001", "previous token drift")
need(previous.get("previous_reason_token") == "IdScalarRequiredSourceSurfacesInventoried", "previous reason drift")
need(previous.get("previous_selected_next_card") == token, "previous next drift")

policy = fixture.get("inventory_policy") or {}
need(policy.get("classification_authority") == "surface_role_then_symbol_return_type_rule_table", "classification authority drift")
need(policy.get("manual_operation_selection") is False, "manual operation selection drift")
need(policy.get("operation_vocabulary_inventory_only") is True, "inventory-only drift")
need(policy.get("source_plan_materialization") is False, "source plan materialization drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "input candidate count drift")
need(pool.get("operation_surface_count") == 102, "surface count drift")
need(pool.get("operation_vocabulary_token_count") == 28, "operation token count drift")
need(pool.get("operation_vocabulary_complete_candidate_count") == 4, "complete candidate count drift")
need(pool.get("unknown_operation_count") == 0, "unknown operation count drift")
need(pool.get("selection_eligible_for_source_plan_count") == 0, "source plan eligibility drift")

rows = fixture.get("candidates") or []
need(len(rows) == 4, "candidate row count drift")
for row in rows:
    need(row.get("operation_vocabulary_complete") is True, "candidate operation vocabulary incomplete")
    need(row.get("unknown_operation_count") == 0, "candidate unknown operation drift")
    need(row.get("next_card") == next_card, "candidate next card drift")
    for op in row.get("operation_rows") or []:
        need(op.get("operation_token") != "UnknownOperation", "unknown operation row")
        need(op.get("raw_i64_interchangeability") == 0, "raw i64 drift")
        for field in ["source_id", "source_path", "symbol", "operation_token", "classification_authority"]:
            need(op.get(field) is not None and op.get(field) != "", f"operation row missing {field}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectSourcePlanAndRecipeDerivabilityRerun", "decision kind drift")
need(decision.get("reason_token") == "IdScalarOperationVocabularyInventoried", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("operation_vocabulary_inventory_defined") == 1, "inventory claim drift")
for key in [
    "manual_operation_selection",
    "source_plan_materialization",
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
    "raw_i64_interchangeability",
    "nominal_id_erasure",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    "operation_vocabulary_token_count = 28",
    "unknown_operation_count = 0",
    "selected_next_card = MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-operation-vocabulary-inventory")
print("operation_vocabulary_token_count=28")
print("unknown_operation_count=0")
print("selected_next_card=MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002")
print("source_selfhost_claim=0")
print("summary=ok")
PY

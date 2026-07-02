#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-id-scalar-source-plan-and-recipe-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_emission_ssa_phi_id_scalar_source_plan_and_recipe.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2048-MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001.md"
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

token = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001"
next_card = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-VERIFIER-RESULT-001"

need(fixture.get("kind") == "MirBuilderEmissionSsaPhiIdScalarSourcePlanAndRecipeV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("selected_owner") or {}).get("owner_edge_id") == "mirbuilder::emission_ssa_phi", "bad owner")
need((fixture.get("selected_owner") or {}).get("selected_by_owner_name") is False, "owner-name selection drift")
need((fixture.get("selected_owner") or {}).get("selected_by_count") is False, "count selection drift")
need((fixture.get("source_plan") or {}).get("standalone_projection_subject") is True, "standalone subject drift")
need((fixture.get("source_plan") or {}).get("descriptor_id") == "emission_ssa_phi_contract_lifecycle_v1", "descriptor drift")
need(fixture.get("behavior_recipe", {}).get("effect_rows"), "missing effect rows")
need(fixture.get("behavior_recipe", {}).get("mutation_frames"), "missing mutation frames")
need(fixture.get("verifier_preconditions"), "missing verifier preconditions")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SourcePlanAndRecipeMaterialized", "bad decision kind")
need(decision.get("reason_token") == "EmissionSsaPhiIdScalarSourcePlanAndRecipeMaterialized", "bad reason")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
need(claims.get("source_plan_materialization") == 1, "source plan not materialized")
need(claims.get("behavior_recipe_materialization") == 1, "behavior recipe not materialized")
for key in [
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "owner_name_as_proof",
    "surface_count_as_proof",
    "row_count_as_proof",
    "coverage_percentage_as_proof",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = EmissionSsaPhiIdScalarSourcePlanAndRecipeMaterialized",
    "selected_next_card = MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-VERIFIER-RESULT-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-emission-ssa-phi-id-scalar-source-plan-and-recipe")
print("selected_owner_edge_id=mirbuilder::emission_ssa_phi")
print("reason_token=EmissionSsaPhiIdScalarSourcePlanAndRecipeMaterialized")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

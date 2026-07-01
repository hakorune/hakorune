#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_source_plan_and_recipe_derivability_resolution_003.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2039-MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003.md"
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

token = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarSourcePlanAndRecipeDerivabilityResolutionV3", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "candidate count drift")
need(pool.get("source_plan_derivable_count") == 2, "source plan derivable drift")
need(pool.get("behavior_recipe_derivable_count") == 2, "recipe derivable drift")
need(pool.get("selection_eligible_count") == 2, "selection eligible drift")
need(pool.get("ambiguous_derivable_count") == 2, "ambiguous count drift")

rows = {row["owner_edge_id"]: row for row in fixture.get("candidates") or []}
need(rows["mirbuilder::context_registry"]["selection_eligible"] is True, "context eligibility drift")
need(rows["mirbuilder::emission_ssa_phi"]["selection_eligible"] is True, "emission eligibility drift")
need(rows["mirbuilder::join_i_r_plan"]["selection_eligible"] is False, "plan eligibility drift")
need(rows["mirbuilder::join_i_r_route_verify"]["selection_eligible"] is False, "route verify eligibility drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "MultipleEqualIdScalarSourcePlanDerivabilityCandidates", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("selected_owner_edge_id") is None, "must not select owner")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
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
    "source_plan_derivable_count = 2",
    "reason_token = MultipleEqualIdScalarSourcePlanDerivabilityCandidates",
    "selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003")
print("source_plan_derivable_count=2")
print("reason_token=MultipleEqualIdScalarSourcePlanDerivabilityCandidates")
print("source_selfhost_claim=0")
print("summary=ok")
PY

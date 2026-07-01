#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_source_plan_and_recipe_derivability_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2022-MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-001.md"
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


token = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarSourcePlanAndRecipeDerivabilityResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_reason_token") == "MultipleEqualIdScalarSeedPacketCandidates", "previous reason drift")
need(previous.get("ambiguous_candidate_count") == 4, "previous ambiguous count drift")

component = fixture.get("component") or {}
need(component.get("component_id") == "SourcePlanAndRecipe", "component drift")
need(component.get("component_order") == 1, "component order drift")
need(component.get("directability_only_is_seed_evidence") is False, "directability-only drift")
need(component.get("directability_may_feed_component_generation") is True, "directability feed drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "input candidate count drift")
need(pool.get("source_plan_derivable_count") == 0, "source plan derivable count drift")
need(pool.get("behavior_recipe_derivable_count") == 0, "behavior recipe derivable count drift")
need(pool.get("selection_eligible_count") == 0, "selection eligible count drift")

rows = fixture.get("candidates") or []
need(len(rows) == 4, "candidate row count drift")
for row in rows:
    need(row.get("owner_edge_confidence") == "FixtureMapped", "candidate confidence drift")
    need(row.get("source_plan_derivable") is False, "source plan must not be derivable")
    need(row.get("behavior_recipe_derivable") is False, "behavior recipe must not be derivable")
    need(row.get("raw_i64_interchangeability") == 0, "raw i64 interchangeability drift")
    blocked = set(row.get("blocked_by") or [])
    for reason in [
        "SourcePlanDerivabilityNotProven",
        "BehaviorRecipeDerivabilityNotProven",
        "DescriptorOnlyIsNotSourcePlanAndRecipe",
    ]:
        need(reason in blocked, f"missing blocker {reason}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoIdScalarSourcePlanAndRecipeDerivabilityCandidate", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("selected_owner_edge_id") is None, "owner must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "seed_packet_candidate_selection_consumed",
    "seed_evidence_contract_consumed",
    "seed_readiness_resolution_002_consumed",
]:
    need(claims.get(key) == 1, f"input consumed drift: {key}")
for key in [
    "manual_owner_selection",
    "cluster_size_as_proof",
    "directable_row_count_as_proof",
    "lexical_order_as_seed_selection_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "source_plan_implied_by_directability",
    "behavior_recipe_implied_by_directability",
    "verifier_result_implied_by_source_plan",
    "derived_artifact_seed_draft_implied_by_verifier",
    "raw_i64_interchangeability",
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
    "source_plan_derivable_count = 0",
    "NoIdScalarSourcePlanAndRecipeDerivabilityCandidate",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution")
print("source_plan_derivable_count=0")
print("selection_eligible_count=0")
print("selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

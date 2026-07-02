#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-derivable-owner-discriminator-resolution-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_derivable_owner_discriminator_resolution_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2047-MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002.md"
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

token = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002"
selected = "mirbuilder::emission_ssa_phi"
next_card = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001"

need(fixture.get("kind") == "MirBuilderIdScalarDerivableOwnerDiscriminatorResolutionV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_derivable_owner_count") == 2, "input count drift")
need(pool.get("selection_eligible_count") == 2, "eligible count drift")
need(pool.get("unique_refined_proof_tuple_count") == 2, "refined tuple count drift")
need(pool.get("selected_owner_count") == 1, "selected owner count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectSourcePlanAndRecipe", "bad decision kind")
need(decision.get("reason_token") == "ExactlyOneIdScalarDerivableOwnerAfterRefinedProofAxes", "bad reason")
need(decision.get("selected_owner_edge_id") == selected, "bad selected owner")
need(decision.get("selected_next_card") == next_card, "bad selected next card")

rows = {row.get("owner_edge_id"): row for row in fixture.get("candidates") or []}
need(set(rows) == {"mirbuilder::context_registry", selected}, "candidate set drift")
need(rows[selected]["refined_proof_axes"]["StandaloneProjectionSubjectEstablished"] is True, "selected standalone axis drift")
need(rows["mirbuilder::context_registry"]["refined_proof_axes"]["StandaloneProjectionSubjectEstablished"] is False, "context standalone axis drift")

claims = fixture.get("claims") or {}
for key in [
    "owner_name_as_proof",
    "historical_descriptor_presence_as_preference",
    "lifecycle_richness_as_proof",
    "mutation_complexity_as_proof",
    "effect_class_count_as_proof",
    "surface_count_as_proof",
    "row_count_as_proof",
    "coverage_percentage_as_proof",
    "source_plan_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = ExactlyOneIdScalarDerivableOwnerAfterRefinedProofAxes",
    "selected_owner_edge_id = mirbuilder::emission_ssa_phi",
    "selected_next_card = MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-derivable-owner-discriminator-resolution-002")
print("selected_owner_edge_id=" + selected)
print("reason_token=ExactlyOneIdScalarDerivableOwnerAfterRefinedProofAxes")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

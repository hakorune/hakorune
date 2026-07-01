#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-owner-scope-boundedness-resolution-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_owner_scope_boundedness_resolution_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2031-MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002.md"
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


token = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001"

need(fixture.get("kind") == "MirBuilderIdScalarOwnerScopeBoundednessResolutionV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("state_target_count") == 22, "state target count drift")
need(previous.get("cross_owner_state_target_count") == 4, "cross owner target drift")
need(previous.get("previous_selected_next_card") == token, "previous next-card drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "candidate count drift")
need(pool.get("owner_scope_bounded_count") == 2, "bounded count drift")
need(pool.get("state_targets_enumerated_count") == 4, "state targets enumerated drift")
need(pool.get("native_seed_file_boundary_derivable_count") == 0, "native seed file boundary drift")
need(pool.get("cross_owner_recipe_required_count") == 2, "cross-owner recipe count drift")
need(pool.get("selection_eligible_for_source_plan_count") == 0, "source plan eligibility drift")

rows = {row["owner_edge_id"]: row for row in fixture.get("candidates") or []}
need(rows["mirbuilder::context_registry"]["owner_scope_bounded"] is True, "context registry bounded drift")
need(rows["mirbuilder::emission_ssa_phi"]["owner_scope_bounded"] is True, "emission ssa phi bounded drift")
need(rows["mirbuilder::join_i_r_plan"]["owner_scope_bounded"] is False, "join_i_r_plan bounded drift")
need(rows["mirbuilder::join_i_r_route_verify"]["owner_scope_bounded"] is False, "route verify bounded drift")
for owner in ["mirbuilder::context_registry", "mirbuilder::emission_ssa_phi"]:
    need("BoundedOwnerScopeRequiresNativeSeedFileBoundary" in rows[owner]["blocked_by"], f"missing boundary blocker: {owner}")
for owner in ["mirbuilder::join_i_r_plan", "mirbuilder::join_i_r_route_verify"]:
    need("OperationTokensRequireCrossOwnerRecipeAuthority" in rows[owner]["blocked_by"], f"missing cross-owner blocker: {owner}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNativeSeedFileBoundaryBasis", "decision kind drift")
need(decision.get("reason_token") == "BoundedOwnerScopeRequiresNativeSeedFileBoundary", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")
need(decision.get("selected_owner_edge_id") is None, "must not select owner")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
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
    "owner_scope_bounded_count = 2",
    "selected_next_card = MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-owner-scope-boundedness-resolution-002")
print("owner_scope_bounded_count=2")
print("selected_next_card=MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

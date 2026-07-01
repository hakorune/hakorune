#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-owner-scope-boundedness-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_owner_scope_boundedness_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2028-MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001.md"
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


token = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarOwnerScopeBoundednessResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("selected_component_id") == "OwnerScopeBoundedness", "previous component drift")
need(previous.get("previous_reason_token") == "OwnerScopeBoundednessSelectedAsSourcePlanRootComponent", "previous reason drift")
need(previous.get("previous_selected_next_card") == token, "previous next drift")

policy = fixture.get("boundedness_policy") or {}
need(policy.get("primary_unit") == "owner_edge", "primary unit drift")
for key in [
    "source_file_path_as_authority",
    "surface_count_as_proof",
    "route_membership_alone_as_proof",
    "manual_owner_selection",
]:
    need(policy.get(key) is False, f"policy drift: {key}")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "candidate count drift")
need(pool.get("owner_scope_bounded_count") == 0, "bounded count drift")
need(pool.get("state_targets_enumerated_count") == 0, "state target count drift")
need(pool.get("native_seed_file_boundary_derivable_count") == 0, "native seed boundary count drift")
need(pool.get("cross_owner_recipe_required_count") == 2, "cross-owner count drift")
need(pool.get("selection_eligible_for_source_plan_count") == 0, "selection eligible drift")

rows = fixture.get("candidates") or []
need(len(rows) == 4, "candidate row count drift")
for row in rows:
    need(row.get("owner_scope_bounded") is False, "owner scope must remain unproven")
    need(row.get("native_seed_file_boundary_derivable") is False, "native seed boundary must remain false")
    need(row.get("selection_eligible_for_source_plan") is False, "selection must remain false")
    need("OwnerScopeBoundedNotProven" in (row.get("blocked_by") or []), "missing owner scope blocker")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "IdScalarOwnerScopeBoundednessNotProven", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("selected_owner_edge_id") is None, "owner must not be selected")

claims = fixture.get("claims") or {}
need(claims.get("owner_scope_boundedness_resolution_completed") == 1, "resolution claim drift")
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
    "owner_scope_bounded_count = 0",
    "IdScalarOwnerScopeBoundednessNotProven",
    "selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-owner-scope-boundedness-resolution")
print("owner_scope_bounded_count=0")
print("reason_token=IdScalarOwnerScopeBoundednessNotProven")
print("selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY

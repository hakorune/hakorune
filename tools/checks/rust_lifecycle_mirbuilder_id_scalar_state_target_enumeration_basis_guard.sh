#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_state_target_enumeration_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2030-MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001.md"
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


token = "MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002"

need(fixture.get("kind") == "MirBuilderIdScalarStateTargetEnumerationBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("selected_component_id") == "StateTargetEnumeration", "previous component drift")
need(previous.get("selected_next_card") == token, "previous next-card drift")
need(previous.get("input_candidate_count") == 4, "previous candidate count drift")
need(previous.get("state_targets_enumerated_count") == 0, "previous state-target count drift")

policy = fixture.get("enumeration_policy") or {}
need(policy.get("primary_unit") == "semantic_state_target", "primary unit drift")
need(policy.get("grouped_by") == "owner_edge", "grouping drift")
need(policy.get("source_file_path_as_authority") is False, "source path authority drift")
need(policy.get("surface_count_as_proof") is False, "surface count proof drift")
need(policy.get("manual_owner_selection") is False, "manual owner selection drift")

rows = fixture.get("owner_edge_targets") or []
need(len(rows) == 4, "owner edge target row count drift")
for row in rows:
    need(row.get("state_targets_enumerated") is True, f"targets not enumerated: {row.get('owner_edge_id')}")
    need(row.get("state_targets"), f"empty targets: {row.get('owner_edge_id')}")
    for target in row["state_targets"]:
        need(target.get("state_target_id"), "missing state_target_id")
        need(target.get("semantic_resource"), "missing semantic_resource")
        need(target.get("target_kind") in [
            "OwnerField",
            "LocalAccumulator",
            "OutputPlanList",
            "DiagnosticState",
            "VerifierObservation",
            "ExternalDependency",
            "ExternalOwnerState",
        ], "bad target kind")
        need(target.get("access"), "missing access")
        need(target.get("operation_tokens"), "missing operation tokens")
        need(target.get("source_surfaces"), "missing source surfaces")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "candidate count drift")
need(pool.get("state_targets_enumerated_owner_edge_count") == 4, "enumerated owner count drift")
need(pool.get("state_target_count") == 22, "state target count drift")
need(pool.get("all_targets_inside_owner_scope_count") == 2, "inside-owner count drift")
need(pool.get("cross_owner_state_target_count") == 4, "cross-owner target count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "StateTargetBasisDefined", "decision kind drift")
need(decision.get("reason_token") == "IdScalarStateTargetsEnumerated", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
    "manual_axis_selection",
    "surface_count_as_proof",
    "cluster_size_as_proof",
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
    "state_target_count = 22",
    "cross_owner_state_target_count = 4",
    "selected_next_card = MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-state-target-enumeration-basis")
print("state_target_count=22")
print("cross_owner_state_target_count=4")
print("selected_next_card=MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002")
print("source_selfhost_claim=0")
print("summary=ok")
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-subsurface-priority-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_subsurface_priority_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2114-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSubsurfacePriorityRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_selected_next_card") == token, "basis next drift")
need(inputs.get("basis_selection_eligible_subsurface_count") == 0, "basis eligible drift")

rows = {row.get("subsurface_id"): row for row in fixture.get("candidate_subsurfaces") or []}
need(set(rows) == {"PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"}, "candidate drift")
for row in rows.values():
    need(row.get("scope_eligible") is True, "scope drift")
    need(row.get("proof_tuple_complete") is False, "proof tuple drift")
    need(row.get("selection_eligible") is False, "selection drift")
    need(set(row.get("blocked_by") or []) == {
        "NoStableResultPublicationContractProof",
        "NoMutationSemanticsPolicyReadinessProof",
        "NoDirectContractShapeReadinessProof",
        "NoTypedValueBoundaryReadinessProof",
    }, "blocker drift")
    for key in [
        "stable_result_publication_contract",
        "mutation_semantics_policy_ready",
        "direct_contract_shape_ready",
        "typed_value_boundary_ready",
    ]:
        need((row.get(key) or {}).get("status") == "Unproven", f"{key} drift")

summary = fixture.get("summary") or {}
need(summary.get("write_subsurface_priority_rerun") == 1, "summary rerun drift")
need(summary.get("write_subsurface_priority_basis_consumed") == 1, "basis consumed drift")
need(summary.get("candidate_write_subsurface_count") == 3, "candidate count drift")
need(summary.get("proof_tuple_complete_subsurface_count") == 0, "proof tuple count drift")
need(summary.get("selection_eligible_subsurface_count") == 0, "eligible count drift")
need(summary.get("selected_write_subsurface_count") == 0, "selected count drift")
for key in [
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoWriteSubsurfacePriorityProofTuple", "reason drift")
need(decision.get("selected_subsurface") is None, "selected drift")
need(decision.get("selected_next_card") == design_stop, "design stop drift")
need(decision.get("recommended_consultation_topic") == "WriteSubsurfacePriorityProofAxis", "consult topic drift")

claims = fixture.get("claims") or {}
for key in ["write_subsurface_priority_rerun", "write_subsurface_priority_basis_consumed"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "write_subsurface_selected",
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "component_specific_direct_contract_materialized",
    "hako_adoption",
    "source_selfhost_claim",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "new_python_semantic_projector",
    "manual_subsurface_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2114-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-subsurface-priority-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_subsurface_priority_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need("decision=KeepStopped" in task_order, "task order decision drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-subsurface-priority-rerun")
print("write_subsurface_priority_rerun=1")
print("selection_eligible_subsurface_count=0")
print("decision=KeepStopped")
print("selected_next_card=" + design_stop)
print("source_selfhost_claim=0")
print("summary=ok")
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_direct_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2129-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-CLOSEOUT-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteDeleteSurfaceDirectCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_decision") == "SelectWriteDeleteSurfaceDirectCloseoutRerun", "basis decision drift")
need(inputs.get("basis_selected_next_card") == token, "basis next drift")

closeouts = fixture.get("accepted_scoped_closeouts") or []
need(len(closeouts) == 5, "accepted closeout count drift")
need({row.get("contract_id") for row in closeouts} == {
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
    "CollectionLenScalarI64TypedDirectCloseoutContract",
    "WritePushSurfaceTypedDirectCloseoutContract",
    "WriteDeleteSurfaceTypedDirectCloseoutContract",
}, "accepted closeout id drift")

materialized = fixture.get("materialized_contract") or {}
expected = {
    "contract_id": "WriteDeleteSurfaceTypedDirectCloseoutContract",
    "surface_id": "WriteScalarI64Routes",
    "subsurface_id": "DeleteSurfacePolicy",
    "core_method_op": "MapDelete",
    "core_method_lowering_tier": "ColdFallback",
    "result_class": "ScalarI64Result",
    "return_shape": "ScalarI64",
    "value_demand": "WriteAny",
    "publication_policy": "NonePublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
}
for key, value in expected.items():
    need(materialized.get(key) == value, f"materialized drift: {key}")
need(materialized.get("routes") == ["MapDeleteAny"], "route drift")
need(materialized.get("proof_or_policy_source") == ["DeleteSurfacePolicy"], "policy source drift")
need(materialized.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(materialized.get("publication_execution") is False, "publication execution drift")

need(fixture.get("remaining_write_subsurfaces") == ["SetSurfacePolicy"], "remaining subsurface drift")
blockers = fixture.get("remaining_write_subsurface_blockers") or {}
need(blockers.get("SetSurfacePolicy") == "NoHakoAdoptedWriteSubsurfacePilot", "set blocker drift")

summary = fixture.get("summary") or {}
need(summary.get("write_delete_surface_direct_closeout_materialized") == 1, "summary materialized drift")
need(summary.get("accepted_scoped_closeout_count") == 5, "summary closeout count drift")
need(summary.get("remaining_write_subsurface_count") == 1, "summary remaining count drift")
for key in [
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteRemainingSubsurfacePostDeleteCloseoutRerun", "decision kind drift")
need(decision.get("reason_token") == "DeleteScopedCloseoutMaterializedSetRemains", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("write_delete_surface_direct_closeout_materialized") == 1, "missing materialized claim")
need(claims.get("accepted_scoped_closeout_count") == 5, "claim closeout count drift")
for key in [
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subsurface_selection",
    "row_count_as_proof",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2129-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_direct_closeout_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun")
print("write_delete_surface_direct_closeout_materialized=1")
print("accepted_scoped_closeout_count=5")
print("remaining_write_subsurface_count=1")
print("write_scalar_i64_routes_closeout=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

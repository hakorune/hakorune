#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_direct_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2138-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-DIRECT-CLOSEOUT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-DIRECT-CLOSEOUT-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-BOUNDARY-SELECTION-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreI64DirectCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_decision") == "SelectWriteSetMapStoreI64DirectCloseoutRerun", "basis decision drift")
need(inputs.get("basis_selected_next_card") == token, "basis next drift")
need(inputs.get("delete_surface_mirror_retired") is True, "delete mirror retirement drift")

closeouts = fixture.get("accepted_scoped_closeouts") or []
need(len(closeouts) == 1, "accepted closeout count drift")
need("WriteSetMapStoreI64TypedDirectCloseoutContract" in {row.get("contract_id") for row in closeouts}, "MapStoreI64 closeout missing")

materialized = fixture.get("materialized_contract") or {}
expected = {
    "contract_id": "WriteSetMapStoreI64TypedDirectCloseoutContract",
    "surface_id": "WriteScalarI64Routes",
    "subsurface_id": "SetSurfacePolicy/MapStoreI64",
    "core_method_op": "MapSet",
    "core_method_lowering_tier": "ColdFallback",
    "result_class": "NoneResult",
    "return_shape": "None",
    "value_demand": "WriteAny",
    "write_value_boundary": "ScalarI64",
    "publication_policy": "NonePublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
}
for key, value in expected.items():
    need(materialized.get(key) == value, f"materialized drift: {key}")
need(materialized.get("routes") == ["MapStoreI64"], "route drift")
need(materialized.get("proof_or_policy_source") == ["SetSurfacePolicy", "TypedScalarWriteBeforeAnyWrite"], "policy source drift")
need(materialized.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(materialized.get("publication_execution") is False, "publication execution drift")
need(materialized.get("mapstore_any_included") is False, "MapStoreAny inclusion drift")
need(materialized.get("any_write_boundary_opened") is False, "Any boundary drift")

need(fixture.get("remaining_write_scoped_surfaces") == ["SetSurfacePolicy/MapStoreAny"], "remaining surface drift")
blockers = fixture.get("remaining_write_surface_blockers") or {}
need(blockers.get("SetSurfacePolicy/MapStoreAny") == "AnyWriteBoundaryRequired", "MapStoreAny blocker drift")

summary = fixture.get("summary") or {}
need(summary.get("write_set_mapstore_i64_direct_closeout_materialized") == 1, "summary materialized drift")
need(summary.get("accepted_scoped_closeout_count") == 1, "summary closeout count drift")
need(summary.get("remaining_write_scoped_surface_count") == 1, "summary remaining count drift")
need(summary.get("mapstore_any_deferred") == 1, "MapStoreAny defer drift")
for key in [
    "any_write_boundary_opened",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteSetMapStoreAnyBoundarySelection", "decision kind drift")
need(decision.get("reason_token") == "MapStoreI64ScopedCloseoutMaterializedMapStoreAnyRemains", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("write_set_mapstore_i64_direct_closeout_materialized") == 1, "missing materialized claim")
need(claims.get("accepted_scoped_closeout_count") == 1, "claim closeout count drift")
need(claims.get("mapstore_any_deferred") == 1, "claim MapStoreAny defer drift")
for key in [
    "any_write_boundary_opened",
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
need(manifest_row.get("card", "").endswith("2138-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-DIRECT-CLOSEOUT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_direct_closeout_rerun_guard.sh"), "manifest guard drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun")
print("write_set_mapstore_i64_direct_closeout_materialized=1")
print("accepted_scoped_closeout_count=1")
print("remaining_write_scoped_surface_count=1")
print("remaining_write_scoped_surface=SetSurfacePolicy/MapStoreAny")
print("mapstore_any_deferred=1")
print("any_write_boundary_opened=0")
print("write_scalar_i64_routes_closeout=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

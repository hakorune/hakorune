#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_direct_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2147-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-rerun"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreAnyDirectCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_decision") == "SelectWriteSetMapStoreAnyDirectCloseoutRerun", "basis decision drift")
need(inputs.get("basis_selected_next_card") == token, "basis next drift")
need(inputs.get("accepted_scoped_closeout_count_before_mapstore_any") == 6, "previous closeout count drift")

materialized = fixture.get("materialized_contract") or {}
expected = {
    "contract_id": "WriteSetMapStoreAnyDirectCloseoutContract",
    "surface_id": "WriteScalarI64Routes",
    "subsurface_id": "SetSurfacePolicy/MapStoreAny",
    "core_method_op": "MapSet",
    "core_method_lowering_tier": "ColdFallback",
    "result_class": "NoneResult",
    "return_shape": "None",
    "value_demand": "WriteAny",
    "write_value_boundary": "Any",
    "publication_policy": "NonePublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
}
for key, value in expected.items():
    need(materialized.get(key) == value, f"materialized drift: {key}")
need(materialized.get("routes") == ["MapStoreAny"], "route drift")
need(materialized.get("proof_or_policy_source") == ["SetSurfacePolicy", "AnyWriteBoundaryDeclared"], "policy source drift")
need(materialized.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(materialized.get("publication_execution") is False, "publication execution drift")
need(materialized.get("mapstore_any_included") is True, "MapStoreAny inclusion drift")
need(materialized.get("any_write_boundary_declared") is True, "Any declaration drift")
need(materialized.get("any_write_boundary_opened") is False, "Any boundary opening drift")

summary = fixture.get("summary") or {}
need(summary.get("write_set_mapstore_any_direct_closeout_materialized") == 1, "summary materialized drift")
need(summary.get("accepted_scoped_closeout_count") == 7, "summary closeout count drift")
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
need(decision.get("kind") == "SelectWriteScalarI64RoutesCloseoutBasis", "decision kind drift")
need(decision.get("reason_token") == "MapStoreAnyScopedCloseoutMaterializedReviewWriteSurfaceCloseoutSeparately", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("write_set_mapstore_any_direct_closeout_materialized") == 1, "missing materialized claim")
need(claims.get("accepted_scoped_closeout_count") == 7, "claim closeout count drift")
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
need(manifest_row.get("card", "").endswith("2147-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-DIRECT-CLOSEOUT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_direct_closeout_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-rerun")
print("write_set_mapstore_any_direct_closeout_materialized=1")
print("accepted_scoped_closeout_count=7")
print("any_write_boundary_declared=1")
print("any_write_boundary_opened=0")
print("write_scalar_i64_routes_closeout=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

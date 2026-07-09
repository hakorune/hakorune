#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3362-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
DELETE_RETIRE_CARD="$ROOT/docs/development/current/main/phases/phase-296x/3353-MIRBUILDER-SCALAR-KNOWN-WRITE-DELETE-SURFACE-MIRROR-RETIRE-001.md"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE" "$DELETE_RETIRE_CARD"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE" "$DELETE_RETIRE_CARD" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
write_source = Path(sys.argv[5]).read_text(encoding="utf-8")
delete_retire_card = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteScalarI64RoutesCloseoutBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("mapstore_any_selected_next") == token, "mapstore any next drift")
for key in [
    "write_push_surface_closeout_hash",
    "write_set_mapstore_i64_closeout_hash",
    "write_set_mapstore_any_closeout_hash",
    "delete_surface_mirror_retire_card_hash",
    "write_source_hash",
]:
    need(inputs.get(key), f"missing input hash: {key}")

review = fixture.get("write_surface_review") or {}
need(review.get("surface_id") == "WriteScalarI64Routes", "surface drift")
need(review.get("scoped_direct_closeout_contract_count") == 3, "scoped count drift")
contracts = review.get("scoped_direct_closeout_contracts") or []
need({c.get("subsurface_id") for c in contracts} == {
    "PushSurfacePolicy",
    "SetSurfacePolicy/MapStoreI64",
    "SetSurfacePolicy/MapStoreAny",
}, "scoped contract set drift")
need({tuple(c.get("routes") or []) for c in contracts} == {
    ("ArrayAppendAny",),
    ("MapStoreI64",),
    ("MapStoreAny",),
}, "scoped route set drift")
for contract in contracts:
    need(contract.get("surface_id") == "WriteScalarI64Routes", "contract surface drift")
    need(contract.get("runtime_mutation_authority") is False, "runtime mutation authority drift")

delete = review.get("delete_surface_policy") or {}
need(delete.get("subsurface_id") == "DeleteSurfacePolicy/MapDeleteAny", "delete subsurface drift")
need(delete.get("hako_mirror_retired") is True, "delete mirror retire drift")
need(delete.get("lifecycle_artifacts_deleted") is True, "delete artifact deletion drift")
need(delete.get("rust_map_delete_route_preserved") is True, "delete Rust route drift")
need(delete.get("direct_closeout_materialized") is False, "delete direct closeout drift")
need(delete.get("closeout_treatment") == "RetiredUnconnectedMirrorLiveRustRoutePreserved", "delete treatment drift")
need(review.get("ready_for_rerun") is True, "ready rerun drift")
need(review.get("rerun_required_before_write_surface_closeout") is True, "rerun required drift")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "basis-only drift")
need(rule.get("write_surface_closeout_requires_rerun") is True, "write rerun rule drift")
need(rule.get("delete_retire_treatment_must_remain_explicit") is True, "delete treatment rule drift")
need(rule.get("delete_retire_does_not_count_as_hako_direct_closeout") is True, "delete count rule drift")
need(rule.get("axis_closeout_forbidden_at_basis") is True, "axis closeout rule drift")
for key in [
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
    "route_count_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
for key in [
    "write_scalar_i64_routes_closeout_basis",
    "push_surface_direct_closeout_materialized",
    "set_mapstore_i64_direct_closeout_materialized",
    "set_mapstore_any_direct_closeout_materialized",
    "delete_surface_hako_mirror_retired",
    "rust_map_delete_route_preserved",
    "write_scalar_i64_routes_closeout_ready_for_rerun",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
need(summary.get("scoped_direct_closeout_contract_count") == 3, "summary scoped count drift")
for key in [
    "delete_surface_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteScalarI64RoutesCloseoutRerun", "decision kind drift")
need(decision.get("reason_token") == "ScopedWriteContractsAndDeleteRetireTreatmentCollectedForCloseoutReview", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_scalar_i64_routes_closeout_basis",
    "basis_only",
    "delete_surface_hako_mirror_retired",
    "rust_map_delete_route_preserved",
    "write_scalar_i64_routes_closeout_ready_for_rerun",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("scoped_direct_closeout_contract_count") == 3, "claim scoped count drift")
for key in [
    "delete_surface_direct_closeout_materialized",
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

for needle in [
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteProof::DeleteSurfacePolicy",
    "CoreMethodOp::MapDelete",
    "GenericMethodReturnShape::ScalarI64",
]:
    need(needle in write_source, f"live delete route missing: {needle}")
for needle in [
    "delete_surface_hako_mirror_retired = 1",
    "rust_map_delete_route_preserved = 1",
    "write_scalar_i64_routes_closeout = 0",
]:
    need(needle in delete_retire_card, f"delete retire card drift: {needle}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3362-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis")
print("write_scalar_i64_routes_closeout_basis=1")
print("scoped_direct_closeout_contract_count=3")
print("delete_surface_hako_mirror_retired=1")
print("rust_map_delete_route_preserved=1")
print("delete_surface_direct_closeout_materialized=0")
print("write_scalar_i64_routes_closeout=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

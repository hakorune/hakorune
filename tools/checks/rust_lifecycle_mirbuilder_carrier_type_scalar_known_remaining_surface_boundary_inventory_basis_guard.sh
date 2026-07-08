#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2106-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
COLLECTION_SOURCE="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
ROUTE_REGISTRY="$ROOT/src/llvm_py/generated/generic_method_route_registry.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$COLLECTION_SOURCE" "$WRITE_SOURCE" "$ROUTE_REGISTRY" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
collection_source = Path(sys.argv[5]).read_text(encoding="utf-8")
write_source = Path(sys.argv[6]).read_text(encoding="utf-8")
route_registry = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownRemainingSurfaceBoundaryInventoryBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("previous_reason_token") == "ScalarKnownTransportAxisStillHasUncoveredSurfaces", "previous reason drift")
need(set(inputs.get("remaining_uncovered_surface_ids") or []) == {"CollectionScalarI64Routes", "WriteScalarI64Routes"}, "remaining surface drift")

closeouts = fixture.get("prior_accepted_scoped_closeouts") or []
need(len(closeouts) == 2, "prior scoped closeout count drift")
need({row.get("contract_id") for row in closeouts} == {
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
}, "prior scoped closeout id drift")

required_dimensions = {
    "surface_id",
    "candidate_contract_id",
    "route_kind_set",
    "return_shape",
    "value_demand",
    "publication_policy",
    "effect_class",
    "prior_scoped_closeout_overlap",
    "write_result_policy_required",
    "direct_closeout_ready",
}
need(set(fixture.get("boundary_dimensions") or []) == required_dimensions, "boundary dimensions drift")

boundaries = fixture.get("surface_boundaries") or []
need(len(boundaries) == 2, "boundary count drift")
by_surface = {row.get("surface_id"): row for row in boundaries}
need(set(by_surface) == {"CollectionScalarI64Routes", "WriteScalarI64Routes"}, "boundary surface drift")

collection = by_surface["CollectionScalarI64Routes"]
need(collection.get("candidate_contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "collection contract drift")
need(collection.get("effect_class") == "observe", "collection effect drift")
need(collection.get("publication_policy") == "NoPublication", "collection publication drift")
need(collection.get("write_result_policy_required") is False, "collection write policy drift")
need(collection.get("direct_closeout_ready") is False, "collection direct ready drift")
need("CollectionBoundarySeparationFromMapLoadRequired" in (collection.get("blocked_by") or []), "collection blocker drift")
need(set(collection.get("route_kind_set") or []) == {"MapEntryCount", "ArraySlotLen", "StringLen", "AnyLength"}, "collection route drift")

write = by_surface["WriteScalarI64Routes"]
need(write.get("candidate_contract_id") == "WriteResultScalarI64ClassificationOnly", "write contract drift")
need(write.get("effect_class") == "mutate", "write effect drift")
need(write.get("publication_policy") == "MixedNoPublicationAndNone", "write publication drift")
need(write.get("write_result_policy_required") is True, "write policy drift")
need(write.get("direct_closeout_ready") is False, "write direct ready drift")
need("WriteResultPolicyRequiredBeforeDirectCloseout" in (write.get("blocked_by") or []), "write blocker drift")
need(set(write.get("route_kind_set") or []) == {"ArrayAppendAny", "MapDeleteAny", "MapStoreI64", "MapStoreAny"}, "write route drift")

for expected in [
    "GenericMethodRouteKind::MapEntryCount",
    "GenericMethodRouteKind::ArraySlotLen",
    "GenericMethodRouteKind::StringLen",
    "GenericMethodRouteKind::AnyLength",
    "GenericMethodRouteProof::LenSurfacePolicy",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::ScalarI64",
    "GenericMethodPublicationPolicy::NoPublication",
]:
    need(expected in collection_source, f"missing collection evidence token: {expected}")

for expected in [
    "GenericMethodRouteKind::ArrayAppendAny",
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteKind::MapStoreI64",
    "GenericMethodRouteKind::MapStoreAny",
    "GenericMethodRouteProof::PushSurfacePolicy",
    "GenericMethodRouteProof::DeleteSurfacePolicy",
    "GenericMethodRouteProof::SetSurfacePolicy",
    "GenericMethodValueDemand::WriteAny",
]:
    need(expected in write_source, f"missing write evidence token: {expected}")

for expected in [
    "'route_kind': 'map_entry_count'",
    "'route_kind': 'array_slot_len'",
    "'route_kind': 'string_len'",
    "'route_kind': 'any_length'",
    "'route_kind': 'array_append_any'",
    "'route_kind': 'map_delete_any'",
    "'route_kind': 'map_store_i64'",
    "'route_kind': 'map_store_any'",
]:
    need(expected in route_registry, f"missing registry evidence token: {expected}")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "basis-only drift")
need(rule.get("direct_contract_selection_allowed") is False, "direct selection rule drift")
need(rule.get("collection_direct_closeout_forbidden_at_basis") is True, "collection basis forbid drift")
need(rule.get("write_direct_closeout_forbidden_at_basis") is True, "write basis forbid drift")
need(rule.get("boundary_inventory_rerun_required") is True, "rerun rule drift")
for key in [
    "route_membership_alone_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("remaining_surface_boundary_inventory_basis") == 1, "summary basis drift")
need(summary.get("remaining_surface_count") == 2, "summary remaining count drift")
need(summary.get("collection_surface_inventory") == 1, "summary collection drift")
need(summary.get("write_surface_inventory") == 1, "summary write drift")
need(summary.get("direct_contract_selection") == 0, "summary direct selection drift")
need(summary.get("collection_direct_closeout_ready") == 0, "summary collection ready drift")
need(summary.get("write_direct_closeout_ready") == 0, "summary write ready drift")
need(summary.get("scalar_known_transport_axis_closeout") == 0, "summary axis closeout drift")
need(summary.get("source_selfhost_claim") == 0, "summary source selfhost drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectRemainingSurfaceBoundaryInventoryRerun", "decision kind drift")
need(decision.get("reason_token") == "CollectionMixedWithPriorMapLoadAndWriteResultPolicyUnresolved", "reason drift")
need(decision.get("selected_surface_id") is None, "surface selected at basis")
need(decision.get("selected_contract_id") is None, "contract selected at basis")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "remaining_surface_boundary_inventory_basis",
    "collection_surface_inventory",
    "write_surface_inventory",
    "basis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_contract_selection",
    "collection_direct_closeout_ready",
    "write_direct_closeout_ready",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2106-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis")
print("remaining_surface_boundary_inventory_basis=1")
print("collection_surface_inventory=1")
print("write_surface_inventory=1")
print("direct_contract_selection=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

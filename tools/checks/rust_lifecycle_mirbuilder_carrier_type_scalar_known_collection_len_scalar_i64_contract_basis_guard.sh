#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2109-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RUST_BOUNDARY="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
COLLECTION_SOURCE="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"
COLLECTION_TEST="$ROOT/src/mir/generic_method_route_plan/tests/string_routes/len_routes.rs"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RUST_BOUNDARY" "$COLLECTION_SOURCE" "$COLLECTION_TEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
rust_boundary = Path(sys.argv[5]).read_text(encoding="utf-8")
collection_source = Path(sys.argv[6]).read_text(encoding="utf-8")
collection_test = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownCollectionLenScalarI64ContractBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("selected_surface_id") == "CollectionScalarI64Routes", "selected surface drift")
need(inputs.get("selected_contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "selected contract drift")

contract = fixture.get("contract") or {}
need(contract.get("contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "contract id drift")
need(contract.get("surface_id") == "CollectionScalarI64Routes", "surface drift")
need(contract.get("rust_boundary_status") == "CandidateNeedsPolicy", "status drift")
need(contract.get("return_shape") == "ScalarI64", "return shape drift")
need(contract.get("value_demand") == "ScalarI64", "value demand drift")
need(contract.get("publication_policy") == "NoPublication", "publication drift")
need(contract.get("core_method_lowering_tier") == "WarmDirectAbi", "tier drift")
need(contract.get("effect_class") == "observe", "effect drift")
need(contract.get("separate_from_map_load_contract") is True, "map load boundary drift")
need(contract.get("write_result_policy_required") is False, "write policy drift")
need(len(contract.get("routes") or []) == 4, "route count drift")
need({row.get("route_kind") for row in contract.get("routes") or []} == {
    "MapEntryCount",
    "ArraySlotLen",
    "StringLen",
    "AnyLength",
}, "route set drift")

for expected in [
    "CollectionLenScalarI64TypedDirectCloseoutContract",
    "ScalarKnownContractId::CollectionLenScalarI64",
    "ScalarKnownSurfaceId::CollectionScalarI64Routes",
    "ScalarKnownEffectClass::Observe",
    "GenericMethodRouteKind::MapEntryCount",
    "GenericMethodRouteKind::ArraySlotLen",
    "GenericMethodRouteKind::StringLen",
    "GenericMethodRouteKind::AnyLength",
]:
    need(expected in rust_boundary, f"missing rust boundary token: {expected}")

for expected in [
    "GenericMethodRouteKind::MapEntryCount",
    "GenericMethodRouteKind::ArraySlotLen",
    "GenericMethodRouteKind::StringLen",
    "GenericMethodRouteKind::AnyLength",
    "GenericMethodRouteProof::LenSurfacePolicy",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::ScalarI64",
    "GenericMethodPublicationPolicy::NoPublication",
    "CoreMethodLoweringTier::WarmDirectAbi",
]:
    need(expected in collection_source or expected in collection_test, f"missing collection evidence token: {expected}")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "basis-only drift")
need(rule.get("contract_materialization_requires_rerun") is True, "rerun rule drift")
need(rule.get("axis_closeout_forbidden_at_basis") is True, "axis closeout rule drift")
for key in [
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("collection_len_scalar_i64_contract_basis") == 1, "summary basis drift")
need(summary.get("collection_len_route_count") == 4, "summary route count drift")
for key in [
    "direct_contract_materialized",
    "collection_direct_closeout_ready",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCollectionLenScalarI64ContractRerun", "decision kind drift")
need(decision.get("reason_token") == "CollectionLenScalarI64ContractBasisDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in ["collection_len_scalar_i64_contract_basis", "basis_only"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_contract_materialized",
    "collection_direct_closeout_ready",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "hako_adoption",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
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
need(manifest_row.get("card", "").endswith("2109-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-basis")
print("collection_len_scalar_i64_contract_basis=1")
print("collection_len_route_count=4")
print("direct_contract_materialized=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

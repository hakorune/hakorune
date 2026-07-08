#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2108-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RUST_BOUNDARY="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" "$RUST_BOUNDARY" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))
rust_boundary = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownRemainingSurfaceBoundaryInventoryRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("accepted_contracts") == [
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
], "accepted input drift")
need(inputs.get("remaining_candidate_surfaces") == [
    "CollectionScalarI64Routes",
    "WriteScalarI64Routes",
], "candidate input drift")

evaluated = fixture.get("evaluated_surfaces") or []
need(len(evaluated) == 2, "evaluated surface count drift")
by_surface = {row.get("surface_id"): row for row in evaluated}
need(set(by_surface) == {"CollectionScalarI64Routes", "WriteScalarI64Routes"}, "surface set drift")

collection = by_surface["CollectionScalarI64Routes"]
need(collection.get("candidate_contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "collection contract drift")
need(collection.get("selection_eligible") is True, "collection eligibility drift")
need(collection.get("collection_boundary_separated_from_map_load") is True, "collection boundary drift")
need(collection.get("write_result_policy_required") is False, "collection write policy drift")
need(collection.get("selected_next_card_if_eligible") == next_card, "collection next drift")
need(collection.get("blocked_by") == [], "collection blocker drift")

write = by_surface["WriteScalarI64Routes"]
need(write.get("candidate_contract_id") == "WriteResultScalarI64ClassificationOnly", "write contract drift")
need(write.get("selection_eligible") is False, "write eligibility drift")
need(write.get("write_result_policy_required") is True, "write policy drift")
need("WriteResultPolicyRequiredBeforeDirectCloseout" in (write.get("blocked_by") or []), "write blocker drift")

for expected in [
    "CollectionLenScalarI64TypedDirectCloseoutContract",
    "WriteResultScalarI64ClassificationOnly",
    "CandidateNeedsPolicy",
    "GenericMethodRouteKind::MapEntryCount",
    "GenericMethodRouteKind::ArraySlotLen",
    "GenericMethodRouteKind::StringLen",
    "GenericMethodRouteKind::AnyLength",
    "GenericMethodRouteKind::ArrayAppendAny",
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteKind::MapStoreI64",
    "GenericMethodRouteKind::MapStoreAny",
]:
    need(expected in rust_boundary, f"missing rust boundary token: {expected}")

summary = fixture.get("summary") or {}
need(summary.get("remaining_surface_boundary_inventory_rerun") == 1, "summary rerun drift")
need(summary.get("evaluated_surface_count") == 2, "summary surface count drift")
need(summary.get("selection_eligible_surface_count") == 1, "summary eligible count drift")
need(summary.get("selected_surface_id") == "CollectionScalarI64Routes", "summary selected surface drift")
need(summary.get("selected_contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "summary selected contract drift")
need(summary.get("collection_boundary_separated_from_map_load") == 1, "summary collection boundary drift")
for key in [
    "write_result_policy_ready",
    "direct_contract_materialized",
    "collection_direct_closeout_ready",
    "write_direct_closeout_ready",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCollectionLenScalarI64ContractBasis", "decision kind drift")
need(decision.get("reason_token") == "ExactlyOneRemainingScalarKnownSurfaceBoundaryEligible", "reason drift")
need(decision.get("selected_surface_id") == "CollectionScalarI64Routes", "decision selected surface drift")
need(decision.get("selected_contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "decision selected contract drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
for key in [
    "remaining_surface_boundary_inventory_rerun",
    "collection_boundary_separated_from_map_load",
    "direct_contract_selection",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_contract_materialized",
    "collection_direct_closeout_ready",
    "write_direct_closeout_ready",
    "write_result_policy_ready",
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
need(manifest_row.get("card", "").endswith("2108-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_rerun_guard.sh"), "manifest guard drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest drift")
need(state.get("current_blocker_token") == next_card, "CURRENT_STATE blocker drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun")
print("remaining_surface_boundary_inventory_rerun=1")
print("selection_eligible_surface_count=1")
print("selected_surface_id=CollectionScalarI64Routes")
print("selected_contract_id=CollectionLenScalarI64TypedDirectCloseoutContract")
print("write_result_policy_ready=0")
print("direct_contract_materialized=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

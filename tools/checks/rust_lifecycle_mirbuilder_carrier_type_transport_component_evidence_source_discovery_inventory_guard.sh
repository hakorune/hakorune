#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_component_evidence_source_discovery_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2085-MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"
next_card = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportComponentEvidenceSourceDiscoveryInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("component_evidence_source_discovery_basis", "").endswith("mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"), "basis input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("basis_decision") == "SelectCarrierTypeComponentEvidenceSourceDiscoveryInventory", "basis decision drift")
need(previous.get("basis_reason_token") == "CarrierTypeComponentEvidenceSourceDiscoveryBasisDefined", "basis reason drift")
need(previous.get("basis_selected_next_card") == token, "basis next-card drift")
need(previous.get("component_requirement_count") == 7, "previous requirement count drift")
need(previous.get("previous_root_component_requirement_count") == 0, "previous root count drift")
need(previous.get("previous_reason_token") == "NoCarrierTypeRemainingAxisRootComponentRequirement", "previous reason drift")

rule = fixture.get("inventory_rule") or {}
need(rule.get("name") == "ComponentEvidenceSourceDiscoveryInventoryV1", "inventory rule drift")
need(rule.get("reads_existing_authority_sources_only") is True, "read-only authority rule drift")
need(rule.get("accepted_source_must_join_current_requirement") is True, "join rule drift")
need(rule.get("accepted_source_must_have_stable_id") is True, "stable id rule drift")
need(rule.get("accepted_source_must_have_proof_source_hash") is True, "proof hash rule drift")
need(rule.get("self_signed_component_authority_forbidden") is True, "self-signed rule drift")
need(rule.get("hardcoded_component_priority_forbidden") is True, "hardcoded rule drift")
need(rule.get("component_specific_card_selection") is False, "component selection rule drift")
need(rule.get("concrete_carrier_type_axis_selection") is False, "axis selection rule drift")
need(rule.get("if_no_accepted_source_return_wider") is True, "wider fallback rule drift")

source_rows = fixture.get("source_kind_rows") or []
expected_sources = {
    "StableComponentPolicyContract",
    "ExplicitBoundaryDeclaration",
    "StableCrossLaneHandoffContract",
    "CollectionOverlapContract",
    "TypedDirectCloseoutContract",
}
need({row.get("source_kind") for row in source_rows} == expected_sources, "source kind set drift")
for row in source_rows:
    need(row.get("accepted_source_count") == 0, f"source count drift: {row.get('source_kind')}")
    need(row.get("discovery_state") == "NoAcceptedSource", f"source state drift: {row.get('source_kind')}")
    need("proof_source_hash" in (row.get("required_fields") or []), f"proof hash missing: {row.get('source_kind')}")

requirement_rows = fixture.get("component_requirement_source_rows") or []
expected_requirements = {
    "TupleFieldDomainBoundaryPolicy",
    "TupleElementTransportPolicy",
    "CollectionPolicyOverlapResolution",
    "CollectionElementCarrierPolicy",
    "IteratorBorrowBoundaryRoutingPolicy",
    "OpaqueTypeBoundaryDeclaration",
    "ScalarKnownCloseoutAuthority",
}
need({row.get("requirement_id") for row in requirement_rows} == expected_requirements, "requirement set drift")
for row in requirement_rows:
    need(row.get("accepted_sources") == [], f"accepted sources drift: {row.get('requirement_id')}")
    need(row.get("proof_tuple_complete") is False, f"proof tuple drift: {row.get('requirement_id')}")
    need(row.get("selection_eligible") is False, f"selection drift: {row.get('requirement_id')}")
    need(row.get("reason_token"), f"missing reason token: {row.get('requirement_id')}")

summary = fixture.get("summary") or {}
for key in [
    "accepted_component_evidence_source_count",
    "component_authority_source_count",
    "component_requirement_with_accepted_source_count",
    "component_specific_card_selection",
    "concrete_carrier_type_axis_selection",
    "stable_component_policy_contract_count",
    "explicit_boundary_declaration_count",
    "stable_cross_lane_handoff_contract_count",
    "collection_overlap_contract_count",
    "typed_direct_closeout_contract_count",
]:
    need(summary.get(key) == 0, f"summary drift: {key}")
need(summary.get("component_requirement_count") == 7, "summary requirement count drift")
need(summary.get("allowed_source_kind_count") == 5, "summary source kind count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWiderRouteSelectionBasis", "decision kind drift")
need(decision.get("reason_token") == "NoCarrierTypeComponentEvidenceSourceAuthority", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") is None, "component must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "component_specific_card_selection",
    "concrete_carrier_type_axis_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "hardcoded_carrier_axis_priority",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "return_type_string_mapping_as_proof",
    "observed_subaxis_set_as_proof",
    "self_signed_component_authority",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2085-MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_component_evidence_source_discovery_inventory_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory")
print("component_requirement_count=7")
print("allowed_source_kind_count=5")
print("accepted_component_evidence_source_count=0")
print("decision=SelectWiderRouteSelectionBasis")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

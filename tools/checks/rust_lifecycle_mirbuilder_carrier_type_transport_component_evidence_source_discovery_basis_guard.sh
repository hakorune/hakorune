#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_component_evidence_source_discovery_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2084-MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-BASIS-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportComponentEvidenceSourceDiscoveryBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("component_requirement_rerun", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"), "rerun input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("component_requirement_count") == 7, "previous requirement count drift")
need(previous.get("accepted_component_evidence_source_count") == 0, "previous evidence count drift")
need(previous.get("ready_component_requirement_count") == 0, "previous ready count drift")
need(previous.get("root_component_requirement_count") == 0, "previous root count drift")
need(previous.get("selected_component_requirement") is None, "previous component selection drift")
need(previous.get("selected_carrier_type_axis") is None, "previous axis selection drift")
need(previous.get("previous_reason_token") == "NoCarrierTypeRemainingAxisRootComponentRequirement", "previous reason drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "ComponentEvidenceSourceDiscoveryAuthorityV1", "selector rule drift")
need(rule.get("basis_selects_concrete_axis") is False, "basis must not select axis")
need(rule.get("basis_selects_component_specific_card") is False, "basis must not select component")
need(rule.get("discovery_source_must_be_independent") is True, "independent source rule drift")
need(rule.get("self_signed_component_authority_forbidden") is True, "self-signed rule drift")
need(rule.get("hardcoded_component_priority_forbidden") is True, "hardcoded priority rule drift")
need(rule.get("source_discovery_may_select_inventory_only") is True, "inventory-only rule drift")
need(rule.get("if_no_accepted_source_after_inventory_return_wider") is True, "fallback rule drift")

allowed = fixture.get("allowed_evidence_source_kinds") or []
allowed_names = {row.get("source_kind") for row in allowed}
need(allowed_names == {
    "StableComponentPolicyContract",
    "ExplicitBoundaryDeclaration",
    "StableCrossLaneHandoffContract",
    "CollectionOverlapContract",
    "TypedDirectCloseoutContract",
}, "allowed source set drift")
for row in allowed:
    need("proof_source_hash" in (row.get("required_fields") or []), f"missing proof hash: {row.get('source_kind')}")
    need(row.get("allowed_for"), f"missing allowed_for: {row.get('source_kind')}")

forbidden = set(fixture.get("forbidden_evidence_source_kinds") or [])
for item in [
    "ReturnTypeStringMapping",
    "SourcePathOrModuleInference",
    "OwnerNameInference",
    "ShapeSignatureInference",
    "RouteMembershipAlone",
    "ObservedSubaxisSet",
    "RowCount",
    "LexicalOrder",
    "ApparentSimplicity",
    "SelfSignedFixture",
]:
    need(item in forbidden, f"missing forbidden source: {item}")

expected_requirements = {
    "TupleFieldDomainBoundaryPolicy",
    "TupleElementTransportPolicy",
    "CollectionPolicyOverlapResolution",
    "CollectionElementCarrierPolicy",
    "IteratorBorrowBoundaryRoutingPolicy",
    "OpaqueTypeBoundaryDeclaration",
    "ScalarKnownCloseoutAuthority",
}
expectations = fixture.get("component_requirement_source_expectations") or []
need({row.get("requirement_id") for row in expectations} == expected_requirements, "expectation set drift")
for row in expectations:
    need(row.get("accepted_source_kinds"), f"missing source kinds: {row.get('requirement_id')}")
    need(row.get("if_no_source_reason"), f"missing no-source reason: {row.get('requirement_id')}")

summary = fixture.get("summary") or {}
need(summary.get("component_requirement_count") == 7, "summary requirement count drift")
need(summary.get("accepted_component_evidence_source_count") == 0, "summary evidence drift")
need(summary.get("component_specific_card_selection") == 0, "summary component selection drift")
need(summary.get("concrete_carrier_type_axis_selection") == 0, "summary concrete axis drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeComponentEvidenceSourceDiscoveryInventory", "decision kind drift")
need(decision.get("reason_token") == "CarrierTypeComponentEvidenceSourceDiscoveryBasisDefined", "reason drift")
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
need(manifest_row.get("card", "").endswith("2084-MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_component_evidence_source_discovery_basis_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis")
print("component_requirement_count=7")
print("basis_selects_concrete_axis=0")
print("component_specific_card_selection=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

#!/usr/bin/env python3
"""Define carrier/type component evidence source discovery authority."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"

COMPONENT_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"
)
COMPONENT_INVENTORY = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory-v0.json"
)
COMPONENT_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"
)


ALLOWED_EVIDENCE_SOURCE_KINDS = [
    {
        "source_kind": "StableComponentPolicyContract",
        "allowed_for": [
            "TupleElementTransportPolicy",
            "CollectionElementCarrierPolicy",
            "IteratorBorrowBoundaryRoutingPolicy",
            "ScalarKnownCloseoutAuthority",
        ],
        "required_fields": [
            "component_policy_contract_id",
            "supported_requirement_id",
            "proof_source_hash",
            "current_input_compatibility",
        ],
    },
    {
        "source_kind": "ExplicitBoundaryDeclaration",
        "allowed_for": [
            "TupleFieldDomainBoundaryPolicy",
            "OpaqueTypeBoundaryDeclaration",
        ],
        "required_fields": [
            "boundary_declaration_id",
            "resource_id",
            "declared_boundary_kind",
            "proof_source_hash",
        ],
    },
    {
        "source_kind": "StableCrossLaneHandoffContract",
        "allowed_for": ["IteratorBorrowBoundaryRoutingPolicy"],
        "required_fields": [
            "handoff_contract_id",
            "source_lane",
            "target_lane",
            "handoff_condition",
            "proof_source_hash",
        ],
    },
    {
        "source_kind": "CollectionOverlapContract",
        "allowed_for": ["CollectionPolicyOverlapResolution"],
        "required_fields": [
            "overlap_contract_id",
            "parent_policy_lane_ref",
            "child_axis_ref",
            "overlap_kind",
            "proof_source_hash",
        ],
    },
    {
        "source_kind": "TypedDirectCloseoutContract",
        "allowed_for": ["ScalarKnownCloseoutAuthority"],
        "required_fields": [
            "closeout_contract_id",
            "all_rows_join_contract",
            "no_carrier_boundary_required_or_already_covered",
            "proof_source_hash",
        ],
    },
]

SOURCE_EXPECTATIONS = [
    {
        "requirement_id": "TupleFieldDomainBoundaryPolicy",
        "candidate_axis": "ProductTupleTransportAxis",
        "accepted_source_kinds": ["ExplicitBoundaryDeclaration"],
        "if_no_source_reason": "TupleFieldDomainBoundaryAuthoritySourceMissing",
    },
    {
        "requirement_id": "TupleElementTransportPolicy",
        "candidate_axis": "ProductTupleTransportAxis",
        "accepted_source_kinds": ["StableComponentPolicyContract"],
        "if_no_source_reason": "TupleElementTransportAuthoritySourceMissing",
    },
    {
        "requirement_id": "CollectionPolicyOverlapResolution",
        "candidate_axis": "CollectionCarrierTransportAxis",
        "accepted_source_kinds": ["CollectionOverlapContract"],
        "if_no_source_reason": "CollectionPolicyOverlapAuthoritySourceMissing",
    },
    {
        "requirement_id": "CollectionElementCarrierPolicy",
        "candidate_axis": "CollectionCarrierTransportAxis",
        "accepted_source_kinds": ["StableComponentPolicyContract"],
        "blocked_until": ["CollectionPolicyOverlapResolution"],
        "if_no_source_reason": "CollectionElementCarrierAuthoritySourceMissing",
    },
    {
        "requirement_id": "IteratorBorrowBoundaryRoutingPolicy",
        "candidate_axis": "IteratorOrBorrowTypeTransportAxis",
        "accepted_source_kinds": [
            "StableCrossLaneHandoffContract",
            "StableComponentPolicyContract",
        ],
        "if_no_source_reason": "IteratorBorrowBoundaryRoutingAuthoritySourceMissing",
    },
    {
        "requirement_id": "OpaqueTypeBoundaryDeclaration",
        "candidate_axis": "OpaqueTypeTransportAxis",
        "accepted_source_kinds": ["ExplicitBoundaryDeclaration"],
        "if_no_source_reason": "OpaqueTypeBoundaryAuthoritySourceMissing",
    },
    {
        "requirement_id": "ScalarKnownCloseoutAuthority",
        "candidate_axis": "ScalarKnownTransportAxis",
        "accepted_source_kinds": ["TypedDirectCloseoutContract"],
        "if_no_source_reason": "ScalarKnownCloseoutAuthoritySourceMissing",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(COMPONENT_BASIS)
    inventory = read_json(COMPONENT_INVENTORY)
    rerun = read_json(COMPONENT_RERUN)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportComponentEvidenceSourceDiscoveryBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "component_requirement_basis": rel(COMPONENT_BASIS),
            "component_requirement_inventory": rel(COMPONENT_INVENTORY),
            "component_requirement_rerun": rel(COMPONENT_RERUN),
        },
        "provenance": {
            "component_requirement_basis_hash": sha256_file(COMPONENT_BASIS),
            "component_requirement_inventory_hash": sha256_file(COMPONENT_INVENTORY),
            "component_requirement_rerun_hash": sha256_file(COMPONENT_RERUN),
        },
        "previous_state": {
            "component_requirement_count": rerun.get("summary", {}).get(
                "component_requirement_count"
            ),
            "accepted_component_evidence_source_count": rerun.get("summary", {}).get(
                "accepted_component_evidence_source_count"
            ),
            "ready_component_requirement_count": rerun.get("summary", {}).get(
                "ready_component_requirement_count"
            ),
            "root_component_requirement_count": rerun.get("summary", {}).get(
                "root_component_requirement_count"
            ),
            "selected_component_requirement": rerun.get("decision", {}).get(
                "selected_component_requirement"
            ),
            "selected_carrier_type_axis": rerun.get("decision", {}).get(
                "selected_carrier_type_axis"
            ),
            "previous_reason_token": rerun.get("decision", {}).get("reason_token"),
            "basis_requirement_count": basis.get("summary", {}).get(
                "component_requirement_count"
            ),
            "inventory_decision": inventory.get("decision", {}).get("kind"),
        },
        "selector_rule": {
            "name": "ComponentEvidenceSourceDiscoveryAuthorityV1",
            "basis_selects_concrete_axis": False,
            "basis_selects_component_specific_card": False,
            "discovery_source_must_be_independent": True,
            "self_signed_component_authority_forbidden": True,
            "hardcoded_component_priority_forbidden": True,
            "accepted_source_requires": [
                "stable_source_id",
                "stable_contract_or_declaration_id",
                "proof_source_hash",
                "current_requirement_join",
                "non_string_non_path_authority",
            ],
            "source_discovery_may_select_inventory_only": True,
            "if_no_accepted_source_after_inventory_return_wider": True,
        },
        "allowed_evidence_source_kinds": ALLOWED_EVIDENCE_SOURCE_KINDS,
        "forbidden_evidence_source_kinds": [
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
        ],
        "component_requirement_source_expectations": SOURCE_EXPECTATIONS,
        "summary": {
            "component_requirement_count": 7,
            "accepted_component_evidence_source_count": 0,
            "component_specific_card_selection": 0,
            "concrete_carrier_type_axis_selection": 0,
        },
        "decision": {
            "kind": "SelectCarrierTypeComponentEvidenceSourceDiscoveryInventory",
            "reason_token": "CarrierTypeComponentEvidenceSourceDiscoveryBasisDefined",
            "selected_carrier_type_axis": None,
            "selected_component_requirement": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "component_specific_card_selection": 0,
            "concrete_carrier_type_axis_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "hardcoded_carrier_axis_priority": 0,
            "row_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "return_type_string_mapping_as_proof": 0,
            "observed_subaxis_set_as_proof": 0,
            "self_signed_component_authority": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

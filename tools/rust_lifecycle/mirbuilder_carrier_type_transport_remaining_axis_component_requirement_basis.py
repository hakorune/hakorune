#!/usr/bin/env python3
"""Define component requirements for remaining carrier/type axes."""

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
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-INVENTORY-001"

PRIORITY_BASIS = (
    FIXTURES / "mirbuilder-carrier-type-transport-remaining-axis-priority-basis-v0.json"
)
PRIORITY_RERUN = (
    FIXTURES / "mirbuilder-carrier-type-transport-remaining-axis-priority-rerun-v0.json"
)
UNCLASSIFIED = FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"
BASIS_008 = FIXTURES / "source-selfhost-wider-route-selection-basis-008-v0.json"

COMPONENT_REQUIREMENTS = [
    {
        "requirement_id": "TupleFieldDomainBoundaryPolicy",
        "candidate_axes": ["ProductTupleTransportAxis"],
        "requirement_kind": "BoundaryPolicy",
        "pre_inventory_required": True,
        "selected_next_card_if_root": "MIRBUILDER-CARRIER-TYPE-PRODUCT-TUPLE-FIELD-DOMAIN-INVENTORY-001",
        "root_status": "NotEvaluatedAtBasis",
    },
    {
        "requirement_id": "TupleElementTransportPolicy",
        "candidate_axes": ["ProductTupleTransportAxis"],
        "requirement_kind": "TransportPolicy",
        "pre_inventory_required": True,
        "selected_next_card_if_root": "MIRBUILDER-CARRIER-TYPE-PRODUCT-TUPLE-ELEMENT-TRANSPORT-POLICY-BASIS-001",
        "root_status": "NotEvaluatedAtBasis",
    },
    {
        "requirement_id": "CollectionPolicyOverlapResolution",
        "candidate_axes": ["CollectionCarrierTransportAxis"],
        "requirement_kind": "OverlapResolution",
        "overlaps_with_parent_policy_lane": "CollectionCarrierPolicyCandidate",
        "selected_next_card_if_root": "MIRBUILDER-CARRIER-TYPE-COLLECTION-POLICY-OVERLAP-RESOLUTION-001",
        "root_status": "NotEvaluatedAtBasis",
    },
    {
        "requirement_id": "CollectionElementCarrierPolicy",
        "candidate_axes": ["CollectionCarrierTransportAxis"],
        "requirement_kind": "ElementCarrierPolicy",
        "blocked_until": ["CollectionPolicyOverlapResolution"],
        "selected_next_card_if_root": "MIRBUILDER-CARRIER-TYPE-COLLECTION-ELEMENT-CARRIER-POLICY-BASIS-001",
        "root_status": "BlockedByComponentDependency",
    },
    {
        "requirement_id": "IteratorBorrowBoundaryRoutingPolicy",
        "candidate_axes": ["IteratorOrBorrowTypeTransportAxis"],
        "requirement_kind": "BoundaryRouting",
        "possible_outcomes": [
            "HandoffToBorrowSurfaceLane",
            "DefineCarrierTypeIteratorPolicy",
            "KeepStopped",
        ],
        "selected_next_card_if_root": "MIRBUILDER-CARRIER-TYPE-ITERATOR-BORROW-BOUNDARY-ROUTING-BASIS-001",
        "root_status": "NotEvaluatedAtBasis",
    },
    {
        "requirement_id": "OpaqueTypeBoundaryDeclaration",
        "candidate_axes": ["OpaqueTypeTransportAxis"],
        "requirement_kind": "ExplicitBoundaryDeclaration",
        "selected_next_card_if_root": "MIRBUILDER-CARRIER-TYPE-OPAQUE-TYPE-BOUNDARY-DECLARATION-BASIS-001",
        "root_status": "NotEvaluatedAtBasis",
    },
    {
        "requirement_id": "ScalarKnownCloseoutAuthority",
        "candidate_axes": ["ScalarKnownTransportAxis"],
        "requirement_kind": "DirectCloseoutOrPolicyBasis",
        "selected_next_card_if_root": "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001",
        "root_status": "NotEvaluatedAtBasis",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def root_authority(status: str) -> dict[str, Any]:
    return {"status": status, "proof_sources": []}


def build_fixture() -> dict[str, Any]:
    priority_basis = read_json(PRIORITY_BASIS)
    priority_rerun = read_json(PRIORITY_RERUN)
    unclassified = read_json(UNCLASSIFIED)
    basis_008 = read_json(BASIS_008)

    requirement_rows = []
    for item in COMPONENT_REQUIREMENTS:
        row = {
            key: value
            for key, value in item.items()
            if key not in {"root_status"}
        }
        row["root_authority"] = root_authority(item["root_status"])
        requirement_rows.append(row)

    requirements_by_axis: dict[str, list[str]] = {}
    for row in requirement_rows:
        for axis in row["candidate_axes"]:
            requirements_by_axis.setdefault(axis, []).append(row["requirement_id"])

    candidate_axes = []
    for row in priority_basis.get("candidate_axes") or []:
        axis = row.get("axis")
        candidate_axes.append(
            {
                "axis": axis,
                "component_requirement_ids": requirements_by_axis.get(axis, []),
                "component_requirement_complete": False,
                "axis_selection_eligible": False,
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "remaining_axis_priority_basis": rel(PRIORITY_BASIS),
            "remaining_axis_priority_rerun_001": rel(PRIORITY_RERUN),
            "carrier_type_transport_unclassified_evidence_resolution_002": rel(UNCLASSIFIED),
            "source_selfhost_wider_route_selection_basis_008": rel(BASIS_008),
        },
        "provenance": {
            "remaining_axis_priority_basis_hash": sha256_file(PRIORITY_BASIS),
            "remaining_axis_priority_rerun_001_hash": sha256_file(PRIORITY_RERUN),
            "carrier_type_transport_unclassified_evidence_resolution_002_hash": sha256_file(
                UNCLASSIFIED
            ),
            "source_selfhost_wider_route_selection_basis_008_hash": sha256_file(BASIS_008),
        },
        "previous_state": {
            "candidate_axis_count": priority_rerun.get("summary", {}).get(
                "candidate_axis_count"
            ),
            "scope_eligible_axis_count": priority_rerun.get("summary", {}).get(
                "scope_eligible_axis_count"
            ),
            "guard_clean_axis_count": priority_rerun.get("summary", {}).get(
                "guard_clean_axis_count"
            ),
            "evidence_inventory_complete_axis_count": priority_rerun.get("summary", {}).get(
                "evidence_inventory_complete_axis_count"
            ),
            "proof_tuple_complete_axis_count": priority_rerun.get("summary", {}).get(
                "proof_tuple_complete_axis_count"
            ),
            "selection_eligible_axis_count": priority_rerun.get("summary", {}).get(
                "selection_eligible_axis_count"
            ),
            "previous_decision": priority_rerun.get("decision", {}).get("kind"),
            "previous_reason_token": priority_rerun.get("decision", {}).get(
                "reason_token"
            ),
            "selected_carrier_type_axis": priority_rerun.get("decision", {}).get(
                "selected_carrier_type_axis"
            ),
            "basis_008_domain_object_id_parked": basis_008.get("summary", {}).get(
                "domain_object_id_lane_parked"
            ),
            "unclassified_resolved_axis_count": unclassified.get("summary", {}).get(
                "resolved_axis_count"
            ),
        },
        "selector_rule": {
            "name": "CarrierTypeRemainingAxisComponentRequirementSelectorV1",
            "basis_selects_concrete_axis": False,
            "axis_selection_deferred_to_remaining_axis_priority_rerun": True,
            "component_specific_card_selection_allowed_if_exactly_one_root_requirement": True,
            "root_component_requirement_requires": [
                "component_scope_eligible",
                "guard_clean_authority",
                "evidence_inventory_completeness",
                "one_of: component_dependency_root_authority, component_policy_contract_readiness, prior_closed_component_contract_continuation, typed_direct_closeout_authority, explicit_boundary_declaration_authority",
            ],
            "tie_breaking_forbidden": True,
            "if_multiple_root_requirements_keep_stopped": True,
            "forbidden_priority_sources": [
                "row_count",
                "source_path",
                "owner_name",
                "route_membership_alone",
                "return_type_string_mapping",
                "lexical_order",
                "coverage_percentage",
                "apparent_simplicity",
                "observed_subaxis_set",
                "hardcoded_carrier_axis_priority",
                "manual_axis_selection",
            ],
        },
        "component_requirements": requirement_rows,
        "candidate_axes": candidate_axes,
        "summary": {
            "candidate_axis_count": len(candidate_axes),
            "component_requirement_count": len(requirement_rows),
            "root_component_requirement_count": 0,
            "component_specific_card_selection_eligible_count": 0,
            "concrete_carrier_type_axis_selection": 0,
        },
        "decision": {
            "kind": "SelectCarrierTypeRemainingAxisComponentRequirementInventory",
            "reason_token": "CarrierTypeRemainingAxisComponentRequirementsDefined",
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
        print("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Inventory typed evidence for remaining carrier/type component requirements."""

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
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-001"

BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"
)
PRIORITY_RERUN = (
    FIXTURES / "mirbuilder-carrier-type-transport-remaining-axis-priority-rerun-v0.json"
)
UNCLASSIFIED = FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"
POLICY_INVENTORY = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"

EVIDENCE_REQUIREMENTS = {
    "TupleFieldDomainBoundaryPolicy": {
        "required_evidence_kind": "TupleFieldDomainBoundaryDeclaration",
        "missing_reason": "TupleFieldDomainBoundaryInventoryMissing",
    },
    "TupleElementTransportPolicy": {
        "required_evidence_kind": "TupleElementTransportPolicyContract",
        "missing_reason": "TupleElementTransportPolicyMissing",
    },
    "CollectionPolicyOverlapResolution": {
        "required_evidence_kind": "CollectionPolicyOverlapResolution",
        "missing_reason": "CollectionPolicyOverlapResolutionMissing",
    },
    "CollectionElementCarrierPolicy": {
        "required_evidence_kind": "CollectionElementCarrierPolicyContract",
        "missing_reason": "CollectionElementCarrierPolicyBlockedByOverlapResolution",
        "blocked_by_requirement": "CollectionPolicyOverlapResolution",
    },
    "IteratorBorrowBoundaryRoutingPolicy": {
        "required_evidence_kind": "IteratorBorrowBoundaryRoutingPolicy",
        "missing_reason": "IteratorBorrowBoundaryRoutingPolicyMissing",
    },
    "OpaqueTypeBoundaryDeclaration": {
        "required_evidence_kind": "OpaqueTypeBoundaryDeclaration",
        "missing_reason": "OpaqueTypeBoundaryDeclarationMissing",
    },
    "ScalarKnownCloseoutAuthority": {
        "required_evidence_kind": "ScalarKnownCloseoutAuthority",
        "missing_reason": "ScalarKnownCloseoutAuthorityMissing",
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_inventory_row(requirement: dict[str, Any]) -> dict[str, Any]:
    requirement_id = requirement["requirement_id"]
    spec = EVIDENCE_REQUIREMENTS[requirement_id]
    blocked = spec.get("blocked_by_requirement")
    state = "BlockedByComponentDependency" if blocked else "Missing"
    status = "BlockedByComponentDependency" if blocked else "Unproven"
    blocked_by = [spec["missing_reason"]]
    if blocked:
        blocked_by.append(f"{blocked}NotResolved")

    return {
        "requirement_id": requirement_id,
        "candidate_axes": requirement.get("candidate_axes") or [],
        "requirement_kind": requirement.get("requirement_kind"),
        "required_evidence_kind": spec["required_evidence_kind"],
        "inventory_state": state,
        "accepted_evidence_sources": [],
        "rejected_evidence_sources": [],
        "root_authority": {
            "status": status,
            "reason_token": spec["missing_reason"],
            "proof_sources": [],
        },
        "component_scope_eligible": True,
        "guard_clean_authority": "Proven",
        "evidence_inventory_completeness": "Proven",
        "proof_tuple_complete": False,
        "selection_eligible": False,
        "blocked_by": blocked_by,
        "selected_next_card_if_root": requirement.get("selected_next_card_if_root"),
    }


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    priority_rerun = read_json(PRIORITY_RERUN)
    unclassified = read_json(UNCLASSIFIED)
    policy_inventory = read_json(POLICY_INVENTORY)

    inventory_rows = [
        build_inventory_row(row) for row in basis.get("component_requirements") or []
    ]
    requirements_by_axis: dict[str, list[str]] = {}
    for row in inventory_rows:
        for axis in row["candidate_axes"]:
            requirements_by_axis.setdefault(axis, []).append(row["requirement_id"])

    candidate_axes = []
    for row in basis.get("candidate_axes") or []:
        axis = row.get("axis")
        requirement_ids = requirements_by_axis.get(axis, [])
        candidate_axes.append(
            {
                "axis": axis,
                "component_requirement_ids": requirement_ids,
                "ready_component_requirement_count": 0,
                "root_component_requirement_count": 0,
                "component_requirement_complete": False,
                "axis_selection_eligible": False,
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "remaining_axis_component_requirement_basis": rel(BASIS),
            "remaining_axis_priority_rerun_001": rel(PRIORITY_RERUN),
            "carrier_type_transport_unclassified_evidence_resolution_002": rel(UNCLASSIFIED),
            "carrier_type_transport_policy_inventory_rerun_003": rel(POLICY_INVENTORY),
        },
        "provenance": {
            "remaining_axis_component_requirement_basis_hash": sha256_file(BASIS),
            "remaining_axis_priority_rerun_001_hash": sha256_file(PRIORITY_RERUN),
            "carrier_type_transport_unclassified_evidence_resolution_002_hash": sha256_file(
                UNCLASSIFIED
            ),
            "carrier_type_transport_policy_inventory_rerun_003_hash": sha256_file(
                POLICY_INVENTORY
            ),
        },
        "previous_state": {
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_reason_token": basis.get("decision", {}).get("reason_token"),
            "basis_selected_next_card": basis.get("decision", {}).get("selected_next_card"),
            "basis_component_requirement_count": basis.get("summary", {}).get(
                "component_requirement_count"
            ),
            "basis_root_component_requirement_count": basis.get("summary", {}).get(
                "root_component_requirement_count"
            ),
            "priority_rerun_reason_token": priority_rerun.get("decision", {}).get(
                "reason_token"
            ),
            "priority_rerun_selection_eligible_axis_count": priority_rerun.get(
                "summary", {}
            ).get("selection_eligible_axis_count"),
            "unclassified_resolved_axis_count": unclassified.get("summary", {}).get(
                "resolved_axis_count"
            ),
            "policy_lane_candidates_present": bool(policy_inventory),
        },
        "inventory_rule": {
            "name": "CarrierTypeRemainingAxisComponentRequirementInventoryV1",
            "reads_existing_typed_component_evidence_only": True,
            "component_evidence_must_be_non_self_signed": True,
            "component_evidence_must_have_stable_proof_source_hash": True,
            "basis_selects_concrete_axis": False,
            "concrete_carrier_type_axis_selection": False,
            "row_count_as_proof": False,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "route_membership_alone_as_proof": False,
            "return_type_string_mapping_as_proof": False,
            "observed_subaxis_set_as_proof": False,
            "hardcoded_carrier_axis_priority": False,
        },
        "component_evidence_search_scope": [
            {
                "source": rel(BASIS),
                "used_for": "component_requirement_definitions",
                "policy_authority": False,
            },
            {
                "source": rel(UNCLASSIFIED),
                "used_for": "candidate_axis_scope",
                "policy_authority": False,
            },
            {
                "source": rel(POLICY_INVENTORY),
                "used_for": "parent_policy_lane_diagnostics",
                "policy_authority": False,
            },
        ],
        "component_evidence_inventory_rows": inventory_rows,
        "candidate_axes": candidate_axes,
        "summary": {
            "candidate_axis_count": len(candidate_axes),
            "component_requirement_count": len(inventory_rows),
            "accepted_component_evidence_source_count": 0,
            "ready_component_requirement_count": 0,
            "root_component_requirement_count": 0,
            "component_specific_card_selection_eligible_count": 0,
            "concrete_carrier_type_axis_selection": 0,
        },
        "decision": {
            "kind": "SelectCarrierTypeRemainingAxisComponentRequirementRerun",
            "reason_token": "CarrierTypeRemainingAxisComponentRequirementInventoryRecorded",
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
        print("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

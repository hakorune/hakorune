#!/usr/bin/env python3
"""Define the remaining carrier/type axis priority basis after DomainObject/Id park."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-remaining-axis-priority-basis-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001"

BASIS_008 = FIXTURES / "source-selfhost-wider-route-selection-basis-008-v0.json"
POLICY_RERUN_003 = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"
EVIDENCE_RERUN_003 = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-rerun-003-v0.json"
UNCLASSIFIED_RESOLUTION_002 = (
    FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"
)
DOMAIN_DECL_INVENTORY = (
    FIXTURES / "mirbuilder-domain-object-id-semantic-resource-domain-declaration-inventory-v0.json"
)

DEFERRED_PARENT_POLICY_LANES = [
    "ResultCarrierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "SelfConstructorTransportPolicyCandidate",
    "CollectionCarrierPolicyCandidate",
]

CANDIDATE_AXIS_REQUIREMENTS = {
    "ProductTupleTransportAxis": {
        "selected_next_card_if_selected": "MIRBUILDER-CARRIER-TYPE-PRODUCT-TUPLE-TRANSPORT-POLICY-BASIS-001",
        "component_policy_requirements": [
            "TupleFieldDomainBoundaryPolicy",
            "TupleElementTransportPolicy",
        ],
    },
    "CollectionCarrierTransportAxis": {
        "selected_next_card_if_selected": "MIRBUILDER-CARRIER-TYPE-COLLECTION-CARRIER-TRANSPORT-POLICY-BASIS-001",
        "component_policy_requirements": [
            "CollectionElementCarrierPolicy",
            "CollectionPolicyOverlapResolution",
        ],
        "overlap_with_parent_policy_lane": "CollectionCarrierPolicyCandidate",
    },
    "IteratorOrBorrowTypeTransportAxis": {
        "selected_next_card_if_selected": "MIRBUILDER-CARRIER-TYPE-ITERATOR-BORROW-TRANSPORT-POLICY-BASIS-001",
        "component_policy_requirements": [
            "IteratorBorrowBoundaryPolicy",
            "BorrowSurfaceOverlapCheck",
        ],
    },
    "OpaqueTypeTransportAxis": {
        "selected_next_card_if_selected": "MIRBUILDER-CARRIER-TYPE-OPAQUE-TYPE-TRANSPORT-POLICY-BASIS-001",
        "component_policy_requirements": ["OpaqueTypeBoundaryPolicy"],
    },
    "ScalarKnownTransportAxis": {
        "selected_next_card_if_selected": "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001",
        "component_policy_requirements": ["ScalarKnownCloseoutOrPolicyBasis"],
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def unevaluated() -> dict[str, Any]:
    return {"status": "NotEvaluatedAtBasis", "proof_sources": []}


def build_fixture() -> dict[str, Any]:
    basis_008 = read_json(BASIS_008)
    policy_rerun = read_json(POLICY_RERUN_003)
    evidence_rerun = read_json(EVIDENCE_RERUN_003)
    unclassified = read_json(UNCLASSIFIED_RESOLUTION_002)
    domain_decl = read_json(DOMAIN_DECL_INVENTORY)

    basis_008_summary = basis_008.get("summary") or {}
    axis_counts = (unclassified.get("summary") or {}).get("axis_counts") or {}
    policy_lane_counts = (policy_rerun.get("summary") or {}).get(
        "policy_lane_candidate_counts"
    ) or {}

    deferred_parent_policy_lanes = []
    for lane in DEFERRED_PARENT_POLICY_LANES:
        deferred_parent_policy_lanes.append(
            {
                "policy_lane": lane,
                "source": "carrier_type_transport_policy_inventory_rerun_003",
                "diagnostic_count": policy_lane_counts.get(lane, 0),
                "selection_scope": "DeferredParentPolicyLane",
                "selection_eligible_in_this_basis": False,
            }
        )

    candidate_axes = []
    for axis in sorted(CANDIDATE_AXIS_REQUIREMENTS):
        spec = CANDIDATE_AXIS_REQUIREMENTS[axis]
        row = {
            "axis": axis,
            "source": "carrier_type_transport_unclassified_evidence_resolution_002",
            "diagnostic_count": axis_counts.get(axis, 0),
            "scope_eligible": True,
            "guard_clean_authority": {"status": "NotEvaluatedAtBasis"},
            "evidence_inventory_completeness": {"status": "NotEvaluatedAtBasis"},
            "dependency_root_authority": unevaluated(),
            "prior_closed_lane_continuation_authority": unevaluated(),
            "policy_contract_readiness": {
                "status": "NotEvaluatedAtBasis",
                "policy_contract_id": None,
                "proof_sources": [],
            },
            "component_policy_requirements": spec["component_policy_requirements"],
            "proof_tuple_complete": False,
            "selection_eligible": False,
            "selected_next_card_if_selected": spec["selected_next_card_if_selected"],
        }
        if "overlap_with_parent_policy_lane" in spec:
            row["overlap_with_parent_policy_lane"] = spec[
                "overlap_with_parent_policy_lane"
            ]
        candidate_axes.append(row)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportRemainingAxisPriorityBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "wider_route_selection_basis_008": rel(BASIS_008),
            "carrier_type_transport_policy_inventory_rerun_003": rel(POLICY_RERUN_003),
            "carrier_type_transport_evidence_inventory_rerun_003": rel(
                EVIDENCE_RERUN_003
            ),
            "carrier_type_transport_unclassified_evidence_resolution_002": rel(
                UNCLASSIFIED_RESOLUTION_002
            ),
            "domain_object_id_semantic_resource_domain_declaration_inventory": rel(
                DOMAIN_DECL_INVENTORY
            ),
        },
        "provenance": {
            "wider_route_selection_basis_008_hash": sha256_file(BASIS_008),
            "carrier_type_transport_policy_inventory_rerun_003_hash": sha256_file(
                POLICY_RERUN_003
            ),
            "carrier_type_transport_evidence_inventory_rerun_003_hash": sha256_file(
                EVIDENCE_RERUN_003
            ),
            "carrier_type_transport_unclassified_evidence_resolution_002_hash": sha256_file(
                UNCLASSIFIED_RESOLUTION_002
            ),
            "domain_object_id_semantic_resource_domain_declaration_inventory_hash": sha256_file(
                DOMAIN_DECL_INVENTORY
            ),
        },
        "previous_state": {
            "domain_object_id_lane_parked": basis_008_summary.get(
                "domain_object_id_lane_parked"
            ),
            "domain_object_id_park_reason_token": basis_008.get(
                "domain_object_id_lane_parking", {}
            ).get("park_reason_token"),
            "domain_object_id_subaxis_selection_eligible": basis_008_summary.get(
                "domain_object_id_subaxis_selection_eligible"
            ),
            "basis_008_selected_next_card": basis_008.get("decision", {}).get(
                "selected_next_card"
            ),
            "policy_lane_candidates_present": bool(policy_lane_counts),
            "unclassified_axis_resolution_present": bool(axis_counts),
            "domain_object_id_declaration_inventory_decision": domain_decl.get(
                "decision", {}
            ).get("kind"),
            "carrier_type_evidence_inventory_decision": evidence_rerun.get(
                "decision", {}
            ).get("kind"),
        },
        "selector_rule": {
            "name": "CarrierTypeRemainingAxisMechanicalSelectorV1",
            "basis_selects_concrete_axis": False,
            "rerun_may_select_axis_only_if_exactly_one_proof_tuple_complete": True,
            "selection_scope": "ResolvedNonDomainObjectAxesFromCarrierTypeTransportEvidenceInventoryRequired",
            "parent_policy_lanes_deferred_until_unclassified_branch_closed_or_parked": True,
            "scope_eligibility_requires": [
                "domain_object_id_lane_parked",
                "resolved_axis_from_unclassified_evidence_resolution",
                "axis != DomainObjectOrIdTransportAxis",
            ],
            "proof_tuple_complete_requires": [
                "scope_eligible",
                "guard_clean_authority",
                "evidence_inventory_completeness",
                "one_of: dependency_root_authority, prior_closed_lane_continuation_authority, policy_contract_readiness",
            ],
            "forbidden_priority_sources": [
                "row_count",
                "owner_name",
                "source_path",
                "route_membership_alone",
                "lexical_order",
                "coverage_percentage",
                "apparent_simplicity",
                "return_type_string_mapping",
                "observed_subaxis_set",
                "hardcoded_carrier_axis_priority",
            ],
        },
        "deferred_parent_policy_lanes": deferred_parent_policy_lanes,
        "parked_axes": [
            {
                "axis": "DomainObjectOrIdTransportAxis",
                "parked": True,
                "park_reason_token": "ExplicitSemanticResourceDomainDeclarationSourceMissing",
                "selection_eligible": False,
            }
        ],
        "candidate_axes": candidate_axes,
        "allowed_proof_axes": {
            "dependency_root_authority": "selection proof if typed dependency graph proves unique root",
            "prior_closed_lane_continuation_authority": "selection proof if stable closed contract/resource joins current rows",
            "policy_contract_readiness": "selection proof if current durable policy/verifier contract is reusable and compatible",
            "guard_clean_authority": "required filter only",
            "evidence_inventory_completeness": "required filter only",
            "parent_scope_continuation_authority": "scope proof only",
        },
        "summary": {
            "domain_object_id_lane_parked": basis_008_summary.get(
                "domain_object_id_lane_parked"
            ),
            "parked_axis_count": 1,
            "deferred_parent_policy_lane_count": len(deferred_parent_policy_lanes),
            "candidate_axis_count": len(candidate_axes),
            "basis_selection_eligible_axis_count": 0,
            "basis_selects_concrete_axis": 0,
        },
        "decision": {
            "kind": "SelectCarrierTypeRemainingAxisPriorityRerun",
            "reason_token": "CarrierTypeRemainingAxisPriorityBasisDefined",
            "selected_carrier_type_axis": None,
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "hardcoded_carrier_axis_priority": 0,
            "row_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "observed_subaxis_set_as_proof": 0,
            "return_type_string_mapping_as_proof": 0,
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
        print("mirbuilder-carrier-type-transport-remaining-axis-priority-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Inventory carrier/type component evidence source authority."""

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
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-COMPONENT-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009"

BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"
)
COMPONENT_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"
)
POLICY_INVENTORY = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"
UNCLASSIFIED = FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"


SOURCE_COUNT_KEYS = {
    "StableComponentPolicyContract": "stable_component_policy_contract_count",
    "ExplicitBoundaryDeclaration": "explicit_boundary_declaration_count",
    "StableCrossLaneHandoffContract": "stable_cross_lane_handoff_contract_count",
    "CollectionOverlapContract": "collection_overlap_contract_count",
    "TypedDirectCloseoutContract": "typed_direct_closeout_contract_count",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_source_kind_rows(basis: dict[str, Any]) -> list[dict[str, Any]]:
    rows = []
    for source in basis.get("allowed_evidence_source_kinds") or []:
        rows.append(
            {
                "source_kind": source.get("source_kind"),
                "allowed_for": source.get("allowed_for") or [],
                "required_fields": source.get("required_fields") or [],
                "accepted_source_count": 0,
                "discovery_state": "NoAcceptedSource",
                "blocked_by": [
                    f"{source.get('source_kind')}SourceMissing",
                    "NoStableContractOrDeclarationSource",
                ],
            }
        )
    return rows


def build_requirement_rows(basis: dict[str, Any]) -> list[dict[str, Any]]:
    rows = []
    for expectation in basis.get("component_requirement_source_expectations") or []:
        blocked = expectation.get("blocked_until") or []
        rows.append(
            {
                "requirement_id": expectation.get("requirement_id"),
                "candidate_axis": expectation.get("candidate_axis"),
                "accepted_source_kinds": expectation.get("accepted_source_kinds") or [],
                "accepted_sources": [],
                "discovery_state": "BlockedByPrerequisiteAndNoAcceptedSource"
                if blocked
                else "NoAcceptedSource",
                "blocked_until": blocked,
                "reason_token": expectation.get("if_no_source_reason"),
                "proof_tuple_complete": False,
                "selection_eligible": False,
            }
        )
    return rows


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    component_rerun = read_json(COMPONENT_RERUN)
    policy_inventory = read_json(POLICY_INVENTORY)
    unclassified = read_json(UNCLASSIFIED)

    source_kind_rows = build_source_kind_rows(basis)
    requirement_rows = build_requirement_rows(basis)
    source_counts = {
        SOURCE_COUNT_KEYS[row["source_kind"]]: row["accepted_source_count"]
        for row in source_kind_rows
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportComponentEvidenceSourceDiscoveryInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "component_evidence_source_discovery_basis": rel(BASIS),
            "component_requirement_rerun": rel(COMPONENT_RERUN),
            "carrier_type_transport_policy_inventory_rerun_003": rel(POLICY_INVENTORY),
            "carrier_type_transport_unclassified_evidence_resolution_002": rel(UNCLASSIFIED),
        },
        "provenance": {
            "component_evidence_source_discovery_basis_hash": sha256_file(BASIS),
            "component_requirement_rerun_hash": sha256_file(COMPONENT_RERUN),
            "carrier_type_transport_policy_inventory_rerun_003_hash": sha256_file(
                POLICY_INVENTORY
            ),
            "carrier_type_transport_unclassified_evidence_resolution_002_hash": sha256_file(
                UNCLASSIFIED
            ),
        },
        "previous_state": {
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_reason_token": basis.get("decision", {}).get("reason_token"),
            "basis_selected_next_card": basis.get("decision", {}).get(
                "selected_next_card"
            ),
            "component_requirement_count": basis.get("summary", {}).get(
                "component_requirement_count"
            ),
            "previous_root_component_requirement_count": component_rerun.get(
                "summary", {}
            ).get("root_component_requirement_count"),
            "previous_reason_token": component_rerun.get("decision", {}).get(
                "reason_token"
            ),
            "policy_lane_candidates_present": bool(policy_inventory),
            "unclassified_axis_resolution_present": bool(unclassified),
        },
        "inventory_rule": {
            "name": "ComponentEvidenceSourceDiscoveryInventoryV1",
            "reads_existing_authority_sources_only": True,
            "accepted_source_must_join_current_requirement": True,
            "accepted_source_must_have_stable_id": True,
            "accepted_source_must_have_proof_source_hash": True,
            "self_signed_component_authority_forbidden": True,
            "hardcoded_component_priority_forbidden": True,
            "component_specific_card_selection": False,
            "concrete_carrier_type_axis_selection": False,
            "if_no_accepted_source_return_wider": True,
        },
        "source_search_scope": [
            {
                "source": rel(BASIS),
                "used_for": "allowed_source_kinds",
                "authority_source": False,
            },
            {
                "source": rel(POLICY_INVENTORY),
                "used_for": "diagnostic_parent_policy_lanes",
                "authority_source": False,
            },
            {
                "source": rel(UNCLASSIFIED),
                "used_for": "diagnostic_axis_scope",
                "authority_source": False,
            },
        ],
        "source_kind_rows": source_kind_rows,
        "component_requirement_source_rows": requirement_rows,
        "summary": {
            "component_requirement_count": len(requirement_rows),
            "allowed_source_kind_count": len(source_kind_rows),
            "accepted_component_evidence_source_count": 0,
            "component_authority_source_count": 0,
            "component_requirement_with_accepted_source_count": 0,
            "component_specific_card_selection": 0,
            "concrete_carrier_type_axis_selection": 0,
            **source_counts,
        },
        "decision": {
            "kind": "SelectWiderRouteSelectionBasis",
            "reason_token": "NoCarrierTypeComponentEvidenceSourceAuthority",
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
        print("mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

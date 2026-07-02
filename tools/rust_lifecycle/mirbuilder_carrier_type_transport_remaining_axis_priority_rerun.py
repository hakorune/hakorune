#!/usr/bin/env python3
"""Rerun remaining carrier/type axis priority under the documented basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-remaining-axis-priority-rerun-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001"

BASIS = FIXTURES / "mirbuilder-carrier-type-transport-remaining-axis-priority-basis-v0.json"
UNCLASSIFIED = FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"
EVIDENCE = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-rerun-003-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def unproven(reason: str) -> dict[str, Any]:
    return {
        "status": "Unproven",
        "reason_token": reason,
        "proof_sources": [],
    }


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    unclassified = read_json(UNCLASSIFIED)
    evidence = read_json(EVIDENCE)
    candidate_rows = []

    for row in basis.get("candidate_axes") or []:
        axis = row.get("axis")
        dependency_root = unproven("CarrierTypeRemainingAxisDependencyRootNotProven")
        prior_closed = unproven("CarrierTypeRemainingAxisPriorClosedLaneContinuationNotProven")
        policy_contract = unproven("CarrierTypeRemainingAxisPolicyContractReadinessNotProven")
        component_requirements = row.get("component_policy_requirements") or []
        candidate_rows.append(
            {
                "axis": axis,
                "diagnostic_count": row.get("diagnostic_count"),
                "scope_eligible": row.get("scope_eligible") is True,
                "guard_clean_authority": {
                    "status": "Proven",
                    "proof_sources": [rel(BASIS)],
                },
                "evidence_inventory_completeness": {
                    "status": "Proven",
                    "proof_sources": [rel(UNCLASSIFIED), rel(EVIDENCE)],
                },
                "dependency_root_authority": dependency_root,
                "prior_closed_lane_continuation_authority": prior_closed,
                "policy_contract_readiness": policy_contract,
                "component_policy_requirements": component_requirements,
                "component_requirement_basis_required": bool(component_requirements),
                "proof_tuple_complete": False,
                "selection_eligible": False,
                "selected_next_card_if_selected": row.get("selected_next_card_if_selected"),
                "blocked_by": [
                    dependency_root["reason_token"],
                    prior_closed["reason_token"],
                    policy_contract["reason_token"],
                    "CarrierTypeRemainingAxisComponentRequirementBasisRequired",
                ],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportRemainingAxisPriorityRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "remaining_axis_priority_basis": rel(BASIS),
            "carrier_type_transport_unclassified_evidence_resolution_002": rel(UNCLASSIFIED),
            "carrier_type_transport_evidence_inventory_rerun_003": rel(EVIDENCE),
        },
        "provenance": {
            "remaining_axis_priority_basis_hash": sha256_file(BASIS),
            "carrier_type_transport_unclassified_evidence_resolution_002_hash": sha256_file(
                UNCLASSIFIED
            ),
            "carrier_type_transport_evidence_inventory_rerun_003_hash": sha256_file(
                EVIDENCE
            ),
        },
        "previous_state": {
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_reason_token": basis.get("decision", {}).get("reason_token"),
            "basis_selected_next_card": basis.get("decision", {}).get("selected_next_card"),
            "domain_object_id_lane_parked": basis.get("summary", {}).get(
                "domain_object_id_lane_parked"
            ),
            "basis_candidate_axis_count": basis.get("summary", {}).get(
                "candidate_axis_count"
            ),
            "basis_deferred_parent_policy_lane_count": basis.get("summary", {}).get(
                "deferred_parent_policy_lane_count"
            ),
        },
        "selector_rule": basis.get("selector_rule"),
        "candidate_axes": candidate_rows,
        "summary": {
            "candidate_axis_count": len(candidate_rows),
            "scope_eligible_axis_count": sum(1 for row in candidate_rows if row["scope_eligible"]),
            "guard_clean_axis_count": len(candidate_rows),
            "evidence_inventory_complete_axis_count": len(candidate_rows),
            "dependency_root_authority_proven_count": 0,
            "prior_closed_lane_continuation_authority_proven_count": 0,
            "policy_contract_readiness_proven_count": 0,
            "proof_tuple_complete_axis_count": 0,
            "selection_eligible_axis_count": 0,
            "component_requirement_basis_required_count": sum(
                1 for row in candidate_rows if row["component_requirement_basis_required"]
            ),
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoCarrierTypeRemainingAxisMechanicalCandidate",
            "selected_carrier_type_axis": None,
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "remaining_axis_priority_basis_consumed": 1,
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
        print("mirbuilder-carrier-type-transport-remaining-axis-priority-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

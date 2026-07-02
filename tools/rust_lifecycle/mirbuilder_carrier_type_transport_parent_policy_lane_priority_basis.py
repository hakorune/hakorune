#!/usr/bin/env python3
"""Define the carrier/type parent policy lane priority selector basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001"
NEXT = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS_009 = FIXTURES / "source-selfhost-wider-route-selection-basis-009-v0.json"
POLICY_INVENTORY = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"
EVIDENCE_INVENTORY = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-rerun-003-v0.json"

PARENT_POLICY_LANES = [
    {
        "policy_lane": "ResultCarrierPolicyCandidate",
        "component_policy_requirements": [
            "ResultCarrierVerifierContractReadiness",
            "ResultCarrierProjectionPolicyCurrentCompatibility",
        ],
        "selected_next_card_if_selected": "MIRBUILDER-RESULT-CARRIER-VERIFIER-POLICY-CURRENT-READINESS-BASIS-001",
    },
    {
        "policy_lane": "OptionCarrierPolicyCandidate",
        "component_policy_requirements": [
            "OptionCarrierVerifierContractReadiness",
            "OptionCarrierProjectionPolicyCurrentCompatibility",
        ],
        "selected_next_card_if_selected": "MIRBUILDER-OPTION-CARRIER-VERIFIER-POLICY-BASIS-001",
    },
    {
        "policy_lane": "SelfConstructorTransportPolicyCandidate",
        "component_policy_requirements": [
            "SelfConstructorTransportBoundaryPolicy",
            "SelfConstructorReturnContractReadiness",
        ],
        "selected_next_card_if_selected": "MIRBUILDER-SELF-CONSTRUCTOR-TRANSPORT-POLICY-BASIS-001",
    },
    {
        "policy_lane": "CollectionCarrierPolicyCandidate",
        "component_policy_requirements": [
            "CollectionCarrierParentPolicyReadiness",
            "CollectionCarrierChildAxisOverlapClosed",
        ],
        "selected_next_card_if_selected": "MIRBUILDER-COLLECTION-CARRIER-PARENT-POLICY-BASIS-001",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def policy_lane_counts(policy_inventory: dict[str, Any]) -> dict[str, int]:
    counts = policy_inventory.get("summary", {}).get("policy_lane_candidate_counts", {})
    return {lane["policy_lane"]: int(counts.get(lane["policy_lane"], 0)) for lane in PARENT_POLICY_LANES}


def build_fixture() -> dict[str, Any]:
    basis_009 = read_json(BASIS_009)
    policy_inventory = read_json(POLICY_INVENTORY)
    evidence_inventory = read_json(EVIDENCE_INVENTORY)
    counts = policy_lane_counts(policy_inventory)

    candidate_lanes: list[dict[str, Any]] = []
    for lane in PARENT_POLICY_LANES:
        policy_lane = lane["policy_lane"]
        candidate_lanes.append(
            {
                "policy_lane": policy_lane,
                "diagnostic_count": counts[policy_lane],
                "diagnostic_count_as_proof": False,
                "scope_eligible": counts[policy_lane] > 0,
                "guard_clean_authority": {"status": "NotEvaluatedAtBasis"},
                "evidence_inventory_completeness": {"status": "NotEvaluatedAtBasis"},
                "dependency_root_authority": {"status": "NotEvaluatedAtBasis", "proof_sources": []},
                "prior_closed_policy_continuation_authority": {
                    "status": "NotEvaluatedAtBasis",
                    "proof_sources": [],
                },
                "current_policy_contract_readiness": {
                    "status": "NotEvaluatedAtBasis",
                    "policy_contract_id": None,
                    "proof_sources": [],
                },
                "component_policy_requirements": lane["component_policy_requirements"],
                "proof_tuple_complete": False,
                "selection_eligible": False,
                "selected_next_card_if_selected": lane["selected_next_card_if_selected"],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportParentPolicyLanePriorityBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "wider_route_selection_basis_009": rel(BASIS_009),
            "carrier_type_transport_policy_inventory_rerun_003": rel(POLICY_INVENTORY),
            "carrier_type_transport_evidence_inventory_rerun_003": rel(EVIDENCE_INVENTORY),
        },
        "provenance": {
            "wider_route_selection_basis_009_hash": sha256_file(BASIS_009),
            "carrier_type_transport_policy_inventory_rerun_003_hash": sha256_file(POLICY_INVENTORY),
            "carrier_type_transport_evidence_inventory_rerun_003_hash": sha256_file(EVIDENCE_INVENTORY),
        },
        "previous_state": {
            "basis_009_decision": basis_009.get("decision", {}).get("kind"),
            "basis_009_reason_token": basis_009.get("decision", {}).get("reason_token"),
            "basis_009_selected_next_card": basis_009.get("decision", {}).get("selected_next_card"),
            "carrier_type_remaining_lane_parked": basis_009.get("summary", {}).get(
                "carrier_type_remaining_lane_parked"
            ),
            "direct_parent_policy_candidate_selection": basis_009.get("claims", {}).get(
                "direct_parent_policy_candidate_selection"
            ),
            "policy_lane_candidate_counts": counts,
            "evidence_inventory_complete_count": evidence_inventory.get("summary", {}).get(
                "evidence_inventory_complete_count"
            ),
        },
        "selector_rule": {
            "name": "CarrierTypeParentPolicyLaneMechanicalSelectorV1",
            "basis_selects_concrete_parent_policy_candidate": False,
            "rerun_may_select_parent_policy_only_if_exactly_one_proof_tuple_complete": True,
            "selection_scope": "DeferredCarrierTypeParentPolicyLanes",
            "scope_eligibility_requires": [
                "carrier_type_remaining_lane_parked",
                "parent_policy_lane_candidate_present",
                "direct_parent_policy_candidate_selection_forbidden",
            ],
            "proof_tuple_complete_requires": [
                "scope_eligible",
                "guard_clean_authority",
                "evidence_inventory_completeness",
                "one_of: dependency_root_authority, prior_closed_policy_continuation_authority, current_policy_contract_readiness",
            ],
            "current_policy_contract_readiness_requires": [
                "stable_policy_contract_id",
                "current_input_fixture_hash_compatibility",
                "supported_policy_lane_declared",
                "verifier_contract_declared",
                "not_closed_provenance_only",
            ],
            "tie_breaking_forbidden": True,
            "if_multiple_parent_policy_lanes_keep_stopped": True,
            "forbidden_priority_sources": [
                "row_count",
                "return_type_count",
                "owner_name",
                "source_path",
                "route_membership_alone",
                "lexical_order",
                "coverage_percentage",
                "historical_preference",
                "result_history_as_direct_selection_proof",
                "apparent_simplicity",
                "hardcoded_parent_policy_priority",
            ],
        },
        "candidate_parent_policy_lanes": candidate_lanes,
        "allowed_proof_axes": {
            "dependency_root_authority": "selection proof only if typed dependency graph proves a unique parent policy prerequisite",
            "prior_closed_policy_continuation_authority": "selection proof only if stable closed policy or contract ID joins current rows",
            "current_policy_contract_readiness": "selection proof only if a current durable verifier/policy contract is reusable and compatible",
            "guard_clean_authority": "required filter only",
            "evidence_inventory_completeness": "required filter only",
        },
        "summary": {
            "candidate_parent_policy_lane_count": len(candidate_lanes),
            "scope_eligible_parent_policy_lane_count": sum(1 for lane in candidate_lanes if lane["scope_eligible"]),
            "basis_selection_eligible_parent_policy_lane_count": 0,
            "basis_selects_concrete_parent_policy_candidate": 0,
            "direct_parent_policy_candidate_selection": 0,
        },
        "decision": {
            "kind": "SelectCarrierTypeParentPolicyLanePriorityRerun",
            "reason_token": "CarrierTypeParentPolicyLanePriorityBasisDefined",
            "selected_parent_policy_candidate": None,
            "selected_next_card": NEXT,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "direct_parent_policy_candidate_selection": 0,
            "manual_lane_selection": 0,
            "manual_carrier_selection": 0,
            "hardcoded_parent_policy_priority": 0,
            "row_count_as_proof": 0,
            "return_type_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "historical_preference_as_proof": 0,
            "result_history_as_direct_selection_proof": 0,
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
        print("mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Rerun carrier/type parent policy lane priority under the current basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-carrier-type-transport-parent-policy-lane-priority-basis-v0.json"
RESULT_POLICY = FIXTURES / "mirbuilder-result-carrier-verifier-policy-v0.json"
RESULT_CONTRACT = FIXTURES / "mirbuilder-result-carrier-verifier-contract-v0.json"
RESULT_PROJECTION = FIXTURES / "mirbuilder-result-carrier-verifier-projection-policy-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def result_contract_evidence() -> dict[str, Any]:
    policy = read_json(RESULT_POLICY)
    contract = read_json(RESULT_CONTRACT)
    projection = read_json(RESULT_PROJECTION)
    return {
        "historical_policy_present": policy.get("summary", {}).get("result_carrier_policy_ready") == 1,
        "historical_contract_present": contract.get("summary", {}).get("result_carrier_contract_ready") == 1,
        "historical_projection_policy_present": projection.get("summary", {}).get(
            "result_carrier_projection_policy_selected"
        )
        == 1,
        "historical_contract_row_count": contract.get("summary", {}).get("result_carrier_contract_row_count"),
        "historical_policy_lane": policy.get("summary", {}).get("selected_policy_lane"),
    }


def readiness_for(policy_lane: str, diagnostic_count: int) -> dict[str, Any]:
    if policy_lane == "ResultCarrierPolicyCandidate":
        result = result_contract_evidence()
        return {
            "status": "Unproven",
            "policy_contract_id": "ResultCarrierVerifierContractV1",
            "historical_contract_present": int(result["historical_contract_present"]),
            "historical_projection_policy_present": int(result["historical_projection_policy_present"]),
            "historical_contract_row_count": result["historical_contract_row_count"],
            "current_candidate_count": diagnostic_count,
            "current_input_fixture_hash_compatibility": 0,
            "supported_policy_lane_matches_current_lane": 0,
            "not_closed_provenance_only": 0,
            "blocked_by": [
                "HistoricalResultContractCoversPriorThreeRowsOnly",
                "CurrentPolicyLaneCompatibilityMissing",
                "ResultHistoryIsNotDirectSelectionProof",
            ],
            "proof_sources": [rel(RESULT_POLICY), rel(RESULT_CONTRACT), rel(RESULT_PROJECTION)],
        }
    return {
        "status": "Unproven",
        "policy_contract_id": None,
        "historical_contract_present": 0,
        "current_input_fixture_hash_compatibility": 0,
        "supported_policy_lane_matches_current_lane": 0,
        "not_closed_provenance_only": 0,
        "blocked_by": ["CurrentPolicyContractMissing"],
        "proof_sources": [],
    }


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    candidate_lanes = []
    for lane in basis.get("candidate_parent_policy_lanes") or []:
        policy_lane = lane["policy_lane"]
        diagnostic_count = int(lane["diagnostic_count"])
        current_readiness = readiness_for(policy_lane, diagnostic_count)
        candidate_lanes.append(
            {
                "policy_lane": policy_lane,
                "diagnostic_count": diagnostic_count,
                "diagnostic_count_as_proof": False,
                "scope_eligible": bool(lane.get("scope_eligible")),
                "guard_clean_authority": {"status": "Proven"},
                "evidence_inventory_completeness": {"status": "Proven"},
                "dependency_root_authority": {
                    "status": "Unproven",
                    "typed_dependency_edges_present": 0,
                    "proof_sources": [],
                },
                "prior_closed_policy_continuation_authority": {
                    "status": "Unproven",
                    "stable_closed_policy_or_contract_id_present": 0,
                    "proof_sources": [],
                },
                "current_policy_contract_readiness": current_readiness,
                "proof_tuple_complete": False,
                "selection_eligible": False,
                "blocked_by": sorted(
                    set(
                        [
                            "NoDependencyRootAuthority",
                            "NoPriorClosedPolicyContinuationAuthority",
                        ]
                        + current_readiness.get("blocked_by", [])
                    )
                ),
                "selected_next_card_if_selected": lane.get("selected_next_card_if_selected"),
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportParentPolicyLanePriorityRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "parent_policy_lane_priority_basis": rel(BASIS),
            "historical_result_carrier_policy": rel(RESULT_POLICY),
            "historical_result_carrier_contract": rel(RESULT_CONTRACT),
            "historical_result_carrier_projection_policy": rel(RESULT_PROJECTION),
        },
        "provenance": {
            "parent_policy_lane_priority_basis_hash": sha256_file(BASIS),
            "historical_result_carrier_policy_hash": sha256_file(RESULT_POLICY),
            "historical_result_carrier_contract_hash": sha256_file(RESULT_CONTRACT),
            "historical_result_carrier_projection_policy_hash": sha256_file(RESULT_PROJECTION),
        },
        "previous_state": {
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_reason_token": basis.get("decision", {}).get("reason_token"),
            "basis_selected_next_card": basis.get("decision", {}).get("selected_next_card"),
            "candidate_parent_policy_lane_count": basis.get("summary", {}).get(
                "candidate_parent_policy_lane_count"
            ),
            "basis_selection_eligible_parent_policy_lane_count": basis.get("summary", {}).get(
                "basis_selection_eligible_parent_policy_lane_count"
            ),
        },
        "selector_rule": {
            "name": "CarrierTypeParentPolicyLaneMechanicalSelectorV1",
            "selection_requires_exactly_one_proof_tuple_complete": True,
            "historical_policy_contract_is_diagnostic_until_current_compatibility_proven": True,
            "result_history_as_direct_selection_proof": False,
            "row_count_as_proof": False,
            "return_type_count_as_proof": False,
            "hardcoded_parent_policy_priority": False,
            "manual_parent_policy_selection": False,
        },
        "candidate_parent_policy_lanes": candidate_lanes,
        "summary": {
            "candidate_parent_policy_lane_count": len(candidate_lanes),
            "scope_eligible_parent_policy_lane_count": sum(1 for lane in candidate_lanes if lane["scope_eligible"]),
            "guard_clean_parent_policy_lane_count": len(candidate_lanes),
            "evidence_inventory_complete_parent_policy_lane_count": len(candidate_lanes),
            "current_policy_contract_ready_count": 0,
            "dependency_root_candidate_count": 0,
            "prior_closed_policy_continuation_candidate_count": 0,
            "proof_tuple_complete_parent_policy_lane_count": 0,
            "selection_eligible_parent_policy_lane_count": 0,
            "historical_result_contract_present": 1,
            "historical_result_contract_as_direct_selection_proof": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoCarrierTypeParentPolicyLaneMechanicalCandidate",
            "selected_parent_policy_candidate": None,
            "selected_next_card": DESIGN_STOP,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "direct_parent_policy_candidate_selection": 0,
            "manual_parent_policy_selection": 0,
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
        print("mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

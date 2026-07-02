#!/usr/bin/env python3
"""Define evidence-source discovery authority for carrier/type parent policy lanes."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-BASIS-001"
NEXT = "MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

PARENT_RERUN = FIXTURES / "mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun-v0.json"

SOURCE_KINDS = [
    {
        "source_kind": "CurrentReusablePolicyContract",
        "allowed_for": [
            "ResultCarrierPolicyCandidate",
            "OptionCarrierPolicyCandidate",
            "SelfConstructorTransportPolicyCandidate",
            "CollectionCarrierPolicyCandidate",
        ],
        "required_fields": [
            "policy_contract_id",
            "supported_policy_lane",
            "current_input_fixture_hash",
            "verifier_contract_id",
            "proof_source_hash",
        ],
    },
    {
        "source_kind": "CurrentVerifierContractCompatibility",
        "allowed_for": [
            "ResultCarrierPolicyCandidate",
            "OptionCarrierPolicyCandidate",
            "SelfConstructorTransportPolicyCandidate",
            "CollectionCarrierPolicyCandidate",
        ],
        "required_fields": [
            "verifier_contract_id",
            "current_candidate_set_hash",
            "compatibility_rule",
            "proof_source_hash",
        ],
    },
    {
        "source_kind": "StableParentPolicyDependencyRoot",
        "allowed_for": [
            "ResultCarrierPolicyCandidate",
            "OptionCarrierPolicyCandidate",
            "SelfConstructorTransportPolicyCandidate",
            "CollectionCarrierPolicyCandidate",
        ],
        "required_fields": [
            "typed_dependency_edge_id",
            "dependent_policy_lane",
            "prerequisite_policy_lane",
            "proof_source_hash",
        ],
    },
    {
        "source_kind": "PriorClosedPolicyContinuationContract",
        "allowed_for": [
            "ResultCarrierPolicyCandidate",
            "OptionCarrierPolicyCandidate",
            "SelfConstructorTransportPolicyCandidate",
            "CollectionCarrierPolicyCandidate",
        ],
        "required_fields": [
            "closed_policy_contract_id",
            "current_policy_lane_ref",
            "reopen_closed_lane_required",
            "proof_source_hash",
        ],
    },
    {
        "source_kind": "CrossLanePolicyHandoffContract",
        "allowed_for": [
            "CollectionCarrierPolicyCandidate",
            "SelfConstructorTransportPolicyCandidate",
        ],
        "required_fields": [
            "handoff_contract_id",
            "source_lane",
            "target_lane",
            "handoff_condition",
            "proof_source_hash",
        ],
    },
]

FORBIDDEN_SOURCE_KINDS = [
    "RowCount",
    "ReturnTypeCount",
    "HistoricalPreference",
    "ResultHistoryAlone",
    "OwnerNameInference",
    "SourcePathOrModuleInference",
    "RouteMembershipAlone",
    "LexicalOrder",
    "HardcodedParentPolicyPriority",
    "SelfSignedFixture",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    parent_rerun = read_json(PARENT_RERUN)
    candidates = parent_rerun.get("candidate_parent_policy_lanes") or []
    expectations = []
    for row in candidates:
        lane = row["policy_lane"]
        expectations.append(
            {
                "policy_lane": lane,
                "diagnostic_count": row["diagnostic_count"],
                "accepted_source_kinds": [
                    source["source_kind"]
                    for source in SOURCE_KINDS
                    if lane in source["allowed_for"]
                ],
                "if_no_source_reason": f"{lane}AuthoritySourceMissing",
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeParentPolicyLaneEvidenceSourceDiscoveryBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "parent_policy_lane_priority_rerun": rel(PARENT_RERUN),
        },
        "provenance": {
            "parent_policy_lane_priority_rerun_hash": sha256_file(PARENT_RERUN),
        },
        "previous_state": {
            "previous_decision": parent_rerun.get("decision", {}).get("kind"),
            "previous_reason_token": parent_rerun.get("decision", {}).get("reason_token"),
            "candidate_parent_policy_lane_count": parent_rerun.get("summary", {}).get(
                "candidate_parent_policy_lane_count"
            ),
            "selection_eligible_parent_policy_lane_count": parent_rerun.get("summary", {}).get(
                "selection_eligible_parent_policy_lane_count"
            ),
            "historical_result_contract_present": parent_rerun.get("summary", {}).get(
                "historical_result_contract_present"
            ),
            "historical_result_contract_as_direct_selection_proof": parent_rerun.get("summary", {}).get(
                "historical_result_contract_as_direct_selection_proof"
            ),
        },
        "selector_rule": {
            "name": "ParentPolicyLaneEvidenceSourceDiscoveryAuthorityV1",
            "basis_selects_parent_policy_candidate": False,
            "discovery_source_must_be_independent": True,
            "self_signed_parent_policy_authority_forbidden": True,
            "hardcoded_parent_policy_priority_forbidden": True,
            "historical_result_contract_is_diagnostic_until_current_compatibility_proven": True,
            "accepted_source_requires": [
                "stable_source_id",
                "stable_contract_or_dependency_id",
                "proof_source_hash",
                "current_policy_lane_join",
                "non_string_non_count_non_history_authority",
            ],
            "source_discovery_may_select_inventory_only": True,
            "if_no_accepted_source_after_inventory_return_wider": True,
        },
        "allowed_evidence_source_kinds": SOURCE_KINDS,
        "forbidden_evidence_source_kinds": FORBIDDEN_SOURCE_KINDS,
        "parent_policy_lane_source_expectations": expectations,
        "summary": {
            "candidate_parent_policy_lane_count": len(candidates),
            "allowed_source_kind_count": len(SOURCE_KINDS),
            "accepted_parent_policy_evidence_source_count": 0,
            "parent_policy_candidate_selection": 0,
        },
        "decision": {
            "kind": "SelectParentPolicyLaneEvidenceSourceDiscoveryInventory",
            "reason_token": "ParentPolicyLaneEvidenceSourceDiscoveryBasisDefined",
            "selected_parent_policy_candidate": None,
            "selected_next_card": NEXT,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "parent_policy_candidate_selection": 0,
            "direct_parent_policy_candidate_selection": 0,
            "manual_parent_policy_selection": 0,
            "hardcoded_parent_policy_priority": 0,
            "row_count_as_proof": 0,
            "return_type_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "historical_preference_as_proof": 0,
            "result_history_as_direct_selection_proof": 0,
            "self_signed_parent_policy_authority": 0,
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
        print("mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

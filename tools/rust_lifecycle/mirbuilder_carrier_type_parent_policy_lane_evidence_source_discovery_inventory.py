#!/usr/bin/env python3
"""Inventory evidence sources for carrier/type parent policy lanes."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-inventory-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"
NEXT = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    source_rows = []
    for row in basis.get("allowed_evidence_source_kinds") or []:
        source_rows.append(
            {
                "source_kind": row["source_kind"],
                "allowed_for": row["allowed_for"],
                "required_fields": row["required_fields"],
                "accepted_source_count": 0,
                "discovery_state": "NoAcceptedSource",
                "rejected_reason": f"{row['source_kind']}Missing",
            }
        )

    expectation_rows = []
    for row in basis.get("parent_policy_lane_source_expectations") or []:
        expectation_rows.append(
            {
                "policy_lane": row["policy_lane"],
                "diagnostic_count": row["diagnostic_count"],
                "accepted_source_kinds": row["accepted_source_kinds"],
                "accepted_sources": [],
                "authority_source_count": 0,
                "proof_tuple_complete": False,
                "selection_eligible": False,
                "reason_token": row["if_no_source_reason"],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeParentPolicyLaneEvidenceSourceDiscoveryInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "parent_policy_lane_evidence_source_discovery_basis": rel(BASIS),
        },
        "provenance": {
            "parent_policy_lane_evidence_source_discovery_basis_hash": sha256_file(BASIS),
        },
        "previous_state": {
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_reason_token": basis.get("decision", {}).get("reason_token"),
            "basis_selected_next_card": basis.get("decision", {}).get("selected_next_card"),
            "candidate_parent_policy_lane_count": basis.get("summary", {}).get(
                "candidate_parent_policy_lane_count"
            ),
            "allowed_source_kind_count": basis.get("summary", {}).get("allowed_source_kind_count"),
        },
        "inventory_rule": {
            "name": "ParentPolicyLaneEvidenceSourceDiscoveryInventoryV1",
            "reads_existing_authority_sources_only": True,
            "accepted_source_must_join_current_policy_lane": True,
            "accepted_source_must_have_stable_id": True,
            "accepted_source_must_have_proof_source_hash": True,
            "self_signed_parent_policy_authority_forbidden": True,
            "historical_result_contract_alone_is_not_authority": True,
            "hardcoded_parent_policy_priority_forbidden": True,
            "parent_policy_candidate_selection": False,
            "if_no_accepted_source_return_wider": True,
        },
        "source_kind_rows": source_rows,
        "parent_policy_lane_source_rows": expectation_rows,
        "summary": {
            "candidate_parent_policy_lane_count": len(expectation_rows),
            "allowed_source_kind_count": len(source_rows),
            "accepted_parent_policy_evidence_source_count": 0,
            "parent_policy_authority_source_count": 0,
            "parent_policy_lane_with_accepted_source_count": 0,
            "current_reusable_policy_contract_count": 0,
            "current_verifier_contract_compatibility_count": 0,
            "stable_parent_policy_dependency_root_count": 0,
            "prior_closed_policy_continuation_contract_count": 0,
            "cross_lane_policy_handoff_contract_count": 0,
            "parent_policy_candidate_selection": 0,
        },
        "decision": {
            "kind": "SelectWiderRouteSelectionBasis",
            "reason_token": "NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority",
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
        print("mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

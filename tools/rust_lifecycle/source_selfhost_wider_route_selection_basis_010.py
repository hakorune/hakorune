#!/usr/bin/env python3
"""Select wider lane after carrier/type parent policy authority exhaustion."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-wider-route-selection-basis-010-v0.json"

TOKEN = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010"
NEXT = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

PARENT_INVENTORY = FIXTURES / "mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-inventory-v0.json"
PARENT_RERUN = FIXTURES / "mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun-v0.json"
BASIS_009 = FIXTURES / "source-selfhost-wider-route-selection-basis-009-v0.json"
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
NATIVE_LEDGER = FIXTURES / "native-owner-adoption-ledger-v0.json"
MISSING_PROJECTION_V4 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def maybe_hash(path: Path) -> str | None:
    return sha256_file(path) if path.exists() else None


def candidate_lane(
    lane_id: str,
    authority: str,
    eligible: bool,
    required_proof: list[str],
    selected_next: str,
) -> dict[str, Any]:
    return {
        "lane_id": lane_id,
        "selection_authority": authority,
        "selection_eligible": eligible,
        "required_proof": required_proof,
        "selected_next_card_if_eligible": selected_next,
    }


def build_fixture() -> dict[str, Any]:
    parent_inventory = read_json(PARENT_INVENTORY)
    parent_rerun = read_json(PARENT_RERUN)
    basis_009 = read_json(BASIS_009)
    missing_projection_v4 = read_json(MISSING_PROJECTION_V4)

    candidate_lanes = [
        candidate_lane(
            "UnconvertedSurfaceReportRerun",
            "FreshnessRepair",
            False,
            [
                "source_surface_input_hash_changed",
                "projection_descriptor_ledger_hash_changed",
                "native_owner_adoption_ledger_hash_changed",
            ],
            "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-005",
        ),
        candidate_lane(
            "NativeOwnerCheckpointRerun",
            "AdoptionDeltaCheckpoint",
            False,
            [
                "native_owner_adoption_delta_count > 0",
                "or checkpoint_hash_stale",
            ],
            "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-003",
        ),
        candidate_lane(
            "MissingProjectionPolicyNextLane",
            "CarrierTypeParentExhaustionFallback",
            True,
            [
                "DomainObjectIdLane parked = 1",
                "CarrierTypeRemainingAxisLane parked = 1",
                "CarrierTypeParentPolicyLane parked = 1",
                "missing_projection_policy_evidence_quality_present = 1",
                "freshness_repair_eligible = 0",
                "native_checkpoint_eligible = 0",
                "borrow_surface_policy_lane_eligible = 0",
                "guard_consolidation_eligible = 0",
            ],
            NEXT,
        ),
        candidate_lane(
            "BorrowSurfacePolicyLane",
            "IndependentBorrowBlockerAuthority",
            False,
            [
                "borrow_surface_policy_blocker_count = 1",
                "borrow_policy_fixture_proves_guard_clean = 1",
                "not selected by IteratorBorrow naming",
            ],
            "MIRBUILDER-BORROW-SURFACE-POLICY-BASIS-001",
        ),
        candidate_lane(
            "GuardConsolidation",
            "CodeFacingGuardConsolidationRequired",
            False,
            [
                "lane_guard_profile_missing_or_stale = 1",
                "guard_duplication_blocks_next_lane = 1",
            ],
            "MIRBUILDER-CARRIER-TYPE-LANE-GUARD-CONSOLIDATION-001",
        ),
    ]

    eligible = [lane for lane in candidate_lanes if lane["selection_eligible"]]
    if len(eligible) == 1:
        decision = {
            "kind": "SelectMissingProjectionPolicyClusterResolutionRerun",
            "reason_token": "CarrierTypeParentPolicyLaneExhaustedReturnToMissingProjectionPolicy",
            "selected_lane": eligible[0]["lane_id"],
            "selected_next_card": eligible[0]["selected_next_card_if_eligible"],
            "selected_parent_policy_candidate": None,
            "selected_carrier_type_axis": None,
        }
    elif len(eligible) == 0:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedPostCarrierTypeParentWiderLane",
            "selected_lane": None,
            "selected_next_card": DESIGN_STOP,
            "selected_parent_policy_candidate": None,
            "selected_carrier_type_axis": None,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultiplePostCarrierTypeParentWiderLaneCandidates",
            "selected_lane": None,
            "selected_next_card": DESIGN_STOP,
            "selected_parent_policy_candidate": None,
            "selected_carrier_type_axis": None,
        }

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostWiderRouteSelectionBasis010V1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "carrier_type_parent_policy_evidence_source_discovery_inventory": rel(PARENT_INVENTORY),
            "carrier_type_parent_policy_priority_rerun": rel(PARENT_RERUN),
            "source_selfhost_wider_route_selection_basis_009": rel(BASIS_009),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
            "native_owner_adoption_ledger": rel(NATIVE_LEDGER),
            "missing_projection_policy_cluster_resolution_v4": rel(MISSING_PROJECTION_V4),
        },
        "provenance": {
            "carrier_type_parent_policy_evidence_source_discovery_inventory_hash": sha256_file(PARENT_INVENTORY),
            "carrier_type_parent_policy_priority_rerun_hash": sha256_file(PARENT_RERUN),
            "source_selfhost_wider_route_selection_basis_009_hash": sha256_file(BASIS_009),
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
            "native_owner_adoption_ledger_hash": maybe_hash(NATIVE_LEDGER),
            "missing_projection_policy_cluster_resolution_v4_hash": sha256_file(MISSING_PROJECTION_V4),
        },
        "previous_state": {
            "latest_card": parent_inventory.get("token"),
            "previous_decision": parent_inventory.get("decision", {}).get("kind"),
            "previous_reason_token": parent_inventory.get("decision", {}).get("reason_token"),
            "candidate_parent_policy_lane_count": parent_inventory.get("summary", {}).get(
                "candidate_parent_policy_lane_count"
            ),
            "accepted_parent_policy_evidence_source_count": parent_inventory.get("summary", {}).get(
                "accepted_parent_policy_evidence_source_count"
            ),
            "parent_policy_authority_source_count": parent_inventory.get("summary", {}).get(
                "parent_policy_authority_source_count"
            ),
            "parent_policy_lane_with_accepted_source_count": parent_inventory.get("summary", {}).get(
                "parent_policy_lane_with_accepted_source_count"
            ),
            "current_reusable_policy_contract_count": parent_inventory.get("summary", {}).get(
                "current_reusable_policy_contract_count"
            ),
            "current_verifier_contract_compatibility_count": parent_inventory.get("summary", {}).get(
                "current_verifier_contract_compatibility_count"
            ),
            "stable_parent_policy_dependency_root_count": parent_inventory.get("summary", {}).get(
                "stable_parent_policy_dependency_root_count"
            ),
            "prior_closed_policy_continuation_contract_count": parent_inventory.get("summary", {}).get(
                "prior_closed_policy_continuation_contract_count"
            ),
            "cross_lane_policy_handoff_contract_count": parent_inventory.get("summary", {}).get(
                "cross_lane_policy_handoff_contract_count"
            ),
            "parent_policy_candidate_selection": parent_inventory.get("summary", {}).get(
                "parent_policy_candidate_selection"
            ),
            "result_history_as_direct_selection_proof": parent_inventory.get("claims", {}).get(
                "result_history_as_direct_selection_proof"
            ),
        },
        "parked_lanes": [
            {
                "lane_id": "DomainObjectIdLane",
                "parked": True,
                "park_reason_token": "ExplicitSemanticResourceDomainDeclarationSourceMissing",
            },
            {
                "lane_id": "CarrierTypeRemainingAxisLane",
                "parked": True,
                "park_reason_token": "NoCarrierTypeComponentEvidenceSourceAuthority",
            },
            {
                "lane_id": "CarrierTypeParentPolicyLane",
                "parked": True,
                "park_reason_token": "NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority",
                "parent_policy_candidate_selection": 0,
                "accepted_parent_policy_evidence_source_count": 0,
            },
        ],
        "selector_rule": {
            "name": "PostCarrierTypeParentPolicyAuthorityExhaustionWiderLaneSelectorV1",
            "park_carrier_type_parent_policy_lane_before_wider_selection": True,
            "direct_parent_policy_candidate_selection_forbidden": True,
            "result_history_as_direct_selection_proof": False,
            "freshness_repair_precedes_missing_projection_selection": True,
            "native_checkpoint_requires_adoption_delta_or_stale_checkpoint": True,
            "borrow_surface_lane_requires_independent_borrow_blocker_authority": True,
            "guard_consolidation_requires_guard_blocker": True,
            "missing_projection_lane_requires_carrier_type_parent_exhausted": True,
            "select_only_if_exactly_one_machine_derived_lane": True,
            "forbidden_proof_axes": [
                "row_count",
                "return_type_count",
                "source_path",
                "owner_name",
                "route_membership_alone",
                "observed_subaxis_set",
                "historical_preference",
                "ResultHistoryAlone",
                "lexical_order",
                "hardcoded_parent_policy_priority",
                "self_signed_fixture",
                "manual_lane_selection",
                "apparent_simplicity",
                "return_type_string_mapping",
            ],
        },
        "candidate_lanes": candidate_lanes,
        "blocked_reopen_lanes": [
            {
                "lane_id": "ParentPolicyAuthorityReopen",
                "blocked": True,
                "reason_token": "NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority",
                "consultation_only": True,
            },
            {
                "lane_id": "CurrentResultCompatibility",
                "blocked": True,
                "reason_token": "CurrentResultCompatibilitySourceMissing",
                "consultation_only": True,
                "result_history_as_direct_selection_proof": 0,
            },
        ],
        "evidence_quality": {
            "missing_projection_policy_cluster_resolution_v4_present": True,
            "missing_projection_policy_evidence_quality_present": bool(missing_projection_v4.get("decision")),
        },
        "summary": {
            "carrier_type_parent_policy_lane_parked": 1,
            "candidate_lane_count": len(candidate_lanes),
            "selection_eligible_lane_count": len(eligible),
        },
        "decision": decision,
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "parent_policy_candidate_selection": 0,
            "direct_parent_policy_candidate_selection": 0,
            "result_history_as_direct_selection_proof": 0,
            "manual_lane_selection": 0,
            "hardcoded_lane_priority": 0,
            "hardcoded_parent_policy_priority": 0,
            "row_count_as_proof": 0,
            "return_type_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "observed_subaxis_set_as_proof": 0,
            "historical_preference_as_proof": 0,
            "return_type_string_mapping_as_proof": 0,
            "self_signed_fixture_as_proof": 0,
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
        print("source-selfhost-wider-route-selection-basis-010 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

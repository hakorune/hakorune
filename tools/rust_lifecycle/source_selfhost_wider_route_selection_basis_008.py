#!/usr/bin/env python3
"""Select the post-DomainObject/Id authority-exhaustion wider lane."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-wider-route-selection-basis-008-v0.json"

TOKEN = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001"

DOMAIN_DECL_INVENTORY = (
    FIXTURES / "mirbuilder-domain-object-id-semantic-resource-domain-declaration-inventory-v0.json"
)
REGISTRY_RERUN = (
    FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun-v0.json"
)
CARRIER_UNCLASSIFIED = (
    FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"
)
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
NATIVE_CHECKPOINT = FIXTURES / "source-selfhost-native-owner-checkpoint-rerun-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def candidate_lane(
    *,
    lane_id: str,
    selection_authority: str,
    selection_eligible: bool,
    required_proof: list[str],
    selected_next_card_if_eligible: str,
    reason_token: str,
) -> dict[str, Any]:
    return {
        "lane_id": lane_id,
        "selection_authority": selection_authority,
        "selection_eligible": selection_eligible,
        "required_proof": required_proof,
        "reason_token": reason_token,
        "selected_next_card_if_eligible": selected_next_card_if_eligible,
    }


def build_fixture() -> dict[str, Any]:
    declaration_inventory = read_json(DOMAIN_DECL_INVENTORY)
    registry_rerun = read_json(REGISTRY_RERUN)
    carrier_unclassified = read_json(CARRIER_UNCLASSIFIED)
    unconverted_report = read_json(UNCONVERTED_REPORT)
    native_checkpoint = read_json(NATIVE_CHECKPOINT)

    declaration_summary = declaration_inventory.get("summary") or {}
    carrier_summary = carrier_unclassified.get("summary") or {}
    axis_counts = carrier_summary.get("axis_counts") or {}
    non_domain_axes = {
        key: value for key, value in axis_counts.items() if key != "DomainObjectOrIdTransportAxis"
    }

    domain_object_id_parked = (
        declaration_inventory.get("decision", {}).get("kind") == "SelectWiderRouteSelectionBasis"
        and declaration_summary.get("explicit_semantic_resource_domain_declaration_source_count") == 0
        and declaration_summary.get("stable_closed_resource_manifest_count") == 0
        and declaration_summary.get("registry_ready_row_count") == 0
        and declaration_summary.get("accepted_typed_dependency_edge_count") == 0
    )
    remaining_non_domain_axes_present = any(count > 0 for count in non_domain_axes.values())
    carrier_parent_ledger_fresh = (
        carrier_unclassified.get("decision", {}).get("selected_axis")
        == "DomainObjectOrIdTransportAxis"
        and carrier_unclassified.get("claims", {}).get("unclassified_evidence_resolution_ready") == 1
    )

    carrier_lane_eligible = (
        domain_object_id_parked
        and carrier_parent_ledger_fresh
        and remaining_non_domain_axes_present
    )

    candidate_lanes = [
        candidate_lane(
            lane_id="UnconvertedSurfaceReportRerun",
            selection_authority="FreshnessRepair",
            selection_eligible=False,
            required_proof=[
                "source_surface_input_hash_changed",
                "projection_descriptor_ledger_hash_changed",
                "native_owner_adoption_ledger_hash_changed",
            ],
            selected_next_card_if_eligible="MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-005",
            reason_token="NoFreshnessRepairRequired",
        ),
        candidate_lane(
            lane_id="NativeOwnerCheckpointRerun",
            selection_authority="AdoptionDeltaCheckpoint",
            selection_eligible=False,
            required_proof=[
                "native_owner_adoption_delta_count > 0",
                "or checkpoint_hash_stale",
            ],
            selected_next_card_if_eligible="SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-003",
            reason_token="NativeOwnerCheckpointRerunNotRequired",
        ),
        candidate_lane(
            lane_id="CarrierTypeTransportRemainingLanePriority",
            selection_authority="NearestUnexhaustedParentLane",
            selection_eligible=carrier_lane_eligible,
            required_proof=[
                "domain_object_id_lane_parked = 1",
                "carrier_type_parent_ledger_fresh = 1",
                "remaining_non_domain_object_carrier_axes_present = 1",
                "selected_next_card_is_priority_basis_not_axis = 1",
            ],
            selected_next_card_if_eligible=NEXT_CARD,
            reason_token="CarrierTypeRemainingAxisPriorityBasisSelected"
            if carrier_lane_eligible
            else "CarrierTypeParentLaneNotEligible",
        ),
        candidate_lane(
            lane_id="MissingProjectionPolicyNextLane",
            selection_authority="BroaderParentFallback",
            selection_eligible=False,
            required_proof=[
                "carrier_type_parent_exhausted = 1",
                "missing_projection_policy_evidence_quality_present = 1",
            ],
            selected_next_card_if_eligible="MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005",
            reason_token="CarrierTypeParentNotExhausted",
        ),
        candidate_lane(
            lane_id="BorrowSurfacePolicyLane",
            selection_authority="ExactlyOneBorrowPolicyBlocker",
            selection_eligible=False,
            required_proof=[
                "borrow_surface_policy_blocker_count = 1",
                "borrow_policy_fixture_proves_guard_clean = 1",
            ],
            selected_next_card_if_eligible="MIRBUILDER-BORROW-SURFACE-POLICY-BASIS-001",
            reason_token="NoExactlyOneBorrowPolicyBlocker",
        ),
        candidate_lane(
            lane_id="GuardConsolidation",
            selection_authority="CodeFacingGuardConsolidationRequired",
            selection_eligible=False,
            required_proof=[
                "domain_object_id_lane_guard_profile_missing_or_stale = 1",
                "row_specific_guard_duplication_blocks_next_lane = 1",
            ],
            selected_next_card_if_eligible="MIRBUILDER-DOMAIN-OBJECT-ID-LANE-GUARD-CONSOLIDATION-001",
            reason_token="GuardConsolidationNotRootBlocker",
        ),
    ]

    eligible_lanes = [row for row in candidate_lanes if row["selection_eligible"]]
    if len(eligible_lanes) == 1:
        decision = {
            "kind": "SelectCarrierTypeTransportRemainingAxisPriorityBasis",
            "reason_token": "DomainObjectIdAuthorityExhaustedReturnToNearestUnexhaustedParentLane",
            "selected_domain_subaxis": None,
            "selected_next_card": eligible_lanes[0]["selected_next_card_if_eligible"],
        }
    elif not eligible_lanes:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedPostDomainObjectIdWiderLane",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultiplePostDomainObjectIdWiderLaneCandidates",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostWiderRouteSelectionBasis008V1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "latest_card": declaration_inventory.get("token"),
            "domain_object_id_semantic_resource_domain_declaration_inventory": rel(
                DOMAIN_DECL_INVENTORY
            ),
            "stable_type_resource_registry_authority_rerun": rel(REGISTRY_RERUN),
            "carrier_type_transport_unclassified_evidence_resolution": rel(
                CARRIER_UNCLASSIFIED
            ),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
            "native_owner_checkpoint": rel(NATIVE_CHECKPOINT),
        },
        "provenance": {
            "domain_object_id_semantic_resource_domain_declaration_inventory_hash": sha256_file(
                DOMAIN_DECL_INVENTORY
            ),
            "stable_type_resource_registry_authority_rerun_hash": sha256_file(
                REGISTRY_RERUN
            ),
            "carrier_type_transport_unclassified_evidence_resolution_hash": sha256_file(
                CARRIER_UNCLASSIFIED
            ),
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
            "native_owner_checkpoint_hash": sha256_file(NATIVE_CHECKPOINT),
        },
        "selector_rule": {
            "name": "PostDomainObjectIdAuthorityExhaustionWiderLaneSelectorV1",
            "domain_object_id_lane_must_be_parked_before_wider_selection": True,
            "subaxis_selection_forbidden": True,
            "semantic_owner_selection_forbidden": True,
            "freshness_repair_precedes_semantic_lane_selection": True,
            "native_checkpoint_requires_adoption_delta_or_stale_checkpoint": True,
            "nearest_unexhausted_parent_lane_allowed": True,
            "remaining_axis_priority_must_open_basis_not_axis": True,
            "row_count_as_proof": False,
            "owner_name_as_proof": False,
            "source_path_as_authority": False,
            "shape_signature_as_proof": False,
            "route_membership_alone_as_proof": False,
            "observed_subaxis_set_as_proof": False,
            "manual_family_shape_axis_selection": False,
        },
        "domain_object_id_lane_parking": {
            "parked": domain_object_id_parked,
            "park_reason_token": "ExplicitSemanticResourceDomainDeclarationSourceMissing",
            "authority_exhaustion": {
                "candidate_registry_row_count": declaration_summary.get(
                    "candidate_registry_row_count"
                ),
                "explicit_semantic_resource_domain_declaration_source_count": declaration_summary.get(
                    "explicit_semantic_resource_domain_declaration_source_count"
                ),
                "stable_closed_resource_manifest_count": declaration_summary.get(
                    "stable_closed_resource_manifest_count"
                ),
                "resource_domain_declaration_ready_count": declaration_summary.get(
                    "resource_domain_declaration_ready_count"
                ),
                "registry_ready_row_count": declaration_summary.get("registry_ready_row_count"),
                "accepted_typed_dependency_edge_count": declaration_summary.get(
                    "accepted_typed_dependency_edge_count"
                ),
            },
            "safe_reentry_requires_one_of": [
                "new_explicit_semantic_resource_domain_declaration_source",
                "new_stable_closed_resource_manifest",
                "new_non_self_signed_resource_taxonomy_authority",
            ],
            "forbidden_reentry_reasons": [
                "type_identity_coverage_only",
                "return_type_string_mapping",
                "source_path_or_module_inference",
                "observed_subaxis_set_inference",
                "owner_name_or_shape_signature_continuity",
            ],
        },
        "parent_lane_evidence": {
            "carrier_type_parent_ledger_fresh": carrier_parent_ledger_fresh,
            "remaining_non_domain_object_carrier_axes_present": remaining_non_domain_axes_present,
            "non_domain_object_axis_counts": non_domain_axes,
            "carrier_type_previous_selected_axis": carrier_unclassified.get(
                "decision", {}
            ).get("selected_axis"),
            "missing_projection_policy_count": unconverted_report.get("summary", {}).get(
                "missing_projection_policy_count"
            ),
            "borrow_surface_needs_policy_count": unconverted_report.get("summary", {}).get(
                "borrow_policy_needed_count"
            ),
            "native_checkpoint_previous_decision": native_checkpoint.get("decision", {}).get(
                "kind"
            ),
        },
        "candidate_lanes": candidate_lanes,
        "summary": {
            "domain_object_id_lane_parked": 1 if domain_object_id_parked else 0,
            "domain_object_id_subaxis_selection_eligible": 0,
            "candidate_lane_count": len(candidate_lanes),
            "selection_eligible_lane_count": len(eligible_lanes),
        },
        "decision": decision,
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "manual_subaxis_selection": 0,
            "manual_type_to_subaxis_assignment": 0,
            "return_type_string_to_subaxis_mapping": 0,
            "source_path_as_policy_authority": 0,
            "observed_subaxis_set_as_policy_authority": 0,
            "row_count_as_proof": 0,
            "owner_name_as_proof": 0,
            "shape_signature_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "self_signed_taxonomy": 0,
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
        print("source-selfhost-wider-route-selection-basis-008 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

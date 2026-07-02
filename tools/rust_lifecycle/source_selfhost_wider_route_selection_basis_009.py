#!/usr/bin/env python3
"""Select the next wider lane after carrier/type remaining authority exhaustion."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-wider-route-selection-basis-009-v0.json"

TOKEN = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-009"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PARENT-POLICY-LANE-PRIORITY-BASIS-001"

DISCOVERY_INVENTORY = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"
)
DISCOVERY_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"
)
COMPONENT_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"
)
POLICY_INVENTORY = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
NATIVE_LEDGER = FIXTURES / "native-owner-adoption-ledger-v0.json"

DEFERRED_PARENT_POLICY_LANES = [
    "ResultCarrierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "SelfConstructorTransportPolicyCandidate",
    "CollectionCarrierPolicyCandidate",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def maybe_hash(path: Path) -> str | None:
    return sha256_file(path) if path.exists() else None


def candidate_lanes() -> list[dict[str, Any]]:
    return [
        {
            "lane_id": "UnconvertedSurfaceReportRerun",
            "selection_authority": "FreshnessRepair",
            "selection_eligible": False,
            "required_proof": [
                "source_surface_input_hash_changed",
                "projection_descriptor_ledger_hash_changed",
                "native_owner_adoption_ledger_hash_changed",
            ],
            "selected_next_card_if_eligible": "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-005",
        },
        {
            "lane_id": "NativeOwnerCheckpointRerun",
            "selection_authority": "AdoptionDeltaCheckpoint",
            "selection_eligible": False,
            "required_proof": [
                "native_owner_adoption_delta_count > 0",
                "or checkpoint_hash_stale",
            ],
            "selected_next_card_if_eligible": "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-003",
        },
        {
            "lane_id": "CarrierTypeParentPolicyLanePriority",
            "selection_authority": "NearestUnexhaustedParentLane",
            "selection_eligible": True,
            "required_proof": [
                "carrier_type_remaining_axis_lane_parked = 1",
                "carrier_type_parent_policy_lanes_present = 1",
                "carrier_type_parent_policy_lanes_not_parked = 1",
                "selected_next_card_is_priority_basis_not_concrete_policy = 1",
            ],
            "selected_next_card_if_eligible": NEXT_CARD,
        },
        {
            "lane_id": "MissingProjectionPolicyNextLane",
            "selection_authority": "BroaderParentFallback",
            "selection_eligible": False,
            "required_proof": [
                "carrier_type_parent_policy_lane_exhausted = 1",
                "missing_projection_policy_evidence_quality_present = 1",
            ],
            "selected_next_card_if_eligible": "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005",
        },
        {
            "lane_id": "BorrowSurfacePolicyLane",
            "selection_authority": "IndependentBorrowBlockerAuthority",
            "selection_eligible": False,
            "required_proof": [
                "borrow_surface_policy_blocker_count = 1",
                "borrow_policy_fixture_proves_guard_clean = 1",
                "not selected by IteratorBorrow axis name",
            ],
            "selected_next_card_if_eligible": "MIRBUILDER-BORROW-SURFACE-POLICY-BASIS-001",
        },
        {
            "lane_id": "GuardConsolidation",
            "selection_authority": "CodeFacingGuardConsolidationRequired",
            "selection_eligible": False,
            "required_proof": [
                "lane_guard_profile_missing_or_stale = 1",
                "guard_duplication_blocks_next_lane = 1",
            ],
            "selected_next_card_if_eligible": "MIRBUILDER-CARRIER-TYPE-LANE-GUARD-CONSOLIDATION-001",
        },
    ]


def build_fixture() -> dict[str, Any]:
    discovery_inventory = read_json(DISCOVERY_INVENTORY)
    discovery_basis = read_json(DISCOVERY_BASIS)
    component_rerun = read_json(COMPONENT_RERUN)
    policy_inventory = read_json(POLICY_INVENTORY)

    lanes = candidate_lanes()
    eligible = [lane for lane in lanes if lane["selection_eligible"]]
    decision_kind = (
        "SelectCarrierTypeParentPolicyLanePriorityBasis"
        if len(eligible) == 1
        else "KeepStopped"
    )
    reason_token = (
        "CarrierTypeRemainingLaneParkedReturnToParentPolicyLanePriority"
        if len(eligible) == 1
        else "NoMachineDerivedPostCarrierTypeWiderLane"
    )
    selected_lane = eligible[0]["lane_id"] if len(eligible) == 1 else None
    selected_next = eligible[0]["selected_next_card_if_eligible"] if len(eligible) == 1 else DESIGN_STOP

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostWiderRouteSelectionBasis009V1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "component_evidence_source_discovery_inventory": rel(DISCOVERY_INVENTORY),
            "component_evidence_source_discovery_basis": rel(DISCOVERY_BASIS),
            "component_requirement_rerun": rel(COMPONENT_RERUN),
            "carrier_type_transport_policy_inventory_rerun_003": rel(POLICY_INVENTORY),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
            "native_owner_adoption_ledger": rel(NATIVE_LEDGER),
        },
        "provenance": {
            "component_evidence_source_discovery_inventory_hash": sha256_file(
                DISCOVERY_INVENTORY
            ),
            "component_evidence_source_discovery_basis_hash": sha256_file(
                DISCOVERY_BASIS
            ),
            "component_requirement_rerun_hash": sha256_file(COMPONENT_RERUN),
            "carrier_type_transport_policy_inventory_rerun_003_hash": sha256_file(
                POLICY_INVENTORY
            ),
            "unconverted_surface_report_hash": maybe_hash(UNCONVERTED_REPORT),
            "native_owner_adoption_ledger_hash": maybe_hash(NATIVE_LEDGER),
        },
        "previous_state": {
            "latest_card": discovery_inventory.get("token"),
            "previous_decision": discovery_inventory.get("decision", {}).get("kind"),
            "previous_reason_token": discovery_inventory.get("decision", {}).get(
                "reason_token"
            ),
            "component_requirement_count": discovery_inventory.get("summary", {}).get(
                "component_requirement_count"
            ),
            "allowed_source_kind_count": discovery_inventory.get("summary", {}).get(
                "allowed_source_kind_count"
            ),
            "accepted_component_evidence_source_count": discovery_inventory.get(
                "summary", {}
            ).get("accepted_component_evidence_source_count"),
            "component_authority_source_count": discovery_inventory.get("summary", {}).get(
                "component_authority_source_count"
            ),
            "component_requirement_with_accepted_source_count": discovery_inventory.get(
                "summary", {}
            ).get("component_requirement_with_accepted_source_count"),
            "stable_component_policy_contract_count": discovery_inventory.get(
                "summary", {}
            ).get("stable_component_policy_contract_count"),
            "explicit_boundary_declaration_count": discovery_inventory.get(
                "summary", {}
            ).get("explicit_boundary_declaration_count"),
            "stable_cross_lane_handoff_contract_count": discovery_inventory.get(
                "summary", {}
            ).get("stable_cross_lane_handoff_contract_count"),
            "collection_overlap_contract_count": discovery_inventory.get("summary", {}).get(
                "collection_overlap_contract_count"
            ),
            "typed_direct_closeout_contract_count": discovery_inventory.get(
                "summary", {}
            ).get("typed_direct_closeout_contract_count"),
            "component_rerun_reason_token": component_rerun.get("decision", {}).get(
                "reason_token"
            ),
            "discovery_basis_reason_token": discovery_basis.get("decision", {}).get(
                "reason_token"
            ),
        },
        "selector_rule": {
            "name": "PostCarrierTypeRemainingAuthorityExhaustionWiderLaneSelectorV1",
            "carrier_type_remaining_lane_must_be_parked_before_wider_selection": True,
            "concrete_carrier_type_axis_selection_forbidden": True,
            "component_specific_card_selection_forbidden": True,
            "direct_parent_policy_candidate_selection_forbidden": True,
            "freshness_repair_precedes_parent_lane_selection": True,
            "native_checkpoint_requires_adoption_delta_or_stale_checkpoint": True,
            "nearest_unexhausted_parent_lane_allowed": True,
            "missing_projection_lane_requires_carrier_type_parent_exhausted": True,
            "borrow_surface_lane_requires_independent_borrow_blocker_authority": True,
            "guard_consolidation_requires_guard_blocker": True,
            "select_only_if_exactly_one_machine_derived_lane": True,
            "forbidden_proof_axes": [
                "row_count",
                "source_path",
                "owner_name",
                "route_membership_alone",
                "observed_subaxis_set",
                "apparent_simplicity",
                "manual_lane_selection",
                "hardcoded_lane_priority",
                "historical_preference",
                "return_type_string_mapping",
            ],
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
                "accepted_component_evidence_source_count": 0,
                "component_authority_source_count": 0,
                "concrete_carrier_type_axis_selection": 0,
            },
        ],
        "deferred_parent_policy_lanes": [
            {
                "policy_lane": lane,
                "direct_selection_allowed": False,
                "selection_scope": "CarrierTypeParentPolicyLanePriorityBasis",
            }
            for lane in DEFERRED_PARENT_POLICY_LANES
        ],
        "candidate_lanes": lanes,
        "summary": {
            "carrier_type_remaining_lane_parked": 1,
            "component_authority_source_count": 0,
            "candidate_lane_count": len(lanes),
            "selection_eligible_lane_count": len(eligible),
            "deferred_parent_policy_lane_count": len(DEFERRED_PARENT_POLICY_LANES),
            "carrier_type_parent_policy_lanes_present": 1 if policy_inventory else 0,
        },
        "decision": {
            "kind": decision_kind,
            "reason_token": reason_token,
            "selected_lane": selected_lane,
            "selected_next_card": selected_next,
            "selected_carrier_type_axis": None,
            "selected_component_requirement": None,
            "selected_parent_policy_candidate": None,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "component_specific_card_selection": 0,
            "concrete_carrier_type_axis_selection": 0,
            "direct_parent_policy_candidate_selection": 0,
            "manual_lane_selection": 0,
            "hardcoded_lane_priority": 0,
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
        print("source-selfhost-wider-route-selection-basis-009 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

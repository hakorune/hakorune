#!/usr/bin/env python3
"""Select wider stop after MissingProjectionPolicy post-Type exhaustion."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-wider-route-selection-basis-011-v0.json"

TOKEN = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

POST_TYPE_RERUN = FIXTURES / "mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun-v0.json"
RERUN_005 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"
V4 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


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
    post_type = read_json(POST_TYPE_RERUN)
    rerun_005 = read_json(RERUN_005)
    summary = post_type.get("summary") or {}

    candidate_lanes = [
        candidate_lane(
            "NativeOwnerCheckpointRerun",
            "FreshCheckpointOrAdoptionDelta",
            False,
            [
                "native_owner_adoption_delta_count > 0",
                "or checkpoint_hash_stale = 1",
            ],
            "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-003",
        ),
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
            "BorrowSurfacePolicyLane",
            "IndependentBorrowBlockerAuthority",
            False,
            [
                "borrow_surface_policy_blocker_count = 1",
                "borrow_policy_fixture_proves_guard_clean = 1",
            ],
            "MIRBUILDER-BORROW-SURFACE-POLICY-BASIS-001",
        ),
        candidate_lane(
            "GuardConsolidation",
            "GuardActuallyBlocksNextLane",
            False,
            [
                "lane_guard_profile_missing_or_stale = 1",
                "guard_duplication_blocks_next_lane = 1",
            ],
            "MIRBUILDER-SOURCE-SELFHOST-LANE-GUARD-CONSOLIDATION-001",
        ),
    ]

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostWiderRouteSelectionBasis011V1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "post_type_exhaustion_selection_rerun": rel(POST_TYPE_RERUN),
            "missing_projection_policy_rerun_005": rel(RERUN_005),
            "missing_projection_policy_cluster_resolution_v4": rel(V4),
        },
        "provenance": {
            "post_type_exhaustion_selection_rerun_hash": sha256_file(POST_TYPE_RERUN),
            "missing_projection_policy_rerun_005_hash": sha256_file(RERUN_005),
            "missing_projection_policy_cluster_resolution_v4_hash": sha256_file(V4),
        },
        "previous_state": {
            "previous_decision": post_type.get("decision", {}).get("kind"),
            "previous_reason_token": post_type.get("decision", {}).get("reason_token"),
            "candidate_lane_count": summary.get("candidate_lane_count"),
            "selection_eligible_lane_count": summary.get("selection_eligible_lane_count"),
            "residual_owner_edge_shape_lane_selection_eligible": False,
            "type_only_projection_policy_lane_selection_eligible": False,
            "projection_descriptor_overlay_freshness_selection_eligible": False,
            "remaining_blocker_cluster_count": summary.get("remaining_blocker_cluster_count"),
            "remaining_blocker_candidate_count": summary.get("remaining_blocker_candidate_count"),
            "type_only_cluster_count": summary.get("type_only_cluster_count"),
            "type_only_candidate_count": summary.get("type_only_candidate_count"),
        },
        "parked_lanes": [
            {
                "lane_id": "MissingProjectionPolicyPostTypeTransportLane",
                "parked": True,
                "park_reason_token": "NoMachineDerivedMissingProjectionPolicyRerun005Lane",
                "projection_policy_selected": 0,
                "remaining_blocker_cluster_count": summary.get("remaining_blocker_cluster_count"),
                "remaining_blocker_candidate_count": summary.get("remaining_blocker_candidate_count"),
                "type_only_cluster_count": summary.get("type_only_cluster_count"),
                "type_only_candidate_count": summary.get("type_only_candidate_count"),
            }
        ],
        "selector_rule": {
            "name": "PostMissingProjectionPolicyPostTypeTransportExhaustionWiderSelectorV1",
            "park_missing_projection_post_type_lane_before_wider_selection": True,
            "projection_policy_selection_forbidden": True,
            "select_only_if_exactly_one_machine_derived_wider_lane": True,
            "keep_stopped_when_no_progress_lane_is_eligible": True,
            "forbidden_proof_axes": [
                "row_count",
                "cluster_size",
                "coverage_percentage",
                "source_path",
                "owner_name",
                "family_name",
                "route_membership_alone",
                "historical_preference",
                "lexical_order",
                "apparent_simplicity",
                "manual_lane_selection",
                "manual_family_selection",
                "manual_shape_selection",
                "manual_axis_selection",
                "self_signed_fixture",
            ],
        },
        "candidate_lanes": candidate_lanes,
        "summary": {
            "missing_projection_post_type_lane_parked": 1,
            "candidate_lane_count": len(candidate_lanes),
            "selection_eligible_progress_lane_count": 0,
            "keep_stopped": 1,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedPostMissingProjectionPolicyWiderLane",
            "selected_lane": None,
            "selected_next_card": DESIGN_STOP,
            "selected_projection_policy_cluster": None,
        },
        "claims": {
            "projection_policy_selected": 0,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "row_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "historical_preference_as_proof": 0,
            "basis_010_exactly_one_wider_lane_as_projection_policy_proof": 0,
            "type_transport_exhausted_as_projection_policy_proof": 0,
            "type_only_cluster_direct_selection": 0,
            "owner_edge_repair_as_projection_policy_proof": 0,
            "shape_signature_inventory_as_projection_policy_proof": 0,
            "residual_blocker_count_as_root_proof": 0,
            "type_only_cluster_count_as_root_proof": 0,
            "freshness_rerun_as_semantic_priority": 0,
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
        print("source-selfhost-wider-route-selection-basis-011 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

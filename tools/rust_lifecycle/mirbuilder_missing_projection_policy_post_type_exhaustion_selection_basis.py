#!/usr/bin/env python3
"""Define MissingProjectionPolicy selector after TypeTransport exhaustion."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-post-type-exhaustion-selection-basis-v0.json"

TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-BASIS-001"
NEXT = "MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

RERUN_005 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"
V4 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"
BASIS_010 = FIXTURES / "source-selfhost-wider-route-selection-basis-010-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def candidate_lane(
    lane_id: str,
    scope: str,
    cluster_count: int | None,
    row_count: int | None,
    required_authority: list[str],
    selected_next: str | None,
) -> dict[str, Any]:
    row: dict[str, Any] = {
        "lane_id": lane_id,
        "scope": scope,
        "selection_eligible": False,
        "required_authority": required_authority,
    }
    if cluster_count is not None:
        row["candidate_cluster_count"] = cluster_count
    if row_count is not None:
        row["candidate_row_count"] = row_count
    if selected_next is not None:
        row["selected_next_card_if_eligible"] = selected_next
    return row


def build_fixture() -> dict[str, Any]:
    rerun_005 = read_json(RERUN_005)
    inventory = rerun_005.get("post_type_transport_inventory") or {}

    candidate_lanes = [
        candidate_lane(
            "ResidualOwnerEdgeAndShapeSignatureBlockerInventory",
            "remaining blocker clusters",
            inventory.get("remaining_blocker_cluster_count"),
            inventory.get("remaining_blocker_candidate_count"),
            [
                "residual clusters are exactly owner-edge/shape blockers",
                "repair lane has stable fixture input",
                "does not select projection policy directly",
            ],
            "MIRBUILDER-MISSING-PROJECTION-POLICY-RESIDUAL-OWNER-EDGE-SHAPE-BLOCKER-INVENTORY-001",
        ),
        candidate_lane(
            "TypeOnlyProjectionPolicySelectorBasis",
            "type-only clusters after TypeTransport parked",
            inventory.get("type_only_cluster_count"),
            inventory.get("type_only_candidate_count"),
            [
                "type-only clusters are not directly selectable",
                "selector basis must define proof tuple before policy selection",
            ],
            "MIRBUILDER-MISSING-PROJECTION-POLICY-TYPE-ONLY-CLUSTER-SELECTION-BASIS-001",
        ),
        candidate_lane(
            "ProjectionDescriptorOverlayFreshnessRerun",
            "projection descriptor overlay freshness",
            None,
            None,
            [
                "overlay_or_descriptor_fixture_hash_stale",
                "freshness delta affects projection-policy cluster selection",
            ],
            "MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-RERUN-002",
        ),
        candidate_lane(
            "KeepStopped",
            "no exactly-one lane",
            None,
            None,
            ["no exactly-one lane"],
            None,
        ),
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyPostTypeExhaustionSelectionBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "missing_projection_policy_rerun_005": rel(RERUN_005),
            "missing_projection_policy_cluster_resolution_v4": rel(V4),
            "wider_route_selection_basis_010": rel(BASIS_010),
        },
        "provenance": {
            "missing_projection_policy_rerun_005_hash": sha256_file(RERUN_005),
            "missing_projection_policy_cluster_resolution_v4_hash": sha256_file(V4),
            "wider_route_selection_basis_010_hash": sha256_file(BASIS_010),
        },
        "selector_rule": {
            "name": "MissingProjectionPolicyPostTypeExhaustionSelectorV1",
            "basis_selects_projection_policy": False,
            "type_transport_missing_is_parked_not_deleted": True,
            "selection_requires_exactly_one_machine_derived_lane_or_card": True,
            "if_zero_or_multiple_keep_stopped": True,
            "allowed_proof_axes": [
                "parked_lane_reason_tokens",
                "v4_cluster_blocker_inventory",
                "post_type_transport_blocker_inventory",
                "prior_overlay_or_descriptor_fixture_freshness_if_hash_consumed",
                "exactly_one_machine_derived_candidate",
            ],
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
            "remaining_blocker_cluster_count": inventory.get("remaining_blocker_cluster_count"),
            "remaining_blocker_candidate_count": inventory.get("remaining_blocker_candidate_count"),
            "type_only_cluster_count": inventory.get("type_only_cluster_count"),
            "type_only_candidate_count": inventory.get("type_only_candidate_count"),
            "selection_eligible_lane_count": 0,
        },
        "decision": {
            "kind": "SelectPostTypeExhaustionSelectionRerun",
            "reason_token": "MissingProjectionPolicyPostTypeExhaustionSelectorBasisDefined",
            "selected_lane": None,
            "selected_next_card": NEXT,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "new_projection_policy_selected": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "manual_lane_selection": 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "row_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "family_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "historical_preference_as_proof": 0,
            "self_signed_fixture_as_proof": 0,
            "basis_010_exactly_one_wider_lane_as_projection_policy_proof": 0,
            "type_transport_exhausted_as_projection_policy_proof": 0,
            "type_only_cluster_direct_selection": 0,
            "owner_edge_repair_as_projection_policy_proof": 0,
            "shape_signature_inventory_as_projection_policy_proof": 0,
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
        print("mirbuilder-missing-projection-policy-post-type-exhaustion-selection-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Record post-TypeTransport MissingProjectionPolicy inventory after carrier/type exhaustion."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from mirbuilder_crate_wide_missing_projection_policy_cluster_resolution import build_resolution


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"

TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-RERUN-005"
NEXT = "MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

V4 = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v4-v0.json"
BASIS_010 = FIXTURES / "source-selfhost-wider-route-selection-basis-010-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def classify_clusters(clusters: list[dict[str, Any]]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    residual: list[dict[str, Any]] = []
    type_only: list[dict[str, Any]] = []
    for cluster in clusters:
        blocked_by = set(cluster.get("blocked_by") or [])
        post_type_blockers = sorted(blocked_by - {"TypeTransportMissing"})
        row = {
            "cluster_id": cluster.get("cluster_id"),
            "candidate_count": cluster.get("candidate_count"),
            "owner_edge_confidence": cluster.get("owner_edge_confidence"),
            "shape_signature": cluster.get("shape_signature"),
            "type_transport_axis": cluster.get("type_transport_axis"),
            "v4_blocked_by": sorted(blocked_by),
            "post_type_blocked_by": post_type_blockers,
        }
        if post_type_blockers:
            residual.append(row)
        elif "TypeTransportMissing" in blocked_by:
            type_only.append(row)

    residual.sort(key=lambda item: (-int(item["candidate_count"]), str(item["cluster_id"])))
    type_only.sort(key=lambda item: (-int(item["candidate_count"]), str(item["cluster_id"])))
    return residual, type_only


def build_fixture() -> dict[str, Any]:
    v4 = read_json(V4)
    basis_010 = read_json(BASIS_010)
    resolution = build_resolution()
    clusters = resolution.get("clusters") or []
    summary = resolution.get("summary") or {}
    residual, type_only = classify_clusters(clusters)
    parked_lanes = basis_010.get("parked_lanes") or []

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyClusterResolutionRerun005V1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "missing_projection_policy_cluster_resolution_v4": rel(V4),
            "wider_route_selection_basis_010": rel(BASIS_010),
        },
        "provenance": {
            "missing_projection_policy_cluster_resolution_v4_hash": sha256_file(V4),
            "wider_route_selection_basis_010_hash": sha256_file(BASIS_010),
        },
        "type_transport_exhaustion_state": {
            "TypeTransportMissing_treated_as": "ParkedExhausted",
            "basis_010_reason_token": basis_010.get("decision", {}).get("reason_token"),
            "parked_lanes": parked_lanes,
            "type_transport_missing_silently_deleted": False,
        },
        "post_type_transport_inventory": {
            "input_candidate_count": summary.get("input_candidate_count"),
            "input_cluster_count": summary.get("cluster_count"),
            "type_transport_missing_cluster_count_from_v4": v4.get("cluster_state", {}).get(
                "type_transport_missing_cluster_count"
            ),
            "type_transport_missing_treated_as_parked_count": v4.get("cluster_state", {}).get(
                "type_transport_missing_cluster_count"
            ),
            "remaining_blocker_cluster_count": len(residual),
            "remaining_blocker_candidate_count": sum(int(row["candidate_count"]) for row in residual),
            "remaining_blocker_classes": [
                "NoExactOrFixtureMappedOwnerEdge",
                "MissingShapeSignatureClusterAxis",
            ],
            "remaining_blocker_clusters": residual,
            "type_only_cluster_count": len(type_only),
            "type_only_candidate_count": sum(int(row["candidate_count"]) for row in type_only),
            "type_only_cluster_samples": type_only[:10],
            "type_only_clusters_are_directly_selectable": False,
        },
        "selector_state": {
            "post_type_exhaustion_selector_defined": False,
            "exactly_one_machine_derived_projection_policy_lane": False,
            "selected_projection_policy_cluster": None,
            "selected_projection_policy_card": None,
            "basis_010_exactly_one_wider_lane_as_projection_policy_proof": 0,
            "type_transport_exhausted_as_projection_policy_proof": 0,
            "type_only_cluster_direct_selection": 0,
            "owner_edge_repair_as_projection_policy_proof": 0,
            "shape_signature_inventory_as_projection_policy_proof": 0,
        },
        "decision": {
            "kind": "SelectPostTypeExhaustionSelectionBasis",
            "reason_token": "PostTypeTransportExhaustionSelectorBasisRequired",
            "selected_cluster_id": None,
            "selected_next_card": NEXT,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "new_projection_policy_selected": 0,
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
        print("mirbuilder-missing-projection-policy-cluster-resolution-rerun-005 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

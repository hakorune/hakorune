#!/usr/bin/env python3
"""Inventory carrier/type transport policy candidates before strict emission."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
INPUT = FIXTURES / "mirbuilder-multi-axis-diagnostic-cluster-resolution-v0.json"
OTHER_SHAPE = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-v0.json"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-policy-inventory-v0.json"
TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-001"
NEXT = "MIRBUILDER-STRICT-CONVERTER-EMISSION-PROBE-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def lane_for(cluster: dict[str, Any]) -> str:
    type_axes = set(cluster.get("type_transport_axes") or [])
    return_families = set(cluster.get("return_families") or [])
    blocked_by = set(cluster.get("blocked_by") or [])

    if "ResultCarrierNeedsVerifier" in type_axes:
        return "ResultCarrierVerifierPolicyCandidate"
    if "ReturnedIteratorNeedsPolicy" in type_axes:
        return "ReturnedIteratorPolicyCandidate"
    if "MissingTypeTransport" in type_axes:
        return "MissingTypeTransportPolicyCandidate"
    if "ConstructorCarrier" in type_axes:
        return "ConstructorCarrierPolicyCandidate"
    if "KnownOptionCarrier" in type_axes:
        return "OptionCarrierPolicyCandidate"
    if "KnownVecCarrier" in type_axes:
        return "VecCarrierPolicyCandidate"
    if "CarrierPolicyGap" in blocked_by:
        return "GenericCarrierPolicyCandidate"
    if return_families == {"unit"}:
        return "NoCarrierTypePolicyNeeded"
    return "KnownTypeTransportNoCarrierPolicy"


def build_inventory() -> dict[str, Any]:
    resolution = read_json(INPUT)
    other_shape = read_json(OTHER_SHAPE)
    clusters = other_shape["clusters"]

    transport_rows = []
    lane_counts: Counter[str] = Counter()
    lane_candidate_counts: Counter[str] = Counter()
    return_counts: Counter[str] = Counter()
    type_axis_counts: Counter[str] = Counter()

    for cluster in clusters:
        lane = lane_for(cluster)
        candidate_count = int(cluster.get("candidate_count", 0))
        lane_counts[lane] += 1
        lane_candidate_counts[lane] += candidate_count
        for family in cluster.get("return_families") or []:
            return_counts[family] += candidate_count
        for axis in cluster.get("type_transport_axes") or []:
            type_axis_counts[axis] += candidate_count
        transport_rows.append(
            {
                "shape_signature": cluster.get("shape_signature"),
                "candidate_count": candidate_count,
                "return_families": cluster.get("return_families") or [],
                "type_transport_axes": cluster.get("type_transport_axes") or [],
                "blocked_by": cluster.get("blocked_by") or [],
                "policy_lane_candidate": lane,
                "policy_selected": False,
            }
        )

    policy_lanes = [
        {
            "lane": lane,
            "cluster_count": lane_counts[lane],
            "candidate_count": lane_candidate_counts[lane],
            "selection_eligible": False,
            "reason_token": "InventoryOnlyPolicyLaneCandidate",
        }
        for lane in sorted(lane_counts)
        if lane not in {"NoCarrierTypePolicyNeeded", "KnownTypeTransportNoCarrierPolicy"}
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportPolicyInventoryV1",
        "token": TOKEN,
        "input_state": {
            "multi_axis_diagnostic_cluster_resolution": rel(INPUT),
            "other_shape_signature_cluster_resolution": rel(OTHER_SHAPE),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "multi_axis_diagnostic_cluster_resolution_hash": sha256_file(INPUT),
            "other_shape_signature_cluster_resolution_hash": sha256_file(OTHER_SHAPE),
        },
        "transport_rows": transport_rows,
        "policy_lane_candidates": policy_lanes,
        "summary": {
            "input_shape_cluster_count": len(clusters),
            "carrier_type_transport_candidate_count": resolution["resolved_axis_summary"]["carrier_type_transport_candidate_count"],
            "return_family_candidate_counts": dict(sorted(return_counts.items())),
            "type_transport_axis_candidate_counts": dict(sorted(type_axis_counts.items())),
            "policy_lane_candidate_counts": dict(sorted(lane_candidate_counts.items())),
            "policy_lane_selected_count": 0,
        },
        "decision": {
            "kind": "SelectStrictConverterEmissionProbe",
            "reason_token": "CarrierTypeTransportPolicyInventoryRecorded",
            "selected_next_card": NEXT,
        },
        "claims": {
            "multi_axis_resolution_consumed": 1,
            "other_shape_resolution_consumed": 1,
            "carrier_type_transport_inventory_ready": 1,
            "policy_lane_selected": 0,
            "manual_carrier_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in inventory fixture.")
    args = parser.parse_args()

    output = stable_json(build_inventory())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-carrier-type-transport-policy-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

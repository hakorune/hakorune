#!/usr/bin/env python3
"""Resolve pure BridgeBlocked policy gaps into repair clusters."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed
from mirbuilder_strict_converter_emission_native_seed_candidate_selection import build_fixture as build_selection


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-bridge-blocked-gap-cluster-resolution-v0.json"

TOKEN = "MIRBUILDER-BRIDGE-BLOCKED-GAP-CLUSTER-RESOLUTION-001"
AXIS_RESOLUTION = FIXTURES / "source-selfhost-bridge-blocked-reason-axis-resolution-v0.json"
RERUN_007 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-007-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def gap_signature(item: dict[str, Any]) -> str:
    borrow = bool(item.get("borrow_policy_gap"))
    carrier = bool(item.get("carrier_type_transport_gap"))
    if carrier and not borrow:
        return "CarrierTypeTransportGapOnly"
    if carrier and borrow:
        return "BorrowAndCarrierTypeTransportGap"
    if borrow:
        return "BorrowPolicyGapOnly"
    return "UnknownPolicyGap"


def build_fixture() -> dict[str, Any]:
    axis_resolution = read_json(AXIS_RESOLUTION)
    rerun = read_json(RERUN_007)
    selection = build_selection(cutoff_token="MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-007")
    candidates = [
        item for item in selection.get("candidates", [])
        if item.get("blocked_by") == ["PolicyGapInDeniedBoundaries"]
    ]
    counts = Counter(gap_signature(item) for item in candidates)

    clusters = [
        {
            "cluster_id": "bridge_gap::carrier_type_transport_only",
            "gap_signature": "CarrierTypeTransportGapOnly",
            "candidate_count": counts.get("CarrierTypeTransportGapOnly", 0),
            "selection_eligible": counts.get("CarrierTypeTransportGapOnly", 0) > 0,
            "reason_token": "CarrierTypeTransportGapOnlyDominatesPurePolicyGapAxis",
            "next_owner_kind": "CarrierTypeTransportPolicyInventoryRerun",
            "next_card": "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-001",
        },
        {
            "cluster_id": "bridge_gap::borrow_and_carrier_type_transport",
            "gap_signature": "BorrowAndCarrierTypeTransportGap",
            "candidate_count": counts.get("BorrowAndCarrierTypeTransportGap", 0),
            "selection_eligible": False,
            "reason_token": "BorrowCarrierMixedGapDeferredUntilSingleAxisGapsClose",
            "next_owner_kind": "MixedGapResolution",
            "next_card": "MIRBUILDER-BRIDGE-BLOCKED-MIXED-GAP-RESOLUTION-001",
        },
        {
            "cluster_id": "bridge_gap::borrow_policy_only",
            "gap_signature": "BorrowPolicyGapOnly",
            "candidate_count": counts.get("BorrowPolicyGapOnly", 0),
            "selection_eligible": False,
            "reason_token": "NoPureBorrowPolicyGapCandidate",
            "next_owner_kind": "BorrowPolicyGapResolution",
            "next_card": "MIRBUILDER-BRIDGE-BLOCKED-BORROW-GAP-RESOLUTION-001",
        },
    ]
    eligible = [cluster for cluster in clusters if cluster["selection_eligible"]]
    if len(eligible) == 1:
        selected = eligible[0]
        decision = {
            "kind": "SelectCarrierTypeTransportPolicyInventoryRerun",
            "reason_token": "ExactlyOneBridgeBlockedGapClusterEligible",
            "selected_cluster_id": selected["cluster_id"],
            "selected_next_card": selected["next_card"],
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoUniqueBridgeBlockedGapCluster",
            "selected_cluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderBridgeBlockedGapClusterResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "bridge_blocked_reason_axis_resolution": rel(AXIS_RESOLUTION),
            "native_owner_seed_rerun_007": rel(RERUN_007),
        },
        "provenance": {
            "bridge_blocked_reason_axis_resolution_hash": sha256_file(AXIS_RESOLUTION),
            "native_owner_seed_rerun_007_hash": sha256_file(RERUN_007),
        },
        "input_decision": axis_resolution["decision"],
        "input_candidate_pool": rerun["candidate_pool"],
        "pure_policy_gap_candidate_count": len(candidates),
        "gap_clusters": clusters,
        "selection_rule": {
            "select_single_axis_gap_before_mixed_gap": True,
            "select_carrier_type_transport_gap_if_unique": True,
            "defer_mixed_borrow_carrier_gap": True,
            "cluster_size_as_proof": False,
            "manual_cluster_selection": False,
        },
        "decision": decision,
        "claims": {
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_cluster_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-bridge-blocked-gap-cluster-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

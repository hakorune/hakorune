#!/usr/bin/env python3
"""Select the next carrier/type transport policy lane by stable priority."""

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
INPUT = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-v0.json"
OUTPUT = FIXTURES / "mirbuilder-carrier-type-transport-policy-lane-priority-resolution-v0.json"

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-LANE-PRIORITY-RESOLUTION-001"
NEXT = "MIRBUILDER-RESULT-CARRIER-VERIFIER-POLICY-001"

LANE_PRIORITY = [
    "ResultCarrierVerifierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "VecOrArrayCarrierPolicyCandidate",
    "GenericCarrierPolicyCandidate",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build_fixture() -> dict[str, Any]:
    evidence = read_json(INPUT)
    rows = evidence.get("evidence_rows", [])
    lane_counts: Counter[str] = Counter(row["normalized_policy_lane_candidate"] for row in rows)
    lane_rows: dict[str, list[dict[str, Any]]] = {
        lane: [row for row in rows if row["normalized_policy_lane_candidate"] == lane]
        for lane in sorted(lane_counts)
    }

    lanes = []
    for lane in sorted(lane_counts):
        priority_index = LANE_PRIORITY.index(lane) if lane in LANE_PRIORITY else None
        lanes.append(
            {
                "lane": lane,
                "candidate_count": lane_counts[lane],
                "priority_index": priority_index,
                "selection_eligible": priority_index is not None,
                "candidate_owner_edges": sorted(row["owner_edge_id"] for row in lane_rows[lane]),
            }
        )

    eligible = [lane for lane in lanes if lane["selection_eligible"]]
    selected = min(eligible, key=lambda lane: (lane["priority_index"], lane["lane"])) if eligible else None
    if selected:
        decision = {
            "kind": "SelectCarrierTypeTransportPolicyLane",
            "reason_token": "StableCarrierTypeTransportLanePrioritySelected",
            "selected_policy_lane": selected["lane"],
            "selected_next_card": NEXT,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoCarrierTypeTransportPolicyLaneEligible",
            "selected_policy_lane": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportPolicyLanePriorityResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "carrier_type_transport_evidence_inventory": rel(INPUT),
        },
        "provenance": {
            "carrier_type_transport_evidence_inventory_hash": sha256_file(INPUT),
        },
        "lane_priority": LANE_PRIORITY,
        "policy_lanes": lanes,
        "summary": {
            "input_candidate_count": len(rows),
            "policy_lane_count": len(lanes),
            "eligible_policy_lane_count": len(eligible),
            "known_type_transport_no_policy_count": lane_counts.get("KnownTypeTransportNoCarrierPolicy", 0),
            "selected_policy_lane_candidate_count": selected["candidate_count"] if selected else 0,
        },
        "selection_rule": {
            "use_precedent_lane_order": True,
            "exclude_known_type_transport_no_policy_from_policy_lane_selection": True,
            "cluster_size_as_proof": False,
            "manual_carrier_selection": False,
            "owner_name_as_transport_policy": False,
        },
        "decision": decision,
        "claims": {
            "carrier_type_transport_evidence_inventory_consumed": 1,
            "policy_lane_priority_resolution_ready": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "owner_name_as_transport_policy": 0,
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
        print("mirbuilder-carrier-type-transport-policy-lane-priority-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

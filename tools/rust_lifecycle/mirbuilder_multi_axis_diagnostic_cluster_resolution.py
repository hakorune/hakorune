#!/usr/bin/env python3
"""Resolve the remaining multi-axis diagnostic clusters into the next lane."""

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
INVENTORY = FIXTURES / "mirbuilder-converter-completion-task-inventory-v0.json"
SOURCE_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
OTHER_REPAIR = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json"
OTHER_SHAPE = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-v0.json"
OUTPUT = FIXTURES / "mirbuilder-multi-axis-diagnostic-cluster-resolution-v0.json"
TOKEN = "MIRBUILDER-MULTI-AXIS-DIAGNOSTIC-CLUSTER-RESOLUTION-001"
NEXT = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sum_candidate_count(clusters: list[dict[str, Any]], axis_names: set[str]) -> int:
    return sum(
        int(cluster.get("candidate_count", 0))
        for cluster in clusters
        if axis_names.intersection(cluster.get("blocked_by") or [])
    )


def build_resolution() -> dict[str, Any]:
    inventory = read_json(INVENTORY)
    report = read_json(SOURCE_REPORT)
    repair = read_json(OTHER_REPAIR)
    other_shape = read_json(OTHER_SHAPE)

    clusters = other_shape.get("clusters", [])
    blocked_axis_counter: Counter[str] = Counter()
    for cluster in clusters:
        blocked_axis_counter.update(cluster.get("blocked_by") or [])

    source_items = report.get("items", [])
    report_multi_axis_count = len([
        item for item in source_items
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("owner_edge_confidence") == "None"
        and item.get("stable_deny_reason") == "OwnerEdgeConfidenceMissing"
    ])

    carrier_type_count = sum_candidate_count(
        clusters, {"CarrierPolicyGap", "TypeTransportOrVerifierGap"}
    )
    borrow_count = sum_candidate_count(clusters, {"BorrowOrReceiverPolicyGap"})
    eligible_shape_count = other_shape.get("summary", {}).get("selection_eligible_shape_count", 0)

    if eligible_shape_count:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "UnexpectedEligibleShapeSignatureRequiresProjectionDescriptor",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }
    elif carrier_type_count:
        decision = {
            "kind": "SelectCarrierTypeTransportPolicyInventory",
            "reason_token": "MultiAxisClustersBlockedByCarrierTypeTransportPolicy",
            "selected_next_card": NEXT,
        }
    elif borrow_count:
        decision = {
            "kind": "SelectBorrowOrReceiverPolicyGapResolution",
            "reason_token": "MultiAxisClustersBlockedOnlyByBorrowOrReceiverPolicy",
            "selected_next_card": "MIRBUILDER-BORROW-OR-RECEIVER-POLICY-GAP-RESOLUTION-001",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedMultiAxisDiagnosticLane",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMultiAxisDiagnosticClusterResolutionV1",
        "token": TOKEN,
        "input_state": {
            "converter_completion_inventory": rel(INVENTORY),
            "unconverted_surface_report": rel(SOURCE_REPORT),
            "other_owner_edge_confidence_repair": rel(OTHER_REPAIR),
            "other_shape_signature_cluster_resolution": rel(OTHER_SHAPE),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "converter_completion_inventory_hash": sha256_file(INVENTORY),
            "unconverted_surface_report_hash": sha256_file(SOURCE_REPORT),
            "other_owner_edge_confidence_repair_hash": sha256_file(OTHER_REPAIR),
            "other_shape_signature_cluster_resolution_hash": sha256_file(OTHER_SHAPE),
        },
        "input_counts": {
            "inventory_needs_multiple_diagnostic_axes_count": inventory["diagnostic_counts"]["needs_multiple_diagnostic_axes_count"],
            "source_report_owner_edge_missing_count": report_multi_axis_count,
            "other_repair_input_other_owner_cluster_count": repair["claims"]["input_other_owner_cluster_count"],
            "other_shape_input_other_owner_cluster_count": other_shape["summary"]["input_other_owner_cluster_count"],
            "other_shape_input_shape_signature_count": other_shape["summary"]["input_shape_signature_count"],
            "other_shape_selection_eligible_shape_count": eligible_shape_count,
        },
        "resolved_axis_summary": {
            "blocked_axis_cluster_counts": dict(sorted(blocked_axis_counter.items())),
            "carrier_type_transport_candidate_count": carrier_type_count,
            "borrow_or_receiver_candidate_count": borrow_count,
            "completed_shape_signature_count": other_shape["summary"]["completed_shape_signature_count"],
        },
        "selection_rules": {
            "consume_existing_other_decomposition": 1,
            "shape_descriptor_candidate_wins_before_policy_inventory": 1,
            "carrier_type_transport_inventory_before_borrow_gap_when_present": 1,
            "cluster_size_as_proof": 0,
            "manual_axis_selection": 0,
        },
        "decision": decision,
        "claims": {
            "converter_completion_inventory_consumed": 1,
            "source_report_consumed": 1,
            "other_owner_edge_repair_consumed": 1,
            "other_shape_signature_resolution_consumed": 1,
            "multi_axis_clusters_resolved_to_next_lane": 1,
            "manual_family_selection": 0,
            "manual_axis_selection": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in resolution fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-multi-axis-diagnostic-cluster-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Audit owner-cluster fields in the crate-wide unconverted surface report."""

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
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-surface-report-owner-cluster-field-v0.json"
TOKEN = "MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def missing_projection_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        item for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
    ]


def build_fixture() -> dict[str, Any]:
    report = read_json(REPORT)
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    priority = read_json(PRIORITY)
    items = report.get("items", [])
    missing_projection = missing_projection_items(report)

    required_fields = [
        "known_owner_edge",
        "owner_edge_confidence",
        "likely_owner_cluster",
        "classification",
        "reason_token",
        "source_id",
        "source_path",
        "symbol",
    ]
    field_presence = {
        field: sum(1 for item in items if field in item)
        for field in required_fields
    }
    likely_cluster_counts = Counter(item.get("likely_owner_cluster") for item in items)
    missing_projection_cluster_counts = Counter(item.get("likely_owner_cluster") for item in missing_projection)
    known_owner_missing_counts = Counter(item.get("classification") for item in items if not item.get("known_owner_edge"))
    owner_confidence_counts = Counter(item.get("owner_edge_confidence") for item in items)

    field_gaps: list[dict[str, Any]] = []
    for field, count in sorted(field_presence.items()):
        if count != len(items):
            field_gaps.append({
                "field": field,
                "present_count": count,
                "expected_count": len(items),
                "reason_token": "RequiredOwnerClusterFieldMissing",
            })

    residual_owner_clusters = [
        {
            "cluster": cluster,
            "count": count,
            "reason_token": (
                "OtherOwnerClusterFieldRequiresDecomposition"
                if cluster == "OtherMissingProjectionPolicyCluster"
                else "KnownOwnerClusterStillUnresolved"
            ),
        }
        for cluster, count in sorted(
            missing_projection_cluster_counts.items(),
            key=lambda item: (-item[1], item[0] or ""),
        )
        if cluster in {"OtherMissingProjectionPolicyCluster", "JoinIRRouteRegistryCluster", "FastMemCluster"}
    ]

    if field_gaps:
        decision = {
            "kind": "SelectReportFieldRepair",
            "selected_next_card": "MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-REPAIR-001",
            "reason_token": "RequiredOwnerClusterFieldMissing",
        }
    elif residual_owner_clusters and residual_owner_clusters[0]["cluster"] == "OtherMissingProjectionPolicyCluster":
        decision = {
            "kind": "SelectOtherOwnerClusterDecomposition",
            "selected_cluster": "OtherMissingProjectionPolicyCluster",
            "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-001",
            "reason_token": "OtherOwnerClusterFieldRequiresDecomposition",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_cluster": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoOwnerClusterFieldRepairCandidate",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideSurfaceReportOwnerClusterFieldV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "priority_resolution": rel(PRIORITY),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reported_item_count": len(items),
            "missing_projection_policy_count": len(missing_projection),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "priority_resolution_hash": sha256_file(PRIORITY),
        },
        "field_presence": field_presence,
        "field_gaps": field_gaps,
        "owner_field_summary": {
            "likely_owner_cluster_counts": dict(sorted(likely_cluster_counts.items())),
            "missing_projection_cluster_counts": dict(sorted(missing_projection_cluster_counts.items())),
            "known_owner_edge_missing_by_classification": dict(sorted(known_owner_missing_counts.items())),
            "owner_edge_confidence_counts": dict(sorted(owner_confidence_counts.items())),
        },
        "residual_owner_clusters": residual_owner_clusters,
        "decision": decision,
        "claims": {
            "source_report_consumed": 1,
            "projection_cluster_resolution_consumed": 1,
            "projection_priority_consumed": 1,
            "owner_cluster_field_audited": 1,
            "likely_owner_cluster_present_for_every_item": 1 if field_presence["likely_owner_cluster"] == len(items) else 0,
            "owner_edge_confidence_present_for_every_item": 1 if field_presence["owner_edge_confidence"] == len(items) else 0,
            "known_owner_edge_field_present_for_every_item": 1 if field_presence["known_owner_edge"] == len(items) else 0,
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_edit_authority": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_source_seed_materialization": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in owner-cluster field fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-crate-wide-surface-report-owner-cluster-field unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

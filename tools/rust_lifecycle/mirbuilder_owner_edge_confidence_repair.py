#!/usr/bin/env python3
"""Derive owner-edge confidence repair rules from source-surface clusters.

This tool materializes a diagnostic mapping from stable `likely_owner_cluster`
labels to synthetic owner-edge ids. It does not choose a family, emit Hako,
define projection policy, or claim Source Selfhost.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-owner-edge-confidence-repair-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def owner_edge_id_for_cluster(cluster: str) -> str:
    body = re.sub(r"Cluster$", "", cluster)
    body = re.sub(r"(?<!^)(?=[A-Z])", "_", body).lower()
    return f"mirbuilder::{body}"


def build_repair() -> dict[str, Any]:
    report = read_json(REPORT)
    missing_items = [
        item for item in report["items"]
        if item["classification"] == "MissingProjectionPolicy"
    ]

    cluster_counts: dict[str, int] = {}
    for item in missing_items:
        cluster = item.get("likely_owner_cluster") or "OtherMissingProjectionPolicyCluster"
        cluster_counts[cluster] = cluster_counts.get(cluster, 0) + 1

    mappings: list[dict[str, Any]] = []
    denied: list[dict[str, Any]] = []
    for cluster, count in sorted(cluster_counts.items()):
        if cluster.startswith("Other"):
            denied.append({
                "likely_owner_cluster": cluster,
                "candidate_count": count,
                "owner_edge_confidence": "None",
                "reason_token": "OtherClusterRequiresOwnerEdgeClassification",
                "selected": False,
            })
            continue
        mappings.append({
            "likely_owner_cluster": cluster,
            "owner_edge_id": owner_edge_id_for_cluster(cluster),
            "owner_edge_confidence": "FixtureMapped",
            "candidate_count": count,
            "reason_token": "ClusterLabelMappedToOwnerEdgeNamespace",
            "selected": True,
        })

    mapped_count = sum(item["candidate_count"] for item in mappings)
    denied_count = sum(item["candidate_count"] for item in denied)

    return {
        "schema_version": 0,
        "kind": "MirBuilderOwnerEdgeConfidenceRepairV1",
        "token": "MIRBUILDER-OWNER-EDGE-CONFIDENCE-REPAIR-001",
        "input_state": {
            "source_report": rel(REPORT),
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "input_missing_projection_policy_count": len(missing_items),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "source_cluster_counts_hash": sha256_text(stable_json(cluster_counts)),
        },
        "mapping_policy": {
            "source_axis": "likely_owner_cluster",
            "confidence_assigned": "FixtureMapped",
            "other_cluster_selectable": 0,
            "manual_family_selection": 0,
            "family_name_based_policy": 0,
            "cluster_size_as_proof": 0,
        },
        "cluster_mappings": mappings,
        "denied_clusters": denied,
        "summary": {
            "input_candidate_count": len(missing_items),
            "mapped_cluster_count": len(mappings),
            "mapped_candidate_count": mapped_count,
            "denied_cluster_count": len(denied),
            "denied_candidate_count": denied_count,
            "fixture_mapped_candidate_count_after_repair": mapped_count,
            "none_candidate_count_after_repair": denied_count,
        },
        "decision": {
            "kind": "ApplyOwnerEdgeConfidenceRepair",
            "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001",
            "reason_token": "OwnerEdgeConfidenceRepairProducesFixtureMappedClusters",
        },
        "claims": {
            "input_missing_projection_policy_count": len(missing_items),
            "owner_edge_confidence_repair_defined": 1,
            "fixture_mapped_candidate_count_after_repair": mapped_count,
            "none_candidate_count_after_repair": denied_count,
            "other_cluster_not_selectable": 1,
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_edit_authority": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "family_name_based_policy": 0,
            "hako_emission": 0,
            "hako_adopted_decision": 0,
            "native_source_seed_materialization": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in repair fixture.")
    args = parser.parse_args()

    output = stable_json(build_repair())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-owner-edge-confidence-repair unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

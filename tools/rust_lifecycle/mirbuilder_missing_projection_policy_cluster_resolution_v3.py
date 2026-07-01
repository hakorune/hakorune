#!/usr/bin/env python3
"""Resolve remaining MissingProjectionPolicy clusters after coverage reclassification."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from mirbuilder_crate_wide_missing_projection_policy_cluster_resolution import build_resolution


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-cluster-resolution-v3-v0.json"

TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V3"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_TRANSPORT = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-002"
CHECKPOINT_RERUN = FIXTURES / "source-selfhost-native-owner-checkpoint-rerun-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def blocked_reason_counts(clusters: list[dict[str, Any]]) -> Counter[str]:
    counts: Counter[str] = Counter()
    for cluster in clusters:
        for reason in cluster.get("blocked_by") or []:
            counts[reason] += 1
    return counts


def build_fixture() -> dict[str, Any]:
    checkpoint = read_json(CHECKPOINT_RERUN)
    report = read_json(REPORT)
    resolution = build_resolution()
    clusters = resolution.get("clusters") or []
    summary = resolution.get("summary") or {}
    eligible = [cluster for cluster in clusters if cluster.get("selection_eligible") is True]
    blocked_counts = blocked_reason_counts(clusters)
    type_transport_missing = blocked_counts.get("TypeTransportMissing", 0)

    if not eligible and type_transport_missing:
        decision = {
            "kind": "SelectCarrierTypeTransportPolicyInventoryRerun002",
            "reason_token": "TypeTransportMissingBlocksProjectionPolicyClusters",
            "selected_next_card": NEXT_TRANSPORT,
            "selected_cluster_id": None,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedMissingProjectionPolicyV3Lane",
            "selected_next_card": DESIGN_STOP,
            "selected_cluster_id": None,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyClusterResolutionV3",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "native_owner_checkpoint_rerun": rel(CHECKPOINT_RERUN),
            "unconverted_surface_report": rel(REPORT),
        },
        "provenance": {
            "native_owner_checkpoint_rerun_hash": sha256_file(CHECKPOINT_RERUN),
            "unconverted_surface_report_hash": sha256_file(REPORT),
        },
        "checkpoint_decision": checkpoint.get("decision"),
        "report_state": {
            "missing_projection_policy_count": (report.get("summary") or {}).get("missing_projection_policy_count"),
            "projection_descriptor_coverage_reclassified_count": (report.get("summary") or {}).get("projection_descriptor_coverage_reclassified_count"),
            "borrow_policy_needed_count": (report.get("summary") or {}).get("borrow_policy_needed_count"),
        },
        "cluster_state": {
            "input_candidate_count": summary.get("input_candidate_count"),
            "cluster_count": summary.get("cluster_count"),
            "selection_eligible_cluster_count": len(eligible),
            "fixture_mapped_count": summary.get("fixture_mapped_count"),
            "heuristic_or_unmapped_count": summary.get("heuristic_or_unmapped_count"),
            "type_transport_missing_cluster_count": type_transport_missing,
            "owner_confidence_missing_cluster_count": blocked_counts.get("NoExactOrFixtureMappedOwnerEdge", 0),
            "missing_shape_signature_cluster_count": blocked_counts.get("MissingShapeSignatureClusterAxis", 0),
        },
        "top_blocked_clusters": [
            {
                "cluster_id": cluster["cluster_id"],
                "candidate_count": cluster["candidate_count"],
                "blocked_by": cluster.get("blocked_by") or [],
                "shape_signature": cluster["shape_signature"],
                "owner_edge_confidence": cluster["owner_edge_confidence"],
                "type_transport_axis": cluster["type_transport_axis"],
            }
            for cluster in sorted(clusters, key=lambda item: (-item["candidate_count"], item["cluster_id"]))[:10]
        ],
        "decision": decision,
        "claims": {
            "all_missing_projection_policy_items_clustered_exactly_once": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "candidate_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "new_projection_policy_selected": 0,
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
        print("mirbuilder-missing-projection-policy-cluster-resolution-v3 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Derive FileScoped owner edges for OtherMissingProjectionPolicyCluster rows."""

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
OTHER_CLUSTER = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-cluster-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-EDGE-CONFIDENCE-REPAIR-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def selected_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    return sorted([
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("likely_owner_cluster") == "OtherMissingProjectionPolicyCluster"
    ], key=lambda item: item["source_id"])


def derive_owner_edge(source_path: str) -> tuple[str, str]:
    if source_path.startswith("src/mir/builder/"):
        stem = source_path[len("src/mir/builder/"):].removesuffix(".rs")
        return "hakorune_mir_builder::" + stem.replace("/", "::"), "FileScoped"
    if source_path.startswith("src/mir/region/"):
        stem = source_path[len("src/mir/region/"):].removesuffix(".rs")
        return "hakorune_mir_region::" + stem.replace("/", "::"), "FileScoped"
    return "", "None"


def build_fixture() -> dict[str, Any]:
    report = read_json(REPORT)
    other_cluster = read_json(OTHER_CLUSTER)
    items = selected_items(report)

    repaired_rows: list[dict[str, Any]] = []
    unrepaired_rows: list[dict[str, Any]] = []
    for item in items:
        repaired_edge, confidence = derive_owner_edge(item.get("source_path") or "")
        row = {
            "source_id": item["source_id"],
            "source_path": item.get("source_path") or "",
            "symbol": item.get("symbol") or "",
            "original_known_owner_edge": item.get("known_owner_edge") or "",
            "original_owner_edge_confidence": item.get("owner_edge_confidence") or "None",
            "repaired_known_owner_edge": repaired_edge,
            "repaired_owner_edge_confidence": confidence,
            "repair_reason_token": (
                "FileScopedOwnerEdgeDerivedFromSourcePath"
                if repaired_edge else
                "OwnerEdgeConfidenceRepairUnavailable"
            ),
        }
        if repaired_edge:
            repaired_rows.append(row)
        else:
            unrepaired_rows.append(row)

    edge_counts = Counter(row["repaired_known_owner_edge"] for row in repaired_rows)
    if unrepaired_rows:
        decision = {
            "kind": "KeepStopped",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "OtherOwnerEdgeConfidenceRepairIncomplete",
        }
    else:
        decision = {
            "kind": "SelectOtherOwnerClusterRerun",
            "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001",
            "reason_token": "OtherOwnerEdgeConfidenceRepairComplete",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyOtherOwnerEdgeConfidenceRepairV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "other_owner_cluster": rel(OTHER_CLUSTER),
            "other_owner_cluster_decision": other_cluster.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_other_owner_cluster_count": len(items),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "other_owner_cluster_hash": sha256_file(OTHER_CLUSTER),
        },
        "repair_policy": {
            "policy": "FileScopedOwnerEdgeFromSourcePath",
            "builder_prefix": "hakorune_mir_builder",
            "region_prefix": "hakorune_mir_region",
            "selection_authority": "source_path_only",
            "semantic_projection_inference": 0,
        },
        "repaired_rows": repaired_rows,
        "unrepaired_rows": unrepaired_rows,
        "summary": {
            "input_other_owner_cluster_count": len(items),
            "repaired_row_count": len(repaired_rows),
            "unrepaired_row_count": len(unrepaired_rows),
            "distinct_repaired_owner_edge_count": len(edge_counts),
            "top_repaired_owner_edges": [
                {"owner_edge": edge, "count": count}
                for edge, count in edge_counts.most_common(20)
            ],
        },
        "decision": decision,
        "claims": {
            "source_report_consumed": 1,
            "other_owner_cluster_consumed": 1,
            "input_other_owner_cluster_count": len(items),
            "all_other_owner_rows_have_repair_attempt": 1,
            "file_scoped_owner_edge_derived_from_source_path": 1,
            "semantic_projection_inference": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in owner-edge confidence repair fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

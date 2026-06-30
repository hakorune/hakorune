#!/usr/bin/env python3
"""Cluster MirBuilder borrow surfaces that still need projection policy.

This resolver consumes the crate-wide unconverted-surface report and classifies
BorrowSurfaceNeedsPolicy rows by borrow kind, return shape, receiver, source
module, and owner-edge confidence. It does not select MutLease or any concrete
borrow replacement policy while owner-edge confidence is missing.
"""

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
OUTPUT = FIXTURES / "mirbuilder-borrow-surface-needs-policy-cluster-resolution-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

TOKEN = "MIRBUILDER-BORROW-SURFACE-NEEDS-POLICY-CLUSTER-RESOLUTION-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_module(path: str | None) -> str:
    if not path:
        return "unknown_module"
    return Path(path).stem


def borrow_kind(item: dict[str, Any]) -> str:
    reason = item.get("reason_token")
    if reason == "ReturnedMutableBorrowPolicyMissing":
        return "ReturnedMutableBorrow"
    if reason == "ReturnedReadBorrowPolicyMissing":
        return "ReturnedReadBorrow"
    return "UnknownBorrow"


def return_shape(return_type: str | None) -> str:
    ty = (return_type or "").strip()
    if ty.startswith("Option<&'static str"):
        return "option_static_str_ref"
    if ty.startswith("Option<&str"):
        return "option_str_ref"
    if ty.startswith("Option<&["):
        return "option_slice_ref"
    if ty.startswith("Option<&"):
        return "option_ref"
    if ty.startswith("&'static str"):
        return "static_str_ref"
    if ty.startswith("&BTreeMap"):
        return "btree_map_ref"
    if ty.startswith("&HashMap"):
        return "hash_map_ref"
    if ty.startswith("&["):
        return "slice_ref"
    if ty.startswith("&mut "):
        return "mutable_ref"
    if ty.startswith("&"):
        return "shared_ref"
    return "unknown_return_shape"


def receiver_axis(receiver: str | None) -> str:
    value = (receiver or "none").strip()
    if value == "&mut self":
        return "mutable_receiver"
    if value == "&self":
        return "shared_receiver"
    if value == "self":
        return "owned_receiver"
    if value in {"", "None", "none"}:
        return "free_function_or_static"
    return "other_receiver"


def cluster_id(item: dict[str, Any]) -> str:
    return "::".join(
        [
            "borrow_surface",
            borrow_kind(item),
            return_shape(item.get("return_type")),
            receiver_axis(item.get("receiver")),
            source_module(item.get("source_path")),
            str(item.get("owner_edge_confidence") or "None"),
        ]
    )


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT)
    items = [
        item for item in report.get("items", [])
        if item.get("classification") == "BorrowSurfaceNeedsPolicy"
    ]

    clusters: dict[str, dict[str, Any]] = {}
    for item in items:
        cid = cluster_id(item)
        if cid not in clusters:
            clusters[cid] = {
                "cluster_id": cid,
                "borrow_kind": borrow_kind(item),
                "return_shape": return_shape(item.get("return_type")),
                "receiver_axis": receiver_axis(item.get("receiver")),
                "source_module": source_module(item.get("source_path")),
                "owner_edge_confidence": item.get("owner_edge_confidence"),
                "candidate_count": 0,
                "selection_eligible": False,
                "blocked_by": ["OwnerEdgeConfidenceMissing"],
                "next_owner_kind": "BorrowSurfaceOwnerEdgeConfidenceRepair",
                "next_card": "MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001",
                "examples": [],
            }
        row = clusters[cid]
        row["candidate_count"] += 1
        if len(row["examples"]) < 3:
            row["examples"].append(
                {
                    "source_id": item.get("source_id"),
                    "symbol": item.get("symbol"),
                    "source_path": item.get("source_path"),
                    "line": item.get("line"),
                    "return_type": item.get("return_type"),
                    "receiver": item.get("receiver"),
                    "reason_token": item.get("reason_token"),
                }
            )

    cluster_rows = sorted(
        clusters.values(),
        key=lambda row: (-row["candidate_count"], row["cluster_id"]),
    )

    confidence_counts = Counter(str(item.get("owner_edge_confidence")) for item in items)
    borrow_counts = Counter(borrow_kind(item) for item in items)
    return_counts = Counter(return_shape(item.get("return_type")) for item in items)

    if items and confidence_counts == Counter({"None": len(items)}):
        decision = {
            "kind": "SelectBorrowSurfaceOwnerEdgeConfidenceRepair",
            "reason_token": "BorrowSurfaceOwnerEdgeConfidenceMissingForAllCandidates",
            "selected_next_card": "MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001",
            "selected_cluster_id": None,
        }
    elif len([row for row in cluster_rows if row["selection_eligible"]]) == 1:
        selected = next(row for row in cluster_rows if row["selection_eligible"])
        decision = {
            "kind": "SelectBorrowProjectionPolicyCluster",
            "reason_token": "ExactlyOneBorrowSurfacePolicyCluster",
            "selected_next_card": "MIRBUILDER-BORROW-SURFACE-PROJECTION-POLICY-001",
            "selected_cluster_id": selected["cluster_id"],
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoBorrowSurfacePolicyClusterWithSufficientEvidence",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "selected_cluster_id": None,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderBorrowSurfaceNeedsPolicyClusterResolutionV1",
        "token": TOKEN,
        "input_authority": {
            "unconverted_surface_report": rel(REPORT),
            "current_state": rel(CURRENT_STATE),
        },
        "provenance": {
            "unconverted_surface_report_hash": sha256_file(REPORT),
        },
        "summary": {
            "borrow_surface_needs_policy_count": len(items),
            "cluster_count": len(cluster_rows),
            "returned_read_borrow_count": borrow_counts.get("ReturnedReadBorrow", 0),
            "returned_mutable_borrow_count": borrow_counts.get("ReturnedMutableBorrow", 0),
            "owner_edge_confidence_none_count": confidence_counts.get("None", 0),
            "selection_eligible_cluster_count": len([row for row in cluster_rows if row["selection_eligible"]]),
        },
        "return_shape_counts": dict(sorted(return_counts.items())),
        "clusters": cluster_rows,
        "decision": decision,
        "claims": {
            "report_consumed": 1,
            "borrow_policy_selected": 0,
            "mut_lease_selected": 0,
            "owned_read_snapshot_selected_for_new_surface": 0,
            "explicit_mutation_api_selected_for_new_surface": 0,
            "manual_borrow_policy_selection": 0,
            "manual_owner_edge_selection": 0,
            "strict_rules_changed": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify the checked-in resolution fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-borrow-surface-needs-policy-cluster-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

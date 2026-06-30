#!/usr/bin/env python3
"""Repair owner-edge confidence for borrow-surface policy candidates.

The repair is intentionally file/module scoped. It does not infer borrow
semantics, select a replacement policy, or emit Hako. It only replaces the
unusable owner_edge_confidence=None state with deterministic FileScoped owner
edges derived from source paths so a later policy resolver can work on stable
clusters.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
CLUSTERS = FIXTURES / "mirbuilder-borrow-surface-needs-policy-cluster-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-borrow-surface-owner-edge-confidence-repair-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

TOKEN = "MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def owner_edge_id(source_module: str) -> str:
    normalized = source_module.replace("-", "_")
    return f"mirbuilder::borrow_surface::{normalized}"


def build_repair() -> dict[str, Any]:
    cluster_data = read_json(CLUSTERS)
    repaired_rows: list[dict[str, Any]] = []
    total_candidates = 0

    for cluster in cluster_data.get("clusters", []):
        count = int(cluster.get("candidate_count", 0))
        total_candidates += count
        module = cluster.get("source_module") or "unknown_module"
        repaired_rows.append(
            {
                "cluster_id": cluster.get("cluster_id"),
                "candidate_count": count,
                "source_module": module,
                "borrow_kind": cluster.get("borrow_kind"),
                "return_shape": cluster.get("return_shape"),
                "receiver_axis": cluster.get("receiver_axis"),
                "old_owner_edge_confidence": cluster.get("owner_edge_confidence"),
                "repaired_owner_edge_id": owner_edge_id(module),
                "repaired_owner_edge_confidence": "FileScoped",
                "evidence": "source_module_from_source_path",
                "selection_eligible_for_borrow_policy": True,
            }
        )

    repaired_rows.sort(key=lambda row: (-row["candidate_count"], row["repaired_owner_edge_id"], row["cluster_id"]))

    decision = {
        "kind": "SelectBorrowSurfacePolicyClusterRerun",
        "reason_token": "BorrowSurfaceOwnerEdgeConfidenceRepaired",
        "selected_next_card": "MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001",
        "selected_owner_edge_id": None,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderBorrowSurfaceOwnerEdgeConfidenceRepairV1",
        "token": TOKEN,
        "input_authority": {
            "borrow_surface_cluster_resolution": rel(CLUSTERS),
            "current_state": rel(CURRENT_STATE),
        },
        "provenance": {
            "borrow_surface_cluster_resolution_hash": sha256_file(CLUSTERS),
            "repair_rule": "source_module_from_source_path_to_file_scoped_owner_edge",
        },
        "summary": {
            "input_borrow_surface_candidate_count": total_candidates,
            "input_cluster_count": len(cluster_data.get("clusters", [])),
            "repaired_cluster_count": len(repaired_rows),
            "repaired_candidate_count": sum(row["candidate_count"] for row in repaired_rows),
            "file_scoped_owner_edge_count": len({row["repaired_owner_edge_id"] for row in repaired_rows}),
            "selection_eligible_for_borrow_policy_count": len(repaired_rows),
        },
        "repaired_clusters": repaired_rows,
        "decision": decision,
        "claims": {
            "borrow_cluster_resolution_consumed": 1,
            "owner_edge_confidence_repaired": 1,
            "manual_owner_edge_selection": 0,
            "borrow_policy_selected": 0,
            "mut_lease_selected": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify the checked-in repair fixture.")
    args = parser.parse_args()

    output = stable_json(build_repair())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-borrow-surface-owner-edge-confidence-repair unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Rerun borrow-surface policy cluster selection after owner-edge repair."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPAIR = FIXTURES / "mirbuilder-borrow-surface-owner-edge-confidence-repair-v0.json"
OUTPUT = FIXTURES / "mirbuilder-borrow-surface-policy-cluster-rerun-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

TOKEN = "MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001"

BORROW_PRIORITY = {
    "ReturnedMutableBorrow": 0,
    "ReturnedReadBorrow": 1,
}
RETURN_PRIORITY = {
    "mutable_ref": 0,
    "option_ref": 1,
    "btree_map_ref": 2,
    "hash_map_ref": 3,
    "slice_ref": 4,
    "option_slice_ref": 5,
    "option_str_ref": 6,
    "option_static_str_ref": 7,
    "static_str_ref": 8,
    "shared_ref": 9,
}
RECEIVER_PRIORITY = {
    "mutable_receiver": 0,
    "shared_receiver": 1,
    "owned_receiver": 2,
    "free_function_or_static": 3,
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def priority_key(row: dict[str, Any]) -> tuple[int, int, int, str, str]:
    return (
        BORROW_PRIORITY.get(row.get("borrow_kind"), 99),
        RETURN_PRIORITY.get(row.get("return_shape"), 99),
        RECEIVER_PRIORITY.get(row.get("receiver_axis"), 99),
        row.get("repaired_owner_edge_id") or "",
        row.get("cluster_id") or "",
    )


def policy_card_for(row: dict[str, Any]) -> str:
    if row.get("borrow_kind") == "ReturnedMutableBorrow":
        return "MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001"
    return "MIRBUILDER-BORROW-SURFACE-RETURNED-READ-BORROW-POLICY-001"


def build_rerun() -> dict[str, Any]:
    repair = read_json(REPAIR)
    candidates = [
        row for row in repair.get("repaired_clusters", [])
        if row.get("selection_eligible_for_borrow_policy") is True
    ]
    sorted_candidates = sorted(candidates, key=priority_key)

    if sorted_candidates:
        selected = sorted_candidates[0]
        decision = {
            "kind": "SelectBorrowProjectionPolicyCluster",
            "reason_token": "HighestRiskBorrowPolicyClusterSelected",
            "selected_cluster_id": selected.get("cluster_id"),
            "selected_next_card": policy_card_for(selected),
        }
    else:
        selected = None
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoBorrowPolicyClusterAfterOwnerEdgeRepair",
            "selected_cluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderBorrowSurfacePolicyClusterRerunV1",
        "token": TOKEN,
        "input_authority": {
            "borrow_surface_owner_edge_confidence_repair": rel(REPAIR),
            "current_state": rel(CURRENT_STATE),
        },
        "provenance": {
            "borrow_surface_owner_edge_confidence_repair_hash": sha256_file(REPAIR),
        },
        "selection_rules": {
            "borrow_kind_priority": ["ReturnedMutableBorrow", "ReturnedReadBorrow"],
            "return_shape_priority": list(RETURN_PRIORITY),
            "receiver_priority": list(RECEIVER_PRIORITY),
            "cluster_size_as_proof": 0,
            "lexical_tie_breaker_only_after_evidence_axes": 1,
        },
        "candidate_pool": {
            "selection_eligible_cluster_count": len(candidates),
            "returned_mutable_borrow_cluster_count": len([
                row for row in candidates if row.get("borrow_kind") == "ReturnedMutableBorrow"
            ]),
            "returned_read_borrow_cluster_count": len([
                row for row in candidates if row.get("borrow_kind") == "ReturnedReadBorrow"
            ]),
        },
        "selected_cluster": selected,
        "decision": decision,
        "claims": {
            "owner_edge_confidence_repair_consumed": 1,
            "borrow_policy_cluster_selected": 1 if selected else 0,
            "borrow_policy_selected": 0,
            "mut_lease_selected": 0,
            "owned_read_snapshot_selected_for_new_surface": 0,
            "explicit_mutation_api_selected_for_new_surface": 0,
            "manual_borrow_policy_selection": 0,
            "cluster_size_as_proof": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify the checked-in rerun fixture.")
    args = parser.parse_args()

    output = stable_json(build_rerun())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-borrow-surface-policy-cluster-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

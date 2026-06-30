#!/usr/bin/env python3
"""Inventory shape signatures for mapped MissingProjectionPolicy surfaces."""

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
STABLE_DENY_REPAIR = FIXTURES / "mirbuilder-missing-projection-stable-deny-reason-repair-v0.json"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-shape-signature-inventory-v0.json"

SHAPE_AXIS_KEYS = [
    "loop_cond_co_statement_lowering_subcluster",
    "loop_cond_co_helper_subcluster",
    "loop_cond_co_group_if_subcluster",
    "loop_cond_co_continue_if_subcluster",
    "loop_cond_co_subcluster",
    "loop_cond_bc_pipeline_subcluster",
    "loop_cond_bc_item_lowering_subcluster",
    "loop_cond_bc_cleanup_subcluster",
    "loop_cond_bc_else_pattern_subcluster",
    "loop_cond_bc_subcluster",
    "loop_cond_feature_subcluster",
    "plan_feature_subcluster",
    "joinir_plan_subcluster",
    "likely_owner_cluster",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def snake(value: str) -> str:
    value = re.sub(r"Cluster$", "", value)
    return re.sub(r"(?<!^)(?=[A-Z])", "_", value).lower()


def shape_axis(item: dict[str, Any]) -> str:
    for key in SHAPE_AXIS_KEYS:
        value = item.get(key)
        if value:
            return value
    return "UnknownShape"


def shape_signature_for_axis(axis: str) -> str:
    return f"shape.{snake(axis)}"


def build_inventory() -> dict[str, Any]:
    report = read_json(REPORT)
    stable_deny_repair = read_json(STABLE_DENY_REPAIR)
    missing_items = [
        item for item in report["items"]
        if item["classification"] == "MissingProjectionPolicy"
    ]

    mapped = [
        item for item in missing_items
        if item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
    ]
    denied = [
        item for item in missing_items
        if item.get("owner_edge_confidence") != "FixtureMapped"
    ]

    counts: dict[str, int] = {}
    for item in mapped:
        axis = shape_axis(item)
        counts[axis] = counts.get(axis, 0) + 1

    signatures = [
        {
            "shape_axis": axis,
            "shape_signature": shape_signature_for_axis(axis),
            "candidate_count": count,
            "selected": True,
            "reason_token": "ShapeSignatureDerivedFromClusterAxis",
        }
        for axis, count in sorted(counts.items())
    ]

    denied_count = len(denied)
    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideShapeSignatureInventoryV1",
        "token": "MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001",
        "input_state": {
            "source_report": rel(REPORT),
            "stable_deny_reason_repair": rel(STABLE_DENY_REPAIR),
            "input_missing_projection_policy_count": len(missing_items),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "stable_deny_repair_decision": stable_deny_repair["decision"],
        },
        "provenance": {
            "shape_axis_counts_hash": sha256_text(stable_json(counts)),
        },
        "shape_axis_order": SHAPE_AXIS_KEYS,
        "shape_signatures": signatures,
        "denied": {
            "candidate_count": denied_count,
            "reason_token": "OwnerEdgeConfidenceMissing",
            "selected": False,
        },
        "summary": {
            "input_candidate_count": len(missing_items),
            "shape_signature_count": len(signatures),
            "shape_signature_candidate_count": len(mapped),
            "denied_candidate_count": denied_count,
            "unknown_shape_candidate_count_after_inventory": denied_count,
        },
        "decision": {
            "kind": "ApplyShapeSignatureInventory",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "ShapeSignatureInventoryExposesProjectionPolicyClusterPriorityNeed",
        },
        "claims": {
            "input_missing_projection_policy_count": len(missing_items),
            "shape_signature_inventory_defined": 1,
            "shape_signature_candidate_count_after_inventory": len(mapped),
            "unknown_shape_candidate_count_after_inventory": denied_count,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in shape inventory fixture.")
    args = parser.parse_args()

    output = stable_json(build_inventory())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-crate-wide-shape-signature-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

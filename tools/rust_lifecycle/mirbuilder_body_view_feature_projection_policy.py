#!/usr/bin/env python3
"""Resolve BodyView feature projection policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
OUTPUT = FIXTURES / "mirbuilder-body-view-feature-projection-policy-v0.json"
TOKEN = "MIRBUILDER-BODY-VIEW-FEATURE-PROJECTION-POLICY-001"
CLUSTER_ID = "projection_policy::UnsupportedDirectShape::shape.body_view_feature::FixtureMapped::BodyViewFeatureCluster"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def require_markers(source: str, markers: list[str]) -> list[str]:
    missing = [marker for marker in markers if marker not in source]
    if missing:
        raise SystemExit(f"source marker drift for BodyView::len: {missing}")
    return markers


def selected_report_item(report: dict[str, Any]) -> dict[str, Any]:
    items = report.get("items", [])
    matches = [
        item
        for item in items
        if item.get("plan_feature_subcluster") == "BodyViewFeatureCluster"
        and item.get("shape_signature") == "shape.body_view_feature"
    ]
    if len(matches) != 1:
        raise SystemExit(f"expected exactly one BodyViewFeatureCluster item, got {len(matches)}")
    return matches[0]


def build_policy() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    report = read_json(REPORT)
    priority_selected = (
        priority["decision"]["selected_cluster_id"] == CLUSTER_ID
        and priority["decision"]["selected_next_card"] == TOKEN
    )
    priority_excluded = any(
        item.get("cluster_id") == CLUSTER_ID
        for item in priority.get("excluded_existing_decision_clusters", [])
    )
    if not (priority_selected or priority_excluded):
        raise SystemExit("priority resolver neither selects nor excludes BodyViewFeatureCluster")

    item = selected_report_item(report)
    if item["symbol"] != "len" or item["source_path"] != "src/mir/builder/control_flow/plan/features/body_view.rs":
        raise SystemExit(f"unexpected BodyView item: {item}")

    source_text = read_source(item["source_path"])
    source_markers = require_markers(
        source_text,
        [
            "enum BodyView",
            "Recipe(&'a RecipeBody)",
            "Slice(&'a [ASTNode])",
            "pub fn len(&self) -> usize",
            "BodyView::Recipe(body) => body.len()",
            "BodyView::Slice(body) => body.len()",
        ],
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderBodyViewFeatureProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "priority_resolution": rel(PRIORITY),
            "unconverted_surface_report": rel(REPORT),
            "selected_cluster_id": CLUSTER_ID,
            "source_count": 1,
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": item["owner_edge_confidence"],
            "stable_deny_reason": item["stable_deny_reason"],
            "shape_signature": item["shape_signature"],
            "borrow_axis": "NoReturnedBorrow",
            "control_flow_axis": "StraightLine",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": [
            {
                "source_id": item["source_id"],
                "symbol": item["symbol"],
                "source_path": item["source_path"],
                "line": item["line"],
                "visibility": item["visibility"],
                "receiver": item["receiver"],
                "params": item["params"],
                "return_type": item["return_type"],
                "known_owner_edge": item["known_owner_edge"],
                "owner_edge_confidence": item["owner_edge_confidence"],
            }
        ],
        "body_view_descriptor": {
            "descriptor_id": "body_view_feature_len_v1",
            "source_extraction": "rust_body_view_len_read_only_view",
            "view_variants": [
                {
                    "variant": "Recipe",
                    "source_type": "RecipeBody",
                    "length_operation": "RecipeBody::len",
                },
                {
                    "variant": "Slice",
                    "source_type": "[ASTNode]",
                    "length_operation": "slice::len",
                },
            ],
            "return_contract": "usize",
            "mutation_frame": [],
            "returned_borrow": 0,
            "source_markers": source_markers,
        },
        "selected_policy": {
            "policy": "SourceExtractedReadOnlyViewLengthDescriptor",
            "owner_edge": "mirbuilder::body_view_feature_len",
            "descriptor_selected": True,
            "hako_projection_selected": False,
            "reason_token": "BodyViewLenDescriptorRequiredBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectProjectionPolicyDescriptor",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "BodyViewFeatureLenDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "descriptor_selected": 1,
            "hako_projection_selected": 0,
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
        "provenance": {
            "tool_role": "FactsAdapterGuardOrchestrator",
            "semantic_projection_inference": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-body-view-feature-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Resolve PhiMaterializerFeature constructor projection policy."""

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
OUTPUT = FIXTURES / "mirbuilder-phi-materializer-feature-projection-policy-v0.json"
TOKEN = "MIRBUILDER-PHI-MATERIALIZER-FEATURE-PROJECTION-POLICY-001"
SELECTED_CLUSTER_IDS = [
    (
        "projection_policy::UnsupportedDirectShape::shape.phi_materializer_feature::"
        "FixtureMapped::PhiMaterializerFeatureCluster::borrow=NoBorrow::"
        "control=PhiRequired::type=Known::call=AllKnown::verifier=Present"
    ),
]

EXPECTED_SURFACES = {
    "src/mir/builder/control_flow/plan/features/loop_carriers.rs::build_phi_info:L8": {
        "role": "core_phi_info_builder",
        "marker": "CorePhiInfo {",
    },
    "src/mir/builder/control_flow/plan/features/loop_carriers.rs::build_loop_bindings:L39": {
        "role": "loop_binding_builder",
        "marker": "create_phi_bindings(bindings)",
    },
    "src/mir/builder/control_flow/plan/features/loop_cond_bc_phi_materializer.rs::new:L29": {
        "role": "route_local_closure_constructor",
        "marker": "Self { phis, final_values }",
    },
    "src/mir/builder/control_flow/plan/features/loop_cond_co_phi_materializer.rs::new:L19": {
        "role": "route_local_closure_constructor",
        "marker": "Self { phis, final_values }",
    },
    "src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_phi_materializer.rs::new:L27": {
        "role": "route_local_closure_constructor",
        "marker": "continue_target",
    },
    "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_phi_materializer.rs::new:L30": {
        "role": "route_local_closure_constructor",
        "marker": "continue_exit",
    },
    "src/mir/builder/control_flow/plan/features/loop_true_break_continue_phi_materializer.rs::new:L19": {
        "role": "route_local_closure_constructor",
        "marker": "Self { phis, final_values }",
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def cluster_axis(item: dict[str, Any]) -> str:
    for key in [
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
    ]:
        value = item.get(key)
        if value:
            return value
    return "Unclustered"


def borrow_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    params = item.get("params") or ""
    if "&mut" in ret:
        return "ReturnedMutableAliasUnknown"
    if "&" in ret:
        return "BorrowPolicyNeeded"
    if "&mut" in params or "&self" in params:
        return "NoReturnedBorrow"
    return "NoBorrow"


def type_transport_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if "unsafe" in (item.get("source_path") or ""):
        return "UnsafeOrFFI"
    if ret in {"", "bool", "usize", "i64", "String"}:
        return "Known"
    if "&" in ret:
        return "Missing"
    return "Missing"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    if item.get("evidence_refs"):
        return "Present"
    return "MissingVerifier"


def selected_report_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    items = [
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and cluster_axis(item) == "PhiMaterializerFeatureCluster"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
        and item.get("shape_signature") == "shape.phi_materializer_feature"
        and borrow_axis(item) == "NoBorrow"
        and type_transport_axis(item) == "Known"
        and verifier_or_oracle_state(item) == "Present"
    ]
    found = {item["source_id"] for item in items}
    expected = set(EXPECTED_SURFACES)
    if found != expected:
        missing = sorted(expected - found)
        extra = sorted(found - expected)
        raise SystemExit(f"PhiMaterializerFeature selected surface drift: missing={missing} extra={extra}")
    return sorted(items, key=lambda item: item["source_id"])


def require_source_markers(items: list[dict[str, Any]]) -> list[dict[str, str]]:
    markers: list[dict[str, str]] = []
    for item in items:
        source_id = item["source_id"]
        marker = EXPECTED_SURFACES[source_id]["marker"]
        source_text = read_source(item["source_path"])
        if marker not in source_text:
            raise SystemExit(f"source marker drift for {source_id}: {marker!r}")
        markers.append({
            "source_id": source_id,
            "marker": marker,
        })
    return markers


def build_policy() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    report = read_json(REPORT)
    priority_selected = (
        priority["decision"]["selected_cluster_id"] in set(SELECTED_CLUSTER_IDS)
        and priority["decision"]["selected_next_card"] == TOKEN
    )
    priority_excluded = any(
        item.get("cluster_id") in set(SELECTED_CLUSTER_IDS)
        for item in priority.get("excluded_existing_decision_clusters", [])
    )
    if not (priority_selected or priority_excluded):
        raise SystemExit("priority resolver neither selects nor excludes PhiMaterializerFeature cluster")

    items = selected_report_items(report)
    source_markers = require_source_markers(items)
    role_counts: dict[str, int] = {}
    for item in items:
        role = EXPECTED_SURFACES[item["source_id"]]["role"]
        role_counts[role] = role_counts.get(role, 0) + 1

    return {
        "schema_version": 0,
        "kind": "MirBuilderPhiMaterializerFeatureProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "priority_resolution": rel(PRIORITY),
            "unconverted_surface_report": rel(REPORT),
            "selected_cluster_id": SELECTED_CLUSTER_IDS[0],
            "selected_cluster_ids": SELECTED_CLUSTER_IDS,
            "source_count": len(items),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.phi_materializer_feature",
            "borrow_axis": "NoBorrow",
            "control_flow_axis": "PhiRequired",
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
                "role": EXPECTED_SURFACES[item["source_id"]]["role"],
            }
            for item in items
        ],
        "phi_materializer_feature_descriptor": {
            "descriptor_id": "phi_materializer_feature_constructors_v1",
            "source_extraction": "rust_phi_materializer_constructor_helpers",
            "role_counts": dict(sorted(role_counts.items())),
            "core_phi_info_builders": [
                "build_phi_info",
            ],
            "loop_binding_builders": [
                "build_loop_bindings",
            ],
            "closure_constructors": [
                "LoopCondBreakContinuePhiClosure::new",
                "LoopCondContinueOnlyPhiClosure::new",
                "LoopCondContinueWithReturnPhiClosure::new",
                "LoopCondReturnInBodyPhiClosure::new",
                "LoopTrueBreakContinuePhiClosure::new",
            ],
            "mutation_frame": [],
            "return_contract": "route-local PHI carrier data constructors",
            "returned_borrow": 0,
            "source_markers": source_markers,
        },
        "selected_policy": {
            "policy": "PhiMaterializerFeatureConstructorDescriptor",
            "owner_edge": "mirbuilder::phi_materializer_feature",
            "descriptor_selected": True,
            "hako_projection_selected": False,
            "reason_token": "PhiMaterializerFeatureConstructorDescriptorRequiredBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectProjectionPolicyDescriptor",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "PhiMaterializerFeatureConstructorDescriptorMaterialized",
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
        print("mirbuilder-phi-materializer-feature-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

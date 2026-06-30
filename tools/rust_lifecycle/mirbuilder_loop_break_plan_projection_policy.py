#!/usr/bin/env python3
"""Resolve LoopBreakPlan boolean predicate projection policy."""

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
OUTPUT = FIXTURES / "mirbuilder-loop-break-plan-projection-policy-v0.json"
TOKEN = "MIRBUILDER-LOOP-BREAK-PLAN-PROJECTION-POLICY-001"
CLUSTER_ID = (
    "projection_policy::UnsupportedDirectShape::shape.loop_break_plan::"
    "FixtureMapped::LoopBreakPlanCluster::borrow=NoBorrow::"
    "control=StructuredLoop::type=Known::call=AllKnown::verifier=Present"
)

EXPECTED_SURFACES = {
    "src/mir/builder/control_flow/plan/loop_break/facts/helpers_common.rs::has_continue_statement:L92": {
        "predicate": "contains_continue_statement",
        "source_marker": "common_has_continue(body)",
    },
    "src/mir/builder/control_flow/plan/loop_break/facts/helpers_common.rs::has_return_statement:L99": {
        "predicate": "contains_return_statement",
        "source_marker": "common_has_return(body)",
    },
    "src/mir/builder/control_flow/plan/loop_break/facts/helpers_condition.rs::matches_ge_zero:L90": {
        "predicate": "binary_greater_equal_zero",
        "source_marker": "operator: BinaryOperator::GreaterEqual",
    },
    "src/mir/builder/control_flow/plan/loop_break/facts/helpers_condition.rs::matches_eq_empty_string:L113": {
        "predicate": "binary_equal_empty_string",
        "source_marker": "matches_eq_empty_string_sides",
    },
    "src/mir/builder/control_flow/plan/loop_break/facts/helpers_loop.rs::has_assignment_after:L99": {
        "predicate": "assignment_after_index_to_variable",
        "source_marker": "body.iter().skip(start_idx + 1)",
    },
    "src/mir/builder/control_flow/plan/loop_break/facts/trim_whitespace_helpers.rs::matches_substring_at_loop_var:L489": {
        "predicate": "substring_at_loop_var",
        "source_marker": 'method != "substring" || arguments.len() != 2',
    },
    "src/mir/builder/control_flow/plan/loop_break/facts/trim_whitespace_helpers.rs::collect_whitespace_terms:L118": {
        "predicate": "collect_whitespace_terms",
        "source_marker": "delimiters.push(delim);",
    },
}
SELECTED_CLUSTER_IDS = [
    CLUSTER_ID,
    (
        "projection_policy::UnsupportedDirectShape::shape.loop_break_plan::"
        "FixtureMapped::LoopBreakPlanCluster::borrow=NoReturnedBorrow::"
        "control=StructuredLoop::type=Known::call=AllKnown::verifier=Present"
    ),
]


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


def call_vocabulary_axis(item: dict[str, Any]) -> str:
    if item.get("owner_edge_confidence") in {"ExactSymbol", "FixtureMapped"}:
        return "AllKnown"
    return "SomeUnknown"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    if item.get("evidence_refs"):
        return "Present"
    return "MissingVerifier"


def selected_report_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    items = [
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and cluster_axis(item) == "LoopBreakPlanCluster"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
        and item.get("shape_signature") == "shape.loop_break_plan"
        and borrow_axis(item) in {"NoBorrow", "NoReturnedBorrow"}
        and type_transport_axis(item) == "Known"
        and call_vocabulary_axis(item) == "AllKnown"
        and verifier_or_oracle_state(item) == "Present"
    ]
    found = {item["source_id"] for item in items}
    expected = set(EXPECTED_SURFACES)
    if found != expected:
        missing = sorted(expected - found)
        extra = sorted(found - expected)
        raise SystemExit(f"LoopBreakPlan selected surface drift: missing={missing} extra={extra}")
    return sorted(items, key=lambda item: item["source_id"])


def require_source_markers(items: list[dict[str, Any]]) -> list[dict[str, str]]:
    markers: list[dict[str, str]] = []
    for item in items:
        source_id = item["source_id"]
        spec = EXPECTED_SURFACES[source_id]
        source_text = read_source(item["source_path"])
        marker = spec["source_marker"]
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
        raise SystemExit("priority resolver neither selects nor excludes LoopBreakPlan predicate cluster")

    items = selected_report_items(report)
    source_markers = require_source_markers(items)

    return {
        "schema_version": 0,
        "kind": "MirBuilderLoopBreakPlanProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "priority_resolution": rel(PRIORITY),
            "unconverted_surface_report": rel(REPORT),
            "selected_cluster_id": CLUSTER_ID,
            "selected_cluster_ids": SELECTED_CLUSTER_IDS,
            "source_count": len(items),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.loop_break_plan",
            "borrow_axis": "NoBorrow",
            "control_flow_axis": "StructuredLoop",
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
                "predicate": EXPECTED_SURFACES[item["source_id"]]["predicate"],
            }
            for item in items
        ],
        "loop_break_plan_predicate_descriptor": {
            "descriptor_id": "loop_break_plan_predicate_and_accumulator_helpers_v1",
            "source_extraction": "rust_loop_break_plan_boolean_predicate_helpers",
            "predicate_count": 6,
            "accumulator_helper_count": 1,
            "predicates": [
                "contains_continue_statement",
                "contains_return_statement",
                "binary_greater_equal_zero",
                "binary_equal_empty_string",
                "assignment_after_index_to_variable",
                "substring_at_loop_var",
            ],
            "accumulator_helpers": [
                "collect_whitespace_terms",
            ],
            "mutation_frame": [
                "collect_whitespace_terms mutates caller-owned haystack_var Option",
                "collect_whitespace_terms mutates caller-owned direction Option",
                "collect_whitespace_terms appends to caller-owned delimiters Vec",
            ],
            "return_contract": "bool",
            "returned_borrow": 0,
            "source_markers": source_markers,
        },
        "selected_policy": {
            "policy": "LoopBreakPlanPredicateAndAccumulatorDescriptor",
            "owner_edge": "mirbuilder::join_i_r_plan",
            "descriptor_selected": True,
            "hako_projection_selected": False,
            "reason_token": "LoopBreakPlanPredicateDescriptorRequiredBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectProjectionPolicyDescriptor",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "LoopBreakPlanPredicateDescriptorMaterialized",
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
        print("mirbuilder-loop-break-plan-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Resolve LoopCondPlan projection policy."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-loop-cond-plan-projection-policy-v0.json"
SELECTED_CLUSTER_ID = (
    "projection_policy::UnsupportedDirectShape::shape.loop_cond_plan::"
    "FixtureMapped::LoopCondPlanCluster"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def no_borrow(item: dict[str, Any]) -> bool:
    ret = item.get("return_type") or ""
    params = item.get("params") or ""
    return "&" not in ret and "&mut" not in params and "&self" not in params


def known_type_transport(item: dict[str, Any]) -> bool:
    ret = item.get("return_type") or ""
    return ret in {"", "bool", "usize", "i64", "String"}


def selected_surfaces(report: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        item for item in report["items"]
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("shape_signature") == "shape.loop_cond_plan"
        and item.get("joinir_plan_subcluster") == "LoopCondPlanCluster"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
        and item.get("evidence_refs")
        and no_borrow(item)
        and known_type_transport(item)
    ]


def role_for(item: dict[str, Any]) -> str:
    path = item["source_path"]
    symbol = item["symbol"]
    if "validator_prelude" in path:
        return "prelude_validator"
    if "validator_exit" in path:
        return "exit_shape_validator"
    if "validator_else" in path:
        return "else_shape_validator"
    if "validator_cond" in path:
        return "conditional_update_validator"
    if "break_continue_helpers" in path:
        return "branch_exit_helper"
    if "break_continue_classify" in path:
        return "general_if_classifier"
    if "true_break_continue_helpers" in path:
        return "nested_loop_condition_helper"
    return f"loop_cond_plan_predicate::{symbol}"


def build_policy() -> dict[str, Any]:
    report = read_json(REPORT)
    surfaces = selected_surfaces(report)
    source_text = "\n".join(read_source(item["source_path"]) for item in surfaces)
    role_counts = Counter(role_for(item) for item in surfaces)

    evidence_markers = [
        "loop_cond_break_continue facts extraction",
        "pattern validators",
        "is_supported_bool_expr_with_canon",
        "branch_has_exit_or_loop",
        "exit_prelude_is_allowed",
        "return_prelude_is_allowed",
        "is_exit_if_stmt",
        "allow_extended",
        "allow_return",
        "ASTNode::If",
        "ASTNode::Break",
        "ASTNode::Return",
    ]
    present_markers = [marker for marker in evidence_markers if marker in source_text]

    return {
        "schema_version": 0,
        "kind": "MirBuilderLoopCondPlanProjectionPolicyV1",
        "token": "MIRBUILDER-LOOP-COND-PLAN-PROJECTION-POLICY-001",
        "input_state": {
            "source_report": rel(REPORT),
            "priority_resolution": rel(PRIORITY),
            "selected_cluster_id": SELECTED_CLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.loop_cond_plan",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": [
            {
                "source_id": item["source_id"],
                "symbol": item["symbol"],
                "source_path": item["source_path"],
                "visibility": item["visibility"],
                "return_type": item["return_type"],
                "role": role_for(item),
            }
            for item in surfaces
        ],
        "role_counts": dict(sorted(role_counts.items())),
        "plan_evidence": present_markers,
        "selected_policy": {
            "policy": "KeepParentOwner",
            "owner_edge": "mirbuilder::loop_cond_plan",
            "projection_surface_selected": False,
            "reason_token": "LoopCondPlanShapePredicatesAreParentOwned",
        },
        "decision": {
            "kind": "KeepParentOwner",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "LoopCondPlanDoesNotOpenStandaloneProjectionOwner",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-loop-cond-plan-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

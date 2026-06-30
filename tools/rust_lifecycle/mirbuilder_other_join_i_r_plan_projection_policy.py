#!/usr/bin/env python3
"""Resolve OtherJoinIRPlan projection policy for parent-owned plan helpers."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-other-join-i-r-plan-projection-policy-v0.json"
SELECTED_CLUSTER_ID = (
    "projection_policy::UnsupportedDirectShape::shape.other_join_i_r_plan::"
    "FixtureMapped::OtherJoinIRPlanCluster"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def selected_surfaces(report: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        item for item in report["items"]
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("shape_signature") == "shape.other_join_i_r_plan"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
    ]


def surface_bucket(item: dict[str, Any]) -> str:
    path = item["source_path"]
    symbol = item["symbol"]
    if path.endswith("/trace.rs"):
        return "trace_or_debug"
    if "skeletons/" in path or "loop_wiring" in path or "step_mode" in path:
        return "skeleton_or_wiring"
    if "single_planner" in path or path.endswith("/plan_build_session.rs"):
        return "planner_session_or_rule_dispatch"
    if symbol in {"entry_gate_ok", "validate_loop_condition", "count_control_flow_with_returns"}:
        return "planner_gate_or_fact_count"
    if symbol == "build_join_payload":
        return "join_payload"
    if symbol == "branchn_to_if_chain":
        return "route_rewrite_helper"
    return "plan_helper"


def build_policy() -> dict[str, Any]:
    report = read_json(REPORT)
    surfaces = selected_surfaces(report)

    source_text = "\n".join(read_source(item["source_path"]) for item in surfaces)
    evidence_markers = [
        "CorePlan::If",
        "PlanBuildSession",
        "PLAN_RULE_ORDER",
        "GenericLoopSkeleton",
        "LoopTrueSkeleton",
        "LoopStepMode",
        "CoreIfJoin",
        "FragEmitSession",
        "[plan/trace]",
    ]
    present_markers = [marker for marker in evidence_markers if marker in source_text]

    buckets: dict[str, int] = {}
    for item in surfaces:
        bucket = surface_bucket(item)
        buckets[bucket] = buckets.get(bucket, 0) + 1

    return {
        "schema_version": 0,
        "kind": "MirBuilderOtherJoinIRPlanProjectionPolicyV1",
        "token": "MIRBUILDER-OTHER-JOIN-I-R-PLAN-PROJECTION-POLICY-001",
        "input_state": {
            "source_report": rel(REPORT),
            "priority_resolution": rel(PRIORITY),
            "selected_cluster_id": SELECTED_CLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "source_surfaces": [
            {
                "source_id": item["source_id"],
                "symbol": item["symbol"],
                "source_path": item["source_path"],
                "visibility": item["visibility"],
                "return_type": item["return_type"],
                "helper_bucket": surface_bucket(item),
            }
            for item in surfaces
        ],
        "helper_bucket_summary": dict(sorted(buckets.items())),
        "joinir_plan_evidence": present_markers,
        "selected_policy": {
            "policy": "KeepParentOwner",
            "owner_edge": "mirbuilder::join_i_r_plan",
            "projection_surface_selected": False,
            "reason_token": "JoinIRPlanHelpersAreParentOwned",
        },
        "decision": {
            "kind": "KeepParentOwner",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "OtherJoinIRPlanDoesNotOpenStandaloneProjectionOwner",
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
        print("mirbuilder-other-join-i-r-plan-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

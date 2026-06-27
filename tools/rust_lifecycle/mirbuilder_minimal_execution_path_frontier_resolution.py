#!/usr/bin/env python3
"""Resolve the explicit design-stop frontier for the minimal MirBuilder path.

This is a code-facing frontier resolver, not a new semantic projector. It
consumes the semantic closure report, the composed execution route, the
current-state pointer, the task-order pointer, and the role/adoption SSOT to
derive a stable next-slice decision.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

import tomllib

from shared_family_generator import read_json, sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

REPORT_PATH = FIXTURES / "minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"
COMPOSED_ROUTE_PATH = (
    ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution.route.json"
)
CURRENT_STATE_PATH = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER_PATH = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ROLE_SSOT_PATH = (
    ROOT / "docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
)
DESIGN_STOP_CONTRACT_PATH = ROOT / "tools/checks/current_state_design_stop_contract.txt"
OUTPUT_PATH = FIXTURES / "mirbuilder-minimal-execution-path-frontier-resolution-v0.json"


class FrontierResolutionError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise FrontierResolutionError(message)


def read_toml(path: Path) -> dict[str, Any]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def parse_current_state() -> dict[str, Any]:
    state = read_toml(CURRENT_STATE_PATH)
    require(state.get("latest_card"), "latest_card must be present")
    require(state.get("latest_card_path"), "latest_card_path must be present")
    require(bool(state.get("current_blocker_token")), "current_blocker_token must be present")
    return state


def validate_task_order() -> dict[str, str]:
    text = TASK_ORDER_PATH.read_text(encoding="utf-8")
    require(
        "MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001" in text,
        "task-order must select the guard-drift repair card",
    )
    require(
        "stale current-state exact pins" in text,
        "task-order must describe the guard-drift repair",
    )
    require(
        "manual next-owner selection" in text,
        "task-order must keep manual selection forbidden",
    )
    return {
        "path": rel(TASK_ORDER_PATH),
        "sha256": sha256_file(TASK_ORDER_PATH),
    }


def validate_role_ssot() -> dict[str, str]:
    text = ROLE_SSOT_PATH.read_text(encoding="utf-8")
    for needle in [
        "existing Python SemanticProjector = bootstrap/oracle only",
        "new Python SemanticProjector = forbidden by default",
        "HakoAdopted artifact write by Python = forbidden",
    ]:
        require(needle in text, f"role SSOT missing: {needle}")
    return {
        "path": rel(ROLE_SSOT_PATH),
        "sha256": sha256_file(ROLE_SSOT_PATH),
    }


def validate_design_stop_contract() -> dict[str, str]:
    text = DESIGN_STOP_CONTRACT_PATH.read_text(encoding="utf-8")
    require("blocker_token_contains=DESIGN-STOP" in text, "design-stop contract must still be active")
    return {
        "path": rel(DESIGN_STOP_CONTRACT_PATH),
        "sha256": sha256_file(DESIGN_STOP_CONTRACT_PATH),
    }


def validate_report(report: dict[str, Any]) -> dict[str, Any]:
    require(
        report.get("kind") == "MinimalMirBuilderExecutionPathSemanticClosureReportV1",
        "semantic closure report has wrong kind",
    )
    closure = report.get("closure") or {}
    require(closure.get("semantic_plan_closure") == "Closed", "semantic closure must remain closed")
    require(closure.get("rust_smoke_observation") == "Green", "rust smoke must remain green")
    require(closure.get("generated_hako_executable_closure") == "Open", "generated Hako closure must stay open")
    require(closure.get("full_path_mainline_eligible") is False, "mainline eligibility must stay false")
    require(closure.get("source_selfhost_eligible") is False, "source selfhost eligibility must stay false")
    require(closure.get("artifact_selfhost_checkpoint_complete") is False, "artifact selfhost checkpoint must stay open")

    first_gap = report.get("first_executable_materialization_gap") or {}
    require(first_gap.get("edge_id") == "minimal_path.completion_design_stop", "design-stop frontier drift")
    require(
        first_gap.get("required_capability") == "MinimalExecutionPathCompletionDesignReviewRequired",
        "design-stop capability drift",
    )
    require(
        first_gap.get("next_slice_token") == "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001",
        "design-stop next slice drift",
    )

    decomposition = report.get("materialization_decomposition") or {}
    require(
        decomposition.get("owner_kind") == "CompletionDesignStopReached",
        "design-stop owner kind drift",
    )
    require(
        (decomposition.get("composite_owner") or {}).get("edge_id") == first_gap.get("edge_id"),
        "design-stop owner drift",
    )
    require((decomposition.get("ordered_child_owners") or []) == [], "design-stop should not expose child owners")
    require(
        (decomposition.get("first_leaf_owner") or {}).get("edge_id") == first_gap.get("edge_id"),
        "design-stop first leaf drift",
    )

    return {
        "first_gap": first_gap,
        "decomposition": decomposition,
    }


def validate_composed_route(route: dict[str, Any]) -> dict[str, Any]:
    require(route.get("kind") == "MinimalMirBuilderComposedExecutionRouteV1", "route has wrong kind")
    require(
        route.get("route_slot_id") == "hakorune_mir_builder.minimal_path.composed_execution.v1",
        "route slot drift",
    )
    same_state_handoff = route.get("same_state_handoff") or {}
    require(same_state_handoff.get("state_transport") == "PreparedMirBuilderStateShell", "state transport drift")
    require(same_state_handoff.get("observed") == 1, "same-state handoff must stay observed")
    require(same_state_handoff.get("selected_existing_contracts_consumed") == 1, "existing contracts must be consumed")
    require(same_state_handoff.get("fallback_to_standalone_harness") == 0, "standalone harness fallback must stay off")
    claims = route.get("claims") or {}
    require(claims.get("generated_route_change") == 1, "route change claim must remain 1")
    for key in [
        "generated_hako_change",
        "semantic_recipe_recopy",
        "fallback_to_standalone_harness",
        "runtime_fallback",
        "new_backend_route",
        "new_abi",
        "source_selfhost_claim",
        "manual_next_edge_selection",
    ]:
        require(claims.get(key) == 0, f"route claim must remain 0: {key}")
    return {
        "same_state_handoff": same_state_handoff,
        "claims": claims,
    }


def decide_frontier(report: dict[str, Any], route: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    first_gap = report["first_executable_materialization_gap"]
    design_stop = report["design_stop"]
    same_state_handoff = route["same_state_handoff"]
    blocker_token = state.get("current_blocker_token") or ""

    # Current evidence is intentionally blocked at the explicit design-stop
    # frontier. The resolver remains mechanical: it classifies the frontier
    # instead of hand-picking a semantic owner.
    if first_gap.get("edge_id") == "minimal_path.completion_design_stop":
        return {
            "kind": "MinimalMirBuilderExecutionPathFrontierResolutionV1",
            "resolution_scope": "DesignStopFrontier",
            "source_authority": {
                "semantic_closure_report": {
                    "path": rel(REPORT_PATH),
                    "sha256": sha256_file(REPORT_PATH),
                },
                "composed_execution_route": {
                    "path": rel(COMPOSED_ROUTE_PATH),
                    "sha256": sha256_file(COMPOSED_ROUTE_PATH),
                },
            "current_state": {
                "path": rel(CURRENT_STATE_PATH),
                "sha256": sha256_file(CURRENT_STATE_PATH),
            },
                "task_order_pointer": validate_task_order(),
                "role_ssot": validate_role_ssot(),
                "design_stop_contract": validate_design_stop_contract(),
            },
            "current_state": {
                "latest_card": state.get("latest_card"),
                "latest_card_path": state.get("latest_card_path"),
                "current_blocker_token": blocker_token,
            },
            "composed_execution": {
                "same_state_handoff_observed": same_state_handoff.get("observed"),
                "selected_existing_contracts_consumed": same_state_handoff.get("selected_existing_contracts_consumed"),
                "fallback_to_standalone_harness": same_state_handoff.get("fallback_to_standalone_harness"),
            },
            "decision": {
                "kind": "Blocked",
                "reason": design_stop.get("deny_detail"),
                "reason_detail": first_gap.get("reason"),
                "next_slice_token": first_gap.get("next_slice_token"),
                "owner_scope": "integration",
            },
            "selected_evidence": [
                {
                    "kind": "semantic_closure_report",
                    "edge_id": first_gap.get("edge_id"),
                    "next_slice_token": first_gap.get("next_slice_token"),
                },
                {
                    "kind": "composed_execution_route",
                    "state_transport": same_state_handoff.get("state_transport"),
                    "observed": same_state_handoff.get("observed"),
                },
                {
                    "kind": "current_state",
                    "path": rel(CURRENT_STATE_PATH),
                    "sha256": sha256_file(CURRENT_STATE_PATH),
                },
                {
                    "kind": "task_order_pointer",
                    "latest_workstream_card": state.get("latest_workstream_card"),
                },
                {
                    "kind": "role_ssot",
                    "python_semantic_projector_growth_freeze": "forbidden_by_default",
                },
            ],
            "claims": {
                "existing_evidence_consumed": 1,
                "manual_next_edge_selection": 0,
                "resolver_output_is_derived": 1,
                "generated_hako_change": 0,
                "new_backend_route": 0,
                "new_abi": 0,
                "runtime_fallback": 0,
                "source_selfhost_claim": 0,
            },
        }

    raise FrontierResolutionError(
        f"resolver only knows the current design-stop frontier; found {first_gap.get('edge_id')}"
    )


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT_PATH)
    report_parts = validate_report(report)
    route = read_json(COMPOSED_ROUTE_PATH)
    route_parts = validate_composed_route(route)
    state = parse_current_state()
    resolution = decide_frontier(report, route, state)
    resolution["source_authority"]["semantic_closure_report"]["reason"] = report_parts["first_gap"].get("reason")
    resolution["source_authority"]["design_stop"] = {
        "edge_id": report_parts["first_gap"].get("edge_id"),
        "callsite": report_parts["first_gap"].get("callsite"),
        "reason": report_parts["first_gap"].get("reason"),
        "next_slice_token": report_parts["first_gap"].get("next_slice_token"),
    }
    resolution["source_authority"]["composed_execution_route"]["same_state_handoff"] = route_parts["same_state_handoff"]
    resolution["source_authority"]["composed_execution_route"]["claims"] = route_parts["claims"]
    return resolution


def run(check: bool) -> None:
    resolution = build_resolution()
    resolution_text = stable_json(resolution)
    if check:
        if not OUTPUT_PATH.exists() or OUTPUT_PATH.read_text(encoding="utf-8") != resolution_text:
            raise FrontierResolutionError(f"{rel(OUTPUT_PATH)} is stale")
    else:
        write_if_changed(OUTPUT_PATH, resolution_text)

    print("output_contract=rust-lifecycle-mirbuilder-minimal-execution-path-frontier-resolution-v0")
    print("resolver_guard=green")
    print(f"decision_kind={resolution['decision']['kind']}")
    print(f"next_slice_token={resolution['decision']['next_slice_token']}")
    print(f"reason={resolution['decision']['reason']}")
    print(f"owner_scope={resolution['decision']['owner_scope']}")
    print("manual_next_edge_selection=0")
    print("generated_hako_change=0")
    print("runtime_fallback=0")
    print("new_backend_route=0")
    print("new_abi=0")
    print("source_selfhost_claim=0")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        run(check=args.check)
    except FrontierResolutionError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Continue the composed minimal MirBuilder execution prefix.

This is a code-facing continuation resolver, not a new semantic projector.
It consumes the landed semantic closure report, the composed execution route,
the explicit design-stop frontier resolution, the current-state pointer, the
task-order pointer, the role/adoption SSOT, and the design-stop pause
contract. The result is fixture-backed and keeps the same-state handoff green
unless a stable composition red edge is exposed.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

import tomllib

from shared_family_generator import read_json, sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

REPORT_PATH = FIXTURES / "minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"
ROUTE_PATH = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution.route.json"
FRONTIER_PATH = FIXTURES / "mirbuilder-minimal-execution-path-frontier-resolution-v0.json"
CURRENT_STATE_PATH = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER_PATH = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ROLE_SSOT_PATH = ROOT / "docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
DESIGN_STOP_CONTRACT_PATH = ROOT / "tools/checks/current_state_design_stop_contract.txt"
OUTPUT_PATH = FIXTURES / "mirbuilder-minimal-path-composed-execution-continuation-v2.json"

EXPECTED_COMPOSITION_PREFIX = (
    "prepare_module.module_new",
    "prepare_module.next_block",
    "prepare_module.function_new",
    "prepare_module.state_install",
    "lower_root.literal_integer",
)

EXPECTED_ROUTE_STATE = "DerivedShadow"
EXPECTED_STABLE_NEXT_SLICE_TOKEN = "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001"


class ContinuationError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_toml(path: Path) -> dict[str, Any]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ContinuationError(message)


def parse_current_state() -> dict[str, Any]:
    state = read_toml(CURRENT_STATE_PATH)
    latest_card = state.get("latest_card")
    latest_card_path = state.get("latest_card_path")
    require(bool(latest_card), "latest_card must be present")
    require(bool(latest_card_path), "latest_card_path must be present")
    require(latest_card in latest_card_path, "latest_card_path must reference latest_card")
    require(bool(state.get("current_blocker_token")), "current_blocker_token must be present")
    return state


def validate_task_order() -> dict[str, Any]:
    text = TASK_ORDER_PATH.read_text(encoding="utf-8")
    for needle in [
        "same-state composed prefix evidence",
        "next_unconsumed_edge = Closed",
        "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001",
        "MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001",
    ]:
        require(needle in text, f"task-order missing: {needle}")
    return {
        "path": rel(TASK_ORDER_PATH),
        "sha256": sha256_file(TASK_ORDER_PATH),
    }


def validate_role_ssot() -> dict[str, Any]:
    text = ROLE_SSOT_PATH.read_text(encoding="utf-8")
    for needle in [
        "existing Python SemanticProjector = bootstrap/oracle only",
        "new Python SemanticProjector = forbidden by default",
        "HakoAdopted artifact write by Python = forbidden",
        "tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution_continuation.py",
    ]:
        require(needle in text, f"role SSOT missing: {needle}")
    return {
        "path": rel(ROLE_SSOT_PATH),
        "sha256": sha256_file(ROLE_SSOT_PATH),
    }


def validate_design_stop_contract() -> dict[str, Any]:
    text = DESIGN_STOP_CONTRACT_PATH.read_text(encoding="utf-8")
    require("blocker_token_contains=DESIGN-STOP" in text, "design-stop contract must still be active")
    return {
        "path": rel(DESIGN_STOP_CONTRACT_PATH),
        "sha256": sha256_file(DESIGN_STOP_CONTRACT_PATH),
    }


def validate_semantic_closure_report() -> dict[str, Any]:
    report = read_json(REPORT_PATH)
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

    design_stop = report.get("design_stop") or {}
    require(design_stop.get("next_slice_token") == EXPECTED_STABLE_NEXT_SLICE_TOKEN, "design-stop next slice drift")
    require(
        design_stop.get("deny_detail") == "MinimalExecutionPathCompletionDesignReviewRequired",
        "design-stop deny detail drift",
    )
    return {
        "path": rel(REPORT_PATH),
        "sha256": sha256_file(REPORT_PATH),
        "design_stop": design_stop,
    }


def validate_route() -> dict[str, Any]:
    route = read_json(ROUTE_PATH)
    require(route.get("kind") == "MinimalMirBuilderComposedExecutionRouteV1", "route has wrong kind")
    require(
        route.get("route_slot_id") == "hakorune_mir_builder.minimal_path.composed_execution.v1",
        "route slot drift",
    )
    require(route.get("selected_scope") == "PreparedMirBuilderStateV1", "selected scope drift")
    require(route.get("input_profile", {}).get("ast") == "ASTNode::Literal(Integer(0))", "input profile drift")

    source_prefix = route.get("source_order_prefix") or []
    require([row.get("edge_id") for row in source_prefix] == [
        "entry.prepared_state_profile",
        "build_module.prepare_module",
        "prepare_module.module_new",
        "prepare_module.next_block",
        "prepare_module.function_new",
        "prepare_module.state_install",
        "lower_root.literal_integer",
    ], "source order prefix drift")

    composition_prefix = route.get("composition_prefix") or []
    require([row.get("edge_id") for row in composition_prefix] == list(EXPECTED_COMPOSITION_PREFIX), "composition prefix drift")
    require(all(row.get("route_state") == EXPECTED_ROUTE_STATE for row in composition_prefix), "composition route state drift")
    require(len(route.get("selected_existing_contracts") or []) == 5, "route must consume five existing contracts")

    same_state = route.get("same_state_handoff") or {}
    require(same_state.get("state_transport") == "PreparedMirBuilderStateShell", "prepared state transport drift")
    require(same_state.get("observed") == 1, "same-state handoff must stay observed")
    require(same_state.get("selected_existing_contracts_consumed") == 1, "selected existing contracts must be consumed")
    require(same_state.get("fallback_to_standalone_harness") == 0, "standalone harness fallback must stay off")

    claims = route.get("claims") or {}
    for key in [
        "generated_route_change",
        "same_state_handoff_observed",
        "selected_existing_contracts_consumed",
    ]:
        require(claims.get(key) == 1, f"route claim must remain 1: {key}")
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
        "path": rel(ROUTE_PATH),
        "sha256": sha256_file(ROUTE_PATH),
        "route": route,
    }


def validate_frontier_resolution() -> dict[str, Any]:
    resolution = read_json(FRONTIER_PATH)
    require(
        resolution.get("kind") == "MinimalMirBuilderExecutionPathFrontierResolutionV1",
        "frontier resolution has wrong kind",
    )
    require(resolution.get("resolution_scope") == "DesignStopFrontier", "resolution scope drift")
    decision = resolution.get("decision") or {}
    require(decision.get("kind") == "Blocked", "frontier resolution must remain blocked")
    require(decision.get("next_slice_token") == EXPECTED_STABLE_NEXT_SLICE_TOKEN, "next slice token drift")
    require(decision.get("owner_scope") == "integration", "owner scope drift")
    return {
        "path": rel(FRONTIER_PATH),
        "sha256": sha256_file(FRONTIER_PATH),
        "decision": decision,
    }


def build_continuation() -> dict[str, Any]:
    state = parse_current_state()
    report = validate_semantic_closure_report()
    route_info = validate_route()
    frontier_info = validate_frontier_resolution()
    task_order = validate_task_order()
    role_ssot = validate_role_ssot()
    design_stop = validate_design_stop_contract()
    route = route_info["route"]
    resolution = read_json(FRONTIER_PATH)
    same_state = route["same_state_handoff"]

    continuation = {
        "schema_version": 0,
        "kind": "MinimalMirBuilderExecutionPathComposedExecutionContinuationV2",
        "continuation_scope": "SameStateComposedExecutionPrefix",
        "input_profile": {"ast": "ASTNode::Literal(Integer(0))"},
        "source_authority": {
            "semantic_closure_report": {
                "path": report["path"],
                "sha256": report["sha256"],
            },
            "composed_execution_route": {
                "path": route_info["path"],
                "sha256": route_info["sha256"],
            },
            "frontier_resolution": {
                "path": frontier_info["path"],
                "sha256": frontier_info["sha256"],
            },
            "current_state": {
                "path": rel(CURRENT_STATE_PATH),
                "sha256": sha256_file(CURRENT_STATE_PATH),
                "latest_card": state.get("latest_card"),
                "current_blocker_token": state.get("current_blocker_token"),
            },
            "task_order_pointer": task_order,
            "role_ssot": role_ssot,
            "design_stop_contract": design_stop,
        },
        "same_state_handoff": {
            "state_transport": same_state.get("state_transport"),
            "observed": same_state.get("observed"),
            "selected_existing_contracts_consumed": same_state.get("selected_existing_contracts_consumed"),
            "fallback_to_standalone_harness": same_state.get("fallback_to_standalone_harness"),
            "generated_hako_change": same_state.get("generated_hako_change"),
        },
        "composition_prefix": route.get("composition_prefix"),
        "continuation": {
            "kind": "ContinueComposedExecutionPrefix",
            "prefix_state": "Green",
            "first_composition_red_edge": None,
            "stable_reason_token": "COMPOSED_PREFIX_REMAINS_GREEN",
            "stable_next_slice_token": resolution.get("decision", {}).get("next_slice_token"),
            "owner_scope": "integration",
        },
        "selected_evidence": [
            {
                "kind": "semantic_closure_report",
                "path": report["path"],
                "sha256": report["sha256"],
                "prefix_state": "Green",
            },
            {
                "kind": "composed_execution_route",
                "path": route_info["path"],
                "sha256": route_info["sha256"],
                "same_state_handoff_observed": same_state.get("observed"),
            },
            {
                "kind": "frontier_resolution",
                "path": frontier_info["path"],
                "sha256": frontier_info["sha256"],
                "decision_kind": resolution.get("decision", {}).get("kind"),
            },
        ],
        "claims": {
            "existing_evidence_consumed": 1,
            "manual_next_edge_selection": 0,
            "semantic_recipe_recopy": 0,
            "new_semantic_projection": 0,
            "generated_hako_change": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
            "hako_adopted": 0,
            "full_minimal_path_mainline_selected": 0,
            "same_state_handoff_observed": 1,
            "first_red_edge_if_any_is_stable": 1,
            "stable_next_slice_token": 1,
        },
    }

    require(continuation["continuation"]["stable_next_slice_token"] == EXPECTED_STABLE_NEXT_SLICE_TOKEN, "stable next slice token drift")
    require(continuation["continuation"]["prefix_state"] == "Green", "prefix state drift")
    require(continuation["same_state_handoff"]["observed"] == 1, "same-state handoff must stay observed")
    require(continuation["continuation"]["first_composition_red_edge"] is None, "green continuation must not claim a red edge")
    return continuation


def run(*, check: bool) -> None:
    continuation = build_continuation()
    continuation_text = stable_json(continuation)
    if check:
        if not OUTPUT_PATH.exists() or OUTPUT_PATH.read_text(encoding="utf-8") != continuation_text:
            raise ContinuationError(f"{rel(OUTPUT_PATH)} is stale")
    else:
        write_if_changed(OUTPUT_PATH, continuation_text)

    print("output_contract=rust-lifecycle-minimal-path-composed-execution-continuation-v0")
    print("continuation_guard=green")
    print(f"stable_next_slice_token={continuation['continuation']['stable_next_slice_token']}")
    print(f"prefix_state={continuation['continuation']['prefix_state']}")
    print(f"same_state_handoff_observed={continuation['same_state_handoff']['observed']}")
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
    except ContinuationError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()

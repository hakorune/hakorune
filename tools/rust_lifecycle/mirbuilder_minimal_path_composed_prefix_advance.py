#!/usr/bin/env python3
"""Advance the same-state composed minimal MirBuilder execution prefix.

This is a code-facing prefix-advance resolver, not a new semantic projector.
It consumes the landed semantic closure report, the landed composed
continuation evidence, artifact manifests/contracts, route selections, the
task-order pointer, the role/adoption SSOT, and the explicit design-stop
pause contract. The result is fixture-backed and classifies the next
unconsumed edge without hand-picking a new semantic owner.
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
CONTINUATION_PATH = FIXTURES / "mirbuilder-minimal-path-composed-execution-continuation-v2.json"
ROUTE_PATH = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution.route.json"
CURRENT_STATE_PATH = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER_PATH = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ROLE_SSOT_PATH = ROOT / "docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
DESIGN_STOP_CONTRACT_PATH = ROOT / "tools/checks/current_state_design_stop_contract.txt"
OUTPUT_PATH = FIXTURES / "mirbuilder-minimal-path-composed-prefix-advance-v1.json"

EXPECTED_STABLE_NEXT_SLICE_TOKEN = "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001"
EXPECTED_PREFIX_EDGE_ID = "minimal_path.completion_design_stop"


class PrefixAdvanceError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_toml(path: Path) -> dict[str, Any]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PrefixAdvanceError(message)


def parse_current_state() -> dict[str, Any]:
    state = read_toml(CURRENT_STATE_PATH)
    latest_card = state.get("latest_card")
    latest_card_path = state.get("latest_card_path")
    require(bool(latest_card), "latest_card must be present")
    require(bool(latest_card_path), "latest_card_path must be present")
    require(latest_card in latest_card_path, "latest_card_path must reference latest_card")
    require(state.get("current_blocker_token") == EXPECTED_STABLE_NEXT_SLICE_TOKEN, "current blocker token drift")
    return state


def validate_task_order() -> dict[str, Any]:
    text = TASK_ORDER_PATH.read_text(encoding="utf-8")
    for needle in [
        "same-state composed prefix evidence",
        "next_unconsumed_edge = Closed",
        "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001",
        "manual next-owner selection",
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
        "tools/rust_lifecycle/mirbuilder_minimal_path_composed_prefix_advance.py",
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
    require(design_stop.get("edge_id") == EXPECTED_PREFIX_EDGE_ID, "design-stop frontier drift")
    require(
        design_stop.get("deny_detail") == "MinimalExecutionPathCompletionDesignReviewRequired",
        "design-stop deny detail drift",
    )
    require(design_stop.get("next_slice_token") == EXPECTED_STABLE_NEXT_SLICE_TOKEN, "design-stop next slice drift")

    decomposition = report.get("materialization_decomposition") or {}
    require(
        decomposition.get("owner_kind") == "CompletionDesignStopReached",
        "design-stop owner kind drift",
    )
    require((decomposition.get("ordered_child_owners") or []) == [], "design-stop should not expose child owners")
    require(
        (decomposition.get("first_leaf_owner") or {}).get("edge_id") == EXPECTED_PREFIX_EDGE_ID,
        "design-stop first leaf drift",
    )

    return {
        "path": rel(REPORT_PATH),
        "sha256": sha256_file(REPORT_PATH),
        "report": report,
        "design_stop": design_stop,
        "decomposition": decomposition,
    }


def validate_continuation() -> dict[str, Any]:
    continuation = read_json(CONTINUATION_PATH)
    require(
        continuation.get("kind") == "MinimalMirBuilderExecutionPathComposedExecutionContinuationV2",
        "continuation has wrong kind",
    )
    prefix = continuation.get("continuation") or {}
    require(prefix.get("kind") == "ContinueComposedExecutionPrefix", "continuation kind drift")
    require(prefix.get("prefix_state") == "Green", "prefix must remain green")
    require(prefix.get("first_composition_red_edge") is None, "green continuation must not claim a red edge")
    require(prefix.get("stable_next_slice_token") == EXPECTED_STABLE_NEXT_SLICE_TOKEN, "stable next slice drift")
    require(prefix.get("stable_reason_token") == "COMPOSED_PREFIX_REMAINS_GREEN", "stable reason drift")
    same_state = continuation.get("same_state_handoff") or {}
    require(same_state.get("observed") == 1, "same-state handoff must stay observed")
    require(same_state.get("selected_existing_contracts_consumed") == 1, "existing contracts must be consumed")
    require(same_state.get("fallback_to_standalone_harness") == 0, "standalone harness fallback must stay off")
    return {
        "path": rel(CONTINUATION_PATH),
        "sha256": sha256_file(CONTINUATION_PATH),
        "continuation": continuation,
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
    same_state = route.get("same_state_handoff") or {}
    require(same_state.get("observed") == 1, "same-state handoff must stay observed")
    require(same_state.get("selected_existing_contracts_consumed") == 1, "existing contracts must be consumed")
    require(same_state.get("fallback_to_standalone_harness") == 0, "standalone harness fallback must stay off")
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
        "path": rel(ROUTE_PATH),
        "sha256": sha256_file(ROUTE_PATH),
        "route": route,
        "same_state": same_state,
        "claims": claims,
    }


def classify_next_unconsumed_edge(report: dict[str, Any]) -> dict[str, Any]:
    first_gap = report.get("first_executable_materialization_gap") or {}
    decomposition = report.get("materialization_decomposition") or {}

    if (
        first_gap.get("edge_id") == EXPECTED_PREFIX_EDGE_ID
        and decomposition.get("owner_kind") == "CompletionDesignStopReached"
        and (decomposition.get("ordered_child_owners") or []) == []
    ):
        classification = "Closed"
    else:
        classification = "Unknown"

    return {
        "edge_id": first_gap.get("edge_id"),
        "callsite": first_gap.get("callsite"),
        "classification": classification,
        "required_capability": first_gap.get("required_capability"),
        "reason_token": first_gap.get("reason"),
        "next_slice_token": first_gap.get("next_slice_token"),
        "owner_scope": "integration",
    }


def build_prefix_advance() -> dict[str, Any]:
    state = parse_current_state()
    report_info = validate_semantic_closure_report()
    continuation = validate_continuation()
    route_info = validate_route()
    task_order = validate_task_order()
    role_ssot = validate_role_ssot()
    design_stop = validate_design_stop_contract()

    route = route_info["route"]
    same_state = route_info["same_state"]
    prefix = continuation["continuation"].get("continuation") or {}
    next_edge = classify_next_unconsumed_edge(report_info["report"])

    prefix_advance = {
        "kind": "ContinueComposedExecutionPrefix",
        "prefix_state": "Green",
        "first_composition_red_edge": None,
        "stable_reason_token": prefix.get("stable_reason_token"),
        "stable_next_slice_token": prefix.get("stable_next_slice_token"),
        "owner_scope": "integration",
        "next_unconsumed_edge_classification": next_edge["classification"],
    }

    result = {
        "schema_version": 0,
        "kind": "MinimalMirBuilderExecutionPathComposedPrefixAdvanceV1",
        "prefix_scope": "SameStateComposedExecutionPrefix",
        "input_profile": {"ast": "ASTNode::Literal(Integer(0))"},
        "source_authority": {
            "semantic_closure_report": {
                "path": report_info["path"],
                "sha256": report_info["sha256"],
            },
            "composed_execution_continuation": {
                "path": continuation["path"],
                "sha256": continuation["sha256"],
            },
            "composed_execution_route": {
                "path": route_info["path"],
                "sha256": route_info["sha256"],
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
        "prefix_advance": prefix_advance,
        "next_unconsumed_edge": next_edge,
        "selected_evidence": [
            {
                "kind": "semantic_closure_report",
                "path": report_info["path"],
                "sha256": report_info["sha256"],
                "prefix_state": "Green",
            },
            {
                "kind": "composed_execution_continuation",
                "path": continuation["path"],
                "sha256": continuation["sha256"],
                "prefix_state": prefix.get("prefix_state"),
            },
            {
                "kind": "composed_execution_route",
                "path": route_info["path"],
                "sha256": route_info["sha256"],
                "same_state_handoff_observed": same_state.get("observed"),
            },
        ],
        "claims": {
            "existing_evidence_consumed": 1,
            "manual_next_edge_selection": 0,
            "resolver_output_is_derived": 1,
            "next_unconsumed_edge_classified": 1,
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

    require(result["prefix_advance"]["stable_next_slice_token"] == EXPECTED_STABLE_NEXT_SLICE_TOKEN, "stable next slice token drift")
    require(result["prefix_advance"]["prefix_state"] == "Green", "prefix state drift")
    require(result["next_unconsumed_edge"]["classification"] == "Closed", "next unconsumed edge classification drift")
    require(result["next_unconsumed_edge"]["edge_id"] == EXPECTED_PREFIX_EDGE_ID, "next unconsumed edge id drift")
    require(result["same_state_handoff"]["observed"] == 1, "same-state handoff must stay observed")
    require(result["prefix_advance"]["first_composition_red_edge"] is None, "green prefix must not claim a red edge")
    return result


def run(*, check: bool) -> None:
    prefix_advance = build_prefix_advance()
    prefix_text = stable_json(prefix_advance)
    if check:
        if not OUTPUT_PATH.exists() or OUTPUT_PATH.read_text(encoding="utf-8") != prefix_text:
            raise PrefixAdvanceError(f"{rel(OUTPUT_PATH)} is stale")
    else:
        write_if_changed(OUTPUT_PATH, prefix_text)

    print("output_contract=rust-lifecycle-minimal-path-composed-prefix-advance-v0")
    print("prefix_advance_guard=green")
    print(f"stable_next_slice_token={prefix_advance['prefix_advance']['stable_next_slice_token']}")
    print(f"stable_reason_token={prefix_advance['prefix_advance']['stable_reason_token']}")
    print(f"next_unconsumed_edge_classification={prefix_advance['next_unconsumed_edge']['classification']}")
    print(f"same_state_handoff_observed={prefix_advance['same_state_handoff']['observed']}")
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
    except PrefixAdvanceError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()

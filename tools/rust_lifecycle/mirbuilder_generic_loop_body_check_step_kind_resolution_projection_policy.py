#!/usr/bin/env python3
"""Resolve GenericLoop body-check step-kind validation dispatch policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
TAIL_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-v0.json"
IN_BODY_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-v0.json"
CONTINUE_IF_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-continue-if-step-validation-projection-policy-v0.json"
BREAK_ELSE_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-break-else-if-step-validation-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy-v0.json"
TOKEN = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def require_markers(source: str, markers: list[str], label: str) -> list[str]:
    missing = [marker for marker in markers if marker not in source]
    if missing:
        raise SystemExit(f"source marker drift for {label}: {missing}")
    return markers


def build_policy() -> dict[str, Any]:
    tail_policy = read_json(TAIL_POLICY)
    in_body_policy = read_json(IN_BODY_POLICY)
    continue_if_policy = read_json(CONTINUE_IF_POLICY)
    break_else_policy = read_json(BREAK_ELSE_POLICY)
    if break_else_policy["decision"]["selected_next_card"] != TOKEN:
        raise SystemExit("break-else-if step validation does not select step-kind resolution")

    v0_source = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v0.rs"
    v1_source = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs"
    step_types_source = "src/mir/builder/control_flow/generic_loop_canon/types.rs"
    v0_text = read_source(v0_source)
    v1_text = read_source(v1_source)
    step_types_text = read_source(step_types_source)

    return {
        "schema_version": 0,
        "kind": "MirBuilderGenericLoopBodyCheckStepKindResolutionProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "tail_control_flow_probe_policy": rel(TAIL_POLICY),
            "in_body_step_validation_policy": rel(IN_BODY_POLICY),
            "continue_if_step_validation_policy": rel(CONTINUE_IF_POLICY),
            "break_else_if_step_validation_policy": rel(BREAK_ELSE_POLICY),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.generic_loop_body_check_step_kind_resolution",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "step_placement_variants": {
            "source_path": step_types_source,
            "variants": [
                "Last",
                "InBody",
                "InContinueIf",
                "InBreakElseIf",
            ],
            "source_markers": require_markers(
                step_types_text,
                [
                    "enum StepPlacement",
                    "Last",
                    "InBody(usize)",
                    "InContinueIf(usize)",
                    "InBreakElseIf(usize)",
                ],
                "StepPlacement",
            ),
        },
        "dispatch_resolution": {
            "v0": {
                "source_path": v0_source,
                "reject_reason_handoff": "reject_or_none(strict, reason.as_freeze_message())",
                "planner_required_in_body_probe": "has_control_flow_after_step(&flat_body, idx)",
                "dispatch_table": [
                    {
                        "placement": "InBody",
                        "validator": "validate_in_body_step",
                        "policy_fixture": rel(IN_BODY_POLICY),
                    },
                    {
                        "placement": "InContinueIf",
                        "validator": "validate_continue_if_step",
                        "policy_fixture": rel(CONTINUE_IF_POLICY),
                    },
                    {
                        "placement": "InBreakElseIf",
                        "validator": "validate_break_else_if_step",
                        "policy_fixture": rel(BREAK_ELSE_POLICY),
                    },
                    {
                        "placement": "LastOrOther",
                        "validator": "accept_without_step_validator",
                        "policy_fixture": None,
                    },
                ],
                "source_markers": require_markers(
                    v0_text,
                    [
                        "StepPlacement::InBody(idx)",
                        "validate_in_body_step(&flat_body, idx, loop_var, &loop_increment, strict)",
                        "StepPlacement::InContinueIf(idx)",
                        "validate_continue_if_step(&flat_body, idx, strict)",
                        "StepPlacement::InBreakElseIf(idx)",
                        "validate_break_else_if_step(&flat_body, idx, strict)",
                        "_ => true",
                    ],
                    "v0 step-kind dispatch",
                ),
            },
            "v1": {
                "source_path": v1_source,
                "reject_reason_handoff": "StepResolutionErr::Freeze",
                "planner_required_in_body_probe": "has_control_flow_after_step(flat_body, idx)",
                "dispatch_table": [
                    {
                        "placement": "InBody",
                        "validator": "validate_in_body_step_v1",
                        "policy_fixture": rel(IN_BODY_POLICY),
                    },
                    {
                        "placement": "InContinueIf",
                        "validator": "validate_continue_if_step",
                        "policy_fixture": rel(CONTINUE_IF_POLICY),
                    },
                    {
                        "placement": "InBreakElseIf",
                        "validator": "validate_break_else_if_step",
                        "policy_fixture": rel(BREAK_ELSE_POLICY),
                    },
                    {
                        "placement": "LastOrOther",
                        "validator": "accept_without_step_validator",
                        "policy_fixture": None,
                    },
                ],
                "source_markers": require_markers(
                    v1_text,
                    [
                        "StepPlacement::InBody(idx)",
                        "validate_in_body_step_v1(flat_body, idx, loop_var, &loop_increment, strict)",
                        "StepPlacement::InContinueIf(idx)",
                        "validate_continue_if_step(flat_body, idx, strict)",
                        "StepPlacement::InBreakElseIf(idx)",
                        "validate_break_else_if_step(flat_body, idx, strict)",
                        "_ => true",
                    ],
                    "v1 step-kind dispatch",
                ),
            },
        },
        "consumed_validator_descriptors": {
            "tail_probe_descriptor": tail_policy["probe_descriptor"]["descriptor_id"],
            "in_body_descriptor": in_body_policy["validator_descriptor"]["descriptor_id"],
            "continue_if_descriptor": continue_if_policy["validator_descriptor"]["descriptor_id"],
            "break_else_if_descriptor": break_else_policy["validator_descriptor"]["descriptor_id"],
        },
        "selected_policy": {
            "policy": "SourceExtractedStepKindDispatchResolution",
            "owner_edge": "mirbuilder::generic_loop_body_check_step_kind_resolution",
            "dispatch_resolution_selected": True,
            "hako_projection_selected": False,
            "reason_token": "StepKindDispatchResolutionRequiredBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectDispatchResolutionPolicy",
            "selected_next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001",
            "reason_token": "StepKindResolutionMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "dispatch_resolution_selected": 1,
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
        print("mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

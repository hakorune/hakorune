#!/usr/bin/env python3
"""Close out GenericLoop body-check step validation descriptor chain."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PARENT_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-step-validation-projection-policy-v0.json"
TAIL_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-v0.json"
IN_BODY_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-v0.json"
CONTINUE_IF_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-continue-if-step-validation-projection-policy-v0.json"
BREAK_ELSE_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-break-else-if-step-validation-projection-policy-v0.json"
STEP_KIND_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-generic-loop-body-check-step-validation-closeout-v0.json"
TOKEN = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def expect_decision(path: Path, expected_next: str) -> dict[str, Any]:
    data = read_json(path)
    actual = data["decision"]["selected_next_card"]
    if actual != expected_next:
        raise SystemExit(f"{rel(path)} selected_next_card drift: {actual}")
    return data


def build_closeout() -> dict[str, Any]:
    parent = expect_decision(PARENT_POLICY, "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001")
    tail = expect_decision(TAIL_POLICY, "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001")
    in_body = expect_decision(IN_BODY_POLICY, "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTINUE-IF-STEP-VALIDATION-PROJECTION-POLICY-001")
    continue_if = expect_decision(CONTINUE_IF_POLICY, "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-BREAK-ELSE-IF-STEP-VALIDATION-PROJECTION-POLICY-001")
    break_else = expect_decision(BREAK_ELSE_POLICY, "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001")
    step_kind = expect_decision(STEP_KIND_POLICY, TOKEN)

    consumed_descriptors = {
        "tail_probe_descriptor": tail["probe_descriptor"]["descriptor_id"],
        "in_body_descriptor": in_body["validator_descriptor"]["descriptor_id"],
        "continue_if_descriptor": continue_if["validator_descriptor"]["descriptor_id"],
        "break_else_if_descriptor": break_else["validator_descriptor"]["descriptor_id"],
        "step_kind_dispatch": step_kind["selected_policy"]["policy"],
    }
    expected_descriptors = {
        "tail_probe_descriptor": "generic_loop_body_check_tail_control_flow_probe_v1",
        "in_body_descriptor": "generic_loop_body_check_in_body_step_validation_v1",
        "continue_if_descriptor": "generic_loop_body_check_continue_if_step_validation_v1",
        "break_else_if_descriptor": "generic_loop_body_check_break_else_if_step_validation_v1",
        "step_kind_dispatch": "SourceExtractedStepKindDispatchResolution",
    }
    if consumed_descriptors != expected_descriptors:
        raise SystemExit(f"consumed descriptor drift: {consumed_descriptors}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderGenericLoopBodyCheckStepValidationCloseoutV1",
        "token": TOKEN,
        "input_state": {
            "parent_step_validation_policy": rel(PARENT_POLICY),
            "tail_control_flow_probe_policy": rel(TAIL_POLICY),
            "in_body_step_validation_policy": rel(IN_BODY_POLICY),
            "continue_if_step_validation_policy": rel(CONTINUE_IF_POLICY),
            "break_else_if_step_validation_policy": rel(BREAK_ELSE_POLICY),
            "step_kind_resolution_policy": rel(STEP_KIND_POLICY),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "closed_subcluster": {
            "subcluster_id": "BodyCheckStepValidation",
            "parent_token": parent["token"],
            "materialized_leaf_count": 4,
            "dispatch_resolution_selected": True,
            "closed_reason_token": "AllStepValidationDescriptorsMaterialized",
        },
        "consumed_descriptors": consumed_descriptors,
        "closeout_boundary": {
            "docs_only_closeout": 0,
            "machine_checkable_fixture": 1,
            "hako_projection_selected": 0,
            "next_owner_returned_to_priority_resolver": 1,
        },
        "decision": {
            "kind": "CloseSubclusterAndReturnToPriorityResolver",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "BodyCheckStepValidationClosed",
        },
        "claims": {
            "manual_family_selection": 0,
            "docs_only_closeout": 0,
            "machine_checkable_fixture": 1,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in closeout fixture.")
    args = parser.parse_args()

    output = stable_json(build_closeout())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-generic-loop-body-check-step-validation-closeout unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

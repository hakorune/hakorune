#!/usr/bin/env python3
"""Resolve GenericLoop in-body step validation projection policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
STEP_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-step-validation-projection-policy-v0.json"
TAIL_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-v0.json"
TOKEN = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001"
SUBCLUSTER_ID = "InBodyStepValidation"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def require_markers(source: str, markers: list[str], symbol: str) -> list[str]:
    missing = [marker for marker in markers if marker not in source]
    if missing:
        raise SystemExit(f"source marker drift for {symbol}: {missing}")
    return markers


def build_policy() -> dict[str, Any]:
    step_policy = read_json(STEP_POLICY)
    tail_policy = read_json(TAIL_POLICY)
    surfaces = [
        surface
        for surface in step_policy["source_surfaces"]
        if surface["step_validation_subcluster_id"] == SUBCLUSTER_ID
    ]
    expected_symbols = ["validate_in_body_step", "validate_in_body_step_v1"]
    if [surface["symbol"] for surface in surfaces] != expected_symbols:
        raise SystemExit(f"unexpected in-body step validation surfaces: {surfaces}")
    if tail_policy["decision"]["selected_next_card"] != TOKEN:
        raise SystemExit("tail control-flow probe does not select in-body step validation")

    source_text = read_source(surfaces[0]["source_path"])
    validator_descriptors = [
        {
            "symbol": "validate_in_body_step",
            "version": "v0",
            "strict_continue_policy": "RejectAnyContinueInBodyBeforeTailScan",
            "tail_scan": "body[(step_index + 1)..]",
            "allowed_tail_statements": [
                "simple_assignment_to_loop_var",
                "local_init_loop_var",
                "effect_only_stmt",
            ],
            "reject_reasons": [
                "InBodyStepWithContinue",
                "ControlFlowAfterInBodyStep",
                "ExitAfterInBodyStep",
                "LoopVarUsedAfterInBodyStep",
                "UnsupportedStmtAfterInBodyStep",
            ],
            "log_tags": ["generic_loop", "generic_loop_v0"],
            "source_markers": require_markers(
                source_text,
                [
                    "body_has_continue(body)",
                    "RejectReason::InBodyStepWithContinue",
                    "generic_loop_v0",
                    "RejectReason::UnsupportedStmtAfterInBodyStep",
                    "reject_or_false",
                ],
                "validate_in_body_step",
            ),
        },
        {
            "symbol": "validate_in_body_step_v1",
            "version": "v1",
            "strict_continue_policy": "AllowContinueBeforeTailScan",
            "tail_scan": "body[(step_index + 1)..]",
            "allowed_tail_statements": [
                "simple_assignment_to_loop_var",
                "local_init_loop_var",
                "effect_only_stmt",
            ],
            "reject_reasons": [
                "ControlFlowAfterInBodyStep",
                "ExitAfterInBodyStep",
                "LoopVarUsedAfterInBodyStep",
                "UnsupportedStmtAfterInBodyStep",
            ],
            "log_tags": ["generic_loop_v1"],
            "source_markers": require_markers(
                source_text,
                [
                    "generic_loop_v1",
                    "RejectReason::ControlFlowAfterInBodyStep",
                    "RejectReason::ExitAfterInBodyStep",
                    "RejectReason::LoopVarUsedAfterInBodyStep",
                    "RejectReason::UnsupportedStmtAfterInBodyStep",
                    "reject_or_false",
                ],
                "validate_in_body_step_v1",
            ),
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderGenericLoopBodyCheckInBodyStepValidationProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "step_validation_policy": rel(STEP_POLICY),
            "previous_policy": rel(TAIL_POLICY),
            "selected_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "source_module": surfaces[0]["source_path"],
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.generic_loop_body_check_in_body_step_validation",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": [
            {
                "source_id": surface["source_id"],
                "symbol": surface["symbol"],
                "source_path": surface["source_path"],
                "line": surface["line"],
                "params": surface["params"],
                "return_type": surface["return_type"],
                "validator_role": "strict_reject_tail_statement_validator",
            }
            for surface in surfaces
        ],
        "validator_descriptor": {
            "descriptor_id": "generic_loop_body_check_in_body_step_validation_v1",
            "source_extraction": "rust_strict_reject_tail_validation",
            "entry_count": len(validator_descriptors),
            "return_contract": "Result<bool, Freeze>",
            "reject_dispatch": "reject_or_false(strict, reason.as_freeze_message())",
            "handoff_table": "handoff_tables::for_generic_loop",
            "entries": validator_descriptors,
        },
        "selected_policy": {
            "policy": "SourceExtractedStrictRejectValidationDescriptor",
            "owner_edge": "mirbuilder::generic_loop_body_check_in_body_step_validation",
            "validator_descriptor_selected": True,
            "hako_projection_selected": False,
            "reason_token": "InBodyStepValidationRequiresStrictRejectDescriptorBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectValidatorDescriptorPolicy",
            "selected_next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTINUE-IF-STEP-VALIDATION-PROJECTION-POLICY-001",
            "reason_token": "InBodyStepValidationDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "validator_descriptor_selected": 1,
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
        print("mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

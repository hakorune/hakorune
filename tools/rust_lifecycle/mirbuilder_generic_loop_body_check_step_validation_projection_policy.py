#!/usr/bin/env python3
"""Decompose GenericLoop body-check step validation before projection."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
GENERIC_LOOP_DECOMPOSITION = FIXTURES / "mirbuilder-generic-loop-plan-subcluster-decomposition-v0.json"
TRIM_DECOMPOSITION = FIXTURES / "mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition-v0.json"
OUTPUT = FIXTURES / "mirbuilder-generic-loop-body-check-step-validation-projection-policy-v0.json"
TOKEN = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001"
SOURCE_SUBCLUSTER_ID = "BodyCheckStepValidation"


STEP_VALIDATION_SUBCLUSTERS: dict[str, dict[str, Any]] = {
    "TailControlFlowProbe": {
        "symbols": {"has_control_flow_after_step"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001",
        "reason_token": "TailControlFlowProbeIsPureBoolScan",
        "priority": 0,
    },
    "InBodyStepValidation": {
        "symbols": {"validate_in_body_step", "validate_in_body_step_v1"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001",
        "reason_token": "InBodyStepValidationCarriesStrictRejectSemantics",
        "priority": 1,
    },
    "ContinueIfStepValidation": {
        "symbols": {"validate_continue_if_step"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTINUE-IF-STEP-VALIDATION-PROJECTION-POLICY-001",
        "reason_token": "ContinueIfStepValidationHasTrailingExitContract",
        "priority": 2,
    },
    "BreakElseIfStepValidation": {
        "symbols": {"validate_break_else_if_step"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-BREAK-ELSE-IF-STEP-VALIDATION-PROJECTION-POLICY-001",
        "reason_token": "BreakElseIfStepValidationHasFinalStatementContract",
        "priority": 3,
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def subcluster_for(symbol: str) -> str:
    matches = [
        name
        for name, definition in STEP_VALIDATION_SUBCLUSTERS.items()
        if symbol in definition["symbols"]
    ]
    if len(matches) != 1:
        raise SystemExit(f"unclassified or ambiguous step validation symbol: {symbol}")
    return matches[0]


def source_markers_for(symbol: str, source_text: str) -> list[str]:
    markers_by_symbol = {
        "has_control_flow_after_step": [
            "body.iter().skip(step_index + 1)",
            "is_exit_if(stmt)",
            "ASTNode::Break",
            "ASTNode::Continue",
            "ASTNode::Return",
        ],
        "validate_in_body_step": [
            "body_has_continue(body)",
            "RejectReason::InBodyStepWithContinue",
            "generic_loop_v0",
            "reject_or_false",
        ],
        "validate_in_body_step_v1": [
            "generic_loop_v1",
            "RejectReason::ControlFlowAfterInBodyStep",
            "stmt_uses_loop_var",
            "is_effect_only_stmt",
        ],
        "validate_continue_if_step": [
            "ContinueIfStepRequiresTrailingExit",
            "tail.len() == 1",
            "ASTNode::Break",
            "ASTNode::Return",
        ],
        "validate_break_else_if_step": [
            "BreakElseStepMustBeFinalStmt",
            "step_index + 1 == body.len()",
            "reject_or_false",
        ],
    }
    return [marker for marker in markers_by_symbol[symbol] if marker in source_text]


def build_policy() -> dict[str, Any]:
    decomposition = read_json(GENERIC_LOOP_DECOMPOSITION)
    trim = read_json(TRIM_DECOMPOSITION)
    surfaces = [
        surface
        for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SOURCE_SUBCLUSTER_ID
    ]
    expected_symbols = [
        "has_control_flow_after_step",
        "validate_in_body_step",
        "validate_in_body_step_v1",
        "validate_continue_if_step",
        "validate_break_else_if_step",
    ]
    if [surface["symbol"] for surface in surfaces] != expected_symbols:
        raise SystemExit(f"unexpected step validation surfaces: {surfaces}")
    if trim["decision"]["selected_next_card"] != TOKEN:
        raise SystemExit("trim condition decomposition does not select step validation")

    source_text = read_source(surfaces[0]["source_path"])
    source_surfaces = []
    counts: Counter[str] = Counter()
    for surface in surfaces:
        subcluster = subcluster_for(surface["symbol"])
        counts[subcluster] += 1
        source_surfaces.append({
            "source_id": surface["source_id"],
            "symbol": surface["symbol"],
            "source_path": surface["source_path"],
            "line": surface["line"],
            "params": surface["params"],
            "return_type": surface["return_type"],
            "step_validation_subcluster_id": subcluster,
            "source_markers": source_markers_for(surface["symbol"], source_text),
        })

    subclusters = []
    for name, definition in sorted(STEP_VALIDATION_SUBCLUSTERS.items(), key=lambda item: item[1]["priority"]):
        members = [
            surface
            for surface in source_surfaces
            if surface["step_validation_subcluster_id"] == name
        ]
        subclusters.append({
            "step_validation_subcluster_id": name,
            "source_count": len(members),
            "symbols": [member["symbol"] for member in members],
            "classification": "BodyCheckStepValidationSubcluster",
            "next_owner_kind": definition["next_owner_kind"],
            "next_card": definition["next_card"],
            "reason_token": definition["reason_token"],
            "selection_eligible": name == "TailControlFlowProbe",
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderGenericLoopBodyCheckStepValidationProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "generic_loop_decomposition": rel(GENERIC_LOOP_DECOMPOSITION),
            "previous_decomposition": rel(TRIM_DECOMPOSITION),
            "source_subcluster_id": SOURCE_SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "source_module": surfaces[0]["source_path"],
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.generic_loop_body_check_step_validation",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": source_surfaces,
        "step_validation_subclusters": subclusters,
        "step_validation_subcluster_counts": dict(sorted(counts.items())),
        "decomposition_policy": {
            "whole_step_validation_projection_selected": False,
            "module_role_decomposition": True,
            "strict_reject_semantics_isolated": True,
            "candidate_count_as_proof": 0,
            "mixed_responsibility_reason": "PureTailProbeAndStrictRejectValidatorsMustNotShareOneProjectionPolicy",
        },
        "decision": {
            "kind": "SelectStepValidationSubcluster",
            "selected_step_validation_subcluster_id": "TailControlFlowProbe",
            "selected_next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001",
            "reason_token": "SelectPureTailProbeBeforeStrictRejectValidators",
        },
        "claims": {
            "manual_family_selection": 0,
            "whole_step_validation_projection": 0,
            "projection_surface_selected": 0,
            "candidate_count_as_proof": 0,
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
        print("mirbuilder-generic-loop-body-check-step-validation-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

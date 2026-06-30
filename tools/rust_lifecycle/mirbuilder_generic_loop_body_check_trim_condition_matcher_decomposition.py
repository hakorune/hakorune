#!/usr/bin/env python3
"""Resolve GenericLoop body-check trim-condition matcher decomposition."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
EXPR_MATCHERS_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-expr-matchers-projection-policy-v0.json"
CALL_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-call-expr-matchers-projection-policy-v0.json"
COMPARE_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-compare-expr-matchers-projection-policy-v0.json"
CONTROL_POLICY = FIXTURES / "mirbuilder-generic-loop-body-check-control-return-expr-matchers-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition-v0.json"
TOKEN = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001"
SUBCLUSTER_ID = "CompositeTrimConditionMatcher"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def require_markers(source: str, markers: list[str]) -> list[str]:
    missing = [marker for marker in markers if marker not in source]
    if missing:
        raise SystemExit(f"source marker drift for trim condition matcher: {missing}")
    return markers


def descriptor_id(policy: dict[str, Any]) -> str:
    return policy["matcher_descriptor"]["descriptor_id"]


def build_decomposition() -> dict[str, Any]:
    expr_policy = read_json(EXPR_MATCHERS_POLICY)
    call_policy = read_json(CALL_POLICY)
    compare_policy = read_json(COMPARE_POLICY)
    control_policy = read_json(CONTROL_POLICY)
    surfaces = [
        surface
        for surface in expr_policy["source_surfaces"]
        if surface["expr_matcher_subcluster_id"] == SUBCLUSTER_ID
    ]
    if [surface["symbol"] for surface in surfaces] != ["matches_trim_cond_with_methodcall"]:
        raise SystemExit(f"unexpected trim condition matcher surfaces: {surfaces}")

    surface = surfaces[0]
    source_text = read_source(surface["source_path"])
    source_markers = require_markers(
        source_text,
        [
            "BinaryOperator::And",
            "matches_loop_var_compare",
            "matches_is_space_call",
            "left.as_ref()",
            "right.as_ref()",
        ],
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderGenericLoopBodyCheckTrimConditionMatcherDecompositionV1",
        "token": TOKEN,
        "input_state": {
            "expr_matchers_policy": rel(EXPR_MATCHERS_POLICY),
            "call_matcher_policy": rel(CALL_POLICY),
            "compare_matcher_policy": rel(COMPARE_POLICY),
            "previous_policy": rel(CONTROL_POLICY),
            "selected_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "source_module": surface["source_path"],
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.generic_loop_body_check_trim_condition_matcher",
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
                "matcher_role": "composite_binary_and_predicate",
            }
        ],
        "composition_descriptor": {
            "descriptor_id": "generic_loop_body_check_trim_condition_matcher_v1",
            "source_extraction": "rust_binary_and_composition",
            "ast_root": "ASTNode::BinaryOp",
            "operator": "BinaryOperator::And",
            "commutative_conjuncts": True,
            "left_or_right_roles": [
                {
                    "role": "loop_var_relational_compare",
                    "required_matcher": "matches_loop_var_compare",
                    "source_descriptor": descriptor_id(compare_policy),
                },
                {
                    "role": "space_call_with_substring_window",
                    "required_matcher": "matches_is_space_call",
                    "source_descriptor": descriptor_id(call_policy),
                },
            ],
            "source_markers": source_markers,
        },
        "decomposition_policy": {
            "composite_descriptor_selected": True,
            "standalone_projection_selected": False,
            "new_matcher_semantics_invented": False,
            "uses_existing_compare_descriptor": True,
            "uses_existing_call_descriptor": True,
            "reason_token": "TrimConditionMatcherIsCompositionOfCompareAndCallDescriptors",
        },
        "decision": {
            "kind": "SelectNextGenericLoopPlanSubcluster",
            "selected_next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001",
            "reason_token": "BodyCheckExpressionMatchersResolved",
        },
        "claims": {
            "manual_family_selection": 0,
            "composite_descriptor_selected": 1,
            "standalone_projection_selected": 0,
            "new_matcher_semantics_invented": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in decomposition fixture.")
    args = parser.parse_args()

    output = stable_json(build_decomposition())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Resolve GenericLoop body-check compare expression matcher projection policy."""

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
OUTPUT = FIXTURES / "mirbuilder-generic-loop-body-check-compare-expr-matchers-projection-policy-v0.json"
TOKEN = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-COMPARE-EXPR-MATCHERS-PROJECTION-POLICY-001"
SUBCLUSTER_ID = "CompareExprMatchers"


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
    expr_policy = read_json(EXPR_MATCHERS_POLICY)
    surfaces = [
        surface
        for surface in expr_policy["source_surfaces"]
        if surface["expr_matcher_subcluster_id"] == SUBCLUSTER_ID
    ]
    expected_symbols = [
        "matches_loop_var_less_than_literal",
        "matches_loop_var_equal_literal",
        "matches_loop_var_compare",
    ]
    if [surface["symbol"] for surface in surfaces] != expected_symbols:
        raise SystemExit(f"unexpected compare matcher surfaces: {surfaces}")

    source_text = read_source(surfaces[0]["source_path"])
    for surface in surfaces:
        if read_source(surface["source_path"]) != source_text:
            raise SystemExit("compare matcher surfaces must share one source module")

    matcher_descriptors = [
        {
            "symbol": "matches_loop_var_less_than_literal",
            "rust_pattern": "BinaryOp(Less, loop_var, integer literal value)",
            "ast_root": "ASTNode::BinaryOp",
            "operator_set": ["Less"],
            "operand_policy": "LoopVarIntegerLiteralCommutative",
            "literal_type": "LiteralValue::Integer",
            "source_markers": require_markers(
                source_text,
                ["BinaryOperator::Less", "LiteralValue::Integer", "is_loop_var", "is_literal"],
                "matches_loop_var_less_than_literal",
            ),
        },
        {
            "symbol": "matches_loop_var_equal_literal",
            "rust_pattern": "BinaryOp(Equal, loop_var, integer literal value)",
            "ast_root": "ASTNode::BinaryOp",
            "operator_set": ["Equal"],
            "operand_policy": "LoopVarIntegerLiteralCommutative",
            "literal_type": "LiteralValue::Integer",
            "source_markers": require_markers(
                source_text,
                ["BinaryOperator::Equal", "LiteralValue::Integer", "is_loop_var", "is_literal"],
                "matches_loop_var_equal_literal",
            ),
        },
        {
            "symbol": "matches_loop_var_compare",
            "rust_pattern": "BinaryOp(relational, loop_var, any variable)",
            "ast_root": "ASTNode::BinaryOp",
            "operator_set": ["Less", "Greater", "LessEqual", "GreaterEqual"],
            "operand_policy": "LoopVarAnyVariableCommutative",
            "literal_type": "None",
            "source_markers": require_markers(
                source_text,
                [
                    "BinaryOperator::Less",
                    "BinaryOperator::Greater",
                    "BinaryOperator::LessEqual",
                    "BinaryOperator::GreaterEqual",
                    "is_var",
                    "is_loop_var",
                ],
                "matches_loop_var_compare",
            ),
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderGenericLoopBodyCheckCompareExprMatchersProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "expr_matchers_policy": rel(EXPR_MATCHERS_POLICY),
            "previous_policy": rel(CALL_POLICY),
            "selected_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "source_module": surfaces[0]["source_path"],
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.generic_loop_body_check_compare_expr_matchers",
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
                "matcher_role": "binary_op_predicate",
            }
            for surface in surfaces
        ],
        "matcher_descriptor": {
            "descriptor_id": "generic_loop_body_check_compare_expr_matchers_v1",
            "source_extraction": "rust_binary_op_patterns",
            "entry_count": len(matcher_descriptors),
            "ast_root": "ASTNode::BinaryOp",
            "entries": matcher_descriptors,
        },
        "selected_policy": {
            "policy": "SourceExtractedCompareMatcherDescriptor",
            "owner_edge": "mirbuilder::generic_loop_body_check_compare_expr_matchers",
            "matcher_descriptor_selected": True,
            "hako_projection_selected": False,
            "reason_token": "CompareExpressionMatchersRequireDescriptorBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectMatcherDescriptorPolicy",
            "selected_next_card": "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTROL-RETURN-EXPR-MATCHERS-PROJECTION-POLICY-001",
            "reason_token": "CompareExpressionMatcherDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "matcher_descriptor_selected": 1,
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
        print("mirbuilder-generic-loop-body-check-compare-expr-matchers-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

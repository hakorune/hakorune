#!/usr/bin/env python3
"""Project literal integer lowering from live Rust source.

This is a plan-only capability slice for `LiteralValue::Integer` through
`emit_integer`. It does not claim general expression lowering, finalize behavior,
generated Hako, backend routes, or runtime behavior.
"""

from __future__ import annotations

import argparse
import re
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-literal-integer-lowering-plan-v0.json"
)
BUILDER_BUILD = ROOT / "src/mir/builder/builder_build.rs"
CONSTANT_EMISSION = ROOT / "src/mir/builder/emission/constant.rs"
FRONTEND_LITERAL = ROOT / "crates/hakorune_frontend_ast/src/literal.rs"
MIR_TYPES = ROOT / "crates/hakorune_mir_core/src/types.rs"
MIR_INSTRUCTION = ROOT / "src/mir/instruction.rs"


def _read(path: Path) -> str:
    return path.read_text()


def _function_body(source: str, signature: str) -> str:
    start = source.find(signature)
    require(start >= 0, f"missing function signature: {signature}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing function body brace: {signature}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace + 1 : index]
    raise AssertionError(f"unterminated function body: {signature}")


def _enum_body(source: str, name: str) -> str:
    marker = f"pub enum {name}"
    start = source.find(marker)
    require(start >= 0, f"missing enum: {name}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing enum body brace: {name}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace + 1 : index]
    raise AssertionError(f"unterminated enum: {name}")


def _variant_decl(enum_body: str, variant: str) -> str:
    pattern = re.compile(rf"{re.escape(variant)}\s*(?P<shape>\([^,\n]+\))?")
    match = pattern.search(enum_body)
    require(match is not None, f"missing variant: {variant}")
    return match.group(0).strip()


def extract_plan() -> dict[str, Any]:
    builder_build = _read(BUILDER_BUILD)
    constant_emission = _read(CONSTANT_EMISSION)
    frontend_literal = _read(FRONTEND_LITERAL)
    mir_types = _read(MIR_TYPES)
    mir_instruction = _read(MIR_INSTRUCTION)

    build_literal = _function_body(builder_build, "fn build_literal(&mut self, literal: LiteralValue)")
    emit_integer = _function_body(constant_emission, "pub fn emit_integer")
    literal_body = _enum_body(frontend_literal, "LiteralValue")
    const_body = _enum_body(mir_types, "ConstValue")
    mir_type_body = _enum_body(mir_types, "MirType")
    instruction_body = _enum_body(mir_instruction, "MirInstruction")

    require("LiteralValue::Integer(n)" in build_literal, "build_literal no longer matches Integer")
    require(
        "crate::mir::builder::emission::constant::emit_integer(self, n)?" in build_literal,
        "build_literal Integer no longer delegates to emit_integer",
    )
    require(_variant_decl(literal_body, "Integer") == "Integer(i64)", "LiteralValue::Integer type drift")
    require(_variant_decl(const_body, "Integer") == "Integer(i64)", "ConstValue::Integer type drift")
    require(re.search(r"\bInteger\b", mir_type_body), "MirType::Integer missing")
    for marker in [
        "let dst = b.next_value_id();",
        "MirInstruction::Const",
        "dst,",
        "value: ConstValue::Integer(val),",
        "b.type_ctx",
        ".value_types",
        ".insert(dst, crate::mir::MirType::Integer);",
        "Ok(dst)",
    ]:
        require(marker in emit_integer, f"emit_integer marker drift: {marker}")
    for marker in [
        "Const {",
        "dst: ValueId,",
        "value: ConstValue",
    ]:
        require(marker in instruction_body, f"MirInstruction::Const shape drift: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderLiteralIntegerLoweringPlanV1",
        "subject": "MirBuilder::build_literal(LiteralValue::Integer)",
        "source_authority": {
            "dispatcher": "src/mir/builder/builder_build.rs::MirBuilder::build_literal",
            "emitter": "src/mir/builder/emission/constant.rs::emit_integer",
            "frontend_literal": "crates/hakorune_frontend_ast/src/literal.rs::LiteralValue::Integer",
            "mir_const": "crates/hakorune_mir_core/src/types.rs::ConstValue::Integer",
            "mir_instruction": "src/mir/instruction.rs::MirInstruction::Const",
        },
        "selected_source_shape": {
            "input_ast": "ASTNode::Literal(Integer(0))",
            "literal_variant": "LiteralValue::Integer",
            "literal_payload_transport": "ScalarI64",
        },
        "lowering_sequence": [
            {
                "step": "allocate_result",
                "operation": "MirBuilder::next_value_id",
                "required_capability": "MirBuilderAllocationPolicy.prepared_state_next_value_id",
                "result_transport": "ValueIdAsI64",
            },
            {
                "step": "emit_const_instruction",
                "operation": "MirInstruction::Const",
                "dst": "allocated ValueId",
                "value": "ConstValue::Integer(input i64)",
                "value_transport": "ScalarI64",
            },
            {
                "step": "publish_type",
                "operation": "type_ctx.value_types.insert",
                "value_type": "MirType::Integer",
            },
            {
                "step": "return_result",
                "transport": "ValueIdAsI64",
            },
        ],
        "available_capabilities": [
            "LiteralIntegerLowering",
        ],
        "result_contract": {
            "result_value": "ValueIdAsI64",
            "emitted_instruction": "ConstValue::Integer",
            "published_type": "MirType::Integer",
        },
        "non_claims": {
            "typed_integer_literal": 0,
            "float_literal": 0,
            "bool_literal": 0,
            "string_literal": 0,
            "null_literal": 0,
            "void_literal": 0,
            "full_expression_lowering": 0,
            "finalize_module": 0,
            "return_emission": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(plan["kind"] == "MirBuilderLiteralIntegerLoweringPlanV1", "wrong literal plan kind")
    caps = set(plan["available_capabilities"])
    require("LiteralIntegerLowering" in caps, "missing literal integer capability")
    require(
        plan["selected_source_shape"]["literal_payload_transport"] == "ScalarI64",
        "literal payload transport drift",
    )
    steps = [row["step"] for row in plan["lowering_sequence"]]
    require(
        steps == ["allocate_result", "emit_const_instruction", "publish_type", "return_result"],
        f"literal lowering sequence drift: {steps}",
    )
    require(
        plan["result_contract"]["emitted_instruction"] == "ConstValue::Integer",
        "literal emitted instruction drift",
    )
    require(plan["result_contract"]["published_type"] == "MirType::Integer", "literal type drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("payload transport drift", ["selected_source_shape", "literal_payload_transport"], "RuntimeI64"),
        ("missing capability", ["available_capabilities"], []),
        ("return claim drift", ["non_claims", "return_emission"], 1),
    ]
    for label, path, value in probes:
        mutated = deepcopy(plan)
        cursor: Any = mutated
        for key in path[:-1]:
            cursor = cursor[key]
        cursor[path[-1]] = value
        try:
            validate_plan(mutated)
        except AssertionError:
            continue
        raise AssertionError(f"drift probe did not fail: {label}")


def build_plan() -> dict[str, Any]:
    plan = extract_plan()
    validate_plan(plan)
    return plan


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reference", type=Path, default=FIXTURE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    parser.add_argument("--drift-probes", action="store_true")
    args = parser.parse_args()

    plan = build_plan()
    if args.drift_probes:
        run_drift_probes(plan)

    return report_or_emit(
        facts=plan,
        reference=args.reference,
        check_reference=args.check_reference,
        emit_json=args.emit_json,
        report=[
            ("output_contract", "rust-lifecycle-mirbuilder-literal-integer-lowering-v0"),
            ("mirbuilder_literal_integer_lowering", "green"),
            ("capability", "LiteralIntegerLowering"),
            ("value_transport", plan["selected_source_shape"]["literal_payload_transport"]),
            ("published_type", plan["result_contract"]["published_type"]),
            ("return_emission_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

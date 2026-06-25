#!/usr/bin/env python3
"""Project finalize_module return emission from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the edge that
connects the lowered result value to a `MirInstruction::Return` terminator.
It does not claim return-type publication, full finalize behavior, generated
Hako, backend routes, or runtime behavior.
"""

from __future__ import annotations

import argparse
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-return-emission-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
BASIC_BLOCK = ROOT / "src/mir/basic_block.rs"
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


def _require_order(text: str, markers: list[str], label: str) -> list[dict[str, Any]]:
    cursor = -1
    rows: list[dict[str, Any]] = []
    for marker in markers:
        index = text.find(marker, cursor + 1)
        require(index >= 0, f"{label}: missing or out-of-order marker: {marker}")
        rows.append({"marker": marker, "byte_offset": index})
        cursor = index
    return rows


def extract_plan() -> dict[str, Any]:
    lifecycle = _read(MODULE_LIFECYCLE)
    basic_block = _read(BASIC_BLOCK)
    mir_instruction = _read(MIR_INSTRUCTION)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    add_instruction = _function_body(basic_block, "pub fn add_instruction(&mut self, instruction: MirInstruction)")
    add_instruction_with_span = _function_body(
        basic_block, "pub fn add_instruction_with_span(&mut self, instruction: MirInstruction, span: Span)"
    )
    instruction_body = _enum_body(mir_instruction, "MirInstruction")

    order = _require_order(
        finalize,
        [
            "if let Some(block_id) = self.current_block",
            "if let Some(ref mut function) = self.scope_ctx.current_function",
            "if let Some(block) = function.get_block_mut(block_id)",
            "if !block.is_terminated()",
            "block.add_instruction(MirInstruction::Return",
            "value: Some(result_value)",
            "function.signature.return_type = mt;",
        ],
        "MirBuilder::finalize_module return edge",
    )

    for marker in [
        "self.add_instruction_with_span(instruction, Span::unknown());",
    ]:
        require(marker in add_instruction, f"BasicBlock::add_instruction marker drift: {marker}")
    for marker in [
        "if self.is_terminator(&instruction)",
        "self.terminator = Some(instruction);",
        "self.terminator_span = Some(span);",
        "self.update_successors_from_terminator();",
    ]:
        require(
            marker in add_instruction_with_span,
            f"BasicBlock::add_instruction_with_span marker drift: {marker}",
        )
    for marker in [
        "Return {",
        "value: Option<ValueId>",
    ]:
        require(marker in instruction_body, f"MirInstruction::Return shape drift: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnEmissionPlanV1",
        "subject": "MirBuilder::finalize_module append Return(result_value)",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "basic_block": "src/mir/basic_block.rs::BasicBlock::add_instruction",
            "mir_instruction": "src/mir/instruction.rs::MirInstruction::Return",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "current_block": "Present",
            "current_function": "Present",
            "target_block": "Present",
            "target_block_terminated": False,
            "result_value_transport": "ValueIdAsI64",
        },
        "observed_source_order": order,
        "emission_sequence": [
            {
                "step": "select_current_block",
                "source": "self.current_block",
                "transport": "BasicBlockIdAsI64",
            },
            {
                "step": "select_current_function",
                "source": "self.scope_ctx.current_function",
                "observation": "presence_and_mutable_payload",
            },
            {
                "step": "select_target_block",
                "operation": "function.get_block_mut(block_id)",
            },
            {
                "step": "guard_unterminated",
                "predicate": "not block.is_terminated()",
            },
            {
                "step": "append_return",
                "operation": "block.add_instruction(MirInstruction::Return)",
                "value": "Some(result_value)",
            },
            {
                "step": "publish_terminator",
                "owner": "BasicBlock::add_instruction_with_span",
                "terminator": "MirInstruction::Return",
            },
        ],
        "available_capabilities": [
            "ReturnEmission",
        ],
        "result_contract": {
            "terminator": "MirInstruction::Return",
            "value": "Some(result_value)",
            "value_transport": "ValueIdAsI64",
            "successors": "Empty",
        },
        "non_claims": {
            "return_type_publication": 0,
            "full_finalize_module": 0,
            "other_terminator_shapes": 0,
            "already_terminated_block_behavior": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(plan["kind"] == "MirBuilderReturnEmissionPlanV1", "wrong return emission plan kind")
    require("ReturnEmission" in plan["available_capabilities"], "missing ReturnEmission capability")
    require(
        plan["execution_profile"]["result_value_transport"] == "ValueIdAsI64",
        "result value transport drift",
    )
    steps = [row["step"] for row in plan["emission_sequence"]]
    require(
        steps
        == [
            "select_current_block",
            "select_current_function",
            "select_target_block",
            "guard_unterminated",
            "append_return",
            "publish_terminator",
        ],
        f"return emission sequence drift: {steps}",
    )
    require(plan["result_contract"]["terminator"] == "MirInstruction::Return", "terminator drift")
    require(plan["result_contract"]["value"] == "Some(result_value)", "return value drift")
    require(plan["result_contract"]["successors"] == "Empty", "return successor drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("transport drift", ["execution_profile", "result_value_transport"], "RawI64"),
        ("return-type claim drift", ["non_claims", "return_type_publication"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-return-emission-v0"),
            ("mirbuilder_return_emission", "green"),
            ("capability", "ReturnEmission"),
            ("terminator", plan["result_contract"]["terminator"]),
            ("value_transport", plan["result_contract"]["value_transport"]),
            ("return_type_publication_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

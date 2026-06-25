#!/usr/bin/env python3
"""Project finalize_module return-type publication from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the edge that
publishes `type_ctx.value_types[result_value]` into the current function
signature return type. It does not claim module take/sealing, full finalize,
generated Hako, backend routes, or runtime behavior.
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
    / "mirbuilder-return-type-publication-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
FUNCTION_TYPES = ROOT / "src/mir/function/types.rs"
LITERAL_INTEGER_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-literal-integer-lowering-plan-v0.json"
)
RETURN_EMISSION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-return-emission-plan-v0.json"
)


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


def _struct_body(source: str, name: str) -> str:
    marker = f"pub struct {name}"
    start = source.find(marker)
    require(start >= 0, f"missing struct: {name}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing struct body brace: {name}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace + 1 : index]
    raise AssertionError(f"unterminated struct: {name}")


def _require_order(text: str, markers: list[str], label: str) -> list[dict[str, Any]]:
    cursor = -1
    rows: list[dict[str, Any]] = []
    for marker in markers:
        index = text.find(marker, cursor + 1)
        require(index >= 0, f"{label}: missing or out-of-order marker: {marker}")
        rows.append({"marker": marker, "byte_offset": index})
        cursor = index
    return rows


def _read_json(path: Path) -> dict[str, Any]:
    import json

    return json.loads(path.read_text())


def extract_plan() -> dict[str, Any]:
    lifecycle = _read(MODULE_LIFECYCLE)
    function_types = _read(FUNCTION_TYPES)
    literal_plan = _read_json(LITERAL_INTEGER_PLAN)
    return_emission_plan = _read_json(RETURN_EMISSION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    signature_body = _struct_body(function_types, "FunctionSignature")

    order = _require_order(
        finalize,
        [
            "if let Some(mt) = self.type_ctx.value_types.get(&result_value).cloned()",
            "function.signature.return_type = mt;",
            "let mut module = self.current_module.take().unwrap();",
        ],
        "MirBuilder::finalize_module return-type publication",
    )

    require(
        "return_type: MirType" in signature_body,
        "FunctionSignature return_type field shape drift",
    )
    require(
        literal_plan.get("result_contract", {}).get("published_type") == "MirType::Integer",
        "literal integer plan must publish MirType::Integer",
    )
    require(
        return_emission_plan.get("non_claims", {}).get("return_type_publication") == 0,
        "ReturnEmission must not claim return type publication",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnTypePublicationPlanV1",
        "subject": "MirBuilder::finalize_module publish return type from result_value",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "function_signature": "src/mir/function/types.rs::FunctionSignature",
            "input_type_source": "type_ctx.value_types[result_value]",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "current_function": "Present",
            "result_value_transport": "ValueIdAsI64",
            "result_value_type": "MirType::Integer",
            "initial_function_return_type": "MirType::Void",
        },
        "observed_source_order": order,
        "publication_sequence": [
            {
                "step": "lookup_result_type",
                "source": "self.type_ctx.value_types.get(&result_value).cloned()",
                "value_transport": "ValueIdAsI64",
            },
            {
                "step": "publish_signature_return_type",
                "target": "function.signature.return_type",
                "value": "lookup_result_type",
            },
        ],
        "available_capabilities": [
            "ReturnTypePublication",
        ],
        "result_contract": {
            "signature_return_type": "MirType::Integer",
            "source_value_type": "type_ctx.value_types[result_value]",
            "source_value_type_owner": "LiteralIntegerLowering",
        },
        "non_claims": {
            "module_take": 0,
            "verify_typed_values": 0,
            "full_finalize_module": 0,
            "phi_return_type_inference": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(
        plan["kind"] == "MirBuilderReturnTypePublicationPlanV1",
        "wrong return type publication plan kind",
    )
    require(
        "ReturnTypePublication" in plan["available_capabilities"],
        "missing ReturnTypePublication capability",
    )
    profile = plan["execution_profile"]
    require(profile["result_value_transport"] == "ValueIdAsI64", "result value transport drift")
    require(profile["result_value_type"] == "MirType::Integer", "result value type drift")
    steps = [row["step"] for row in plan["publication_sequence"]]
    require(
        steps == ["lookup_result_type", "publish_signature_return_type"],
        f"return type publication sequence drift: {steps}",
    )
    contract = plan["result_contract"]
    require(contract["signature_return_type"] == "MirType::Integer", "signature return type drift")
    require(
        contract["source_value_type_owner"] == "LiteralIntegerLowering",
        "source value type owner drift",
    )
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("transport drift", ["execution_profile", "result_value_transport"], "RawI64"),
        ("module-take claim drift", ["non_claims", "module_take"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-return-type-publication-v0"),
            ("mirbuilder_return_type_publication", "green"),
            ("capability", "ReturnTypePublication"),
            ("signature_return_type", plan["result_contract"]["signature_return_type"]),
            ("module_take_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Project finalize_module current_function take from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the edge that
transports the prepared `scope_ctx.current_function` into finalize via
`scope_ctx.current_function.take().unwrap()`. It does not claim type
propagation, type-hint provision, PHI inference, full finalize, generated Hako,
backend routes, or runtime behavior.
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
    / "mirbuilder-current-function-take-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
FUNCTION_CONSTRUCTOR_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mir-function-constructor-composition-plan-v0.json"
)
TYPED_VALUE_VERIFICATION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-typed-value-verification-plan-v0.json"
)


def _read(path: Path) -> str:
    return path.read_text()


def _read_json(path: Path) -> dict[str, Any]:
    import json

    return json.loads(path.read_text())


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
    function_constructor = _read_json(FUNCTION_CONSTRUCTOR_PLAN)
    typed_value_verification = _read_json(TYPED_VALUE_VERIFICATION_PLAN)
    prepare = _function_body(lifecycle, "pub(super) fn prepare_module(&mut self) -> Result<(), String>")
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    prepare_order = _require_order(
        prepare,
        [
            "let mut main_function = self.new_function_with_metadata(main_signature, entry_block);",
            "main_function.metadata.is_entry_point = true;",
            "self.scope_ctx.current_function = Some(main_function);",
        ],
        "MirBuilder::prepare_module current_function install",
    )
    finalize_order = _require_order(
        finalize,
        [
            "verify_typed_values_are_defined",
            "let mut function = self.scope_ctx.current_function.take().unwrap();",
            "TypePropagationPipeline::run",
        ],
        "MirBuilder::finalize_module current_function take",
    )

    require(
        "MirFunctionConstructorTransport"
        in set(function_constructor.get("available_capabilities") or []),
        "function constructor plan must provide MirFunctionConstructorTransport",
    )
    require(
        "PreparedStateInstall" in set(function_constructor.get("available_capabilities") or []),
        "function constructor plan must provide PreparedStateInstall",
    )
    require(
        typed_value_verification.get("non_claims", {}).get("current_function_take") == 0,
        "TypedValueDefinitionVerification must not claim current_function take",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCurrentFunctionTakePlanV1",
        "subject": "MirBuilder::finalize_module scope_ctx.current_function.take().unwrap()",
        "source_authority": {
            "prepare": "src/mir/builder/module_lifecycle.rs::MirBuilder::prepare_module",
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "function_constructor_plan": "mir-function-constructor-composition-plan-v0.json",
            "predecessor_plan": "mirbuilder-typed-value-verification-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "current_function": "Present",
            "function_transport": "MirFunctionPreparedMain",
        },
        "observed_source_order": {
            "prepare_module": prepare_order,
            "finalize_module": finalize_order,
        },
        "take_sequence": [
            {
                "step": "install_prepared_main_function",
                "source": "prepare_module",
                "operation": "self.scope_ctx.current_function = Some(main_function)",
            },
            {
                "step": "take_prepared_main_function",
                "source": "finalize_module",
                "operation": "self.scope_ctx.current_function.take().unwrap()",
            },
        ],
        "available_capabilities": [
            "CurrentFunctionTake",
        ],
        "result_contract": {
            "taken_value": "MirFunctionPreparedMain",
            "source_state": "self.scope_ctx.current_function",
            "post_take_state": "None",
            "local_binding": "function",
        },
        "non_claims": {
            "type_propagation": 0,
            "type_hint_provision": 0,
            "metadata_value_type_publication": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_function_insertion": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(
        plan["kind"] == "MirBuilderCurrentFunctionTakePlanV1",
        "wrong current function take plan kind",
    )
    require("CurrentFunctionTake" in plan["available_capabilities"], "missing CurrentFunctionTake capability")
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    steps = [row["step"] for row in plan["take_sequence"]]
    require(
        steps == ["install_prepared_main_function", "take_prepared_main_function"],
        f"take sequence drift: {steps}",
    )
    contract = plan["result_contract"]
    require(contract["taken_value"] == "MirFunctionPreparedMain", "taken function transport drift")
    require(contract["post_take_state"] == "None", "post-take state drift")
    require(contract["local_binding"] == "function", "local binding drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("transport drift", ["execution_profile", "function_transport"], "RawFunction"),
        ("type-propagation claim drift", ["non_claims", "type_propagation"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-current-function-take-v0"),
            ("mirbuilder_current_function_take", "green"),
            ("capability", "CurrentFunctionTake"),
            ("function_transport", plan["execution_profile"]["function_transport"]),
            ("type_propagation_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

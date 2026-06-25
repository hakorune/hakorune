#!/usr/bin/env python3
"""Project finalize_module typed-value verification from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the
`verify_typed_values_are_defined(self, "finalize_module")` boundary: typed
ValueIds must be defined by the current function or be function parameters
before finalization proceeds. It does not claim current-function take, type
propagation, PHI inference, full finalize, generated Hako, backend routes, or
runtime behavior.
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
    / "mirbuilder-typed-value-verification-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
VALUE_LIFECYCLE = ROOT / "src/mir/builder/emission/value_lifecycle.rs"
CURRENT_MODULE_TAKE_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-current-module-take-plan-v0.json"
)
LITERAL_INTEGER_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-literal-integer-lowering-plan-v0.json"
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
    value_lifecycle = _read(VALUE_LIFECYCLE)
    current_module_take = _read_json(CURRENT_MODULE_TAKE_PLAN)
    literal_integer = _read_json(LITERAL_INTEGER_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    verifier = _function_body(
        value_lifecycle,
        "pub(in crate::mir::builder) fn verify_typed_values_are_defined",
    )

    finalize_order = _require_order(
        finalize,
        [
            "let mut module = self.current_module.take().unwrap();",
            "verify_typed_values_are_defined",
            '"finalize_module"',
            "let mut function = self.scope_ctx.current_function.take().unwrap();",
        ],
        "MirBuilder::finalize_module typed-value verification",
    )
    verifier_order = _require_order(
        verifier,
        [
            "if !strict_or_dev_planner_required()",
            "builder.scope_ctx.current_function.as_ref()",
            "let def_blocks = compute_def_blocks(func);",
            "let param_set = &func.params;",
            "let mut missing: Vec<(ValueId, MirType)>",
            "collect_referenced_values(func)",
            "builder.pending_phis",
            "builder.pin_slot_names",
            "builder.type_ctx.value_types.remove(&v);",
            "Err(format!(",
        ],
        "value_lifecycle::verify_typed_values_are_defined",
    )

    require(
        current_module_take.get("non_claims", {}).get("verify_typed_values") == 0,
        "CurrentModuleTake must not claim typed-value verification",
    )
    require(
        literal_integer.get("result_contract", {}).get("published_type") == "MirType::Integer",
        "literal integer plan must publish MirType::Integer",
    )
    require(
        "ValueId::INVALID" in verifier,
        "typed-value verifier must explicitly exclude ValueId::INVALID",
    )
    require(
        "[freeze:contract][value_lifecycle/typed_without_def]" in verifier,
        "typed-value verifier must keep stable fail-fast tag",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderTypedValueVerificationPlanV1",
        "subject": "MirBuilder::finalize_module verify typed values are defined",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "verifier": "src/mir/builder/emission/value_lifecycle.rs::verify_typed_values_are_defined",
            "predecessor_plan": "mirbuilder-current-module-take-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "current_function": "Present",
            "type_context": "type_ctx.value_types",
            "strict_gate": "strict_or_dev_planner_required",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "verifier": verifier_order,
        },
        "verification_contract": {
            "typed_values": "builder.type_ctx.value_types",
            "definition_sources": [
                "compute_def_blocks(func)",
                "func.params",
            ],
            "excluded_value": "ValueId::INVALID",
            "fatal_missing_if_referenced_by": [
                "collect_referenced_values(func)",
                "builder.pending_phis",
                "builder.pin_slot_names",
            ],
            "stale_missing_cleanup": [
                "type_ctx.value_types",
                "type_ctx.value_kinds",
                "type_ctx.value_origin_newbox",
            ],
            "fail_fast_tag": "[freeze:contract][value_lifecycle/typed_without_def]",
        },
        "available_capabilities": [
            "TypedValueDefinitionVerification",
        ],
        "result_contract": {
            "boundary": "typed ValueIds are defined or parameters before function take",
            "fatal_missing_behavior": "Err(freeze:contract)",
            "stale_missing_behavior": "prune builder-side stale type entries",
            "minimal_path_expected_result": "Ok",
        },
        "non_claims": {
            "current_function_take": 0,
            "type_propagation": 0,
            "type_hint_provision": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_metadata_publication": 0,
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
        plan["kind"] == "MirBuilderTypedValueVerificationPlanV1",
        "wrong typed-value verification plan kind",
    )
    require(
        "TypedValueDefinitionVerification" in plan["available_capabilities"],
        "missing TypedValueDefinitionVerification capability",
    )
    profile = plan["execution_profile"]
    require(profile["current_function"] == "Present", "current function presence drift")
    contract = plan["verification_contract"]
    require(
        contract["typed_values"] == "builder.type_ctx.value_types",
        "typed-values source drift",
    )
    require(
        contract["definition_sources"] == ["compute_def_blocks(func)", "func.params"],
        "definition sources drift",
    )
    require(contract["excluded_value"] == "ValueId::INVALID", "invalid sentinel drift")
    require(
        contract["fail_fast_tag"] == "[freeze:contract][value_lifecycle/typed_without_def]",
        "fail-fast tag drift",
    )
    result = plan["result_contract"]
    require(result["minimal_path_expected_result"] == "Ok", "minimal path expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("definition source drift", ["verification_contract", "definition_sources"], ["compute_def_blocks(func)"]),
        ("current-function-take claim drift", ["non_claims", "current_function_take"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-typed-value-verification-v0"),
            ("mirbuilder_typed_value_verification", "green"),
            ("capability", "TypedValueDefinitionVerification"),
            ("typed_values", plan["verification_contract"]["typed_values"]),
            ("definition_sources", ",".join(plan["verification_contract"]["definition_sources"])),
            ("current_function_take_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

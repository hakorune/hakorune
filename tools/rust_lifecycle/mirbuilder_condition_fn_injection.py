#!/usr/bin/env python3
"""Project finalize_module condition_fn injection from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the source-required
`condition_fn` stub injection when missing. It does not claim region cleanup,
metadata publication, semantic refresh, all-functions PHI materialization, full
finalize, generated Hako, backend routes, or runtime behavior.
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
    / "mirbuilder-condition-fn-injection-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
MODULE_FUNCTION_INSERTION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-module-function-insertion-plan-v0.json"
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
    module_function = _read_json(MODULE_FUNCTION_INSERTION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    finalize_order = _require_order(
        finalize,
        [
            "module.add_function(function);",
            'if module.functions.get("condition_fn").is_none()',
            'name: "condition_fn".to_string(),',
            "params: vec![MirType::Integer]",
            "return_type: MirType::Integer",
            "effects: EffectMask::PURE",
            "let entry = BasicBlockId::new(0);",
            "let mut f = self.new_function_with_metadata(sig, entry);",
            "crate::mir::function_emission::emit_const_integer(&mut f, entry, 1);",
            "crate::mir::function_emission::emit_return_value(&mut f, entry, one);",
            "module.add_function(f);",
            "crate::mir::region::observer::pop_function_region(self);",
        ],
        "MirBuilder::finalize_module condition_fn injection",
    )
    require(
        module_function.get("non_claims", {}).get("condition_fn_injection") == 0,
        "ModuleFunctionInsertion must not claim condition_fn injection",
    )
    for marker in [
        "Dev stub: provide condition_fn when missing",
        "Returns integer 1 (truthy) and accepts one argument (unused).",
        "ここでは追加の next_value_id()/params.push() は行わず",
        "FunctionEmissionBox",
    ]:
        require(marker in finalize, f"condition_fn source marker missing: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderConditionFnInjectionPlanV1",
        "subject": "MirBuilder::finalize_module condition_fn injection",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "predecessor_plan": "mirbuilder-module-function-insertion-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "module_transport": "MirModuleMinimalShell",
            "condition_fn_initially_missing": True,
            "context": "finalize_module",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
        },
        "injection": {
            "predicate": "module.functions.get(\"condition_fn\").is_none()",
            "function_name": "condition_fn",
            "params": ["MirType::Integer"],
            "return_type": "MirType::Integer",
            "effects": "EffectMask::PURE",
            "entry_block": "BasicBlockId(0)",
            "body": [
                "ConstInteger(1)",
                "ReturnValue(one)",
            ],
            "insert_operation": "module.add_function(f)",
            "required_by_source": True,
        },
        "available_capabilities": [
            "ConditionFnInjection",
        ],
        "result_contract": {
            "mutates": [
                "module.functions",
            ],
            "entrypoint": "MirBuilder::finalize_module condition_fn injection block",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "condition_fn_policy_generalization": 0,
            "region_stack_pop": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
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
        plan["kind"] == "MirBuilderConditionFnInjectionPlanV1",
        "wrong condition_fn injection plan kind",
    )
    require(
        "ConditionFnInjection" in plan["available_capabilities"],
        "missing ConditionFnInjection capability",
    )
    profile = plan["execution_profile"]
    require(profile["module_transport"] == "MirModuleMinimalShell", "module transport drift")
    require(profile["condition_fn_initially_missing"] is True, "condition_fn profile drift")
    require(profile["context"] == "finalize_module", "injection context drift")
    injection = plan["injection"]
    require(
        injection["predicate"] == 'module.functions.get("condition_fn").is_none()',
        "predicate drift",
    )
    require(injection["function_name"] == "condition_fn", "function name drift")
    require(injection["params"] == ["MirType::Integer"], "param type drift")
    require(injection["return_type"] == "MirType::Integer", "return type drift")
    require(injection["effects"] == "EffectMask::PURE", "effect drift")
    require(injection["body"] == ["ConstInteger(1)", "ReturnValue(one)"], "body drift")
    require(injection["insert_operation"] == "module.add_function(f)", "insert operation drift")
    require(injection["required_by_source"] is True, "source requirement drift")
    result = plan["result_contract"]
    require(result["mutates"] == ["module.functions"], "mutation frame drift")
    require(
        result["entrypoint"] == "MirBuilder::finalize_module condition_fn injection block",
        "entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("body drift", ["injection", "body"], ["ConstInteger(0)", "ReturnValue(one)"]),
        ("region cleanup claim drift", ["non_claims", "region_stack_pop"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-condition-fn-injection-v0"),
            ("mirbuilder_condition_fn_injection", "green"),
            ("capability", "ConditionFnInjection"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("function_name", plan["injection"]["function_name"]),
            ("region_stack_pop_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

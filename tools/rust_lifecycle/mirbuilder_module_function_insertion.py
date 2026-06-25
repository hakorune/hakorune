#!/usr/bin/env python3
"""Project finalize_module main-function insertion from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the first
`module.add_function(function)` call and the `MirModule::add_function`
name-keyed insertion behavior. It does not claim condition_fn injection,
all-functions PHI materialization, region cleanup, metadata publication,
semantic refresh, full finalize, generated Hako, backend routes, or runtime
behavior.
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
    / "mirbuilder-module-function-insertion-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
MODULE_IMPL = ROOT / "src/mir/function/module_impl.rs"
DEV_BIRTH_VERIFICATION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-dev-birth-verification-plan-v0.json"
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
    module_impl = _read(MODULE_IMPL)
    dev_birth = _read_json(DEV_BIRTH_VERIFICATION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    add_function = _function_body(module_impl, "pub fn add_function(&mut self, function: MirFunction)")

    finalize_order = _require_order(
        finalize,
        [
            "if crate::config::env::using_is_dev()",
            "module.add_function(function);",
            'if module.functions.get("condition_fn").is_none()',
        ],
        "MirBuilder::finalize_module module function insertion",
    )
    add_function_order = _require_order(
        add_function,
        [
            "let name = function.signature.name.clone();",
            "self.functions.insert(name, function);",
        ],
        "MirModule::add_function",
    )
    require(
        dev_birth.get("non_claims", {}).get("module_function_insertion") == 0,
        "DevBirthVerification must not claim module function insertion",
    )
    require(
        finalize.count("module.add_function(function);") == 1,
        "finalize_module must have exactly one main function insertion call",
    )
    require(
        'if module.functions.get("condition_fn").is_none()' in finalize,
        "condition_fn insertion must remain after main function insertion",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderModuleFunctionInsertionPlanV1",
        "subject": "MirBuilder::finalize_module module.add_function(function)",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "module_add_function": "src/mir/function/module_impl.rs::MirModule::add_function",
            "predecessor_plan": "mirbuilder-dev-birth-verification-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "module_transport": "MirModuleMinimalShell",
            "function_transport": "MirFunctionPreparedMain",
            "context": "finalize_module",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "module_add_function": add_function_order,
        },
        "insertion": {
            "callsite": "module.add_function(function)",
            "inserted_function": "MirFunctionPreparedMain",
            "key_source": "function.signature.name.clone()",
            "container": "MirModule.functions",
            "container_operation": "BTreeMap::insert",
            "collision_policy": "ReplaceExistingByName",
        },
        "available_capabilities": [
            "ModuleFunctionInsertion",
        ],
        "result_contract": {
            "mutates": [
                "module.functions",
            ],
            "entrypoint": "MirModule::add_function",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "condition_fn_injection": 0,
            "all_functions_phi_materialization": 0,
            "region_stack_pop": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
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
        plan["kind"] == "MirBuilderModuleFunctionInsertionPlanV1",
        "wrong module function insertion plan kind",
    )
    require(
        "ModuleFunctionInsertion" in plan["available_capabilities"],
        "missing ModuleFunctionInsertion capability",
    )
    profile = plan["execution_profile"]
    require(profile["module_transport"] == "MirModuleMinimalShell", "module transport drift")
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["context"] == "finalize_module", "insertion context drift")
    insertion = plan["insertion"]
    require(insertion["callsite"] == "module.add_function(function)", "callsite drift")
    require(insertion["key_source"] == "function.signature.name.clone()", "key source drift")
    require(insertion["container"] == "MirModule.functions", "container drift")
    require(insertion["container_operation"] == "BTreeMap::insert", "container operation drift")
    require(insertion["collision_policy"] == "ReplaceExistingByName", "collision policy drift")
    result = plan["result_contract"]
    require(result["mutates"] == ["module.functions"], "mutation frame drift")
    require(result["entrypoint"] == "MirModule::add_function", "entrypoint drift")
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("collision policy drift", ["insertion", "collision_policy"], "Append"),
        ("condition_fn claim drift", ["non_claims", "condition_fn_injection"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-module-function-insertion-v0"),
            ("mirbuilder_module_function_insertion", "green"),
            ("capability", "ModuleFunctionInsertion"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("key_source", plan["insertion"]["key_source"]),
            ("condition_fn_injection_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

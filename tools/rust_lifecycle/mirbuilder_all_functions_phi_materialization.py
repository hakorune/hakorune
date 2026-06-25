#!/usr/bin/env python3
"""Project finalize_module all-functions PHI materialization from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the finalize
sweep over `module.functions.values_mut()` that delegates to the existing
`phi_input_materializer::materialize_all_phi_inputs` provider with the
`finalize_module_all_functions` context. It does not re-own the PHI
materializer internals, full finalize, generated Hako, backend routes, or
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
    / "mirbuilder-all-functions-phi-materialization-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
DIRECT_STATE_PLAN_REFRESH_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-direct-state-plan-refresh-plan-v0.json"
)
PHI_INPUT_MATERIALIZATION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-phi-input-materialization-plan-v0.json"
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
    direct_state = _read_json(DIRECT_STATE_PLAN_REFRESH_PLAN)
    phi_materialization = _read_json(PHI_INPUT_MATERIALIZATION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    finalize_order = _require_order(
        finalize,
        [
            "crate::mir::direct_state_plan::refresh_module_direct_state_plans(&mut module);",
            "for function in module.functions.values_mut()",
            "crate::mir::builder::ssa::phi_input_materializer::materialize_all_phi_inputs",
            '"finalize_module_all_functions"',
            "Ok(module)",
        ],
        "MirBuilder::finalize_module all-functions PHI materialization",
    )
    require(
        direct_state.get("non_claims", {}).get("all_functions_phi_materialization") == 0,
        "DirectStatePlanRefresh must not claim all_functions_phi_materialization",
    )
    require(
        "PhiInputMaterialization" in set(phi_materialization.get("available_capabilities") or []),
        "PhiInputMaterialization provider capability missing",
    )
    require(
        phi_materialization.get("result_contract", {}).get("entrypoint")
        == "phi_input_materializer::materialize_all_phi_inputs",
        "PHI materialization provider entrypoint drift",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderAllFunctionsPhiMaterializationPlanV1",
        "subject": "MirBuilder::finalize_module all-functions PHI materialization",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "delegate_plan": "mirbuilder-phi-input-materialization-plan-v0.json",
            "predecessor_plan": "mirbuilder-direct-state-plan-refresh-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "context": "finalize_module",
            "module_transport": "MirModuleMinimalShell",
            "function_collection": "module.functions.values_mut()",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
        },
        "sweep_policy": {
            "iteration": "for function in module.functions.values_mut()",
            "delegate": "phi_input_materializer::materialize_all_phi_inputs",
            "delegate_context": "finalize_module_all_functions",
            "delegate_capability": "PhiInputMaterialization",
            "error_transport": "ResultPropagatedByQuestionMark",
        },
        "available_capabilities": [
            "AllFunctionsPhiMaterialization",
        ],
        "result_contract": {
            "mutates": [
                "module.functions[*].blocks",
                "module.functions[*].next_value_id",
            ],
            "entrypoint": "MirBuilder::finalize_module all-functions PHI materialization sweep",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
            "source_selfhost_claim": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(
        plan["kind"] == "MirBuilderAllFunctionsPhiMaterializationPlanV1",
        "wrong all-functions PHI materialization plan kind",
    )
    require(
        "AllFunctionsPhiMaterialization" in plan["available_capabilities"],
        "missing AllFunctionsPhiMaterialization capability",
    )
    profile = plan["execution_profile"]
    require(profile["context"] == "finalize_module", "context drift")
    require(profile["module_transport"] == "MirModuleMinimalShell", "module transport drift")
    require(
        profile["function_collection"] == "module.functions.values_mut()",
        "function collection drift",
    )
    sweep = plan["sweep_policy"]
    require(
        sweep["iteration"] == "for function in module.functions.values_mut()",
        "sweep iteration drift",
    )
    require(
        sweep["delegate"] == "phi_input_materializer::materialize_all_phi_inputs",
        "sweep delegate drift",
    )
    require(
        sweep["delegate_context"] == "finalize_module_all_functions",
        "sweep context drift",
    )
    require(sweep["delegate_capability"] == "PhiInputMaterialization", "delegate capability drift")
    require(sweep["error_transport"] == "ResultPropagatedByQuestionMark", "error transport drift")
    result = plan["result_contract"]
    require(
        result["mutates"] == [
            "module.functions[*].blocks",
            "module.functions[*].next_value_id",
        ],
        "mutation frame drift",
    )
    require(
        result["entrypoint"]
        == "MirBuilder::finalize_module all-functions PHI materialization sweep",
        "result entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("delegate drift", ["sweep_policy", "delegate"], "custom_materializer"),
        ("context drift", ["sweep_policy", "delegate_context"], "finalize_module"),
        ("generated Hako claim drift", ["non_claims", "generated_hako_artifact"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-all-functions-phi-materialization-v0"),
            ("mirbuilder_all_functions_phi_materialization", "green"),
            ("capability", "AllFunctionsPhiMaterialization"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("delegate", plan["sweep_policy"]["delegate"]),
            ("delegate_context", plan["sweep_policy"]["delegate_context"]),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

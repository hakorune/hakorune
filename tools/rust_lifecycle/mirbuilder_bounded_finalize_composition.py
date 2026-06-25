#!/usr/bin/env python3
"""Project bounded finalize_module composition from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It records the live
finalize_module sequence required by that profile without claiming full
finalize behavior, generated Hako, backend routes, or runtime behavior.
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
    / "mirbuilder-bounded-finalize-composition-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
MODULE_IMPL = ROOT / "src/mir/function/module_impl.rs"
TYPE_PROPAGATION = ROOT / "src/mir/type_propagation/pipeline.rs"
SEMANTIC_REFRESH = ROOT / "src/mir/semantic_refresh.rs"


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
    type_propagation = _read(TYPE_PROPAGATION)
    semantic_refresh = _read(SEMANTIC_REFRESH)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    add_function = _function_body(module_impl, "pub fn add_function(&mut self, function: MirFunction)")

    order = _require_order(
        finalize,
        [
            "self.hint_scope_leave(0);",
            "if let Some(block_id) = self.current_block",
            "block.add_instruction(MirInstruction::Return",
            "function.signature.return_type = mt;",
            "let mut module = self.current_module.take().unwrap();",
            "verify_typed_values_are_defined",
            "let mut function = self.scope_ctx.current_function.take().unwrap();",
            "TypePropagationPipeline::run(&mut function, &mut self.type_ctx.value_types)?;",
            "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
            "function.metadata.value_types = self.type_ctx.value_types.clone();",
            "function.metadata.value_origin_callers = origin_callers;",
            "phi_type_inference::infer_return_type_from_phi",
            "materialize_all_phi_inputs",
            "if crate::config::env::using_is_dev()",
            "module.add_function(function);",
            'if module.functions.get("condition_fn").is_none()',
            'name: "condition_fn".to_string(),',
            "crate::mir::function_emission::emit_const_integer(&mut f, entry, 1);",
            "module.add_function(f);",
            "crate::mir::region::observer::pop_function_region(self);",
            "self.comp_ctx.current_slot_registry = None;",
            "module.metadata.user_box_decls = self.comp_ctx.user_defined_boxes.clone();",
            "module.metadata.record_decls = self.comp_ctx.record_decls.clone().into_iter().collect();",
            "module.metadata.enum_decls = self.comp_ctx.enum_decls_for_module_metadata();",
            "refresh_module_record_and_packed_layout_plans(&mut module);",
            "refresh_module_typed_object_plans(&mut module);",
            "refresh_module_direct_state_plans(&mut module);",
            'materialize_all_phi_inputs(\n                function,\n                "finalize_module_all_functions",',
            "Ok(module)",
        ],
        "MirBuilder::finalize_module",
    )

    for marker in [
        "pub struct TypePropagationPipeline;",
        "固定順序**: Copy → BinOp → Copy → PHI",
        "Self::step1_copy_propagation(function, value_types)?;",
        "Self::step4_phi_type_inference(function, value_types)?;",
    ]:
        require(marker in type_propagation, f"TypePropagationPipeline marker drift: {marker}")
    for marker in [
        "refresh_module_record_layout_plans(module);",
        "refresh_module_array_record_storage_plans(module);",
        "refresh_module_source_packed_array_direct_read_consumption_plans(module);",
    ]:
        require(marker in semantic_refresh, f"record/packed refresh marker drift: {marker}")
    for marker in [
        "let name = function.signature.name.clone();",
        "self.functions.insert(name, function);",
    ]:
        require(marker in add_function, f"MirModule::add_function marker drift: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderBoundedFinalizeCompositionPlanV1",
        "subject": "MirBuilder::finalize_module(result_value)",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "module_add_function": "src/mir/function/module_impl.rs::MirModule::add_function",
            "type_propagation": "src/mir/type_propagation/pipeline.rs::TypePropagationPipeline",
            "semantic_refresh_subset": "src/mir/semantic_refresh.rs::refresh_module_record_and_packed_layout_plans",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "source_file": None,
            "dev_birth_verification": False,
            "condition_fn_initially_missing": True,
            "user_box_decls": "Empty",
            "record_decls": "Empty",
            "enum_decls": "Empty",
        },
        "observed_source_order": order,
        "composition": [
            {"step": "scope_leave", "operation": "hint_scope_leave(0)"},
            {
                "step": "append_return_if_unterminated",
                "operation": "MirInstruction::Return",
                "value": "result_value",
            },
            {
                "step": "update_return_type_from_result",
                "source": "type_ctx.value_types[result_value]",
                "target": "function.signature.return_type",
            },
            {"step": "take_module", "operation": "current_module.take().unwrap"},
            {"step": "verify_typed_values", "operation": "verify_typed_values_are_defined"},
            {"step": "take_function", "operation": "scope_ctx.current_function.take().unwrap"},
            {"step": "type_propagation", "operation": "TypePropagationPipeline::run"},
            {"step": "type_hint_provision", "operation": "annotate_missing_result_types_from_calls_and_await"},
            {"step": "publish_value_types", "target": "function.metadata.value_types"},
            {"step": "merge_origin_callers", "collision": "metadata_ctx source wins"},
            {"step": "phi_return_type_inference", "operation": "infer_return_type_from_phi"},
            {"step": "materialize_phi_inputs_main", "operation": "materialize_all_phi_inputs"},
            {"step": "dev_birth_verification", "profile": "ExcludedFalse"},
            {"step": "module_add_main_function", "operation": "MirModule::add_function"},
            {
                "step": "inject_condition_fn_if_missing",
                "operation": "condition_fn const 1 return",
                "required_by_source": True,
            },
            {"step": "pop_function_region", "operation": "region::observer::pop_function_region"},
            {"step": "clear_slot_registry", "operation": "current_slot_registry = None"},
            {"step": "publish_module_metadata", "fields": ["user_box_decls", "record_decls", "enum_decls"]},
            {
                "step": "refresh_module_plans_subset",
                "operations": [
                    "refresh_module_record_and_packed_layout_plans",
                    "refresh_module_typed_object_plans",
                    "refresh_module_direct_state_plans",
                ],
            },
            {"step": "materialize_phi_inputs_all_functions", "operation": "materialize_all_phi_inputs"},
            {"step": "return_module", "transport": "MirModuleMinimalShell"},
        ],
        "available_capabilities": [
            "FinalizeModuleComposition",
        ],
        "non_claims": {
            "full_finalize_module": 0,
            "other_root_shapes": 0,
            "condition_fn_policy_generalization": 0,
            "semantic_refresh_full_claim": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(plan["kind"] == "MirBuilderBoundedFinalizeCompositionPlanV1", "wrong finalize plan kind")
    require("FinalizeModuleComposition" in plan["available_capabilities"], "missing finalize capability")
    steps = [row["step"] for row in plan["composition"]]
    required_order = [
        "append_return_if_unterminated",
        "take_module",
        "take_function",
        "type_propagation",
        "module_add_main_function",
        "inject_condition_fn_if_missing",
        "refresh_module_plans_subset",
        "return_module",
    ]
    cursor = -1
    for step in required_order:
        index = steps.index(step)
        require(index > cursor, f"finalize step order drift: {step}")
        cursor = index
    require(
        plan["execution_profile"]["condition_fn_initially_missing"] is True,
        "condition_fn injection profile drift",
    )
    require(
        plan["composition"][14]["required_by_source"] is True,
        "condition_fn injection must remain source-required",
    )
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing finalize capability", ["available_capabilities"], []),
        ("condition_fn source requirement drift", ["composition", 14, "required_by_source"], False),
        ("full finalize claim drift", ["non_claims", "full_finalize_module"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-bounded-finalize-composition-v0"),
            ("mirbuilder_bounded_finalize_composition", "green"),
            ("capability", "FinalizeModuleComposition"),
            ("condition_fn_injection", "source_required"),
            ("full_finalize_module_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

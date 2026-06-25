#!/usr/bin/env python3
"""Project finalize_module TypePropagationPipeline from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the call to the
existing TypePropagationPipeline SSOT entry with a prepared function and the
builder value_types map. It does not claim type-hint provision, metadata
publication, PHI return-type inference, full finalize, generated Hako, backend
routes, or runtime behavior.
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
    / "mirbuilder-type-propagation-pipeline-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
TYPE_PIPELINE = ROOT / "src/mir/type_propagation/pipeline.rs"
CURRENT_FUNCTION_TAKE_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-current-function-take-plan-v0.json"
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
    pipeline_source = _read(TYPE_PIPELINE)
    current_function_take = _read_json(CURRENT_FUNCTION_TAKE_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    run_body = _function_body(
        pipeline_source,
        "pub fn run",
    )

    finalize_order = _require_order(
        finalize,
        [
            "let mut function = self.scope_ctx.current_function.take().unwrap();",
            "use crate::mir::type_propagation::TypePropagationPipeline;",
            "TypePropagationPipeline::run(&mut function, &mut self.type_ctx.value_types)?;",
            "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
        ],
        "MirBuilder::finalize_module type propagation",
    )
    pipeline_order = _require_order(
        run_body,
        [
            "Self::seed_declared_field_types(function, value_types);",
            "Self::step1_copy_propagation(function, value_types)?;",
            "Self::step2_binop_repropagation(function, value_types)?;",
            "Self::step3_copy_propagation(function, value_types)?;",
            "Self::step4_phi_type_inference(function, value_types)?;",
            "Ok(())",
        ],
        "TypePropagationPipeline::run",
    )

    require(
        current_function_take.get("non_claims", {}).get("type_propagation") == 0,
        "CurrentFunctionTake must not claim type propagation",
    )
    require(
        "fn step4_phi_type_inference" in pipeline_source,
        "TypePropagationPipeline must keep PHI inference private step",
    )
    require(
        "CopyTypePropagator::propagate" in pipeline_source,
        "TypePropagationPipeline must use CopyTypePropagator",
    )
    require(
        "PhiTypeResolver::new(function, value_types)" in pipeline_source,
        "TypePropagationPipeline must use PhiTypeResolver with function/value_types",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderTypePropagationPipelinePlanV1",
        "subject": "MirBuilder::finalize_module TypePropagationPipeline::run",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "pipeline": "src/mir/type_propagation/pipeline.rs::TypePropagationPipeline::run",
            "predecessor_plan": "mirbuilder-current-function-take-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "function_transport": "MirFunctionPreparedMain",
            "value_types": "self.type_ctx.value_types",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "pipeline": pipeline_order,
        },
        "pipeline_steps": [
            "seed_declared_field_types",
            "copy_propagation_initial",
            "binop_repropagation",
            "copy_propagation_after_binop",
            "phi_type_inference",
        ],
        "available_capabilities": [
            "TypePropagationPipelineExecution",
        ],
        "result_contract": {
            "mutates": [
                "function",
                "self.type_ctx.value_types",
            ],
            "entrypoint": "TypePropagationPipeline::run",
            "minimal_path_expected_result": "Ok",
        },
        "non_claims": {
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
        plan["kind"] == "MirBuilderTypePropagationPipelinePlanV1",
        "wrong type propagation pipeline plan kind",
    )
    require(
        "TypePropagationPipelineExecution" in plan["available_capabilities"],
        "missing TypePropagationPipelineExecution capability",
    )
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["value_types"] == "self.type_ctx.value_types", "value_types transport drift")
    require(
        plan["pipeline_steps"]
        == [
            "seed_declared_field_types",
            "copy_propagation_initial",
            "binop_repropagation",
            "copy_propagation_after_binop",
            "phi_type_inference",
        ],
        f"pipeline order drift: {plan['pipeline_steps']}",
    )
    result = plan["result_contract"]
    require(result["entrypoint"] == "TypePropagationPipeline::run", "entrypoint drift")
    require(result["minimal_path_expected_result"] == "Ok", "minimal path expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("step order drift", ["pipeline_steps"], list(reversed(plan["pipeline_steps"]))),
        ("type-hint claim drift", ["non_claims", "type_hint_provision"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-type-propagation-pipeline-v0"),
            ("mirbuilder_type_propagation_pipeline", "green"),
            ("capability", "TypePropagationPipelineExecution"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("pipeline_steps", ",".join(plan["pipeline_steps"])),
            ("type_hint_provision_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

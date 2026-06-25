#!/usr/bin/env python3
"""Project finalize_module type-hint provision from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the delegated
`type_hint_providers` pass that fills missing result types for Await and Call
instructions before metadata publication and PHI return inference. It does not
claim metadata publication, PHI return-type inference, PHI input
materialization, full finalize, generated Hako, backend routes, or runtime
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
    / "mirbuilder-type-hint-provision-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
TYPE_HINT_PROVIDERS = ROOT / "src/mir/builder/type_hint_providers.rs"
TYPE_PROPAGATION_PIPELINE_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-type-propagation-pipeline-plan-v0.json"
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
    provider_source = _read(TYPE_HINT_PROVIDERS)
    type_propagation = _read_json(TYPE_PROPAGATION_PIPELINE_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    provider_body = _function_body(
        provider_source,
        "pub(super) fn annotate_missing_result_types_from_calls_and_await",
    )

    finalize_order = _require_order(
        finalize,
        [
            "TypePropagationPipeline::run(&mut function, &mut self.type_ctx.value_types)?;",
            "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
            "function.metadata.value_types = self.type_ctx.value_types.clone();",
        ],
        "MirBuilder::finalize_module type hint provision",
    )
    provider_order = _require_order(
        provider_body,
        [
            "for (_bid, bb) in function.blocks.iter()",
            "for inst in bb.instructions.iter()",
            "MirInstruction::Await { dst, future }",
            "MirInstruction::Call",
        ],
        "type_hint_providers scan order",
    )

    require(
        type_propagation.get("non_claims", {}).get("type_hint_provision") == 0,
        "TypePropagationPipeline must not claim type hint provision",
    )
    for marker in [
        "builder.type_ctx.value_types.contains_key(dst)",
        "Some(MirType::Future(inner)) => (**inner).clone()",
        "Callee::Global(name)",
        ".map(|f| f.signature.return_type.clone())",
        "types::annotation::annotate_from_function",
        "Callee::Constructor { box_type }",
        "builder\n                                    .type_ctx\n                                    .value_origin_newbox",
        "MirType::Unknown",
        "builder.type_ctx.value_types.insert(*dst, inferred);",
    ]:
        require(marker in provider_body, f"type hint provider marker missing: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderTypeHintProvisionPlanV1",
        "subject": "MirBuilder::finalize_module type_hint_providers::annotate_missing_result_types_from_calls_and_await",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "provider": "src/mir/builder/type_hint_providers.rs::annotate_missing_result_types_from_calls_and_await",
            "predecessor_plan": "mirbuilder-type-propagation-pipeline-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "function_transport": "MirFunctionPreparedMain",
            "module_transport": "MirModulePreparedMain",
            "value_types": "self.type_ctx.value_types",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "provider": provider_order,
        },
        "provider_cases": [
            {
                "instruction": "Await",
                "result": "FutureInnerOrUnknown",
                "mutation": "InsertMissingValueType",
            },
            {
                "instruction": "Call(Global)",
                "result": "ModuleSignatureReturnOrAnnotationOrUnknown",
                "mutation": "InsertMissingValueType",
            },
            {
                "instruction": "Call(Constructor)",
                "result": "BoxType",
                "mutation": "InsertMissingValueTypeAndValueOriginNewBox",
            },
            {
                "instruction": "Call(OtherOrMissingCallee)",
                "result": "Unknown",
                "mutation": "InsertMissingValueType",
            },
        ],
        "available_capabilities": [
            "TypeHintProvision",
        ],
        "result_contract": {
            "mutates": [
                "self.type_ctx.value_types",
                "self.type_ctx.value_origin_newbox",
            ],
            "entrypoint": "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
            "minimal_path_expected_result": "OkImplicitUnit",
        },
        "non_claims": {
            "metadata_value_type_publication": 0,
            "metadata_origin_caller_merge": 0,
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
        plan["kind"] == "MirBuilderTypeHintProvisionPlanV1",
        "wrong type hint provision plan kind",
    )
    require(
        "TypeHintProvision" in plan["available_capabilities"],
        "missing TypeHintProvision capability",
    )
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["module_transport"] == "MirModulePreparedMain", "module transport drift")
    require(profile["value_types"] == "self.type_ctx.value_types", "value_types transport drift")
    cases = [case["instruction"] for case in plan["provider_cases"]]
    require(
        cases == [
            "Await",
            "Call(Global)",
            "Call(Constructor)",
            "Call(OtherOrMissingCallee)",
        ],
        f"provider case order drift: {cases}",
    )
    result = plan["result_contract"]
    require(
        result["entrypoint"]
        == "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
        "entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "OkImplicitUnit", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("provider case order drift", ["provider_cases"], list(reversed(plan["provider_cases"]))),
        ("metadata claim drift", ["non_claims", "metadata_value_type_publication"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-type-hint-provision-v0"),
            ("mirbuilder_type_hint_provision", "green"),
            ("capability", "TypeHintProvision"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("provider_cases", ",".join(case["instruction"] for case in plan["provider_cases"])),
            ("metadata_value_type_publication_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

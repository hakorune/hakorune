#!/usr/bin/env python3
"""Project finalize_module PHI return-type inference from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the delegated
`phi_type_inference::infer_return_type_from_phi` call and the resolver-chain
shape used to update `function.signature.return_type` when inference succeeds.
It does not claim PHI input materialization, module insertion, full finalize,
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
    / "mirbuilder-phi-return-type-inference-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
PHI_TYPE_INFERENCE = ROOT / "src/mir/builder/phi_type_inference.rs"
METADATA_ORIGIN_CALLER_MERGE_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-metadata-origin-caller-merge-plan-v0.json"
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
    phi_source = _read(PHI_TYPE_INFERENCE)
    origin_merge = _read_json(METADATA_ORIGIN_CALLER_MERGE_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    infer_body = _function_body(
        phi_source,
        "pub(super) fn infer_return_type_from_phi",
    )

    finalize_order = _require_order(
        finalize,
        [
            "function.metadata.value_origin_callers = origin_callers;",
            "phi_type_inference::infer_return_type_from_phi(self, &mut function)",
            "function.signature.return_type = inferred_type;",
            "phi_input_materializer::materialize_all_phi_inputs",
        ],
        "MirBuilder::finalize_module PHI return-type inference",
    )
    resolver_order = _require_order(
        infer_body,
        [
            "MirType::Void | MirType::Unknown",
            "bb.terminator",
            "builder.type_ctx.value_types.get(v).cloned()",
            "TypeHintPolicy::is_target(&function.signature.name)",
            "TypeHintPolicy::extract_phi_type_hint(&function, *v)",
            "MethodReturnHintBox::resolve_for_return",
            "PhiTypeResolver::new(&function, &builder.type_ctx.value_types)",
            "GenericTypeResolver::resolve_from_phi",
            "inferred",
        ],
        "phi_type_inference resolver chain",
    )
    require(
        origin_merge.get("non_claims", {}).get("phi_return_type_inference") == 0,
        "MetadataOriginCallerMerge must not claim PHI return-type inference",
    )
    for marker in [
        "return None; // Already has concrete type",
        "inferred = Some(mt);",
        "break;",
        "MirType::Unknown",
    ]:
        require(marker in infer_body, f"PHI inference marker missing: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderPhiReturnTypeInferencePlanV1",
        "subject": "MirBuilder::finalize_module phi_type_inference::infer_return_type_from_phi",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "provider": "src/mir/builder/phi_type_inference.rs::infer_return_type_from_phi",
            "predecessor_plan": "mirbuilder-metadata-origin-caller-merge-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "function_transport": "MirFunctionPreparedMain",
            "builder_type_context": "self.type_ctx.value_types",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "resolver": resolver_order,
        },
        "resolver_chain": [
            "SkipConcreteReturnType",
            "TerminatorReturnOnly",
            "DirectValueTypesLookup",
            "TypeHintPolicyExtract",
            "MethodReturnHintBox",
            "PhiTypeResolver",
            "GenericTypeResolver",
            "UnknownFallbackOutsideDebug",
        ],
        "available_capabilities": [
            "PhiReturnTypeInference",
        ],
        "result_contract": {
            "mutates": [
                "function.signature.return_type",
            ],
            "entrypoint": "phi_type_inference::infer_return_type_from_phi",
            "minimal_path_expected_result": "Option<MirType>",
        },
        "non_claims": {
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
        plan["kind"] == "MirBuilderPhiReturnTypeInferencePlanV1",
        "wrong PHI return-type inference plan kind",
    )
    require(
        "PhiReturnTypeInference" in plan["available_capabilities"],
        "missing PhiReturnTypeInference capability",
    )
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["builder_type_context"] == "self.type_ctx.value_types", "type context drift")
    require(
        plan["resolver_chain"]
        == [
            "SkipConcreteReturnType",
            "TerminatorReturnOnly",
            "DirectValueTypesLookup",
            "TypeHintPolicyExtract",
            "MethodReturnHintBox",
            "PhiTypeResolver",
            "GenericTypeResolver",
            "UnknownFallbackOutsideDebug",
        ],
        f"resolver chain drift: {plan['resolver_chain']}",
    )
    result = plan["result_contract"]
    require(result["entrypoint"] == "phi_type_inference::infer_return_type_from_phi", "entrypoint drift")
    require(result["minimal_path_expected_result"] == "Option<MirType>", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("resolver chain drift", ["resolver_chain"], list(reversed(plan["resolver_chain"]))),
        ("phi input materialization claim drift", ["non_claims", "phi_input_materialization"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-phi-return-type-inference-v0"),
            ("mirbuilder_phi_return_type_inference", "green"),
            ("capability", "PhiReturnTypeInference"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("resolver_chain", ",".join(plan["resolver_chain"])),
            ("phi_input_materialization_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

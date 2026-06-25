#!/usr/bin/env python3
"""Project finalize_module metadata value-type publication from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the publication
of `self.type_ctx.value_types` into `function.metadata.value_types` after type
propagation and type-hint provision. It does not claim value-origin caller
merge, PHI return-type inference, PHI input materialization, full finalize,
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
    / "mirbuilder-metadata-value-type-publication-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
TYPE_HINT_PROVISION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-type-hint-provision-plan-v0.json"
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
    type_hint = _read_json(TYPE_HINT_PROVISION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    finalize_order = _require_order(
        finalize,
        [
            "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
            "function.metadata.value_types = self.type_ctx.value_types.clone();",
            "let mut origin_callers = function.metadata.value_origin_callers.clone();",
        ],
        "MirBuilder::finalize_module metadata value-type publication",
    )
    require(
        type_hint.get("non_claims", {}).get("metadata_value_type_publication") == 0,
        "TypeHintProvision must not claim metadata value-type publication",
    )
    require(
        "function.metadata.value_types = self.type_ctx.value_types.clone();" in finalize,
        "metadata value_types publication assignment missing",
    )
    require(
        "function.metadata.value_origin_callers = origin_callers;" in finalize,
        "origin caller merge boundary marker missing",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderMetadataValueTypePublicationPlanV1",
        "subject": "MirBuilder::finalize_module function.metadata.value_types publication",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "predecessor_plan": "mirbuilder-type-hint-provision-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "function_transport": "MirFunctionPreparedMain",
            "value_types_source": "self.type_ctx.value_types",
            "metadata_target": "function.metadata.value_types",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
        },
        "publication": {
            "source": "self.type_ctx.value_types",
            "target": "function.metadata.value_types",
            "operation": "CloneOwnedMap",
            "timing": "AfterTypeHintProvisionBeforeOriginCallerMerge",
        },
        "available_capabilities": [
            "MetadataValueTypePublication",
        ],
        "result_contract": {
            "mutates": [
                "function.metadata.value_types",
            ],
            "entrypoint": "function.metadata.value_types = self.type_ctx.value_types.clone()",
            "minimal_path_expected_result": "OkImplicitUnit",
        },
        "non_claims": {
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
        plan["kind"] == "MirBuilderMetadataValueTypePublicationPlanV1",
        "wrong metadata value-type publication plan kind",
    )
    require(
        "MetadataValueTypePublication" in plan["available_capabilities"],
        "missing MetadataValueTypePublication capability",
    )
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["value_types_source"] == "self.type_ctx.value_types", "source drift")
    require(profile["metadata_target"] == "function.metadata.value_types", "target drift")
    publication = plan["publication"]
    require(publication["operation"] == "CloneOwnedMap", "publication operation drift")
    require(
        publication["timing"] == "AfterTypeHintProvisionBeforeOriginCallerMerge",
        "publication timing drift",
    )
    result = plan["result_contract"]
    require(
        result["entrypoint"] == "function.metadata.value_types = self.type_ctx.value_types.clone()",
        "entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "OkImplicitUnit", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("publication operation drift", ["publication", "operation"], "BorrowedAlias"),
        ("origin caller merge claim drift", ["non_claims", "metadata_origin_caller_merge"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-metadata-value-type-publication-v0"),
            ("mirbuilder_metadata_value_type_publication", "green"),
            ("capability", "MetadataValueTypePublication"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("publication_operation", plan["publication"]["operation"]),
            ("metadata_origin_caller_merge_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

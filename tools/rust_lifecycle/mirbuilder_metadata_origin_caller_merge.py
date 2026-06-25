#!/usr/bin/env python3
"""Project finalize_module metadata origin-caller merge from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the merge of
builder `metadata_ctx.value_origin_callers()` into
`function.metadata.value_origin_callers` after value-type publication. It does
not claim PHI return-type inference, PHI input materialization, full finalize,
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
    / "mirbuilder-metadata-origin-caller-merge-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
METADATA_VALUE_TYPE_PUBLICATION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-metadata-value-type-publication-plan-v0.json"
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
    metadata_value_publication = _read_json(METADATA_VALUE_TYPE_PUBLICATION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    finalize_order = _require_order(
        finalize,
        [
            "function.metadata.value_types = self.type_ctx.value_types.clone();",
            "let mut origin_callers = function.metadata.value_origin_callers.clone();",
            "for (k, v) in self.metadata_ctx.value_origin_callers().iter()",
            "origin_callers.insert(*k, v.clone());",
            "function.metadata.value_origin_callers = origin_callers;",
            "phi_type_inference::infer_return_type_from_phi",
        ],
        "MirBuilder::finalize_module metadata origin-caller merge",
    )
    require(
        metadata_value_publication.get("non_claims", {}).get("metadata_origin_caller_merge")
        == 0,
        "MetadataValueTypePublication must not claim origin-caller merge",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderMetadataOriginCallerMergePlanV1",
        "subject": "MirBuilder::finalize_module function.metadata.value_origin_callers merge",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "predecessor_plan": "mirbuilder-metadata-value-type-publication-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "function_transport": "MirFunctionPreparedMain",
            "source": "self.metadata_ctx.value_origin_callers()",
            "base": "function.metadata.value_origin_callers",
            "target": "function.metadata.value_origin_callers",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
        },
        "merge": {
            "base_operation": "CloneExistingFunctionMap",
            "source_iteration": "BorrowedMetadataContextValueOriginCallersIter",
            "entry_operation": "InsertClonedValue",
            "collision_policy": "SourceWins",
            "target_assignment": "ReplaceFunctionMetadataValueOriginCallers",
        },
        "available_capabilities": [
            "MetadataOriginCallerMerge",
        ],
        "result_contract": {
            "mutates": [
                "function.metadata.value_origin_callers",
            ],
            "entrypoint": "function.metadata.value_origin_callers = origin_callers",
            "minimal_path_expected_result": "OkImplicitUnit",
        },
        "non_claims": {
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
        plan["kind"] == "MirBuilderMetadataOriginCallerMergePlanV1",
        "wrong metadata origin-caller merge plan kind",
    )
    require(
        "MetadataOriginCallerMerge" in plan["available_capabilities"],
        "missing MetadataOriginCallerMerge capability",
    )
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["source"] == "self.metadata_ctx.value_origin_callers()", "source drift")
    require(profile["target"] == "function.metadata.value_origin_callers", "target drift")
    merge = plan["merge"]
    require(merge["base_operation"] == "CloneExistingFunctionMap", "base operation drift")
    require(merge["entry_operation"] == "InsertClonedValue", "entry operation drift")
    require(merge["collision_policy"] == "SourceWins", "collision policy drift")
    result = plan["result_contract"]
    require(
        result["entrypoint"] == "function.metadata.value_origin_callers = origin_callers",
        "entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "OkImplicitUnit", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("collision policy drift", ["merge", "collision_policy"], "BaseWins"),
        ("phi inference claim drift", ["non_claims", "phi_return_type_inference"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-metadata-origin-caller-merge-v0"),
            ("mirbuilder_metadata_origin_caller_merge", "green"),
            ("capability", "MetadataOriginCallerMerge"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("collision_policy", plan["merge"]["collision_policy"]),
            ("phi_return_type_inference_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

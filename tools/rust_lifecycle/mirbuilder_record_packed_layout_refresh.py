#!/usr/bin/env python3
"""Project record/packed layout refresh from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the
`refresh_module_record_and_packed_layout_plans(&mut module)` call and the
ordered helper chain inside that semantic-refresh entry point. It does not claim
typed-object refresh, direct-state refresh, all-functions PHI materialization,
full finalize, generated Hako, backend routes, or runtime behavior.
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
    / "mirbuilder-record-packed-layout-refresh-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
SEMANTIC_REFRESH = ROOT / "src/mir/semantic_refresh.rs"
MODULE_METADATA_PUBLICATION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-module-metadata-publication-plan-v0.json"
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


REFRESH_STEPS = [
    "refresh_module_record_layout_plans",
    "refresh_module_array_record_storage_plans",
    "refresh_module_array_record_autouse_eligibility_plans",
    "refresh_module_array_record_materialization_boundary_plans",
    "refresh_module_array_record_packed_autouse_pilot_plans",
    "refresh_module_source_packed_array_autouse_pilot_plans",
    "refresh_module_source_packed_array_direct_read_consumption_plans",
    "refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans",
    "refresh_module_hako_alloc_huge_page_packed_store_pilot_plans",
]


def extract_plan() -> dict[str, Any]:
    lifecycle = _read(MODULE_LIFECYCLE)
    semantic_refresh = _read(SEMANTIC_REFRESH)
    module_metadata = _read_json(MODULE_METADATA_PUBLICATION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    refresh_body = _function_body(
        semantic_refresh, "pub fn refresh_module_record_and_packed_layout_plans(module: &mut MirModule)"
    )

    finalize_order = _require_order(
        finalize,
        [
            "module.metadata.enum_decls = self.comp_ctx.enum_decls_for_module_metadata();",
            "crate::mir::semantic_refresh::refresh_module_record_and_packed_layout_plans(&mut module);",
            "crate::mir::typed_object_plan::refresh_module_typed_object_plans(&mut module);",
        ],
        "MirBuilder::finalize_module record/packed layout refresh",
    )
    refresh_order = _require_order(
        refresh_body,
        [f"{step}(module);" for step in REFRESH_STEPS],
        "semantic_refresh::refresh_module_record_and_packed_layout_plans",
    )
    require(
        "Keep typed-object and direct-state planning out of this helper" in semantic_refresh,
        "record/packed layout helper boundary comment missing",
    )
    require(
        module_metadata.get("non_claims", {}).get("record_and_packed_layout_refresh") == 0,
        "ModuleMetadataPublication must not claim record_and_packed_layout_refresh",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderRecordPackedLayoutRefreshPlanV1",
        "subject": "MirBuilder::finalize_module record/packed layout refresh",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "refresh_entrypoint": "src/mir/semantic_refresh.rs::refresh_module_record_and_packed_layout_plans",
            "predecessor_plan": "mirbuilder-module-metadata-publication-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "context": "finalize_module",
            "module_transport": "MirModuleMinimalShell",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "refresh_entrypoint": refresh_order,
        },
        "refresh_policy": {
            "entrypoint": "refresh_module_record_and_packed_layout_plans",
            "timing": "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
            "steps": REFRESH_STEPS,
            "module_arg": "&mut MirModule",
        },
        "available_capabilities": [
            "RecordAndPackedLayoutRefresh",
        ],
        "result_contract": {
            "mutates": [
                "module.metadata.record_layout_plans",
                "module.metadata.array_record_storage_plans",
                "module.metadata.array_record_autouse_eligibility_plans",
                "module.metadata.array_record_materialization_boundary_plans",
                "module.metadata.array_record_packed_autouse_pilot_plans",
                "module.metadata.source_packed_array_autouse_pilot_plans",
                "module.metadata.source_packed_array_direct_read_consumption_plans",
                "module.metadata.hako_alloc_aligned_small_packed_store_pilot_plans",
                "module.metadata.hako_alloc_huge_page_packed_store_pilot_plans",
            ],
            "entrypoint": "semantic_refresh::refresh_module_record_and_packed_layout_plans",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "typed_object_plan_refresh": 0,
            "direct_state_plan_refresh": 0,
            "full_semantic_refresh": 0,
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
        plan["kind"] == "MirBuilderRecordPackedLayoutRefreshPlanV1",
        "wrong record/packed layout refresh plan kind",
    )
    require(
        "RecordAndPackedLayoutRefresh" in plan["available_capabilities"],
        "missing RecordAndPackedLayoutRefresh capability",
    )
    profile = plan["execution_profile"]
    require(profile["context"] == "finalize_module", "refresh context drift")
    require(profile["module_transport"] == "MirModuleMinimalShell", "module transport drift")
    refresh = plan["refresh_policy"]
    require(
        refresh["entrypoint"] == "refresh_module_record_and_packed_layout_plans",
        "refresh entrypoint drift",
    )
    require(
        refresh["timing"] == "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
        "refresh timing drift",
    )
    require(refresh["steps"] == REFRESH_STEPS, "refresh step order drift")
    require(refresh["module_arg"] == "&mut MirModule", "refresh module arg drift")
    result = plan["result_contract"]
    require(
        result["entrypoint"]
        == "semantic_refresh::refresh_module_record_and_packed_layout_plans",
        "result entrypoint drift",
    )
    require(len(result["mutates"]) == len(REFRESH_STEPS), "mutation frame count drift")
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("step order drift", ["refresh_policy", "steps"], list(reversed(REFRESH_STEPS))),
        ("typed object claim drift", ["non_claims", "typed_object_plan_refresh"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-record-packed-layout-refresh-v0"),
            ("mirbuilder_record_packed_layout_refresh", "green"),
            ("capability", "RecordAndPackedLayoutRefresh"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("refresh_steps", str(len(plan["refresh_policy"]["steps"]))),
            ("typed_object_plan_refresh_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Project typed object plan refresh from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only
`refresh_module_typed_object_plans(&mut module)` and its assignment of
`module.metadata.typed_object_plans`. It does not claim direct-state refresh,
typed-object field value refresh, collection field element refresh,
all-functions PHI materialization, full finalize, generated Hako, backend
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
    / "mirbuilder-typed-object-plan-refresh-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
TYPED_OBJECT_PLAN = ROOT / "src/mir/typed_object_plan.rs"
RECORD_PACKED_LAYOUT_REFRESH_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-record-packed-layout-refresh-plan-v0.json"
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
    typed_object = _read(TYPED_OBJECT_PLAN)
    record_refresh = _read_json(RECORD_PACKED_LAYOUT_REFRESH_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    refresh_body = _function_body(
        typed_object, "pub fn refresh_module_typed_object_plans(module: &mut MirModule)"
    )
    build_body = _function_body(
        typed_object, "pub fn build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan>"
    )

    finalize_order = _require_order(
        finalize,
        [
            "crate::mir::semantic_refresh::refresh_module_record_and_packed_layout_plans(&mut module);",
            "crate::mir::typed_object_plan::refresh_module_typed_object_plans(&mut module);",
            "crate::mir::direct_state_plan::refresh_module_direct_state_plans(&mut module);",
        ],
        "MirBuilder::finalize_module typed object plan refresh",
    )
    refresh_order = _require_order(
        refresh_body,
        [
            "module.metadata.typed_object_plans = build_typed_object_plans(module);",
        ],
        "typed_object_plan::refresh_module_typed_object_plans",
    )
    build_order = _require_order(
        build_body,
        [
            "storage_inference::build_typed_object_plans(module)",
        ],
        "typed_object_plan::build_typed_object_plans",
    )
    require(
        record_refresh.get("non_claims", {}).get("typed_object_plan_refresh") == 0,
        "RecordAndPackedLayoutRefresh must not claim typed_object_plan_refresh",
    )
    require(
        "MIR owns the object layout truth" in typed_object,
        "typed object plan ownership comment missing",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderTypedObjectPlanRefreshPlanV1",
        "subject": "MirBuilder::finalize_module typed object plan refresh",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "refresh_entrypoint": "src/mir/typed_object_plan.rs::refresh_module_typed_object_plans",
            "predecessor_plan": "mirbuilder-record-packed-layout-refresh-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "context": "finalize_module",
            "module_transport": "MirModuleMinimalShell",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "refresh_entrypoint": refresh_order,
            "build_entrypoint": build_order,
        },
        "refresh_policy": {
            "entrypoint": "refresh_module_typed_object_plans",
            "timing": "AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh",
            "operation": "AssignTypedObjectPlans",
            "source": "build_typed_object_plans(module)",
            "build_provider": "storage_inference::build_typed_object_plans",
            "target": "module.metadata.typed_object_plans",
            "module_arg": "&mut MirModule",
        },
        "available_capabilities": [
            "TypedObjectPlanRefresh",
        ],
        "result_contract": {
            "mutates": [
                "module.metadata.typed_object_plans",
            ],
            "entrypoint": "typed_object_plan::refresh_module_typed_object_plans",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "typed_object_field_value_type_refresh": 0,
            "typed_object_collection_field_element_refresh": 0,
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
        plan["kind"] == "MirBuilderTypedObjectPlanRefreshPlanV1",
        "wrong typed object plan refresh kind",
    )
    require(
        "TypedObjectPlanRefresh" in plan["available_capabilities"],
        "missing TypedObjectPlanRefresh capability",
    )
    profile = plan["execution_profile"]
    require(profile["context"] == "finalize_module", "refresh context drift")
    require(profile["module_transport"] == "MirModuleMinimalShell", "module transport drift")
    refresh = plan["refresh_policy"]
    require(
        refresh["entrypoint"] == "refresh_module_typed_object_plans",
        "refresh entrypoint drift",
    )
    require(
        refresh["timing"] == "AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh",
        "refresh timing drift",
    )
    require(refresh["operation"] == "AssignTypedObjectPlans", "operation drift")
    require(
        refresh["source"] == "build_typed_object_plans(module)",
        "refresh source drift",
    )
    require(
        refresh["build_provider"] == "storage_inference::build_typed_object_plans",
        "build provider drift",
    )
    require(refresh["target"] == "module.metadata.typed_object_plans", "target drift")
    require(refresh["module_arg"] == "&mut MirModule", "module arg drift")
    result = plan["result_contract"]
    require(
        result["mutates"] == ["module.metadata.typed_object_plans"],
        "mutation frame drift",
    )
    require(
        result["entrypoint"] == "typed_object_plan::refresh_module_typed_object_plans",
        "result entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("target drift", ["refresh_policy", "target"], "module.metadata.direct_state_plans"),
        ("direct-state claim drift", ["non_claims", "direct_state_plan_refresh"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-typed-object-plan-refresh-v0"),
            ("mirbuilder_typed_object_plan_refresh", "green"),
            ("capability", "TypedObjectPlanRefresh"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("target", plan["refresh_policy"]["target"]),
            ("direct_state_plan_refresh_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

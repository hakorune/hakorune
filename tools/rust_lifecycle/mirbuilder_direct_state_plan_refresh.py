#!/usr/bin/env python3
"""Project direct state plan refresh from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only
`refresh_module_direct_state_plans(&mut module)` and its assignment of
`module.metadata.direct_state_plans`. It does not claim all-functions PHI
materialization, direct-state lowering, route selection, full finalize,
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
    / "mirbuilder-direct-state-plan-refresh-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
DIRECT_STATE_PLAN = ROOT / "src/mir/direct_state_plan.rs"
TYPED_OBJECT_PLAN_REFRESH_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-typed-object-plan-refresh-plan-v0.json"
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
    direct_state = _read(DIRECT_STATE_PLAN)
    typed_object_refresh = _read_json(TYPED_OBJECT_PLAN_REFRESH_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    refresh_body = _function_body(
        direct_state, "pub fn refresh_module_direct_state_plans(module: &mut MirModule)"
    )
    build_body = _function_body(
        direct_state, "pub fn build_direct_state_plans(module: &MirModule) -> Vec<DirectStatePlan>"
    )

    finalize_order = _require_order(
        finalize,
        [
            "crate::mir::typed_object_plan::refresh_module_typed_object_plans(&mut module);",
            "crate::mir::direct_state_plan::refresh_module_direct_state_plans(&mut module);",
            'materialize_all_phi_inputs(\n                function,\n                "finalize_module_all_functions",',
        ],
        "MirBuilder::finalize_module direct state plan refresh",
    )
    refresh_order = _require_order(
        refresh_body,
        [
            "module.metadata.direct_state_plans = build_direct_state_plans(module);",
        ],
        "direct_state_plan::refresh_module_direct_state_plans",
    )
    build_order = _require_order(
        build_body,
        [
            "user_box_field_decls",
            "names.sort();",
            "build_direct_state_plan(module, box_name, fields)",
        ],
        "direct_state_plan::build_direct_state_plans",
    )
    require(
        typed_object_refresh.get("non_claims", {}).get("direct_state_plan_refresh") == 0,
        "TypedObjectPlanRefresh must not claim direct_state_plan_refresh",
    )
    require(
        "Metadata-only direct-state candidate plans" in direct_state,
        "direct state plan ownership comment missing",
    )
    require(
        "This module does not create a runtime layout and does not enable lowering." in direct_state,
        "direct state non-lowering comment missing",
    )
    require(
        "pub const DIRECT_STATE_REPR_V0: &str = \"direct_v0\";" in direct_state,
        "direct state representation token drift",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderDirectStatePlanRefreshPlanV1",
        "subject": "MirBuilder::finalize_module direct state plan refresh",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "refresh_entrypoint": "src/mir/direct_state_plan.rs::refresh_module_direct_state_plans",
            "predecessor_plan": "mirbuilder-typed-object-plan-refresh-plan-v0.json",
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
            "entrypoint": "refresh_module_direct_state_plans",
            "timing": "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
            "operation": "AssignDirectStatePlans",
            "source": "build_direct_state_plans(module)",
            "build_provider": "direct_state_plan::build_direct_state_plans",
            "target": "module.metadata.direct_state_plans",
            "module_arg": "&mut MirModule",
        },
        "plan_builder_contract": {
            "input_authority": "module.metadata.user_box_field_decls",
            "ordering": "SortBoxNames",
            "field_selection": "TypedObjectFieldStorageUsesIntegerLaneAndNotWeak",
            "state_repr": "direct_v0",
            "runtime_layout_created": 0,
            "lowering_enabled": 0,
        },
        "available_capabilities": [
            "DirectStatePlanRefresh",
        ],
        "result_contract": {
            "mutates": [
                "module.metadata.direct_state_plans",
            ],
            "entrypoint": "direct_state_plan::refresh_module_direct_state_plans",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "all_functions_phi_materialization": 0,
            "direct_state_lowering": 0,
            "route_selection": 0,
            "native_direct_guard": 0,
            "full_semantic_refresh": 0,
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
        plan["kind"] == "MirBuilderDirectStatePlanRefreshPlanV1",
        "wrong direct state plan refresh kind",
    )
    require(
        "DirectStatePlanRefresh" in plan["available_capabilities"],
        "missing DirectStatePlanRefresh capability",
    )
    profile = plan["execution_profile"]
    require(profile["context"] == "finalize_module", "refresh context drift")
    require(profile["module_transport"] == "MirModuleMinimalShell", "module transport drift")
    refresh = plan["refresh_policy"]
    require(
        refresh["entrypoint"] == "refresh_module_direct_state_plans",
        "refresh entrypoint drift",
    )
    require(
        refresh["timing"] == "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
        "refresh timing drift",
    )
    require(refresh["operation"] == "AssignDirectStatePlans", "operation drift")
    require(
        refresh["source"] == "build_direct_state_plans(module)",
        "refresh source drift",
    )
    require(
        refresh["build_provider"] == "direct_state_plan::build_direct_state_plans",
        "build provider drift",
    )
    require(refresh["target"] == "module.metadata.direct_state_plans", "target drift")
    require(refresh["module_arg"] == "&mut MirModule", "module arg drift")
    builder = plan["plan_builder_contract"]
    require(
        builder["input_authority"] == "module.metadata.user_box_field_decls",
        "builder input authority drift",
    )
    require(builder["ordering"] == "SortBoxNames", "builder ordering drift")
    require(
        builder["field_selection"] == "TypedObjectFieldStorageUsesIntegerLaneAndNotWeak",
        "field selection drift",
    )
    require(builder["state_repr"] == "direct_v0", "state repr drift")
    require(builder["runtime_layout_created"] == 0, "runtime layout claim drift")
    require(builder["lowering_enabled"] == 0, "lowering claim drift")
    result = plan["result_contract"]
    require(
        result["mutates"] == ["module.metadata.direct_state_plans"],
        "mutation frame drift",
    )
    require(
        result["entrypoint"] == "direct_state_plan::refresh_module_direct_state_plans",
        "result entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("target drift", ["refresh_policy", "target"], "module.metadata.typed_object_plans"),
        ("runtime layout claim drift", ["plan_builder_contract", "runtime_layout_created"], 1),
        ("all-functions claim drift", ["non_claims", "all_functions_phi_materialization"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-direct-state-plan-refresh-v0"),
            ("mirbuilder_direct_state_plan_refresh", "green"),
            ("capability", "DirectStatePlanRefresh"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("target", plan["refresh_policy"]["target"]),
            ("all_functions_phi_materialization_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

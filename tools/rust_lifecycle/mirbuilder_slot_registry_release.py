#!/usr/bin/env python3
"""Project finalize_module SlotRegistry release from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the
source-required `current_slot_registry = None` release edge. It does not claim
module metadata publication, semantic refresh, full finalize, generated Hako,
backend routes, or runtime behavior.
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
    / "mirbuilder-slot-registry-release-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
FUNCTION_REGION_STACK_POP_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-function-region-stack-pop-plan-v0.json"
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
    region_pop = _read_json(FUNCTION_REGION_STACK_POP_PLAN)
    prepare = _function_body(lifecycle, "pub(super) fn prepare_module(&mut self) -> Result<(), String>")
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    prepare_order = _require_order(
        prepare,
        [
            "self.comp_ctx.current_slot_registry =",
            "Some(crate::mir::region::function_slot_registry::FunctionSlotRegistry::new());",
            "crate::mir::region::observer::observe_function_region(self);",
        ],
        "MirBuilder::prepare_module SlotRegistry init",
    )
    finalize_order = _require_order(
        finalize,
        [
            "crate::mir::region::observer::pop_function_region(self);",
            "self.comp_ctx.current_slot_registry = None;",
            "module.metadata.user_box_decls = self.comp_ctx.user_defined_boxes.clone();",
        ],
        "MirBuilder::finalize_module SlotRegistry release",
    )
    require(
        region_pop.get("non_claims", {}).get("slot_registry_release") == 0,
        "FunctionRegionStackPop must not claim slot_registry_release",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderSlotRegistryReleasePlanV1",
        "subject": "MirBuilder::finalize_module SlotRegistry release",
        "source_authority": {
            "prepare": "src/mir/builder/module_lifecycle.rs::MirBuilder::prepare_module",
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "predecessor_plan": "mirbuilder-function-region-stack-pop-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "context": "finalize_module",
            "prepared_slot_registry": "Some(FunctionSlotRegistry)",
        },
        "observed_source_order": {
            "prepare_module": prepare_order,
            "finalize_module": finalize_order,
        },
        "release_policy": {
            "lifecycle_owner": "CompilationContext.current_slot_registry",
            "init_operation": "Some(FunctionSlotRegistry::new())",
            "release_operation": "current_slot_registry = None",
            "release_timing": "AfterFunctionRegionStackPopBeforeModuleMetadataPublication",
            "released_value": "FunctionSlotRegistry",
        },
        "available_capabilities": [
            "SlotRegistryRelease",
        ],
        "result_contract": {
            "mutates": [
                "builder.comp_ctx.current_slot_registry",
            ],
            "entrypoint": "MirBuilder::finalize_module current_slot_registry release",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "slot_metadata_classification": 0,
            "function_region_stack_pop": 0,
            "module_metadata_publication": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
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
    require(plan["kind"] == "MirBuilderSlotRegistryReleasePlanV1", "wrong plan kind")
    require(
        "SlotRegistryRelease" in plan["available_capabilities"],
        "missing SlotRegistryRelease capability",
    )
    profile = plan["execution_profile"]
    require(profile["context"] == "finalize_module", "release context drift")
    require(
        profile["prepared_slot_registry"] == "Some(FunctionSlotRegistry)",
        "prepared SlotRegistry drift",
    )
    release = plan["release_policy"]
    require(
        release["lifecycle_owner"] == "CompilationContext.current_slot_registry",
        "lifecycle owner drift",
    )
    require(
        release["init_operation"] == "Some(FunctionSlotRegistry::new())",
        "init operation drift",
    )
    require(
        release["release_operation"] == "current_slot_registry = None",
        "release operation drift",
    )
    require(
        release["release_timing"]
        == "AfterFunctionRegionStackPopBeforeModuleMetadataPublication",
        "release timing drift",
    )
    require(release["released_value"] == "FunctionSlotRegistry", "released value drift")
    result = plan["result_contract"]
    require(
        result["mutates"] == ["builder.comp_ctx.current_slot_registry"],
        "mutation frame drift",
    )
    require(
        result["entrypoint"]
        == "MirBuilder::finalize_module current_slot_registry release",
        "entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("release timing drift", ["release_policy", "release_timing"], "AfterMetadata"),
        ("module metadata claim drift", ["non_claims", "module_metadata_publication"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-slot-registry-release-v0"),
            ("mirbuilder_slot_registry_release", "green"),
            ("capability", "SlotRegistryRelease"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("release_timing", plan["release_policy"]["release_timing"]),
            ("module_metadata_publication_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

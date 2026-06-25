#!/usr/bin/env python3
"""Project finalize_module function-region stack pop from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the
source-required `region::observer::pop_function_region(self)` edge. It does not
claim SlotRegistry release, metadata publication, semantic refresh, full
finalize, generated Hako, backend routes, or runtime behavior.
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
    / "mirbuilder-function-region-stack-pop-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
REGION_OBSERVER = ROOT / "src/mir/region/observer.rs"
CONDITION_FN_INJECTION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-condition-fn-injection-plan-v0.json"
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
    observer = _read(REGION_OBSERVER)
    condition_fn = _read_json(CONDITION_FN_INJECTION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    pop_function_region = _function_body(
        observer, "pub fn pop_function_region(builder: &mut MirBuilder)"
    )
    observe_function_region = _function_body(
        observer, "pub fn observe_function_region(builder: &mut MirBuilder)"
    )

    finalize_order = _require_order(
        finalize,
        [
            "module.add_function(f);",
            "crate::mir::region::observer::pop_function_region(self);",
            "self.comp_ctx.current_slot_registry = None;",
        ],
        "MirBuilder::finalize_module function-region cleanup",
    )
    pop_order = _require_order(
        pop_function_region,
        [
            "if !is_region_trace_on()",
            "return;",
            "let _ = builder.metadata_ctx.pop_region();",
        ],
        "region::observer::pop_function_region",
    )
    observe_order = _require_order(
        observe_function_region,
        [
            "if !is_region_trace_on()",
            "builder.metadata_ctx.push_region(id);",
        ],
        "region::observer::observe_function_region",
    )
    require(
        "std::env::var(\"NYASH_REGION_TRACE\").ok().as_deref() == Some(\"1\")"
        in observer,
        "region trace guard source marker missing",
    )
    require(
        condition_fn.get("non_claims", {}).get("region_stack_pop") == 0,
        "ConditionFnInjection must not claim region_stack_pop",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderFunctionRegionStackPopPlanV1",
        "subject": "MirBuilder::finalize_module function region stack pop",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "observer": "src/mir/region/observer.rs::pop_function_region",
            "predecessor_plan": "mirbuilder-condition-fn-injection-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "context": "finalize_module",
            "region_trace": "NYASH_REGION_TRACE=1",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "pop_function_region": pop_order,
            "observe_function_region_push_counterpart": observe_order,
        },
        "pop_policy": {
            "callsite": "region::observer::pop_function_region(self)",
            "guard": "NYASH_REGION_TRACE == 1",
            "operation": "metadata_ctx.pop_region",
            "result_ignored": True,
            "tracing_disabled_effect": "NoOp",
            "push_counterpart_observed": True,
        },
        "available_capabilities": [
            "FunctionRegionStackPop",
        ],
        "result_contract": {
            "mutates_when_guard_enabled": [
                "builder.metadata_ctx.current_region_stack",
            ],
            "entrypoint": "region::observer::pop_function_region",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "observe_function_region_claim": 0,
            "slot_registry_release": 0,
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
    require(
        plan["kind"] == "MirBuilderFunctionRegionStackPopPlanV1",
        "wrong function-region stack pop plan kind",
    )
    require(
        "FunctionRegionStackPop" in plan["available_capabilities"],
        "missing FunctionRegionStackPop capability",
    )
    profile = plan["execution_profile"]
    require(profile["context"] == "finalize_module", "region pop context drift")
    require(profile["region_trace"] == "NYASH_REGION_TRACE=1", "region trace profile drift")
    pop_policy = plan["pop_policy"]
    require(
        pop_policy["callsite"] == "region::observer::pop_function_region(self)",
        "pop callsite drift",
    )
    require(pop_policy["guard"] == "NYASH_REGION_TRACE == 1", "pop guard drift")
    require(pop_policy["operation"] == "metadata_ctx.pop_region", "pop operation drift")
    require(pop_policy["result_ignored"] is True, "pop result handling drift")
    require(pop_policy["tracing_disabled_effect"] == "NoOp", "trace-disabled effect drift")
    require(pop_policy["push_counterpart_observed"] is True, "push counterpart drift")
    result = plan["result_contract"]
    require(
        result["mutates_when_guard_enabled"]
        == ["builder.metadata_ctx.current_region_stack"],
        "mutation frame drift",
    )
    require(
        result["entrypoint"] == "region::observer::pop_function_region",
        "entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("guard drift", ["pop_policy", "guard"], "NYASH_REGION_TRACE != 0"),
        ("slot registry claim drift", ["non_claims", "slot_registry_release"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-function-region-stack-pop-v0"),
            ("mirbuilder_function_region_stack_pop", "green"),
            ("capability", "FunctionRegionStackPop"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("guard", plan["pop_policy"]["guard"]),
            ("slot_registry_release_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

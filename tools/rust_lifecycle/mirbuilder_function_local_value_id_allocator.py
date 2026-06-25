#!/usr/bin/env python3
"""Project function-local ValueId allocator plans from allocation facts.

This is a plan/oracle slice for MirFunction-local allocation only. It does not
claim MirBuilder::next_value_id composition, reserved exclusion handling, or a
generated Hako artifact.
"""

from __future__ import annotations

import argparse
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require
from extract_mirbuilder_allocation_policy_facts import extract_facts


ROOT = Path(__file__).resolve().parents[2]
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "function-local-value-id-allocator-plan-v0.json"
)


def project_function_local_allocator_plan(facts: dict[str, Any]) -> dict[str, Any]:
    """Project the function-local allocator plan from broader allocation facts."""

    function = facts["function_allocator"]
    parameter = facts["parameter_initialization"]
    sentinel = facts["sentinel_policy"]

    require(function["state"] == "MirFunction.next_value_id", "function-local allocator state drift")
    require(function["operation"] == "TakeThenIncrement", "function-local allocator operation drift")
    require(function["param_ids"] == "[0, param_count)", "function-local parameter id range drift")
    require(function["initial_counter_seed"] == "max(param_count, 1)", "function-local counter seed drift")
    require(function["transport"] == "ValueIdAsI64", "function-local transport drift")
    require(parameter["prepopulation"] == "ParameterIdPrepopulation", "parameter prepopulation drift")
    require(parameter["counter_seed"] == "FunctionCounterSeed", "function counter seed fact drift")
    require(sentinel["function_initial_floor"] == 1, "function initial floor drift")
    require(sentinel["formal_invalid_sentinel"] == "u32::MAX", "formal invalid sentinel drift")
    require(sentinel["formal_invalid_exclusion_claim"] is False, "invalid exclusion must remain unclaimed")

    return {
        "schema_version": 0,
        "kind": "FunctionLocalValueIdAllocatorPlanV1",
        "subject": "hakorune_mir::MirFunction::next_value_id",
        "source_facts": "MirBuilderAllocationPolicyFactsV1.function_allocator",
        "state": function["state"],
        "parameter_prepopulation": {
            "id_range": function["param_ids"],
            "transport": function["transport"],
        },
        "counter_seed": {
            "expression": function["initial_counter_seed"],
            "zero_parameter_floor": sentinel["function_initial_floor"],
            "zero_reserved_by_function_constructor_policy": sentinel[
                "zero_reserved_by_function_constructor_policy"
            ],
        },
        "next_operation": {
            "operation": function["operation"],
            "result_transport": function["transport"],
            "mutation": "IncrementAfterTake",
        },
        "oracle_vectors": [
            {
                "param_count": 0,
                "prepopulated_params": [],
                "initial_counter": 1,
                "next_results": [1, 2, 3],
            },
            {
                "param_count": 1,
                "prepopulated_params": [0],
                "initial_counter": 1,
                "next_results": [1, 2, 3],
            },
            {
                "param_count": 3,
                "prepopulated_params": [0, 1, 2],
                "initial_counter": 3,
                "next_results": [3, 4, 5],
            },
        ],
        "directability": {
            "decision": "PlanOnly",
            "allowed_claim": "FunctionLocalAllocatorOnly",
        },
        "non_claims": {
            "reserved_exclusion_set_retry": 0,
            "current_function_composition": 0,
            "module_global_fallback": 0,
            "formal_invalid_sentinel_exclusion": 0,
            "overflow_policy": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
        },
    }


def _simulate_vector(vector: dict[str, Any]) -> dict[str, Any]:
    param_count = vector["param_count"]
    prepopulated = list(range(param_count))
    counter = max(param_count, 1)
    results: list[int] = []
    for _ in range(len(vector["next_results"])):
        results.append(counter)
        counter += 1
    return {
        "prepopulated_params": prepopulated,
        "initial_counter": max(param_count, 1),
        "next_results": results,
    }


def validate_function_local_allocator_plan(plan: dict[str, Any]) -> None:
    require(plan["schema_version"] == 0, "unsupported function-local allocator plan schema")
    require(plan["kind"] == "FunctionLocalValueIdAllocatorPlanV1", "wrong function-local allocator plan kind")
    require(plan["state"] == "MirFunction.next_value_id", "function-local plan state drift")
    require(
        plan["parameter_prepopulation"]["id_range"] == "[0, param_count)",
        "function-local plan parameter range drift",
    )
    require(plan["counter_seed"]["expression"] == "max(param_count, 1)", "counter seed drift")
    require(plan["counter_seed"]["zero_parameter_floor"] == 1, "zero parameter floor drift")
    require(plan["next_operation"]["operation"] == "TakeThenIncrement", "next operation drift")
    require(plan["next_operation"]["result_transport"] == "ValueIdAsI64", "next result transport drift")

    for vector in plan["oracle_vectors"]:
        simulated = _simulate_vector(vector)
        require(
            vector["prepopulated_params"] == simulated["prepopulated_params"],
            f"prepopulated params drift for param_count={vector['param_count']}",
        )
        require(
            vector["initial_counter"] == simulated["initial_counter"],
            f"initial counter drift for param_count={vector['param_count']}",
        )
        require(
            vector["next_results"] == simulated["next_results"],
            f"next sequence drift for param_count={vector['param_count']}",
        )

    non_claims = plan["non_claims"]
    for key, value in non_claims.items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[str], Any]] = [
        ("counter seed drift", ["counter_seed", "expression"], "param_count"),
        ("transport conflation", ["next_operation", "result_transport"], "BasicBlockIdAsI64"),
        ("zero param oracle drift", ["oracle_vectors", 0, "next_results"], [0, 1, 2]),
    ]
    for label, path, value in probes:
        mutated = deepcopy(plan)
        cursor: Any = mutated
        for key in path[:-1]:
            cursor = cursor[key]
        cursor[path[-1]] = value
        try:
            validate_function_local_allocator_plan(mutated)
        except AssertionError:
            continue
        raise AssertionError(f"drift probe did not fail: {label}")


def build_plan() -> dict[str, Any]:
    facts = extract_facts()
    plan = project_function_local_allocator_plan(facts)
    validate_function_local_allocator_plan(plan)
    return plan


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reference", type=Path, default=REFERENCE)
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
            ("output_contract", "rust-lifecycle-function-local-value-id-allocator-plan-v0"),
            ("function_local_allocator_plan", "green"),
            ("source_facts", plan["source_facts"]),
            ("allowed_claim", plan["directability"]["allowed_claim"]),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

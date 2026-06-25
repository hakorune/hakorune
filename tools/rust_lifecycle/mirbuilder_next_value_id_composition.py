#!/usr/bin/env python3
"""Compose MirBuilder::next_value_id policy from resolved sub-plans.

This closes the plan/oracle boundary for allocation composition only. It does
not choose a generated Hako artifact, backend consumer, ABI, or runtime path.
"""

from __future__ import annotations

import argparse
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require
from extract_mirbuilder_allocation_policy_facts import extract_facts
from mirbuilder_function_local_value_id_allocator import (
    project_function_local_allocator_plan,
    validate_function_local_allocator_plan,
)
from mirbuilder_reserved_value_exclusion_policy import (
    project_reserved_exclusion_policy,
    validate_reserved_exclusion_policy,
)


ROOT = Path(__file__).resolve().parents[2]
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-next-value-id-composition-plan-v0.json"
)


def project_next_value_id_composition(facts: dict[str, Any]) -> dict[str, Any]:
    resolved = facts["resolved_policy"]
    function_plan = project_function_local_allocator_plan(facts)
    reserved_plan = project_reserved_exclusion_policy(facts)
    validate_function_local_allocator_plan(function_plan)
    validate_reserved_exclusion_policy(reserved_plan)

    require(resolved["allocator_selector"] == "CurrentFunctionPresent", "allocator selector drift")
    require(resolved["acceptance_predicate"] == "CandidateNotInReservedSet", "acceptance predicate drift")
    require(resolved["rejection_effect"] == "Consumed", "rejection effect drift")
    require(resolved["retry_policy"] == "GenerateNextCandidate", "retry policy drift")
    require(resolved["result_transport"] == "ValueIdAsI64", "result transport drift")

    producers = {row["when"]: row for row in resolved["candidate_producers"]}
    require(producers["present"]["producer"] == "MirFunctionNextValueId", "present producer drift")
    require(producers["present"]["state"] == function_plan["state"], "present producer state drift")
    require(
        producers["present"]["operation"] == function_plan["next_operation"]["operation"],
        "present producer operation drift",
    )
    require(producers["absent"]["producer"] == "CoreContextNextValue", "absent producer drift")
    require(producers["absent"]["state"] == "CoreContext.value_gen", "absent producer state drift")
    require(producers["absent"]["operation"] == "GeneratorNext", "absent producer operation drift")

    return {
        "schema_version": 0,
        "kind": "MirBuilderNextValueIdCompositionPlanV1",
        "subject": "hakorune_mir_builder::MirBuilder::next_value_id",
        "source_facts": "MirBuilderAllocationPolicyFactsV1.resolved_policy",
        "subplans": {
            "function_local": function_plan["kind"],
            "reserved_exclusion": reserved_plan["kind"],
        },
        "allocator_selector": resolved["allocator_selector"],
        "candidate_producers": resolved["candidate_producers"],
        "acceptance_predicate": resolved["acceptance_predicate"],
        "rejection": {
            "effect": resolved["rejection_effect"],
            "retry": resolved["retry_policy"],
        },
        "result_transport": resolved["result_transport"],
        "oracle_vectors": [
            {
                "current_function": "Present",
                "candidate_sequence": [1, 2, 3, 4],
                "reserved_values": [2],
                "accepted_values": [1, 3, 4],
                "rejected_consumed_values": [2],
            },
            {
                "current_function": "Absent",
                "candidate_sequence": [0, 1, 2],
                "reserved_values": [1],
                "accepted_values": [0, 2],
                "rejected_consumed_values": [1],
            },
        ],
        "directability": {
            "decision": "PlanOnly",
            "allowed_claim": "MirBuilderNextValueIdCompositionOnly",
        },
        "non_claims": {
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "formal_invalid_sentinel_exclusion": 0,
            "overflow_policy": 0,
            "silent_fallback": 0,
            "runtime_fallback": 0,
        },
    }


def _simulate_vector(vector: dict[str, Any]) -> dict[str, list[int]]:
    reserved = set(vector["reserved_values"])
    accepted: list[int] = []
    rejected: list[int] = []
    for candidate in vector["candidate_sequence"]:
        if candidate in reserved:
            rejected.append(candidate)
        else:
            accepted.append(candidate)
    return {
        "accepted_values": accepted,
        "rejected_consumed_values": rejected,
    }


def validate_next_value_id_composition(plan: dict[str, Any]) -> None:
    require(plan["schema_version"] == 0, "unsupported next_value_id composition schema")
    require(plan["kind"] == "MirBuilderNextValueIdCompositionPlanV1", "wrong composition plan kind")
    require(plan["allocator_selector"] == "CurrentFunctionPresent", "allocator selector drift")
    require(plan["acceptance_predicate"] == "CandidateNotInReservedSet", "acceptance predicate drift")
    require(plan["rejection"]["effect"] == "Consumed", "rejection effect drift")
    require(plan["rejection"]["retry"] == "GenerateNextCandidate", "retry policy drift")
    require(plan["result_transport"] == "ValueIdAsI64", "result transport drift")
    require(
        plan["subplans"]["function_local"] == "FunctionLocalValueIdAllocatorPlanV1",
        "function-local subplan drift",
    )
    require(
        plan["subplans"]["reserved_exclusion"] == "ReservedValueExclusionPolicyPlanV1",
        "reserved-exclusion subplan drift",
    )

    producers = {row["when"]: row for row in plan["candidate_producers"]}
    require(producers["present"]["producer"] == "MirFunctionNextValueId", "present producer drift")
    require(producers["present"]["operation"] == "TakeThenIncrement", "present operation drift")
    require(producers["absent"]["producer"] == "CoreContextNextValue", "absent producer drift")
    require(producers["absent"]["operation"] == "GeneratorNext", "absent operation drift")

    for vector in plan["oracle_vectors"]:
        simulated = _simulate_vector(vector)
        require(
            vector["accepted_values"] == simulated["accepted_values"],
            f"accepted values drift for {vector['current_function']}",
        )
        require(
            vector["rejected_consumed_values"] == simulated["rejected_consumed_values"],
            f"rejected values drift for {vector['current_function']}",
        )

    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[str], Any]] = [
        ("selector drift", ["allocator_selector"], "AlwaysFunctionLocal"),
        ("function-local subplan drift", ["subplans", "function_local"], "CoreContextGeneratorPlanV1"),
        ("retry drift", ["rejection", "retry"], "ReturnReservedCandidate"),
        ("absent branch oracle drift", ["oracle_vectors", 1, "accepted_values"], [0, 1, 2]),
    ]
    for label, path, value in probes:
        mutated = deepcopy(plan)
        cursor: Any = mutated
        for key in path[:-1]:
            cursor = cursor[key]
        cursor[path[-1]] = value
        try:
            validate_next_value_id_composition(mutated)
        except AssertionError:
            continue
        raise AssertionError(f"drift probe did not fail: {label}")


def build_plan() -> dict[str, Any]:
    facts = extract_facts()
    plan = project_next_value_id_composition(facts)
    validate_next_value_id_composition(plan)
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
            ("output_contract", "rust-lifecycle-mirbuilder-next-value-id-composition-plan-v0"),
            ("mirbuilder_next_value_id_composition", "green"),
            ("source_facts", plan["source_facts"]),
            ("allowed_claim", plan["directability"]["allowed_claim"]),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Typed allocation-policy facts for MirBuilder ValueId issuance.

This is a facts-only authority. It does not choose a Hako representation,
backend route, ABI, or executable lowering.
"""

from __future__ import annotations

from copy import deepcopy
from typing import Any


class AllocationPolicyFactsError(AssertionError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AllocationPolicyFactsError(message)


def resolve_allocation_policy(facts: dict[str, Any]) -> dict[str, Any]:
    """Resolve source facts into one typed allocation policy decision."""

    candidate = facts["candidate_selection"]
    exclusion = facts["exclusion_set"]
    function_allocator = facts["function_allocator"]
    module_allocator = facts["module_allocator"]

    require(
        candidate["predicate"] == "CurrentFunctionPresent",
        "candidate selector must be current_function presence",
    )
    require(
        candidate["present_source"] == "MirFunctionNextValueId",
        "present allocator must be MirFunction::next_value_id",
    )
    require(
        candidate["absent_source"] == "CoreContextNextValue",
        "absent allocator must be CoreContext::next_value",
    )
    require(
        candidate["selection_frequency"] == "PerCandidateAttempt",
        "allocator selection must happen per candidate attempt",
    )
    require(
        exclusion["predicate"] == "CandidateNotInReservedSet",
        "reserved predicate must accept candidates not in the set",
    )
    require(
        exclusion["rejected_candidate_effect"] == "Consumed",
        "rejected reserved candidates must already be consumed",
    )
    require(
        exclusion["retry"] == "GenerateNextCandidate",
        "reserved rejection must retry by generating a new candidate",
    )
    require(
        function_allocator["transport"] == module_allocator["transport"] == "ValueIdAsI64",
        "function/module allocators must share ValueIdAsI64 transport",
    )

    return {
        "schema_version": 0,
        "kind": "ResolvedValueAllocationPolicyV1",
        "allocator_selector": "CurrentFunctionPresent",
        "candidate_producers": [
            {
                "when": "present",
                "producer": candidate["present_source"],
                "state": function_allocator["state"],
                "operation": function_allocator["operation"],
            },
            {
                "when": "absent",
                "producer": candidate["absent_source"],
                "state": module_allocator["state"],
                "operation": module_allocator["operation"],
            },
        ],
        "acceptance_predicate": exclusion["predicate"],
        "rejection_effect": exclusion["rejected_candidate_effect"],
        "retry_policy": exclusion["retry"],
        "result_transport": "ValueIdAsI64",
    }


def directability_decision(facts: dict[str, Any]) -> dict[str, Any]:
    """Facts are accepted, but executable lowering is intentionally denied."""

    _ = resolve_allocation_policy(facts)
    return {
        "kind": "DirectabilityDecision",
        "decision": "Deny",
        "reason": "UnsupportedDirectShape",
        "detail": "CurrentFunctionAndReservedSetCompositionUnselected",
        "unselected_boundaries": [
            "CurrentFunctionOptionTransportUnselected",
            "ReservedValueSetTransportUnselected",
            "ParameterSetupCompatibilityFallbackUnselected",
            "ReservedSetLifetimeProofRequired",
            "FormalInvalidSentinelPolicyUnselected",
            "AllocationCounterOverflowPolicyUnselected",
        ],
    }


def validate_allocation_policy_facts(facts: dict[str, Any]) -> None:
    """Validate extracted source facts and their resolved policy."""

    require(facts["schema_version"] == 0, "unsupported facts schema")
    require(facts["kind"] == "MirBuilderAllocationPolicyFactsV1", "wrong facts kind")
    require(
        facts["subject"] == "hakorune_mir_builder::MirBuilder::next_value_id",
        "wrong allocation-policy subject",
    )

    function_allocator = facts["function_allocator"]
    require(function_allocator["operation"] == "TakeThenIncrement", "function allocator must take then increment")
    require(function_allocator["initial_counter_seed"] == "max(param_count, 1)", "function counter seed drift")
    require(function_allocator["param_ids"] == "[0, param_count)", "parameter id range drift")
    require(function_allocator["zero_floor_policy"] is True, "zero floor policy must be explicit")

    module_allocator = facts["module_allocator"]
    require(module_allocator["operation"] == "GeneratorNext", "module allocator must be CoreContext generator next")
    require(module_allocator["first_candidate"] == 0, "module allocator first candidate drift")

    parameter = facts["parameter_initialization"]
    require(parameter["prepopulation"] == "ParameterIdPrepopulation", "parameter prepopulation missing")
    require(parameter["counter_seed"] == "FunctionCounterSeed", "function counter seed fact missing")
    require(parameter["binding_reuse"] == "ParameterBindingReuse", "parameter binding reuse missing")
    require(parameter["counter_floor_repair"] == "ParameterCounterFloorRepair", "counter floor repair missing")
    require(
        parameter["compatibility_fallback"] == "ParameterSetupCompatibilityFallbackUnselected",
        "parameter compatibility fallback must remain unselected",
    )

    exclusion = facts["exclusion_set"]
    require(exclusion["storage_owner"] == "CompilationContext", "reserved set owner drift")
    require(exclusion["producer"] == "JoinIrHeaderPhiPrebuild", "reserved set producer drift")
    require(exclusion["members"] == ["PhiDestinations", "JoinIrFunctionParameters"], "reserved set member union drift")
    require(exclusion["update_kind"] == "ReplaceSnapshot", "reserved set update kind drift")
    require(exclusion["observation"] == "MembershipOnly", "reserved set observation drift")
    require(exclusion["lifetime"] == "JoinIrMergeTemporary", "reserved set lifetime drift")

    sentinel = facts["sentinel_policy"]
    require(sentinel["function_initial_floor"] == 1, "function initial floor drift")
    require(sentinel["zero_reserved_by_function_constructor_policy"] is True, "zero floor not recorded")
    require(sentinel["formal_invalid_sentinel"] == "u32::MAX", "formal invalid sentinel drift")
    require(sentinel["formal_invalid_exclusion_claim"] is False, "invalid exclusion must remain unclaimed")

    resolved = facts["resolved_policy"]
    require(resolved == resolve_allocation_policy(facts), "resolved policy does not match source facts")
    directability = facts["directability"]
    require(directability == directability_decision(facts), "directability decision does not match source facts")

    claims = facts["claims"]
    require(claims["core_context_reserved_skip_claim"] == 0, "CoreContext reserved-skip claim must stay 0")
    require(claims["generated_hako_changed"] == 0, "generated Hako must remain unchanged")
    require(claims["executable_allocation_policy_claim"] == 0, "executable allocation claim must stay 0")
    require(claims["backend_route_changed"] == 0, "backend route must not change")
    require(claims["abi_changed"] == 0, "ABI must not change")
    require(claims["runtime_fallback"] == 0, "runtime fallback must stay 0")


def run_drift_probes(facts: dict[str, Any]) -> None:
    """Exercise high-value mutation probes against the validator."""

    probes: list[tuple[str, list[str], Any]] = [
        ("reserved predicate inversion", ["exclusion_set", "predicate"], "CandidateInReservedSet"),
        ("counter seed drift", ["function_allocator", "initial_counter_seed"], "param_count"),
        ("reserved union drift", ["exclusion_set", "members"], ["PhiDestinations"]),
    ]
    for label, path, value in probes:
        mutated = deepcopy(facts)
        cursor = mutated
        for key in path[:-1]:
            cursor = cursor[key]
        cursor[path[-1]] = value
        try:
            validate_allocation_policy_facts(mutated)
        except AllocationPolicyFactsError:
            continue
        raise AllocationPolicyFactsError(f"drift probe did not fail: {label}")

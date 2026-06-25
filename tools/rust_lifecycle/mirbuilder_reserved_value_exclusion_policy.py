#!/usr/bin/env python3
"""Project reserved ValueId exclusion policy from allocation facts.

This is a plan/oracle slice for reserved-candidate rejection only. It does not
claim current_function composition, concrete set representation, or executable
MirBuilder::next_value_id lowering.
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
    / "reserved-value-exclusion-policy-plan-v0.json"
)


def project_reserved_exclusion_policy(facts: dict[str, Any]) -> dict[str, Any]:
    exclusion = facts["exclusion_set"]

    require(exclusion["storage_owner"] == "CompilationContext", "reserved set owner drift")
    require(exclusion["producer"] == "JoinIrHeaderPhiPrebuild", "reserved set producer drift")
    require(
        exclusion["members"] == ["PhiDestinations", "JoinIrFunctionParameters"],
        "reserved set member union drift",
    )
    require(exclusion["update_kind"] == "ReplaceSnapshot", "reserved set update kind drift")
    require(exclusion["consumer"] == "MirBuilder::next_value_id", "reserved set consumer drift")
    require(exclusion["predicate"] == "CandidateNotInReservedSet", "reserved predicate drift")
    require(exclusion["observation"] == "MembershipOnly", "reserved observation drift")
    require(exclusion["rejected_candidate_effect"] == "Consumed", "rejected candidate effect drift")
    require(exclusion["retry"] == "GenerateNextCandidate", "reserved retry policy drift")
    require(exclusion["lifetime"] == "JoinIrMergeTemporary", "reserved lifetime drift")

    return {
        "schema_version": 0,
        "kind": "ReservedValueExclusionPolicyPlanV1",
        "subject": "hakorune_mir_builder::MirBuilder::next_value_id.reserved_exclusion",
        "source_facts": "MirBuilderAllocationPolicyFactsV1.exclusion_set",
        "storage": {
            "owner": exclusion["storage_owner"],
            "lifetime": exclusion["lifetime"],
            "concrete_representation": "Unselected",
        },
        "producer": {
            "source": exclusion["producer"],
            "members": exclusion["members"],
            "update_kind": exclusion["update_kind"],
        },
        "consumer": {
            "source": exclusion["consumer"],
            "predicate": exclusion["predicate"],
            "observation": exclusion["observation"],
        },
        "rejection": {
            "effect": exclusion["rejected_candidate_effect"],
            "retry": exclusion["retry"],
        },
        "oracle_vectors": [
            {
                "reserved_values": [2, 4],
                "candidate_sequence": [1, 2, 3, 4, 5],
                "accepted_values": [1, 3, 5],
                "rejected_consumed_values": [2, 4],
            },
            {
                "reserved_values": [],
                "candidate_sequence": [1, 2, 3],
                "accepted_values": [1, 2, 3],
                "rejected_consumed_values": [],
            },
        ],
        "directability": {
            "decision": "PlanOnly",
            "allowed_claim": "ReservedExclusionPolicyOnly",
        },
        "non_claims": {
            "function_allocator": 0,
            "current_function_composition": 0,
            "module_global_fallback": 0,
            "formal_invalid_sentinel_exclusion": 0,
            "concrete_ordered_map_or_set_representation": 0,
            "phi_dst_only_naming": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
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


def validate_reserved_exclusion_policy(plan: dict[str, Any]) -> None:
    require(plan["schema_version"] == 0, "unsupported reserved-exclusion plan schema")
    require(plan["kind"] == "ReservedValueExclusionPolicyPlanV1", "wrong reserved-exclusion plan kind")
    require(plan["storage"]["owner"] == "CompilationContext", "reserved storage owner drift")
    require(plan["storage"]["concrete_representation"] == "Unselected", "reserved representation must stay unselected")
    require(plan["producer"]["source"] == "JoinIrHeaderPhiPrebuild", "reserved producer drift")
    require(
        plan["producer"]["members"] == ["PhiDestinations", "JoinIrFunctionParameters"],
        "reserved members must include PHI destinations and JoinIR parameters",
    )
    require(plan["producer"]["update_kind"] == "ReplaceSnapshot", "reserved update kind drift")
    require(plan["consumer"]["source"] == "MirBuilder::next_value_id", "reserved consumer drift")
    require(plan["consumer"]["predicate"] == "CandidateNotInReservedSet", "reserved predicate drift")
    require(plan["consumer"]["observation"] == "MembershipOnly", "reserved observation drift")
    require(plan["rejection"]["effect"] == "Consumed", "reserved rejection effect drift")
    require(plan["rejection"]["retry"] == "GenerateNextCandidate", "reserved retry drift")

    for vector in plan["oracle_vectors"]:
        simulated = _simulate_vector(vector)
        require(
            vector["accepted_values"] == simulated["accepted_values"],
            f"accepted values drift for reserved={vector['reserved_values']}",
        )
        require(
            vector["rejected_consumed_values"] == simulated["rejected_consumed_values"],
            f"rejected values drift for reserved={vector['reserved_values']}",
        )

    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[str], Any]] = [
        ("member union drift", ["producer", "members"], ["PhiDestinations"]),
        ("predicate inversion", ["consumer", "predicate"], "CandidateInReservedSet"),
        ("rejection effect drift", ["rejection", "effect"], "NotConsumed"),
        ("oracle drift", ["oracle_vectors", 0, "accepted_values"], [1, 2, 3, 5]),
    ]
    for label, path, value in probes:
        mutated = deepcopy(plan)
        cursor: Any = mutated
        for key in path[:-1]:
            cursor = cursor[key]
        cursor[path[-1]] = value
        try:
            validate_reserved_exclusion_policy(mutated)
        except AssertionError:
            continue
        raise AssertionError(f"drift probe did not fail: {label}")


def build_plan() -> dict[str, Any]:
    facts = extract_facts()
    plan = project_reserved_exclusion_policy(facts)
    validate_reserved_exclusion_policy(plan)
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
            ("output_contract", "rust-lifecycle-reserved-value-exclusion-policy-plan-v0"),
            ("reserved_value_exclusion_policy", "green"),
            ("source_facts", plan["source_facts"]),
            ("allowed_claim", plan["directability"]["allowed_claim"]),
            ("concrete_representation", plan["storage"]["concrete_representation"]),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())

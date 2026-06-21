#!/usr/bin/env python3
"""Compile easy-tier CarrierInfo snapshot facts to typed Hako operation IR."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import HakoMethodIR, op


class CarrierSnapshotConversionDeny(RuntimeError):
    def __init__(self, reason: str) -> None:
        super().__init__(f"Deny({reason})")
        self.reason = reason


def _require(condition: bool, reason: str) -> None:
    if not condition:
        raise CarrierSnapshotConversionDeny(reason)


def _require_subject(facts: dict[str, Any], plan: dict[str, Any], *, subject: str) -> None:
    _require(facts.get("kind") == "RustLifecycleFacts", "UnsupportedResolvedCallTarget")
    _require(plan.get("kind") == "HakoLifecyclePlan", "UnsupportedResolvedCallTarget")
    _require(facts.get("subject") == subject and plan.get("subject") == subject, "UnsupportedResolvedCallTarget")


def _method_fact(facts: dict[str, Any]) -> dict[str, Any]:
    method = facts.get("method_fact")
    _require(isinstance(method, dict), "UnsupportedResolvedCallTarget")
    return method


def _plan_entry(plan: dict[str, Any]) -> dict[str, Any]:
    plans = plan.get("plans", [])
    _require(isinstance(plans, list) and plans, "UnsupportedResolvedCallTarget")
    entry = plans[0]
    _require(isinstance(entry, dict), "UnsupportedResolvedCallTarget")
    return entry


def _validate_common_snapshot_facts(facts: dict[str, Any], *, base_facts: list[str], operation: str) -> None:
    _require(facts.get("base_facts") == base_facts, "UnsupportedResolvedCallTarget")
    method = _method_fact(facts)
    _require(method.get("operation") == operation, "UnsupportedResolvedCallTarget")
    input_snapshot = method.get("input_snapshot", {})
    _require(input_snapshot.get("ownership") == "OwnedReadSnapshotProjection", "ReturnedReadBorrow")
    _require(input_snapshot.get("access") == "read", "ReturnedReadBorrow")
    _require(input_snapshot.get("escapes") is False, "ReturnedReadBorrow")
    map_requirements = method.get("map_requirements", {})
    _require(map_requirements.get("deterministic_order_required") is True, "UnsupportedResolvedCallTarget")
    _require(map_requirements.get("value_drop_fact") == "TrivialMemory", "UnsupportedResolvedCallTarget")
    output = method.get("output", {})
    _require(output.get("owns_carrier_names") is True, "UnsupportedResolvedCallTarget")
    _require(output.get("copies_value_ids") is True, "UnsupportedResolvedCallTarget")
    _require(output.get("value_id_copy_kind") == "ImmediateValue", "UnsupportedResolvedCallTarget")
    _require(output.get("join_id_initialized") is False, "UnsupportedResolvedCallTarget")
    denied_followups = set(facts.get("denied_followups", []))
    for item in ["join_id lifecycle", "promoted_body_locals lifecycle", "trim_helper lifecycle", "PHI planner integration"]:
        _require(item in denied_followups, "UnsupportedResolvedCallTarget")
    denied_methods = {row["id"]: row for row in facts.get("denied_methods", [])}
    _require(denied_methods.get("VariableContext::variable_map", {}).get("deny_reason") == "ReturnedReadBorrow", "ReturnedReadBorrow")


def _validate_common_snapshot_plan(plan: dict[str, Any], *, expected_kind: str, required_facts: list[str]) -> None:
    entry = _plan_entry(plan)
    _require(entry.get("plan_kind") == expected_kind, "UnsupportedResolvedCallTarget")
    _require(entry.get("mutation_policy") == "none", "UnsupportedResolvedCallTarget")
    _require(entry.get("publication_policy") == "does_not_publish_variable_map", "UnsupportedResolvedCallTarget")
    output_policy = entry.get("output_policy", {})
    _require(output_policy.get("carrier_names") == "owned_strings", "UnsupportedResolvedCallTarget")
    _require(output_policy.get("host_id") == "copied_ValueId", "UnsupportedResolvedCallTarget")
    _require(output_policy.get("join_id") == "None_uninitialized", "UnsupportedResolvedCallTarget")
    _require(output_policy.get("role") == "LoopState", "UnsupportedResolvedCallTarget")
    _require(output_policy.get("init") == "FromHost", "UnsupportedResolvedCallTarget")
    required = set(entry.get("required_facts", []))
    for fact in required_facts:
        _require(fact in required, "UnsupportedResolvedCallTarget")


def compile_carrier_snapshot_methods(facts: dict[str, Any], plan: dict[str, Any]) -> list[HakoMethodIR]:
    """Compile the owned-snapshot carrier slice to typed Hako method IR."""
    _require_subject(
        facts,
        plan,
        subject="hakorune_mir_builder::variable_context::CarrierInfo.from_variable_map",
    )
    _validate_common_snapshot_facts(
        facts,
        base_facts=["variable-context-simple-map-facts-v0.json", "variable-context-snapshot-restore-facts-v0.json"],
        operation="CarrierSnapshotFromOwnedMap",
    )
    _validate_common_snapshot_plan(
        plan,
        expected_kind="CarrierSnapshotFromOwnedMap",
        required_facts=[
            "input_snapshot.ownership=OwnedReadSnapshotProjection",
            "input_snapshot.escapes=false",
            "map_requirements.deterministic_order_required=true",
            "map_requirements.value_drop_fact=TrivialMemory",
            "output.value_id_copy_kind=ImmediateValue",
        ],
    )
    _require(_method_fact(facts).get("output", {}).get("join_id_initialized") is False, "UnsupportedResolvedCallTarget")
    _require(_method_fact(facts).get("output", {}).get("owns_carrier_names") is True, "UnsupportedResolvedCallTarget")
    _require(_method_fact(facts).get("output", {}).get("copies_value_ids") is True, "UnsupportedResolvedCallTarget")
    return [
        HakoMethodIR(
            "from_snapshot(carrier_data: OrderedMapBox, loop_var_name, snapshot: OrderedMapBox): i64",
            [op("CarrierSnapshotFromOwnedMap", output_arg="carrier_data", loop_var="loop_var_name", map_arg="snapshot")],
        )
    ]


def compile_explicit_carrier_snapshot_methods(facts: dict[str, Any], plan: dict[str, Any]) -> list[HakoMethodIR]:
    """Compile the explicit owned-snapshot carrier slice to typed Hako method IR."""
    _require_subject(
        facts,
        plan,
        subject="hakorune_mir_builder::variable_context::CarrierInfo.with_explicit_carriers",
    )
    _validate_common_snapshot_facts(
        facts,
        base_facts=["variable-context-carrier-snapshot-facts-v0.json", "variable-context-snapshot-restore-facts-v0.json"],
        operation="ExplicitCarrierSnapshotFromOwnedMap",
    )
    _validate_common_snapshot_plan(
        plan,
        expected_kind="ExplicitCarrierSnapshotFromOwnedMap",
        required_facts=[
            "input_snapshot.ownership=OwnedReadSnapshotProjection",
            "input_snapshot.escapes=false",
            "carrier_names.ownership=owned_strings",
            "carrier_names.missing_carrier_policy=fail_fast",
            "map_requirements.value_drop_fact=TrivialMemory",
            "output.value_id_copy_kind=ImmediateValue",
        ],
    )
    method = _method_fact(facts)
    _require(method.get("carrier_names", {}).get("ownership") == "owned_strings", "UnsupportedResolvedCallTarget")
    _require(method.get("carrier_names", {}).get("missing_carrier_policy") == "fail_fast", "UnsupportedResolvedCallTarget")
    return [
        HakoMethodIR(
            "with_explicit_carriers_from_snapshot(carrier_data: OrderedMapBox, loop_var_name, loop_var_id, requested_names: ArrayBox, snapshot: OrderedMapBox): i64",
            [
                op(
                    "ExplicitCarrierSnapshotFromOwnedMap",
                    output_arg="carrier_data",
                    loop_var="loop_var_name",
                    loop_var_id="loop_var_id",
                    requested_names="requested_names",
                    map_arg="snapshot",
                )
            ],
        )
    ]

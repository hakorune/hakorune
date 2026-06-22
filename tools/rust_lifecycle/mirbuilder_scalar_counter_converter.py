#!/usr/bin/env python3
"""Compile bounded scalar-counter facts into verified Hako method IR."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import HakoMethodIR, op


def _body_facts(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("body_facts", [])}


def _plans(plan: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in plan.get("plans", [])}


def _require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def compile_core_context_scalar_methods(facts: dict[str, Any], plan: dict[str, Any]) -> list[HakoMethodIR]:
    """Compile the easy-tier CoreContext scalar counters.

    Generator-backed value/block IDs are intentionally excluded from this slice;
    they need a separate transport decision and must not be silently lowered as
    plain integer counters.
    """

    bodies = _body_facts(facts)
    plans = _plans(plan)

    type_fact = {row["id"]: row for row in facts.get("type_facts", [])}.get("CoreContext")
    _require(type_fact is not None and type_fact.get("drop_fact") == "TrivialMemory", "NonTrivialDrop")
    _require(plans.get("CoreContext", {}).get("plan_kind") == "LocalBox", "UnsupportedCoreContextPlan")

    method_specs = [
        ("CoreContext::next_binding", "next_binding(ctx): i64", "next_binding_id", "BindingIdNewAndIncrement"),
        ("CoreContext::next_temp_slot", "next_temp_slot(ctx): i64", "temp_slot_counter", "CounterNext"),
        ("CoreContext::next_debug_join", "next_debug_join(ctx): i64", "debug_join_counter", "CounterNext"),
    ]
    methods: list[HakoMethodIR] = []
    for method_id, signature, field, operation in method_specs:
        body = bodies.get(method_id)
        _require(body is not None and body.get("operation") == operation, "UnsupportedResolvedCallTarget")
        _require(body.get("selected_field") == field, "UnsupportedCounterField")
        plan_entry = plans.get(method_id)
        _require(plan_entry is not None and plan_entry.get("plan_kind") == operation, "UnsupportedCoreContextPlan")
        _require(plan_entry.get("transport") == "i64", "UnsupportedCounterTransport")
        methods.append(HakoMethodIR(signature, [op("TakeThenSaturatingIncrementU32", field=field)]))

    denied = {row["id"]: row for row in plan.get("denied", [])}
    for method_id in [
        "CoreContext::next_value",
        "CoreContext::next_block",
        "CoreContext::peek_next_value",
        "CoreContext::peek_next_block",
    ]:
        _require(denied.get(method_id, {}).get("deny_reason") == "GeneratorObjectTransportDeferred", "GeneratorObjectTransportDeferred")

    return methods

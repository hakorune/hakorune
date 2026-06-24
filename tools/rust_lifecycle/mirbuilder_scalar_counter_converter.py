#!/usr/bin/env python3
"""Compile bounded CoreContext scalar and nominal ID-counter facts."""

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


def _generator_state_facts(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("generator_state_facts", [])}


def _nominal_transports(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["transport"]: row for row in facts.get("nominal_id_transport_plan", [])}


def compile_core_context_scalar_methods(facts: dict[str, Any], plan: dict[str, Any]) -> list[HakoMethodIR]:
    """Compile the easy-tier CoreContext counters.

    ValueId and BasicBlockId use the same physical i64 lane, but the lowering
    requires distinct nominal transport labels before emitting Hako.
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

    generator_specs = [
        (
            "CoreContext::next_value",
            "next_value(ctx): i64",
            "value_gen",
            "value_next_id",
            "ValueIdGenerator::next",
            "GeneratorNext",
            "GeneratorNextScalar",
            "ValueIdAsI64",
        ),
        (
            "CoreContext::peek_next_value",
            "peek_next_value(ctx): i64",
            "value_gen",
            "value_next_id",
            "ValueIdGenerator::peek_next",
            "GeneratorPeekNext",
            "GeneratorPeekScalar",
            "ValueIdAsI64",
        ),
        (
            "CoreContext::next_block",
            "next_block(ctx): i64",
            "block_gen",
            "block_next_id",
            "BasicBlockIdGenerator::next",
            "GeneratorNext",
            "GeneratorNextScalar",
            "BasicBlockIdAsI64",
        ),
        (
            "CoreContext::peek_next_block",
            "peek_next_block(ctx): i64",
            "block_gen",
            "block_next_id",
            "BasicBlockIdGenerator::peek_next",
            "GeneratorPeekNext",
            "GeneratorPeekScalar",
            "BasicBlockIdAsI64",
        ),
    ]
    generator_facts = _generator_state_facts(facts)
    nominal_transports = _nominal_transports(facts)
    for (
        method_id,
        signature,
        source_field,
        hako_field,
        generator_fact_id,
        body_operation,
        plan_kind,
        transport,
    ) in generator_specs:
        body = bodies.get(method_id)
        _require(body is not None and body.get("operation") == body_operation, "UnsupportedResolvedCallTarget")
        _require(body.get("selected_field") == source_field, "UnsupportedGeneratorField")
        generator_fact = generator_facts.get(generator_fact_id)
        _require(generator_fact is not None, "GeneratorStateNotScalar")
        _require(generator_fact.get("state_field") == "next_id", "GeneratorStateNotScalar")
        _require(generator_fact.get("range") == "u32", "GeneratorStateNotScalar")
        _require(generator_fact.get("transport") == transport, "NominalIdTransportMismatch")
        nominal_transport = nominal_transports.get(transport)
        _require(nominal_transport is not None, "NominalIdTransportMismatch")
        _require(nominal_transport.get("physical_lane") == "i64", "UnsupportedTypeTransport")
        _require(nominal_transport.get("raw_i64_equivalence") is False, "NominalIdTransportMismatch")
        plan_entry = plans.get(method_id)
        _require(plan_entry is not None and plan_entry.get("plan_kind") == plan_kind, "UnsupportedCoreContextPlan")
        _require(plan_entry.get("source_field") == source_field, "UnsupportedGeneratorField")
        _require(plan_entry.get("field") == hako_field, "UnsupportedGeneratorField")
        _require(plan_entry.get("transport") == transport, "NominalIdTransportMismatch")
        if plan_kind == "GeneratorNextScalar":
            methods.append(HakoMethodIR(signature, [op("TakeThenSaturatingIncrementU32", field=hako_field)]))
        else:
            methods.append(HakoMethodIR(signature, [op("FieldGet", field=hako_field)]))

    return methods

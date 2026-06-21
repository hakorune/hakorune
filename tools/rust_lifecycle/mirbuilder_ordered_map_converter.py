#!/usr/bin/env python3
"""Compile easy-tier MirBuilder ordered-map facts to typed Hako operation IR."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import HakoMethodIR, op


class OrderedMapConversionDeny(RuntimeError):
    def __init__(self, reason: str) -> None:
        super().__init__(f"Deny({reason})")
        self.reason = reason


def _require(condition: bool, reason: str) -> None:
    if not condition:
        raise OrderedMapConversionDeny(reason)


def _method_shape(facts: dict[str, Any], method_id: str) -> dict[str, Any]:
    body_facts = {row["id"]: row for row in facts.get("body_facts", [])}
    shape = body_facts.get(method_id)
    _require(shape is not None, "UnsupportedResolvedCallTarget")
    return shape


def _require_plan(plan: dict[str, Any], field_id: str) -> None:
    plans = {row["id"]: row for row in plan.get("plans", [])}
    field_plan = plans.get(field_id)
    _require(field_plan is not None, "ConstructorLifecycleMismatch")
    _require(field_plan.get("plan_kind") == "OrderedMapBox", "ConstructorLifecycleMismatch")


def compile_variable_context_simple_map_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    field_name: str = "variable_map",
) -> list[HakoMethodIR]:
    """Return operation IR for the bounded VariableContext simple-map pilot."""
    _require_plan(plan, "VariableContext.variable_map")
    constructor = _method_shape(facts, "VariableContext::new")
    _require(constructor.get("operation") == "NewOrderedMap", "ConstructorLifecycleMismatch")
    _require(constructor.get("selected_field") == field_name, "ConstructorLifecycleMismatch")

    expected = {
        "VariableContext::lookup": "MapGet",
        "VariableContext::contains": "MapHas",
        "VariableContext::len": "MapLength",
        "VariableContext::is_empty": "MapIsEmpty",
        "VariableContext::insert": "MapSet",
        "VariableContext::remove": "MapRemove",
    }
    for method_id, operation in expected.items():
        shape = _method_shape(facts, method_id)
        _require(shape.get("operation") == operation, "UnsupportedResolvedCallTarget")
        _require(shape.get("selected_field") == field_name, "UnsupportedResolvedCallTarget")

    return [
        HakoMethodIR("is_empty(ctx): i64", [op("MapIsEmpty", field=field_name)]),
        HakoMethodIR("len(ctx): i64", [op("MapLength", field=field_name)]),
        HakoMethodIR("contains(ctx, name): i64", [op("MapHas", field=field_name, key="name")]),
        HakoMethodIR("lookup(ctx, name)", [op("MapGet", field=field_name, key="name")]),
        HakoMethodIR("insert(ctx, name, value_id): i64", [op("MapSet", field=field_name, key="name", value="value_id"), op("ReturnI64", return_value=0)]),
        HakoMethodIR("remove(ctx, name)", [op("MapRemove", field=field_name, key="name")]),
    ]

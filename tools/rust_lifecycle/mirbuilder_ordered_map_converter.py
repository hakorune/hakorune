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


def compile_ordered_map_family_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    field_id: str,
    field_name: str,
    value_arg: str,
    include_clear: bool = False,
) -> list[HakoMethodIR]:
    """Return operation IR for a bounded ordered-map context family."""
    _require_plan(plan, field_id)
    constructor = _method_shape(facts, f"{type_name}::new")
    _require(constructor.get("operation") == "NewOrderedMap", "ConstructorLifecycleMismatch")
    _require(constructor.get("selected_field") == field_name, "ConstructorLifecycleMismatch")

    expected = {
        f"{type_name}::lookup": "MapGet",
        f"{type_name}::contains": "MapHas",
        f"{type_name}::len": "MapLength",
        f"{type_name}::is_empty": "MapIsEmpty",
        f"{type_name}::insert": "MapSet",
        f"{type_name}::remove": "MapRemove",
    }
    if include_clear:
        expected[f"{type_name}::clear_for_function_entry"] = "MapClear"
    for method_id, operation in expected.items():
        shape = _method_shape(facts, method_id)
        _require(shape.get("operation") == operation, "UnsupportedResolvedCallTarget")
        _require(shape.get("selected_field") == field_name, "UnsupportedResolvedCallTarget")

    methods = [
        HakoMethodIR("is_empty(ctx): i64", [op("MapIsEmpty", field=field_name)]),
        HakoMethodIR("len(ctx): i64", [op("MapLength", field=field_name)]),
        HakoMethodIR("contains(ctx, name): i64", [op("MapHas", field=field_name, key="name")]),
        HakoMethodIR("lookup(ctx, name)", [op("MapGet", field=field_name, key="name", return_shape="mixed_runtime_i64_or_handle")]),
        HakoMethodIR(f"insert(ctx, name, {value_arg}): i64", [op("MapSet", field=field_name, key="name", value=value_arg, storage="OrderedMapBox"), op("ReturnI64", return_value=0)]),
        HakoMethodIR("remove(ctx, name)", [op("MapRemove", field=field_name, key="name")]),
    ]
    if include_clear:
        methods.append(HakoMethodIR("clear_for_function_entry(ctx): i64", [op("MapClear", field=field_name), op("ReturnI64", return_value=0)]))
    return methods


def compile_variable_context_simple_map_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    field_name: str = "variable_map",
) -> list[HakoMethodIR]:
    """Return operation IR for the bounded VariableContext simple-map pilot."""
    return compile_ordered_map_family_methods(
        facts,
        plan,
        type_name="VariableContext",
        field_id="VariableContext.variable_map",
        field_name=field_name,
        value_arg="value_id",
    )


def compile_binding_context_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    field_name: str = "binding_map",
) -> list[HakoMethodIR]:
    return compile_ordered_map_family_methods(
        facts,
        plan,
        type_name="BindingContext",
        field_id="BindingContext.binding_map",
        field_name=field_name,
        value_arg="binding_id",
        include_clear=True,
    )


def compile_variable_context_snapshot_restore_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    field_name: str = "variable_map",
) -> list[HakoMethodIR]:
    plans = {row["id"]: row for row in plan.get("plans", [])}
    _require(plans.get("VariableContext::snapshot", {}).get("plan_kind") == "CloneOwnedMap", "UnsupportedResolvedCallTarget")
    _require(plans.get("VariableContext::restore", {}).get("plan_kind") == "ReplaceOwned", "UnsupportedResolvedCallTarget")

    method_facts = {row["id"]: row for row in facts.get("method_facts", [])}
    snapshot = method_facts.get("VariableContext::snapshot")
    restore = method_facts.get("VariableContext::restore")
    _require(snapshot is not None and snapshot.get("operation") == "CloneOwnedMap", "UnsupportedResolvedCallTarget")
    _require(restore is not None and restore.get("operation") == "ReplaceOwned", "UnsupportedResolvedCallTarget")

    body_facts = {row["id"]: row for row in facts.get("body_facts", [])}
    snapshot_body = body_facts.get("VariableContext::snapshot")
    restore_body = body_facts.get("VariableContext::restore")
    _require(snapshot_body is not None and snapshot_body.get("operation") == "CloneOwnedMap", "UnsupportedResolvedCallTarget")
    _require(restore_body is not None and restore_body.get("operation") == "ReplaceOwnedMap", "UnsupportedResolvedCallTarget")
    _require(snapshot_body.get("selected_field") == field_name, "UnsupportedResolvedCallTarget")
    _require(restore_body.get("selected_field") == field_name, "UnsupportedResolvedCallTarget")

    return [
        HakoMethodIR("snapshot(ctx): OrderedMapBox", [op("CloneOwnedMap", field=field_name)]),
        HakoMethodIR("restore(ctx, snapshot)", [op("ReplaceOwnedMap", field=field_name, value="snapshot")]),
    ]


def compile_box_compilation_context_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    field_names: list[str] | None = None,
) -> list[HakoMethodIR]:
    field_names = field_names or ["variable_map", "value_origin_newbox", "value_types"]
    subject = "hakorune_mir_builder::context::BoxCompilationContext"
    _require(facts.get("kind") == "RustLifecycleFacts", "UnsupportedResolvedCallTarget")
    _require(plan.get("kind") == "HakoLifecyclePlan", "UnsupportedResolvedCallTarget")
    _require(facts.get("subject") == subject and plan.get("subject") == subject, "UnsupportedResolvedCallTarget")

    type_facts = {row["id"]: row for row in facts.get("type_facts", [])}
    field_facts = {row["id"]: row for row in facts.get("field_facts", [])}
    method_facts = {row["id"]: row for row in facts.get("method_facts", [])}
    body_facts = {row["id"]: row for row in facts.get("body_facts", [])}
    plans = {row["id"]: row for row in plan.get("plans", [])}

    _require(type_facts.get("BoxCompilationContext", {}).get("drop_fact") == "TrivialMemory", "ConstructorLifecycleMismatch")
    for field_name in field_names:
        field_id = f"BoxCompilationContext.{field_name}"
        field_fact = field_facts.get(field_id)
        _require(field_fact is not None, "ConstructorLifecycleMismatch")
        _require(field_fact.get("deterministic_order_required") is True, "ConstructorLifecycleMismatch")
        _require(field_fact.get("drop_fact") == "TrivialMemory", "ConstructorLifecycleMismatch")
        field_plan = plans.get(field_id)
        _require(field_plan is not None and field_plan.get("plan_kind") == "OrderedMapBox", "ConstructorLifecycleMismatch")

    constructor = method_facts.get("BoxCompilationContext::new")
    _require(constructor is not None, "ConstructorLifecycleMismatch")
    _require(constructor.get("returns", {}).get("copy_kind") == "NonCopyOwned", "ConstructorLifecycleMismatch")
    _require(constructor.get("returns", {}).get("drop_fact") == "TrivialMemory", "ConstructorLifecycleMismatch")
    constructor_body = body_facts.get("BoxCompilationContext::new")
    _require(constructor_body is not None and constructor_body.get("operation") == "DefaultConstruct", "ConstructorLifecycleMismatch")
    _require(constructor_body.get("selected_fields") == field_names, "ConstructorLifecycleMismatch")

    is_empty = method_facts.get("BoxCompilationContext::is_empty")
    _require(is_empty is not None, "UnsupportedResolvedCallTarget")
    _require(is_empty.get("returns", {}).get("copy_kind") == "ImmediateValue", "UnsupportedResolvedCallTarget")
    _require(is_empty.get("returns", {}).get("drop_fact") == "TrivialMemory", "UnsupportedResolvedCallTarget")
    is_empty_body = body_facts.get("BoxCompilationContext::is_empty")
    _require(is_empty_body is not None and is_empty_body.get("operation") == "CompositeMapIsEmpty", "UnsupportedResolvedCallTarget")
    _require(is_empty_body.get("selected_fields") == field_names, "UnsupportedResolvedCallTarget")
    _require(plans.get("BoxCompilationContext", {}).get("plan_kind") == "LocalBox", "ConstructorLifecycleMismatch")
    _require(plans.get("BoxCompilationContext::new", {}).get("plan_kind") == "DefaultConstruct", "ConstructorLifecycleMismatch")
    _require(plans.get("BoxCompilationContext::is_empty", {}).get("plan_kind") == "BorrowView", "UnsupportedResolvedCallTarget")
    return [
        HakoMethodIR(
            "is_empty(ctx): i64",
            [op("AllFieldsMapIsEmpty", source="ctx", fields=field_names)],
        )
    ]

#!/usr/bin/env python3
"""Generic direct lowering for scalar + optional atom metadata contexts."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import HakoMethodIR, op


def _body_facts_by_id(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("body_facts", [])}


def _require_body(
    body_facts: dict[str, dict[str, Any]],
    method_id: str,
    *,
    operation: str,
    field: str | None,
) -> None:
    body_fact = body_facts.get(method_id)
    if body_fact is None:
        raise ValueError(f"Deny(UnsupportedDirectShape): missing body fact {method_id}")
    if body_fact.get("operation") != operation:
        raise ValueError(f"Deny(UnsupportedResolvedCallTarget): {method_id}")
    if field is not None and body_fact.get("selected_field") != field:
        raise ValueError(f"Deny(UnsupportedDirectShape): {method_id} selected field")


def compile_scalar_option_atom_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    span_field: str,
    option_field: str,
    new_arg: str,
    span_arg: str,
    option_arg: str,
    method_ids: dict[str, str],
    signatures: dict[str, str],
) -> list[HakoMethodIR]:
    """Compile `metadata.scalar_option_atom` without family-name dispatch."""

    body_facts = _body_facts_by_id(facts)
    for method_key, operation, field in [
        ("new", "ConstructScalarOptionContext", None),
        ("get_scalar", "FieldGet", span_field),
        ("set_scalar", "FieldSet", span_field),
        ("set_option", "SetSome", option_field),
        ("clear_option", "ClearOption", option_field),
        ("get_option", "CloneImmutableString", option_field),
    ]:
        method_id = method_ids.get(method_key)
        if method_id is None:
            raise ValueError(f"Deny(UnsupportedDirectShape): missing method id {method_key}")
        _require_body(body_facts, method_id, operation=operation, field=field)

    plan_entries = {row["id"]: row for row in plan.get("plans", [])}
    type_plan = plan_entries.get(type_name)
    if type_plan is None or type_plan.get("shape_rule") != "metadata.scalar_option_atom":
        raise ValueError("Deny(UnsupportedDirectShape): expected metadata.scalar_option_atom")
    if type_plan.get("concrete_instantiation") != "MetadataContext<i64, i64>":
        raise ValueError("Deny(UnsupportedTypeTransport): expected concrete MetadataContext<i64, i64>")
    span_plan = plan_entries.get(f"{type_name}.{span_field}")
    if span_plan is None or span_plan.get("transport") != "i64":
        raise ValueError("Deny(UnsupportedTypeTransport): expected i64 span transport")
    option_plan = plan_entries.get(f"{type_name}.{option_field}")
    if option_plan is None or option_plan.get("transport") != "OptionStringBox":
        raise ValueError("Deny(UnsupportedTypeTransport): expected OptionStringBox transport")

    return [
        HakoMethodIR(
            signature=signatures["new"],
            operations=[
                op(
                    "NewBoxWithFieldValues",
                    target="ctx",
                    box=type_name,
                    field_values={
                        span_field: new_arg,
                        option_field: {"expr": "Option::None()"},
                    },
                )
            ],
        ),
        HakoMethodIR(
            signature=signatures["get_scalar"],
            operations=[op("FieldGet", field=span_field)],
        ),
        HakoMethodIR(
            signature=signatures["set_scalar"],
            operations=[op("FieldSet", field=span_field, value=span_arg)],
        ),
        HakoMethodIR(
            signature=signatures["set_option"],
            operations=[op("SetSome", field=option_field, value=option_arg)],
        ),
        HakoMethodIR(
            signature=signatures["clear_option"],
            operations=[op("ClearOption", field=option_field)],
        ),
        HakoMethodIR(
            signature=signatures["get_option"],
            operations=[op("CloneImmutableString", field=option_field)],
        ),
    ]

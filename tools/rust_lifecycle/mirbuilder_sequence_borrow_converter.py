#!/usr/bin/env python3
"""Direct lowering for call-local aggregate sequence borrow elimination."""

from __future__ import annotations

from typing import Any

from mirbuilder_borrow_use_classifier import ELIDE_TO_LEAF_PROJECTION, require_decision
from verified_hako_family_ir import HakoMethodIR, op


def _body_facts_by_id(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("body_facts", [])}


def _require_body(body_facts: dict[str, dict[str, Any]], method_id: str, *, operation: str, field: str) -> None:
    body_fact = body_facts.get(method_id)
    if body_fact is None:
        raise ValueError(f"Deny(UnsupportedDirectShape): missing body fact {method_id}")
    if body_fact.get("operation") != operation:
        raise ValueError(f"Deny(UnsupportedResolvedCallTarget): {method_id}")
    if body_fact.get("selected_field") != field:
        raise ValueError(f"Deny(UnsupportedDirectShape): {method_id} selected field")


def compile_sequence_last_copy_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    field_name: str,
    value_arg: str,
    method_ids: dict[str, str],
    push_signature: str,
    pop_signature: str,
    last_signature: str,
) -> list[HakoMethodIR]:
    """Compile `current_region_stack().last().copied()`-style use elimination."""

    body_facts = _body_facts_by_id(facts)
    for method_key, operation in [
        ("new", "NewSequence"),
        ("push", "SequencePush"),
        ("pop", "SequencePopOption"),
        ("last_copy", "SequenceLastOption"),
    ]:
        method_id = method_ids.get(method_key)
        if method_id is None:
            raise ValueError(f"Deny(UnsupportedDirectShape): missing method id {method_key}")
        _require_body(body_facts, method_id, operation=operation, field=field_name)

    field_facts = {row["id"]: row for row in facts.get("field_facts", [])}
    field_fact = field_facts.get(f"{type_name}.{field_name}")
    if field_fact is None or field_fact.get("transport") != "ArrayBox":
        raise ValueError("Deny(UnsupportedTypeTransport): expected ArrayBox sequence field")
    if field_fact.get("element_transport") != "i64":
        raise ValueError("Deny(UnsupportedTypeTransport): expected i64 sequence elements")
    if field_fact.get("identity_escapes") is not False:
        raise ValueError("Deny(ReturnedReadBorrow): detail=ReadLeaseRequired")

    use_facts = {row["id"]: row for row in facts.get("borrow_use_facts", [])}
    use_fact = use_facts.get(method_ids["last_copy"])
    if use_fact is None:
        raise ValueError("Deny(UnsupportedDirectShape): missing borrow-use fact")
    require_decision(use_fact, ELIDE_TO_LEAF_PROJECTION)

    plan_entries = {row["id"]: row for row in plan.get("plans", [])}
    field_plan = plan_entries.get(f"{type_name}.{field_name}")
    if field_plan is None or field_plan.get("shape_rule") != "borrow_use.sequence_last_copy":
        raise ValueError("Deny(UnsupportedDirectShape): expected borrow_use.sequence_last_copy")

    return [
        HakoMethodIR(signature=push_signature, operations=[op("SequencePush", field=field_name, value=value_arg)]),
        HakoMethodIR(signature=pop_signature, operations=[op("SequencePopOption", field=field_name)]),
        HakoMethodIR(signature=last_signature, operations=[op("SequenceLastOption", field=field_name)]),
    ]

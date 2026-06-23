#!/usr/bin/env python3
"""Generic direct lowering for optional map methods."""

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
    field: str,
) -> None:
    body_fact = body_facts.get(method_id)
    if body_fact is None:
        raise ValueError(f"Deny(UnsupportedDirectShape): missing body fact {method_id}")
    if body_fact.get("operation") != operation:
        raise ValueError(f"Deny(UnsupportedResolvedCallTarget): {method_id}")
    if body_fact.get("selected_field") != field:
        raise ValueError(f"Deny(UnsupportedDirectShape): {method_id} selected field")


def compile_optional_copy_default_map_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    field_name: str,
    key_arg: str,
    value_arg: str,
    default_value: str,
    method_ids: dict[str, str],
    try_get_signature: str,
    get_signature: str,
    set_signature: str,
) -> list[HakoMethodIR]:
    """Compile the generic `map.optional_copy_default` shape.

    The rule is intentionally family-neutral. The caller supplies names, while
    facts/plan must prove the semantic shape: map construction, optional get,
    defaulted get, and set on the same selected field.
    """

    body_facts = _body_facts_by_id(facts)
    for method_key, operation in [
        ("new", "NewMap"),
        ("try_get", "MapGetOption"),
        ("get_default", "MapGetDefault"),
        ("set", "MapSet"),
    ]:
        method_id = method_ids.get(method_key)
        if method_id is None:
            raise ValueError(f"Deny(UnsupportedDirectShape): missing method id {method_key}")
        _require_body(body_facts, method_id, operation=operation, field=field_name)

    plan_entries = {row["id"]: row for row in plan.get("plans", [])}
    field_plan = plan_entries.get(f"{type_name}.{field_name}")
    if field_plan is None or field_plan.get("plan_kind") != "MapBox":
        raise ValueError("Deny(UnsupportedTypeTransport): expected MapBox field plan")
    if field_plan.get("shape_rule") != "map.optional_copy_default":
        raise ValueError("Deny(UnsupportedDirectShape): expected map.optional_copy_default")

    return [
        HakoMethodIR(
            signature=try_get_signature,
            operations=[op("MapGetOption", field=field_name, key=key_arg, storage="MapBox")],
        ),
        HakoMethodIR(
            signature=get_signature,
            operations=[
                op("MapGetOption", field=field_name, key=key_arg, target="kind", storage="MapBox"),
                op("ReturnDefaultIfMissing", source="kind", default=default_value),
            ],
        ),
        HakoMethodIR(
            signature=set_signature,
            operations=[op("MapSet", field=field_name, key=key_arg, value=value_arg, storage="MapBox")],
        ),
    ]


def compile_optional_immutable_atom_map_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    field_name: str,
    key_arg: str,
    value_arg: str,
    method_ids: dict[str, str],
    get_signature: str,
    set_signature: str,
    clear_signature: str,
) -> list[HakoMethodIR]:
    """Compile the generic `map.optional_immutable_atom` shape.

    This covers map-backed immutable atom projection, such as Rust
    `Option<&str>` over an owned `String` value. It must not return the map
    itself or infer value transports from method names.
    """

    body_facts = _body_facts_by_id(facts)
    for method_key, operation in [
        ("new", "NewMap"),
        ("get", "MapGetOption"),
        ("set", "MapSet"),
        ("clear", "MapClear"),
    ]:
        method_id = method_ids.get(method_key)
        if method_id is None:
            raise ValueError(f"Deny(UnsupportedDirectShape): missing method id {method_key}")
        _require_body(body_facts, method_id, operation=operation, field=field_name)

    plan_entries = {row["id"]: row for row in plan.get("plans", [])}
    field_plan = plan_entries.get(f"{type_name}.{field_name}")
    if field_plan is None or field_plan.get("plan_kind") != "MapBox":
        raise ValueError("Deny(UnsupportedTypeTransport): expected MapBox field plan")
    if field_plan.get("shape_rule") != "map.optional_immutable_atom":
        raise ValueError("Deny(UnsupportedDirectShape): expected map.optional_immutable_atom")
    if field_plan.get("key_transport") != "ValueIdAsI64":
        raise ValueError("Deny(UnsupportedKeyTransport): expected ValueIdAsI64")
    if field_plan.get("value_transport") != "ImmutableStringAtom":
        raise ValueError("Deny(UnsupportedTypeTransport): expected ImmutableStringAtom")

    return [
        HakoMethodIR(
            signature=get_signature,
            operations=[op("MapGetOption", field=field_name, key=key_arg, storage="MapBox")],
        ),
        HakoMethodIR(
            signature=set_signature,
            operations=[op("MapSet", field=field_name, key=key_arg, value=value_arg, storage="MapBox")],
        ),
        HakoMethodIR(
            signature=clear_signature,
            operations=[op("MapClear", field=field_name, storage="MapBox")],
        ),
    ]


def compile_optional_owned_recursive_enum_map_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    field_name: str,
    key_arg: str,
    value_arg: str,
    method_ids: dict[str, str],
    get_signature: str,
    set_signature: str,
    enum_type: str,
) -> list[HakoMethodIR]:
    """Compile `map.optional_owned_recursive_enum`.

    This rule covers `Option<&Enum>` Rust reads only when facts prove the
    returned borrow is projected to an owned enum value and the map identity
    itself does not escape.
    """

    body_facts = _body_facts_by_id(facts)
    for method_key, operation in [
        ("new", "NewMap"),
        ("get", "MapGetOption"),
        ("set", "MapSet"),
    ]:
        method_id = method_ids.get(method_key)
        if method_id is None:
            raise ValueError(f"Deny(UnsupportedDirectShape): missing method id {method_key}")
        _require_body(body_facts, method_id, operation=operation, field=field_name)

    get_fact = body_facts[method_ids["get"]]
    if get_fact.get("returned_borrow_projected_to_owned") is not True:
        raise ValueError("Deny(ReturnedReadBorrow): owned enum projection is required")
    if get_fact.get("returned_aggregate_alias") is not False:
        raise ValueError("Deny(ReturnedReadBorrow): map aggregate must not escape")

    field_facts = {row["id"]: row for row in facts.get("field_facts", [])}
    field_fact = field_facts.get(f"{type_name}.{field_name}")
    if field_fact is None:
        raise ValueError("Deny(UnsupportedDirectShape): missing field fact")
    if field_fact.get("map_identity_escapes") is not False:
        raise ValueError("Deny(ReturnedReadBorrow): map identity must not escape")
    if field_fact.get("value_transport") != "OwnedRecursiveEnum":
        raise ValueError("Deny(UnsupportedTypeTransport): expected OwnedRecursiveEnum")

    type_facts = {row["id"]: row for row in facts.get("type_facts", [])}
    enum_fact = type_facts.get(enum_type)
    if enum_fact is None or enum_fact.get("transport") != "OwnedRecursiveEnum":
        raise ValueError("Deny(UnsupportedTypeTransport): missing owned recursive enum facts")
    if enum_fact.get("recursive") is not True:
        raise ValueError("Deny(UnsupportedTypeTransport): recursive enum fact must be explicit")

    plan_entries = {row["id"]: row for row in plan.get("plans", [])}
    field_plan = plan_entries.get(f"{type_name}.{field_name}")
    if field_plan is None or field_plan.get("plan_kind") != "MapBox":
        raise ValueError("Deny(UnsupportedTypeTransport): expected MapBox field plan")
    if field_plan.get("shape_rule") != "map.optional_owned_recursive_enum":
        raise ValueError("Deny(UnsupportedDirectShape): expected map.optional_owned_recursive_enum")
    if field_plan.get("key_transport") != "ValueIdAsI64":
        raise ValueError("Deny(UnsupportedKeyTransport): expected ValueIdAsI64")
    if field_plan.get("value_transport") != f"{enum_type}OwnedRecursiveEnum":
        raise ValueError("Deny(UnsupportedTypeTransport): expected owned recursive enum plan")

    return [
        HakoMethodIR(
            signature=get_signature,
            operations=[op("MapGetOption", field=field_name, key=key_arg, storage="MapBox")],
        ),
        HakoMethodIR(
            signature=set_signature,
            operations=[op("MapSet", field=field_name, key=key_arg, value=value_arg, storage="MapBox")],
        ),
    ]


def compile_immutable_leaf_projection_map_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    field_name: str,
    key_arg: str,
    method_ids: dict[str, str],
    get_signature: str,
) -> list[HakoMethodIR]:
    """Compile a get-only immutable leaf borrow projection.

    This rule is for Rust `Option<&str>`-style reads where the returned leaf is
    projected to an owned/immutable Hako atom and the containing map never
    escapes. Mutating or formatting producers stay outside this rule.
    """

    body_facts = _body_facts_by_id(facts)
    for method_key, operation in [
        ("new", "NewMap"),
        ("get", "MapGetOption"),
    ]:
        method_id = method_ids.get(method_key)
        if method_id is None:
            raise ValueError(f"Deny(UnsupportedDirectShape): missing method id {method_key}")
        _require_body(body_facts, method_id, operation=operation, field=field_name)

    get_fact = body_facts[method_ids["get"]]
    if get_fact.get("value_projection") != "ImmutableStringAtom":
        raise ValueError("Deny(UnsupportedTypeTransport): expected immutable string projection")
    if get_fact.get("returned_aggregate_alias") is not False:
        raise ValueError("Deny(ReturnedReadBorrow): aggregate alias must not escape")

    field_facts = {row["id"]: row for row in facts.get("field_facts", [])}
    field_fact = field_facts.get(f"{type_name}.{field_name}")
    if field_fact is None:
        raise ValueError("Deny(UnsupportedDirectShape): missing field fact")
    if field_fact.get("key_transport") != "ValueIdAsI64":
        raise ValueError("Deny(UnsupportedKeyTransport): expected ValueIdAsI64")
    if field_fact.get("value_transport") != "ImmutableStringAtom":
        raise ValueError("Deny(UnsupportedTypeTransport): expected ImmutableStringAtom")
    if field_fact.get("map_identity_escapes") is not False:
        raise ValueError("Deny(ReturnedReadBorrow): map identity must not escape")

    plan_entries = {row["id"]: row for row in plan.get("plans", [])}
    field_plan = plan_entries.get(f"{type_name}.{field_name}")
    if field_plan is None or field_plan.get("plan_kind") != "MapBox":
        raise ValueError("Deny(UnsupportedTypeTransport): expected MapBox field plan")
    if field_plan.get("shape_rule") != "map.immutable_leaf_projection":
        raise ValueError("Deny(UnsupportedDirectShape): expected map.immutable_leaf_projection")
    if field_plan.get("value_transport") != "ImmutableStringAtom":
        raise ValueError("Deny(UnsupportedTypeTransport): expected ImmutableStringAtom plan")

    return [
        HakoMethodIR(
            signature=get_signature,
            operations=[op("MapGetOption", field=field_name, key=key_arg, storage="MapBox")],
        ),
    ]

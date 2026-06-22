#!/usr/bin/env python3
"""Generic direct lowering for aggregate take/restore with defaults."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import HakoMethodIR, op


def _body_facts_by_id(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("body_facts", [])}


def _plans_by_id(plan: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in plan.get("plans", [])}


def _require_field_sequence(actual: Any, expected: list[str], label: str) -> None:
    if actual != expected:
        raise ValueError(f"Deny(UnsupportedOwnedTransferShape): {label} field order")


def _move_ops(
    *,
    source_owner: str,
    target_owner: str,
    fields: list[str],
) -> list[Any]:
    return [
        op(
            "MoveFieldAndResetSource",
            source_owner=source_owner,
            source_field=field,
            target_owner=target_owner,
            target_field=field,
            replacement={"kind": "NewMap"},
        )
        for field in fields
    ]


def compile_aggregate_take_restore_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    type_name: str,
    snapshot_type: str,
    fields: list[str],
    method_ids: dict[str, str],
) -> list[HakoMethodIR]:
    """Compile exact `std::mem::take` aggregate snapshot/restore shapes.

    Field content is opaque for this rule. The selected methods must only move
    whole containers and replace the source field with a known default.
    """

    body_facts = _body_facts_by_id(facts)
    take_fact = body_facts.get(method_ids["take"])
    restore_fact = body_facts.get(method_ids["restore"])
    if take_fact is None or restore_fact is None:
        raise ValueError("Deny(UnsupportedOwnedTransferShape): missing body facts")
    if take_fact.get("operation") != "AggregateTakeWithDefaults":
        raise ValueError("Deny(UnsupportedResolvedCallTarget): take_snapshot")
    if restore_fact.get("operation") != "AggregateRestoreWithDefaults":
        raise ValueError("Deny(UnsupportedResolvedCallTarget): restore_snapshot")
    _require_field_sequence(take_fact.get("fields"), fields, "take")
    _require_field_sequence(restore_fact.get("fields"), fields, "restore")
    if take_fact.get("entry_access_required") is not False:
        raise ValueError("Deny(UnsupportedFieldTransport): take must not read entries")
    if restore_fact.get("entry_access_required") is not False:
        raise ValueError("Deny(UnsupportedFieldTransport): restore must not read entries")
    if restore_fact.get("snapshot_parameter") != "by_value":
        raise ValueError("Deny(UnsupportedOwnedTransferShape): snapshot must be by-value")

    plans = _plans_by_id(plan)
    shape_plan = plans.get(f"{type_name}.{snapshot_type}")
    if shape_plan is None or shape_plan.get("shape_rule") != "aggregate.take_restore_with_defaults":
        raise ValueError("Deny(UnsupportedDirectShape): expected aggregate.take_restore_with_defaults")
    if shape_plan.get("opaque_container_move") is not True:
        raise ValueError("Deny(UnsupportedFieldTransport): opaque container move required")
    if shape_plan.get("default_replacement") != "NewMap":
        raise ValueError("Deny(UnsupportedDefaultConstruction): expected NewMap replacement")

    for field in fields:
        field_plan = plans.get(f"{type_name}.{field}")
        if field_plan is None:
            raise ValueError(f"Deny(UnsupportedFieldTransport): missing plan for {field}")
        if field_plan.get("snapshot_transport") != "OpaqueOwnedMapStorage":
            raise ValueError(f"Deny(UnsupportedFieldTransport): expected opaque map storage for {field}")

    return [
        HakoMethodIR(
            signature=f"take_snapshot(ctx): {snapshot_type}",
            operations=[
                op("NewLocalBox", target="snapshot", box=snapshot_type),
                *_move_ops(source_owner="ctx", target_owner="snapshot", fields=fields),
                op("ReturnSource", source="snapshot"),
            ],
        ),
        HakoMethodIR(
            signature=f"restore_snapshot(ctx, snapshot: {snapshot_type}): i64",
            operations=[
                op("AssertNotConsumed", source="snapshot.consumed", fail_message="type_context_snapshot_already_consumed=fail", fail_code=7),
                *_move_ops(source_owner="snapshot", target_owner="ctx", fields=fields),
                op("MarkConsumed", source="snapshot.consumed"),
            ],
        ),
    ]

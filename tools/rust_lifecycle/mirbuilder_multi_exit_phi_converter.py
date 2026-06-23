#!/usr/bin/env python3
"""Direct lowering for explicit multi-carrier PHI across loop exits."""

from __future__ import annotations

from typing import Any

from mirbuilder_structured_loop_converter import _deny, _require_expr
from verified_hako_family_ir import HakoMethodIR, op


def _body_facts_by_id(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("body_facts", [])}


def _plans_by_id(plan: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in plan.get("plans", [])}


def _require_exit_values(exits: Any, *, carrier_count: int) -> None:
    if not isinstance(exits, list):
        _deny("UnsupportedDirectShape", "exits")
    kinds = [row.get("kind") for row in exits if isinstance(row, dict)]
    if kinds != ["break", "continue", "early_return"]:
        _deny("UnstructuredControlFlow", "expected break/continue/early_return exits")
    allowed_expr = {"Var", "I64", "EqI64"}
    for row in exits:
        condition = row.get("condition")
        values = row.get("values")
        _require_expr(condition, allowed=allowed_expr, detail="exit.condition")
        if not isinstance(values, list) or len(values) != carrier_count:
            _deny("PhiJoinRequired", "carrier arity mismatch")
        for value in values:
            _require_expr(value, allowed=allowed_expr, detail="exit.value")


def compile_multi_carrier_exit_phi_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    method_id: str,
    signature: str,
) -> list[HakoMethodIR]:
    body_fact = _body_facts_by_id(facts).get(method_id)
    if body_fact is None:
        _deny("UnsupportedDirectShape", "missing body fact")
    if body_fact.get("operation") != "MultiCarrierExitPhi":
        _deny("UnsupportedResolvedCallTarget", str(body_fact.get("operation")))
    if body_fact.get("phi_kind") != "explicit":
        _deny("PhiJoinRequired", "inferred phi")
    carriers = body_fact.get("carriers")
    if not isinstance(carriers, list) or len(carriers) < 2:
        _deny("PhiJoinRequired", "expected multiple carriers")
    for carrier in carriers:
        if not isinstance(carrier, dict) or carrier.get("type") != "i64":
            _deny("UnsupportedTypeTransport", "carrier must be i64")
        if carrier.get("escapes") is not False:
            _deny("CarrierSensitiveAlias", "carrier escapes")
    exits = body_fact.get("exits")
    _require_exit_values(exits, carrier_count=len(carriers))

    plans = _plans_by_id(plan)
    shape_plan = plans.get(method_id)
    if shape_plan is None or shape_plan.get("shape_rule") != "control.multi_carrier_exit_phi":
        _deny("UnsupportedDirectShape", "missing multi-exit PHI shape plan")
    if shape_plan.get("raw_hako_body") is not False:
        _deny("UnsupportedDirectShape", "raw Hako body disabled")

    return [
        HakoMethodIR(
            signature=signature,
            operations=[
                op(
                    "ExplicitMultiExitPhiI64Array",
                    target=body_fact.get("target", "carriers"),
                    selector=body_fact.get("selector"),
                    exits=exits,
                    fail_message="multi_exit_phi_unknown_exit=fail",
                    fail_code=7,
                ),
                op("ReturnSource", source=body_fact.get("target", "carriers")),
            ],
        )
    ]

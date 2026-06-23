#!/usr/bin/env python3
"""Direct lowering for structured loops without semantic carried state."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import HakoMethodIR, op


def _body_facts_by_id(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("body_facts", [])}


def _plans_by_id(plan: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in plan.get("plans", [])}


def _deny(reason: str, detail: str) -> None:
    raise ValueError(f"Deny({reason}): {detail}")


def _require_expr(expr: Any, *, allowed: set[str], detail: str) -> None:
    if not isinstance(expr, dict):
        _deny("UnsupportedDirectShape", f"{detail} must be typed expression")
    kind = expr.get("kind")
    if kind not in allowed:
        _deny("UnsupportedDirectShape", f"{detail} unsupported expression {kind}")
    if kind in {"LtI64", "EqI64", "AddI64"}:
        _require_expr(expr.get("left"), allowed=allowed, detail=f"{detail}.left")
        _require_expr(expr.get("right"), allowed=allowed, detail=f"{detail}.right")
    if kind == "ArrayLength" and not isinstance(expr.get("source"), str):
        _deny("UnsupportedDirectShape", f"{detail}.source")
    if kind == "ArrayGet":
        if not isinstance(expr.get("source"), str):
            _deny("UnsupportedDirectShape", f"{detail}.source")
        _require_expr(expr.get("index"), allowed=allowed, detail=f"{detail}.index")
    if kind == "Var" and not isinstance(expr.get("name"), str):
        _deny("UnsupportedDirectShape", f"{detail}.name")
    if kind == "I64" and not isinstance(expr.get("value"), int):
        _deny("UnsupportedDirectShape", f"{detail}.value")


def _require_loop_body(body: Any) -> None:
    if not isinstance(body, list) or not body:
        _deny("UnsupportedDirectShape", "loop body")
    allowed_expr = {"Var", "I64", "ArrayLength", "ArrayGet", "AddI64", "LtI64", "EqI64"}
    for item in body:
        if not isinstance(item, dict):
            _deny("UnsupportedDirectShape", "body entry")
        kind = item.get("kind")
        if kind == "ArrayPush":
            if not isinstance(item.get("target"), str):
                _deny("UnsupportedDirectShape", "ArrayPush.target")
            _require_expr(item.get("value"), allowed=allowed_expr, detail="ArrayPush.value")
            continue
        if kind == "Assign":
            if not isinstance(item.get("target"), str):
                _deny("UnsupportedDirectShape", "Assign.target")
            _require_expr(item.get("value"), allowed=allowed_expr, detail="Assign.value")
            continue
        if kind in {"Break", "Continue", "ReturnI64", "ReturnSource"}:
            _deny("UnstructuredControlFlow", kind)
        _deny("UnsupportedResolvedCallTarget", str(kind))


def compile_structured_loop_without_carried_state_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    method_id: str,
    signature: str,
) -> list[HakoMethodIR]:
    body_fact = _body_facts_by_id(facts).get(method_id)
    if body_fact is None:
        _deny("UnsupportedDirectShape", "missing body fact")
    if body_fact.get("operation") != "StructuredLoopWithoutCarriedState":
        _deny("UnsupportedResolvedCallTarget", str(body_fact.get("operation")))
    if body_fact.get("break_count") != 0 or body_fact.get("continue_count") != 0:
        _deny("UnstructuredControlFlow", "break/continue")
    if body_fact.get("early_return_count") != 0:
        _deny("UnstructuredControlFlow", "early return")
    if body_fact.get("phi_required") is not False:
        _deny("PhiJoinRequired", "phi required")
    if body_fact.get("loop_carried_state") is not False:
        _deny("LoopCarriedStateRequired", "carried state")

    plans = _plans_by_id(plan)
    shape_plan = plans.get(method_id)
    if shape_plan is None or shape_plan.get("shape_rule") != "control.structured_loop_without_carried_state":
        _deny("UnsupportedDirectShape", "missing loop shape plan")
    if shape_plan.get("raw_hako_body") is not False:
        _deny("UnsupportedDirectShape", "raw Hako body disabled")

    condition = body_fact.get("condition")
    body = body_fact.get("body")
    _require_expr(condition, allowed={"Var", "I64", "ArrayLength", "LtI64", "EqI64"}, detail="loop.condition")
    _require_loop_body(body)

    return [
        HakoMethodIR(
            signature=signature,
            operations=[
                op("LocalI64", target=body_fact.get("index_var", "i"), value={"kind": "I64", "value": 0}),
                op("StructuredLoop", condition=condition, body=body),
                op("ReturnI64", return_value=0),
            ],
        )
    ]


def compile_single_scalar_loop_carrier_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    method_id: str,
    signature: str,
) -> list[HakoMethodIR]:
    body_fact = _body_facts_by_id(facts).get(method_id)
    if body_fact is None:
        _deny("UnsupportedDirectShape", "missing body fact")
    if body_fact.get("operation") != "SingleScalarLoopCarrier":
        _deny("UnsupportedResolvedCallTarget", str(body_fact.get("operation")))
    if body_fact.get("break_count") != 0 or body_fact.get("continue_count") != 0:
        _deny("UnstructuredControlFlow", "break/continue")
    if body_fact.get("early_return_count") != 0:
        _deny("UnstructuredControlFlow", "early return")
    if body_fact.get("phi_required") is not False:
        _deny("PhiJoinRequired", "phi required")
    if body_fact.get("loop_carried_state") != "single_scalar":
        _deny("LoopCarriedStateRequired", "expected one scalar carrier")
    carrier = body_fact.get("carrier")
    if not isinstance(carrier, dict):
        _deny("UnsupportedDirectShape", "missing carrier")
    carrier_name = carrier.get("name")
    if not isinstance(carrier_name, str):
        _deny("UnsupportedDirectShape", "carrier.name")
    if carrier.get("type") != "i64":
        _deny("UnsupportedTypeTransport", "carrier must be i64")
    if carrier.get("escapes") is not False:
        _deny("CarrierSensitiveAlias", "carrier escapes")

    plans = _plans_by_id(plan)
    shape_plan = plans.get(method_id)
    if shape_plan is None or shape_plan.get("shape_rule") != "control.single_scalar_loop_carrier":
        _deny("UnsupportedDirectShape", "missing scalar carrier shape plan")
    if shape_plan.get("raw_hako_body") is not False:
        _deny("UnsupportedDirectShape", "raw Hako body disabled")

    condition = body_fact.get("condition")
    body = body_fact.get("body")
    _require_expr(condition, allowed={"Var", "I64", "ArrayLength", "LtI64", "EqI64"}, detail="loop.condition")
    _require_loop_body(body)

    return [
        HakoMethodIR(
            signature=signature,
            operations=[
                op("LocalI64", target=body_fact.get("index_var", "i"), value={"kind": "I64", "value": 0}),
                op("LocalI64", target=carrier_name, value={"kind": "I64", "value": carrier.get("initial_value", 0)}),
                op("StructuredLoop", condition=condition, body=body),
                op("ReturnSource", source=carrier_name),
            ],
        )
    ]

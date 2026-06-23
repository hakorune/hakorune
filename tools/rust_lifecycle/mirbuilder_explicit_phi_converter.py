#!/usr/bin/env python3
"""Direct lowering for canonical explicit scalar PHI facts."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import HakoMethodIR, op
from mirbuilder_structured_loop_converter import _deny, _require_expr


def _body_facts_by_id(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("body_facts", [])}


def _plans_by_id(plan: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in plan.get("plans", [])}


def compile_canonical_explicit_phi_methods(
    facts: dict[str, Any],
    plan: dict[str, Any],
    *,
    method_id: str,
    signature: str,
) -> list[HakoMethodIR]:
    body_fact = _body_facts_by_id(facts).get(method_id)
    if body_fact is None:
        _deny("UnsupportedDirectShape", "missing body fact")
    if body_fact.get("operation") != "CanonicalExplicitPhi":
        _deny("UnsupportedResolvedCallTarget", str(body_fact.get("operation")))
    if body_fact.get("phi_kind") != "explicit":
        _deny("PhiJoinRequired", "inferred phi")
    if body_fact.get("predecessor_count") != 2:
        _deny("PhiJoinRequired", "expected two explicit predecessors")
    if body_fact.get("value_type") != "i64":
        _deny("UnsupportedTypeTransport", "PHI value must be i64")

    target = body_fact.get("target")
    if not isinstance(target, str):
        _deny("UnsupportedDirectShape", "target")
    condition = body_fact.get("condition")
    true_value = body_fact.get("true_value")
    false_value = body_fact.get("false_value")
    allowed_expr = {"Var", "I64", "EqI64", "LtI64"}
    _require_expr(condition, allowed=allowed_expr, detail="phi.condition")
    _require_expr(true_value, allowed=allowed_expr, detail="phi.true_value")
    _require_expr(false_value, allowed=allowed_expr, detail="phi.false_value")

    plans = _plans_by_id(plan)
    shape_plan = plans.get(method_id)
    if shape_plan is None or shape_plan.get("shape_rule") != "control.canonical_explicit_phi":
        _deny("UnsupportedDirectShape", "missing explicit PHI shape plan")
    if shape_plan.get("raw_hako_body") is not False:
        _deny("UnsupportedDirectShape", "raw Hako body disabled")

    return [
        HakoMethodIR(
            signature=signature,
            operations=[
                op(
                    "ExplicitPhiI64",
                    target=target,
                    condition=condition,
                    true_value=true_value,
                    false_value=false_value,
                ),
                op("ReturnSource", source=target),
            ],
        )
    ]

#!/usr/bin/env python3
"""Project Hako ProgramJSON into parser-neutral recursive grammar evidence."""

from __future__ import annotations

from typing import Any


class HakoProjectionError(ValueError):
    stable_reject_tag = "parser/witness_missing"


def _node(
    kind: str,
    *,
    value: str | None = None,
    children: list[dict[str, Any]] | None = None,
) -> dict[str, Any]:
    node: dict[str, Any] = {"kind": kind, "children": children or []}
    if value is not None:
        node["value"] = value
    return node


def _object(value: Any, detail: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise HakoProjectionError(detail)
    return value


def _single_body(program: dict[str, Any]) -> dict[str, Any]:
    body = program.get("body")
    if not isinstance(body, list) or len(body) != 1:
        raise HakoProjectionError("Hako grammar witness requires one body item")
    return _object(body[0], "Hako body item must be an object")


def _loop_body(statement: dict[str, Any]) -> dict[str, Any]:
    body = statement.get("body")
    if not isinstance(body, list):
        raise HakoProjectionError("Hako loop body is missing")
    children = []
    for item in body:
        kind = _object(item, "Hako loop body item must be an object").get("type")
        if kind not in {"Break", "Continue"}:
            raise HakoProjectionError("Hako loop witness only accepts control exits")
        children.append(_node(kind))
    return _node("LoopBody", children=children)


def _guard_expr(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_body(program)
    if statement.get("type") != "If":
        raise HakoProjectionError("Hako guard expression did not produce If")
    return _node(
        "GuardElse",
        children=[_node("Condition"), _node("NoFallthroughElse")],
    )


def _guard_let(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_body(program)
    if statement.get("type") != "GuardLet":
        raise HakoProjectionError("Hako guard let shape is missing")
    return _node(
        "GuardLetElse",
        children=[_node("Pattern"), _node("Expr"), _node("NoFallthroughElse")],
    )


def _try_shape(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    statement = _single_body(program)
    if row_id == "fini":
        if statement.get("type") != "FiniReg":
            raise HakoProjectionError("Hako fini shape is missing")
        return _node("Fini", children=[_node("CleanupBody")])
    if statement.get("type") != "Try":
        raise HakoProjectionError("Hako postfix/try shape is missing")
    has_catch = isinstance(statement.get("catches"), list) and bool(statement["catches"])
    has_cleanup = isinstance(statement.get("finally"), list) and bool(statement["finally"])
    if row_id == "postfix_catch" and has_catch and not has_cleanup:
        return _node("PostfixCatch", children=[_node("Body"), _node("CatchHandler")])
    if row_id == "postfix_cleanup" and has_cleanup and not has_catch:
        return _node("PostfixCleanup", children=[_node("Body"), _node("CleanupBody")])
    if row_id == "try_statement" and has_catch and has_cleanup:
        return _node(
            "PostfixCatchCleanup",
            children=[_node("Body"), _node("CatchHandler"), _node("CleanupBody")],
        )
    raise HakoProjectionError("Hako postfix/try shape disagrees with its grammar row")


def _match(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_body(program)
    expression = statement.get("expr") if statement.get("type") == "Expr" else statement
    expression = _object(expression, "Hako match expression is missing")
    if expression.get("type") != "EnumMatch":
        raise HakoProjectionError("Hako match witness requires EnumMatch")
    arms = expression.get("arms")
    if not isinstance(arms, list):
        raise HakoProjectionError("Hako match arms are missing")
    normalized_arms = [
        _node("MatchArm", children=[_node("Pattern"), _node("ArmBody")])
        for _ in arms
    ]
    return _node(
        "Match",
        children=[_node("Scrutinee"), _node("OrderedArms", children=normalized_arms)],
    )


def _delegate(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_body(program)
    if statement.get("type") != "DelegateExposes":
        raise HakoProjectionError("Hako delegate evidence shape is missing")
    return _node(
        "DelegateExposes",
        children=[_node("Field"), _node("ExposedMembers")],
    )


def _loop(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    statement = _single_body(program)
    if row_id == "loop_range":
        if statement.get("type") != "LoopRange":
            raise HakoProjectionError("Hako range loop shape is missing")
        return _node(
            "LoopRange",
            children=[
                _node("RangeIndex"),
                _node("RangeStart"),
                _node("RangeEnd"),
                _loop_body(statement),
            ],
        )
    if statement.get("type") != "Loop":
        raise HakoProjectionError("Hako loop shape is missing")
    body = _loop_body(statement)
    if row_id in {"loop_infinite", "break_statement", "continue_statement"}:
        condition = _object(statement.get("cond"), "Hako loop condition is missing")
        if condition.get("type") != "Bool" or condition.get("value") is not True:
            raise HakoProjectionError("Hako infinite loop condition must be true")
        return _node("LoopInfinite", children=[body])
    condition = _object(statement.get("cond"), "Hako loop condition is missing")
    if condition.get("type") == "RecordLiteral":
        raise HakoProjectionError("Hako loop condition consumed the body brace")
    return _node("LoopCondition", children=[_node("Condition"), body])


def _map_literal(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_body(program)
    expression = statement.get("expr") if statement.get("type") == "Expr" else None
    expression = _object(expression, "Hako map expression is missing")
    args = expression.get("args")
    if expression.get("type") != "Call" or expression.get("name") != "map.of":
        raise HakoProjectionError("Hako map witness requires map.of evidence")
    if not isinstance(args, list) or len(args) != 2:
        raise HakoProjectionError("Hako map witness requires one canonical entry")
    if _object(args[0], "Hako map key is missing").get("type") != "Str":
        raise HakoProjectionError("Hako map key must be a string")
    if _object(args[1], "Hako map value is missing").get("type") != "Int":
        raise HakoProjectionError("Hako map fixture value must be integer")
    return _node("MapLiteral", children=[_node("StringKey"), _node("IntegerLiteral")])


def project_hako_normalized_form(row_id: str, program: Any) -> dict[str, Any]:
    program = _object(program, "Hako ProgramJSON must be an object")
    if row_id == "guard_expr_else":
        return _guard_expr(program)
    if row_id == "guard_let_else":
        return _guard_let(program)
    if row_id in {"postfix_catch", "postfix_cleanup", "fini", "try_statement"}:
        return _try_shape(program, row_id)
    if row_id in {"match", "peek"}:
        return _match(program)
    if row_id == "delegate_exposes":
        return _delegate(program)
    if row_id in {
        "while_loop_condition",
        "loop_infinite",
        "loop_condition",
        "loop_range",
        "break_statement",
        "continue_statement",
    }:
        return _loop(program, row_id)
    if row_id == "map_literal_percent_brace":
        return _map_literal(program)
    raise HakoProjectionError(f"Hako grammar row projection is missing: {row_id}")

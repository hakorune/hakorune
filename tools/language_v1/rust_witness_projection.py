#!/usr/bin/env python3
"""Project Rust AST JSON into parser-neutral recursive grammar evidence."""

from __future__ import annotations

from typing import Any


class RustProjectionError(ValueError):
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


def _require_dict(value: Any, detail: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise RustProjectionError(detail)
    return value


def _single_statement(program: dict[str, Any]) -> dict[str, Any]:
    statements = program.get("statements")
    if not isinstance(statements, list):
        raise RustProjectionError("Rust grammar witness statements are missing")
    semantic_statements = [
        statement
        for statement in statements
        if isinstance(statement, dict)
        and statement.get("kind")
        not in {"EnumDeclaration", "BrandDeclaration", "TypeAliasDeclaration"}
    ]
    if len(semantic_statements) != 1:
        raise RustProjectionError("Rust grammar witness requires one statement")
    return semantic_statements[0]


def _loop_body(statement: dict[str, Any]) -> dict[str, Any]:
    body = statement.get("body")
    if not isinstance(body, list):
        raise RustProjectionError("Rust loop body is missing")
    children = []
    for item in body:
        kind = _require_dict(item, "Rust loop body item must be an object").get("kind")
        if kind not in {"Break", "Continue"}:
            raise RustProjectionError("Rust loop witness only accepts control exits")
        children.append(_node(kind))
    return _node("LoopBody", children=children)


def _guard_expr(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_statement(program)
    if statement.get("kind") != "If":
        raise RustProjectionError("guard expression did not lower to If")
    condition = _require_dict(statement.get("condition"), "guard condition is missing")
    if condition.get("kind") != "UnaryOp" or condition.get("op") != "!":
        raise RustProjectionError("guard expression must invert its condition")
    then_body = statement.get("then")
    if not isinstance(then_body, list) or not then_body:
        raise RustProjectionError("guard else must not fall through")
    return _node(
        "GuardElse",
        children=[_node("Condition"), _node("NoFallthroughElse")],
    )


def _guard_let(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_statement(program)
    if statement.get("kind") != "ScopeBox":
        raise RustProjectionError("guard let did not produce its scoped binding shape")
    return _node(
        "GuardLetElse",
        children=[_node("Pattern"), _node("Expr"), _node("NoFallthroughElse")],
    )


def _try_shape(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    statement = _single_statement(program)
    if statement.get("kind") != "TryCatch":
        raise RustProjectionError("postfix/try witness requires TryCatch")
    catches = statement.get("catch")
    cleanup = statement.get("cleanup")
    has_catch = isinstance(catches, list) and bool(catches)
    has_cleanup = isinstance(cleanup, list) and bool(cleanup)
    if row_id == "fini":
        if not has_catch or statement.get("try") != []:
            raise RustProjectionError("fini marker shape is missing")
        return _node("Fini", children=[_node("CleanupBody")])
    if row_id == "postfix_catch" and has_catch and not has_cleanup:
        return _node("PostfixCatch", children=[_node("Body"), _node("CatchHandler")])
    if row_id == "postfix_cleanup" and has_cleanup and not has_catch:
        return _node("PostfixCleanup", children=[_node("Body"), _node("CleanupBody")])
    if row_id == "try_statement" and has_catch and has_cleanup:
        return _node(
            "PostfixCatchCleanup",
            children=[_node("Body"), _node("CatchHandler"), _node("CleanupBody")],
        )
    raise RustProjectionError("postfix/try shape disagrees with its grammar row")


def _match(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_statement(program)
    if statement.get("kind") not in {"EnumMatchExpr", "Match"}:
        raise RustProjectionError("match witness requires a match expression")
    arms = statement.get("arms")
    if not isinstance(arms, list):
        raise RustProjectionError("match arms are missing")
    normalized_arms = [
        _node("MatchArm", children=[_node("Pattern"), _node("ArmBody")])
        for _ in arms
    ]
    return _node(
        "Match",
        children=[_node("Scrutinee"), _node("OrderedArms", children=normalized_arms)],
    )


def _delegate(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_statement(program)
    delegates = statement.get("delegates")
    if statement.get("kind") != "BoxDeclaration" or not isinstance(delegates, list):
        raise RustProjectionError("delegate witness requires box declaration metadata")
    if len(delegates) != 1:
        raise RustProjectionError("delegate witness requires one declaration")
    return _node(
        "DelegateExposes",
        children=[_node("Field"), _node("ExposedMembers")],
    )


def _loop(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    statement = _single_statement(program)
    if row_id == "loop_range":
        if statement.get("kind") != "LoopRange":
            raise RustProjectionError("range loop shape is missing")
        return _node(
            "LoopRange",
            children=[
                _node("RangeIndex"),
                _node("RangeStart"),
                _node("RangeEnd"),
                _loop_body(statement),
            ],
        )
    if statement.get("kind") != "Loop":
        raise RustProjectionError("loop shape is missing")
    body = _loop_body(statement)
    if row_id in {"loop_infinite", "break_statement", "continue_statement"}:
        condition = _require_dict(statement.get("condition"), "loop condition is missing")
        if condition.get("type") != "Bool" or condition.get("value") is not True:
            raise RustProjectionError("infinite loop condition must be true")
        return _node("LoopInfinite", children=[body])
    return _node("LoopCondition", children=[_node("Condition"), body])


def _map_literal(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_statement(program)
    entries = statement.get("entries")
    if statement.get("kind") != "Map" or not isinstance(entries, list) or len(entries) != 1:
        raise RustProjectionError("map witness requires one canonical map entry")
    value = _require_dict(entries[0], "map entry must be an object").get("v")
    if _require_dict(value, "map value is missing").get("type") != "Int":
        raise RustProjectionError("map witness value must be integer fixture shape")
    return _node("MapLiteral", children=[_node("StringKey"), _node("IntegerLiteral")])


def project_rust_normalized_form(row_id: str, program: Any) -> dict[str, Any]:
    program = _require_dict(program, "Rust AST program must be an object")
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
    raise RustProjectionError(f"Rust grammar row projection is missing: {row_id}")

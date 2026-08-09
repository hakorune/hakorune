#!/usr/bin/env python3
"""Project Hako ProgramJSON into parser-neutral recursive grammar evidence."""

from __future__ import annotations

from typing import Any


class HakoProjectionError(ValueError):
    stable_reject_tag = "parser/witness_missing"

    def __init__(self, detail: str, stable_reject_tag: str | None = None) -> None:
        super().__init__(detail)
        if stable_reject_tag is not None:
            self.stable_reject_tag = stable_reject_tag


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


def _scalar_text(value: Any, detail: str) -> str:
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float, str)):
        return str(value)
    raise HakoProjectionError(detail)


def _expression(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_body(program)
    return _object(
        statement.get("expr") if statement.get("type") == "Expr" else statement,
        "Hako expression evidence is missing",
    )


def _expression_node(expression: Any) -> dict[str, Any]:
    expression = _object(expression, "Hako expression evidence must be an object")
    expression_type = expression.get("type")
    normalized_kind = {
        "Int": "IntegerLiteral",
        "Float": "FloatLiteral",
        "Str": "StringLiteral",
        "Bool": "BoolLiteral",
        "Null": "NullLiteral",
        "Void": "VoidLiteral",
    }.get(expression_type)
    if normalized_kind is not None:
        if expression_type in {"Null", "Void"}:
            return _node(normalized_kind)
        return _node(
            normalized_kind,
            value=_scalar_text(expression.get("value"), "Hako literal value is missing"),
        )
    if expression_type == "Var":
        name = expression.get("name")
        if not isinstance(name, str):
            raise HakoProjectionError("Hako variable name is missing")
        return _node("Variable", value=name)
    if expression_type == "RecordLiteral":
        record_type = expression.get("record")
        if not isinstance(record_type, str):
            raise HakoProjectionError("Hako record literal type is missing")
        return _node(
            "RecordLiteral",
            children=[
                _node("TypeRef", value=record_type),
                _record_field_list(expression.get("fields")),
            ],
        )
    if expression_type == "RecordUpdate":
        return _node(
            "RecordWithUpdate",
            children=[
                _expression_node(expression.get("base")),
                _record_field_list(expression.get("updates")),
            ],
        )
    if expression_type == "Call" and expression.get("name") == "array.of":
        arguments = expression.get("args")
        if not isinstance(arguments, list):
            raise HakoProjectionError("Hako array arguments are missing")
        return _node("ArrayLiteral", children=[_expression_node(item) for item in arguments])
    if expression_type == "New":
        box_name = expression.get("class")
        arguments = expression.get("args")
        if not isinstance(box_name, str) or not isinstance(arguments, list):
            raise HakoProjectionError("Hako new-box evidence is incomplete")
        return _node(
            "NewBoxExpression",
            children=[
                _node("TypeRef", value=box_name),
                _node("Arguments", children=[_expression_node(item) for item in arguments]),
            ],
        )
    raise HakoProjectionError(f"Hako expression projection is missing: {expression_type}")


def _record_field_list(value: Any) -> dict[str, Any]:
    if not isinstance(value, list):
        raise HakoProjectionError("Hako record field list is missing")
    fields = []
    for item in value:
        item = _object(item, "Hako record field must be an object")
        name = item.get("name")
        if not isinstance(name, str):
            raise HakoProjectionError("Hako record field name is missing")
        fields.append(
            _node(
                "RecordField",
                children=[
                    _node("Identifier", value=name),
                    _expression_node(item.get("value")),
                ],
            )
        )
    return _node("RecordFieldList", children=fields)


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
    then_body = statement.get("then")
    else_body = statement.get("else")
    if then_body != [] or not isinstance(else_body, list) or not else_body:
        raise HakoProjectionError("Hako guard expression branch shape is invalid")
    final_kind = _object(
        else_body[-1], "Hako guard else item must be an object"
    ).get("type")
    if final_kind not in {"Return", "Break", "Continue", "Fault"}:
        raise HakoProjectionError("Hako guard else may fall through")
    return _node(
        "GuardElse",
        children=[_node("Condition"), _node("NoFallthroughElse")],
    )


def _guard_let(program: dict[str, Any]) -> dict[str, Any]:
    body = program.get("body")
    if not isinstance(body, list) or len(body) != 3:
        raise HakoProjectionError("Hako guard let requires Local/If/Local")
    subject = _object(body[0], "Hako guard let subject is missing")
    guard = _object(body[1], "Hako guard let branch is missing")
    binding = _object(body[2], "Hako guard let binding is missing")
    if subject.get("type") != "Local" or guard.get("type") != "If" or binding.get("type") != "Local":
        raise HakoProjectionError("Hako guard let shape is invalid")
    condition = _object(guard.get("cond"), "Hako guard let condition is missing")
    binding_expr = _object(binding.get("expr"), "Hako guard let binding expression is missing")
    if condition.get("type") != "EnumMatch" or binding_expr.get("type") != "EnumMatch":
        raise HakoProjectionError("Hako guard let requires EnumMatch evidence")
    then_body = guard.get("then")
    if not isinstance(then_body, list) or not then_body:
        raise HakoProjectionError(
            "Hako guard let else may fall through",
            "parser/guard_let_no_fallthrough_required",
        )
    final_kind = _object(
        then_body[-1], "Hako guard let else item must be an object"
    ).get("type")
    if final_kind not in {"Return", "Break", "Continue", "Fault"}:
        raise HakoProjectionError(
            "Hako guard let else may fall through",
            "parser/guard_let_no_fallthrough_required",
        )
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


def _declaration_node(value: Any) -> dict[str, Any]:
    node = _object(value, "Hako declaration normalized node must be an object")
    if set(node) - {"kind", "value", "children"}:
        raise HakoProjectionError("Hako declaration node exposes internal fields")
    kind = node.get("kind")
    children = node.get("children")
    if not isinstance(kind, str) or not isinstance(children, list):
        raise HakoProjectionError("Hako declaration node is malformed")
    if "value" in node and not isinstance(node["value"], str):
        raise HakoProjectionError("Hako declaration node value must be a string")
    return _node(
        kind,
        value=node.get("value"),
        children=[_declaration_node(child) for child in children],
    )


def _declaration(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    if program.get("body") != []:
        raise HakoProjectionError("Hako declaration leaked into semantic body")
    sidecar = _object(
        program.get("parser_evidence"), "Hako declaration sidecar is missing"
    )
    for field in (
        "semantic_publication",
        "mir_lowering_allowed",
        "runtime_allowed",
        "backend_allowed",
    ):
        if sidecar.get(field) is not False:
            raise HakoProjectionError(f"Hako declaration sidecar permits {field}")
    declarations = sidecar.get("declarations")
    if not isinstance(declarations, list) or len(declarations) != 1:
        raise HakoProjectionError("Hako declaration sidecar requires one evidence row")
    evidence = _object(declarations[0], "Hako declaration evidence is malformed")
    if evidence.get("row_id") != row_id or evidence.get("accepted") is not True:
        raise HakoProjectionError("Hako declaration row identity drifted")
    for field in (
        "semantic_publication_allowed",
        "mir_lowering_allowed",
        "runtime_allowed",
        "backend_allowed",
    ):
        if evidence.get(field) is not False:
            raise HakoProjectionError(f"Hako declaration evidence permits {field}")
    return _declaration_node(evidence.get("normalized_form"))


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


def _remaining_expression(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    expression = _expression(program)
    if row_id == "weak_unary_expr":
        if expression.get("type") != "Unary" or expression.get("op") != "weak":
            raise HakoProjectionError("Hako weak unary evidence is missing")
        return _node("WeakExpr", children=[_expression_node(expression.get("operand"))])
    return _expression_node(expression)


def _release_statement(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_body(program)
    root = statement.get("root")
    if statement.get("type") != "Release" or not isinstance(root, str):
        raise HakoProjectionError(
            "Hako contextual release evidence is missing",
            "parser/release_contextual_not_selected",
        )
    return _node("ReleaseStatement", value=root)


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
    if row_id in {
        "delegate_exposes",
        "record_declaration",
        "weak_stored_field",
        "weak_visibility_field",
        "weak_legacy_init_field",
    }:
        return _declaration(program, row_id)
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
    if row_id == "release_statement":
        return _release_statement(program)
    if row_id in {
        "weak_unary_expr",
        "record_literal",
        "record_with_update",
        "literal_integer",
        "literal_float",
        "literal_string",
        "literal_bool",
        "literal_null",
        "literal_void",
        "array_literal",
        "construction_new_box",
    }:
        return _remaining_expression(program, row_id)
    raise HakoProjectionError(f"Hako grammar row projection is missing: {row_id}")

#!/usr/bin/env python3
"""Project Rust AST JSON into parser-neutral recursive grammar evidence."""

from __future__ import annotations

from typing import Any


class RustProjectionError(ValueError):
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


def _scalar_text(value: Any, detail: str) -> str:
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float, str)):
        return str(value)
    raise RustProjectionError(detail)


def _expression_node(expression: Any) -> dict[str, Any]:
    expression = _require_dict(expression, "Rust expression evidence must be an object")
    kind = expression.get("kind")
    if kind == "Literal":
        literal_type = expression.get("type")
        normalized_kind = {
            "Int": "IntegerLiteral",
            "Float": "FloatLiteral",
            "String": "StringLiteral",
            "Bool": "BoolLiteral",
            "Null": "NullLiteral",
            "Void": "VoidLiteral",
        }.get(literal_type)
        if normalized_kind is None:
            raise RustProjectionError("Rust literal kind is unsupported")
        if literal_type in {"Null", "Void"}:
            return _node(normalized_kind)
        return _node(
            normalized_kind,
            value=_scalar_text(expression.get("value"), "Rust literal value is missing"),
        )
    if kind == "Variable":
        name = expression.get("name")
        if not isinstance(name, str):
            raise RustProjectionError("Rust variable name is missing")
        return _node("Variable", value=name)
    if kind == "RecordLiteral":
        record_type = expression.get("record_type")
        if not isinstance(record_type, str):
            raise RustProjectionError("Rust record literal type is missing")
        return _node(
            "RecordLiteral",
            children=[
                _node("TypeRef", value=record_type),
                _record_field_list(expression.get("fields")),
            ],
        )
    if kind == "RecordUpdate":
        return _node(
            "RecordWithUpdate",
            children=[
                _expression_node(expression.get("base")),
                _record_field_list(expression.get("updates")),
            ],
        )
    if kind == "Array":
        elements = expression.get("elements")
        if not isinstance(elements, list):
            raise RustProjectionError("Rust array elements are missing")
        return _node("ArrayLiteral", children=[_expression_node(item) for item in elements])
    if kind == "New":
        box_name = expression.get("box_name")
        arguments = expression.get("args")
        if not isinstance(box_name, str) or not isinstance(arguments, list):
            raise RustProjectionError("Rust new-box evidence is incomplete")
        return _node(
            "NewBoxExpression",
            children=[
                _node("TypeRef", value=box_name),
                _node("Arguments", children=[_expression_node(item) for item in arguments]),
            ],
        )
    raise RustProjectionError(f"Rust expression projection is missing: {kind}")


def _record_field_list(value: Any) -> dict[str, Any]:
    if not isinstance(value, list):
        raise RustProjectionError("Rust record field list is missing")
    fields = []
    for item in value:
        item = _require_dict(item, "Rust record field must be an object")
        name = item.get("name")
        if not isinstance(name, str):
            raise RustProjectionError("Rust record field name is missing")
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
    body = statement.get("body")
    if not isinstance(body, list) or len(body) != 3:
        raise RustProjectionError("guard let scope requires Local/If/Local")
    guard = _require_dict(body[1], "guard let branch is missing")
    then_body = guard.get("then")
    if guard.get("kind") != "If" or not isinstance(then_body, list) or not then_body:
        raise RustProjectionError(
            "guard let else may fall through",
            "parser/guard_let_no_fallthrough_required",
        )
    final_kind = _require_dict(
        then_body[-1], "guard let else item must be an object"
    ).get("kind")
    if final_kind not in {"Return", "Break", "Continue", "Fault"}:
        raise RustProjectionError(
            "guard let else may fall through",
            "parser/guard_let_no_fallthrough_required",
        )
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


def _record_declaration(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_statement(program)
    if statement.get("kind") != "BoxDeclaration" or statement.get("is_record") is not True:
        raise RustProjectionError("record declaration evidence is missing")
    name = statement.get("name")
    fields = statement.get("field_decls")
    if not isinstance(name, str) or not isinstance(fields, list) or not fields:
        raise RustProjectionError("record declaration metadata is incomplete")
    normalized_fields = []
    for field in fields:
        field = _require_dict(field, "record field metadata must be an object")
        field_name = field.get("name")
        type_name = field.get("declared_type")
        if not isinstance(field_name, str) or not isinstance(type_name, str):
            raise RustProjectionError("record field requires identifier and type")
        normalized_fields.append(
            _node(
                "RecordField",
                children=[
                    _node("Identifier", value=field_name),
                    _node("TypeRef", value=type_name),
                ],
            )
        )
    return _node(
        "RecordDeclaration",
        children=[
            _node("Identifier", value=name),
            _node("RecordFieldList", children=normalized_fields),
        ],
    )


def _brand_declaration(program: dict[str, Any]) -> dict[str, Any]:
    statements = program.get("statements")
    if not isinstance(statements, list):
        raise RustProjectionError("Brand declaration statements are missing")
    declarations = [
        statement
        for statement in statements
        if isinstance(statement, dict) and statement.get("kind") == "BrandDeclaration"
    ]
    if len(declarations) != 1:
        raise RustProjectionError("Brand declaration witness requires one declaration")
    declaration = declarations[0]
    name = declaration.get("name")
    underlying_type = declaration.get("underlying_type")
    if not isinstance(name, str) or not isinstance(underlying_type, str):
        raise RustProjectionError("Brand declaration metadata is incomplete")
    return _node(
        "BrandDeclaration",
        children=[
            _node("Identifier", value=name),
            _node("TypeRef", value=underlying_type),
        ],
    )


def _weak_field(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    statement = _single_statement(program)
    if statement.get("kind") != "BoxDeclaration":
        raise RustProjectionError("weak field requires box declaration evidence")
    box_name = statement.get("name")
    weak_fields = statement.get("weak_fields")
    if not isinstance(box_name, str) or not isinstance(weak_fields, list) or len(weak_fields) != 1:
        raise RustProjectionError("weak field metadata is incomplete")
    field_name = weak_fields[0]
    if not isinstance(field_name, str):
        raise RustProjectionError("weak field name is missing")

    init_fields = statement.get("init_fields")
    if row_id == "weak_legacy_init_field":
        if not isinstance(init_fields, list) or field_name not in init_fields:
            raise RustProjectionError("legacy init weak field evidence is missing")
    elif isinstance(init_fields, list) and field_name in init_fields:
        raise RustProjectionError("direct weak field unexpectedly used init syntax")

    public_fields = statement.get("public_fields")
    private_fields = statement.get("private_fields")
    visibility = "Default"
    if isinstance(public_fields, list) and field_name in public_fields:
        visibility = "Public"
    elif isinstance(private_fields, list) and field_name in private_fields:
        visibility = "Private"
    if row_id == "weak_visibility_field" and visibility == "Default":
        raise RustProjectionError("weak visibility evidence is missing")
    if row_id != "weak_visibility_field" and visibility != "Default":
        raise RustProjectionError("unexpected weak visibility evidence")

    children = [_node("Identifier", value=field_name)]
    field_decls = statement.get("field_decls")
    if isinstance(field_decls, list):
        declaration = next(
            (
                field
                for field in field_decls
                if isinstance(field, dict) and field.get("name") == field_name
            ),
            None,
        )
        if isinstance(declaration, dict) and isinstance(declaration.get("declared_type"), str):
            children.append(_node("TypeRef", value=declaration["declared_type"]))
    children.append(_node("Visibility", value=visibility))
    return _node(
        "WeakStoredField",
        children=[
            _node("BoxName", value=box_name),
            _node("WeakField", children=children),
        ],
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


def _remaining_expression(program: dict[str, Any], row_id: str) -> dict[str, Any]:
    expression = _single_statement(program)
    if row_id == "weak_unary_expr":
        if expression.get("kind") != "UnaryOp" or expression.get("op") != "weak":
            raise RustProjectionError("Rust weak unary evidence is missing")
        return _node("WeakExpr", children=[_expression_node(expression.get("operand"))])
    return _expression_node(expression)


def _release_statement(program: dict[str, Any]) -> dict[str, Any]:
    statement = _single_statement(program)
    root = statement.get("root")
    if statement.get("kind") != "Release" or not isinstance(root, str):
        raise RustProjectionError(
            "Rust contextual release evidence is missing",
            "parser/release_contextual_not_selected",
        )
    return _node("ReleaseStatement", value=root)


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
    if row_id == "record_declaration":
        return _record_declaration(program)
    if row_id == "brand_declaration":
        return _brand_declaration(program)
    if row_id in {
        "weak_stored_field",
        "weak_visibility_field",
        "weak_legacy_init_field",
    }:
        return _weak_field(program, row_id)
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
    raise RustProjectionError(f"Rust grammar row projection is missing: {row_id}")

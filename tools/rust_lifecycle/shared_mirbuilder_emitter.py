#!/usr/bin/env python3
"""Shared MirBuilder Hako emitter for verified family IR.

This emitter is intentionally dumb: it only renders a verified family spec into
deterministic `.hako` source. It does not inspect family names, rust syntax, or
route-selection state.
"""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from typing import Any

from shared_mirbuilder_emitter_common import (
    indent,
    render_call_args,
    render_field_initializer,
    render_initializer,
    render_main_value,
    render_string_literal,
)
from shared_mirbuilder_operation_emitter import render_method_body


def render_main_operation(operation: Mapping[str, Any]) -> list[str]:
    kind = operation["kind"]
    if kind == "NewBox":
        target = operation.get("target")
        box_name = operation.get("box")
        if target is None or box_name is None:
            raise ValueError("NewBox requires target and box")
        return [f"local {target} = new {box_name}()"]
    if kind == "NewArray":
        target = operation.get("target")
        if target is None:
            raise ValueError("NewArray requires target")
        return [f"local {target} = new ArrayBox()"]
    if kind == "StaticCall":
        target = operation.get("target")
        callee = operation.get("callee")
        if callee is None:
            raise ValueError("StaticCall requires callee")
        args = render_call_args(operation.get("args"))
        call = f"{callee}({args})" if args else f"{callee}()"
        if target is None:
            return [call]
        return [f"local {target} = {call}"]
    if kind == "MethodCall":
        target = operation.get("target")
        receiver = operation.get("receiver")
        method = operation.get("method")
        if receiver is None or method is None:
            raise ValueError("MethodCall requires receiver and method")
        args = render_call_args(operation.get("args"))
        call = f"{receiver}.{method}({args})" if args else f"{receiver}.{method}()"
        if target is None:
            return [call]
        return [f"local {target} = {call}"]
    if kind == "ArrayPush":
        target = operation.get("target")
        value = operation.get("value")
        if target is None or "value" not in operation:
            raise ValueError("ArrayPush requires target and value")
        return [f"{target}.push({render_main_value(value)})"]
    if kind == "MapReadFoldOwnedCopy":
        source = operation.get("source")
        destination = operation.get("destination")
        if not isinstance(source, str) or not isinstance(destination, str):
            raise ValueError("MapReadFoldOwnedCopy requires source and destination")
        return [
            f"local keys = {source}.keys()",
            "local i = 0",
            "loop(i < keys.length()) {",
            "    local key = BoxHelpers.array_get(keys, i)",
            f"    {destination}.set(key, {source}.get(key))",
            "    i = i + 1",
            "}",
        ]
    if kind == "AssertEq":
        left = operation.get("left")
        right = operation.get("right")
        fail_message = operation.get("fail_message")
        fail_code = operation.get("fail_code", 1)
        if left is None or "right" not in operation or fail_message is None:
            raise ValueError("AssertEq requires left, right, and fail_message")
        return [
            f"if {left} != {render_main_value(right)} {{",
            f"    print({render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
        ]
    if kind == "AssertArrayValueEq":
        array = operation.get("array")
        index = operation.get("index")
        expected = operation.get("expected")
        fail_message = operation.get("fail_message")
        fail_code = operation.get("fail_code", 1)
        if array is None or index is None or "expected" not in operation or fail_message is None:
            raise ValueError("AssertArrayValueEq requires array, index, expected, and fail_message")
        return [
            f"if BoxHelpers.array_get({array}, {index}) != {render_main_value(expected)} {{",
            f"    print({render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
        ]
    if kind == "AssertOptionSomeStringEq":
        source = operation.get("source")
        expected = operation.get("expected")
        fail_message = operation.get("fail_message")
        fail_code = operation.get("fail_code", 1)
        if not isinstance(source, str) or "expected" not in operation or fail_message is None:
            raise ValueError("AssertOptionSomeStringEq requires source, expected, and fail_message")
        value_name = operation.get("value_name", f"{source}_some_value")
        if not isinstance(value_name, str):
            raise ValueError("AssertOptionSomeStringEq value_name must be a string")
        return [
            f"guard let Option::Some({value_name}) = {source} else {{",
            f"    print({render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
            f"if {value_name} != {render_main_value(expected)} {{",
            f"    print({render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
        ]
    if kind == "AssertOptionSomeI64Eq":
        source = operation.get("source")
        expected = operation.get("expected")
        fail_message = operation.get("fail_message")
        fail_code = operation.get("fail_code", 1)
        if not isinstance(source, str) or "expected" not in operation or fail_message is None:
            raise ValueError("AssertOptionSomeI64Eq requires source, expected, and fail_message")
        value_name = operation.get("value_name", f"{source}_some_value")
        if not isinstance(value_name, str):
            raise ValueError("AssertOptionSomeI64Eq value_name must be a string")
        return [
            f"guard let Option::Some({value_name}) = {source} else {{",
            f"    print({render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
            f"if {value_name} != {render_main_value(expected)} {{",
            f"    print({render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
        ]
    if kind == "AssertOwnedProductSequence":
        array = operation.get("array")
        expected = operation.get("expected")
        if not isinstance(array, str) or not isinstance(expected, list):
            raise ValueError("AssertOwnedProductSequence requires array and expected list")
        lines: list[str] = []
        for index, item in enumerate(expected):
            if not isinstance(item, Mapping):
                raise ValueError("AssertOwnedProductSequence expected entries must be objects")
            checks = item.get("checks")
            if not isinstance(checks, list):
                raise ValueError("AssertOwnedProductSequence expected entry requires checks")
            slot_name = f"slot_{index}"
            lines.append(f"local {slot_name} = {array}.get({index})")
            for check in checks:
                if not isinstance(check, Mapping):
                    raise ValueError("AssertOwnedProductSequence checks must be objects")
                field = check.get("field")
                expected_value = check.get("expected")
                fail_message = check.get("fail_message")
                fail_code = check.get("fail_code", 1)
                if not isinstance(field, str) or "expected" not in check or not isinstance(fail_message, str):
                    raise ValueError("AssertOwnedProductSequence check requires field, expected, and fail_message")
                lines.extend(
                    [
                        f"if {slot_name}.{field} != {render_main_value(expected_value)} {{",
                        f"    print({render_string_literal(fail_message)})",
                        f"    return {fail_code}",
                        "}",
                    ]
                )
        return lines
    if kind == "Print":
        text = operation.get("text")
        if text is None:
            raise ValueError("Print requires text")
        return [f"print({render_string_literal(text)})"]
    if kind == "ReturnI64":
        return [f"return {operation['return_value']}"]
    raise ValueError(f"unsupported Main operation: {kind}")


def _render_main_body(main: Mapping[str, Any]) -> list[str]:
    if "operations" not in main:
        raise ValueError("main has no operations")
    lines: list[str] = []
    for operation in main["operations"]:
        lines.extend(render_main_operation(operation))
    return lines


def _render_static_box(name: str, methods: Sequence[Mapping[str, Any]], trailing_blank_line: bool) -> list[str]:
    lines = [f"static box {name} {{"]
    for index, method in enumerate(methods):
        lines.append(f"    {method['signature']} {{")
        lines.extend(indent(render_method_body(method), 8))
        lines.append("    }")
        if index != len(methods) - 1 or trailing_blank_line:
            lines.append("")
    lines.append("}")
    return lines


def _render_box(box: Mapping[str, Any]) -> list[str]:
    box_fields = box.get("fields")
    if box_fields is not None:
        lines = [f"box {box['name']} {{"]
        if not box_fields:
            lines.append("}")
            return lines
        for field in box_fields:
            lines.append(f"    {field['name']}: {field['field_type']}")
        lines.append("")
        lines.append("    birth() {")
        for field in box_fields:
            lines.append(f"        me.{field['name']} = {render_field_initializer(field)}")
        lines.append("    }")
        for method in box.get("methods", []):
            lines.append("")
            lines.append(f"    {method['signature']} {{")
            lines.extend(indent(render_method_body(method), 8))
            lines.append("    }")
        lines.append("}")
        return lines

    field_name = box["field_name"]
    return [
        f"box {box['name']} {{",
        f"    {field_name}: {box['field_type']}",
        "",
        "    birth() {",
        f"        me.{field_name} = {render_initializer(box)}",
        "    }",
        "}",
    ]


def emit_verified_family_hako(verified_ir: Mapping[str, Any]) -> str:
    lines: list[str] = [
        f"// @generated by {verified_ir['generated_by']}",
        f"// artifact-manifest: {verified_ir['artifact_manifest']}",
        f"// family: {verified_ir['family_comment']}",
    ]
    pilot_scope = verified_ir.get("pilot_scope")
    if pilot_scope:
        lines.append(f"// pilot-scope: {pilot_scope}")
    lines.extend(["// manual-edit: forbidden", ""])

    using_module = verified_ir["using_module"]
    if using_module:
        lines.extend([f"using {using_module} as OrderedMap", ""])
    for extra_using in verified_ir.get("extra_using_modules", []):
        lines.append(f"using {extra_using}")
    if verified_ir.get("extra_using_modules"):
        lines.append("")
    lines.extend(["using selfhost.shared.common.box_helpers as BoxHelpers", ""])

    for enum_decl in verified_ir.get("enum_declarations", []):
        lines.append(f"enum {enum_decl['name']} {{")
        for variant in enum_decl["variants"]:
            if isinstance(variant, str):
                lines.append(f"    {variant}")
            else:
                payload = variant.get("payload")
                if payload is None:
                    lines.append(f"    {variant['name']}")
                else:
                    lines.append(f"    {variant['name']}({payload})")
        lines.append("}")
        lines.append("")

    boxes = [verified_ir["box"], *verified_ir.get("additional_boxes", [])]
    for box_index, box in enumerate(boxes):
        lines.extend(_render_box(box))
        lines.append("")
        if box_index != len(boxes) - 1:
            lines.append("")

    static_boxes = verified_ir.get("static_boxes")
    if static_boxes is None:
        api = verified_ir["api"]
        static_boxes = [
            {
                "name": api["name"],
                "methods": api["methods"],
                "trailing_blank_line": bool(api.get("trailing_blank_line", False)),
            }
        ]

    for index, static_box in enumerate(static_boxes):
        lines.extend(
            _render_static_box(
                static_box["name"],
                static_box["methods"],
                bool(static_box.get("trailing_blank_line", False)),
            )
        )
        if index != len(static_boxes) - 1:
            lines.append("")
    lines.append("")

    main = verified_ir["main"]
    lines.extend(
        [
            "static box Main {",
            "    main() {",
            *indent(_render_main_body(main), 8),
            "    }",
            "}",
            "",
        ]
    )
    return "\n".join(lines)

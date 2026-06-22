#!/usr/bin/env python3
"""Shared MirBuilder Hako emitter for verified family IR.

This emitter is intentionally dumb: it only renders a verified family spec into
deterministic `.hako` source. It does not inspect family names, rust syntax, or
route-selection state.
"""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from typing import Any


def _indent(lines: Sequence[str], spaces: int) -> list[str]:
    prefix = " " * spaces
    return [prefix + line if line else "" for line in lines]


def _render_initializer(box: Mapping[str, Any]) -> str:
    operation = box.get("initializer_operation")
    if operation is None:
        return str(box["initializer"])
    if operation.get("kind") == "NewOrderedMap":
        return "OrderedMap.create()"
    raise ValueError(f"unsupported initializer operation: {operation.get('kind')}")


def _render_field_initializer(field: Mapping[str, Any]) -> str:
    operation = field.get("initializer_operation")
    if operation is None:
        return str(field["initializer"])
    if operation.get("kind") == "NewOrderedMap":
        return "OrderedMap.create()"
    raise ValueError(f"unsupported initializer operation: {operation.get('kind')}")


def _render_source_expr(operation: Mapping[str, Any]) -> str:
    source = operation.get("source")
    if source is not None:
        return str(source)
    field = operation.get("field")
    if field is not None:
        return f"ctx.{field}"
    raise ValueError(f"operation is missing source/field: {operation.get('kind')}")


def _render_call_args(args: Any) -> str:
    if args is None:
        return ""
    if isinstance(args, str):
        return args
    if isinstance(args, Sequence):
        return ", ".join(_render_main_value(arg) for arg in args)
    return str(args)


def _render_string_literal(value: Any) -> str:
    text = str(value)
    escaped = text.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def _render_main_value(value: Any) -> str:
    if isinstance(value, Mapping):
        if "literal" in value:
            return _render_string_literal(value["literal"])
        if "expr" in value:
            return str(value["expr"])
        raise ValueError(f"unsupported Main value object: {value}")
    if value is None:
        return "null"
    return str(value)


def _render_main_operation(operation: Mapping[str, Any]) -> list[str]:
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
        args = _render_call_args(operation.get("args"))
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
        args = _render_call_args(operation.get("args"))
        call = f"{receiver}.{method}({args})" if args else f"{receiver}.{method}()"
        if target is None:
            return [call]
        return [f"local {target} = {call}"]
    if kind == "ArrayPush":
        target = operation.get("target")
        value = operation.get("value")
        if target is None or "value" not in operation:
            raise ValueError("ArrayPush requires target and value")
        return [f"{target}.push({_render_main_value(value)})"]
    if kind == "AssertEq":
        left = operation.get("left")
        right = operation.get("right")
        fail_message = operation.get("fail_message")
        fail_code = operation.get("fail_code", 1)
        if left is None or "right" not in operation or fail_message is None:
            raise ValueError("AssertEq requires left, right, and fail_message")
        return [
            f"if {left} != {_render_main_value(right)} {{",
            f"    print({_render_string_literal(fail_message)})",
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
            f"if BoxHelpers.array_get({array}, {index}) != {_render_main_value(expected)} {{",
            f"    print({_render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
        ]
    if kind == "Print":
        text = operation.get("text")
        if text is None:
            raise ValueError("Print requires text")
        return [f"print({_render_string_literal(text)})"]
    if kind == "ReturnI64":
        return [f"return {operation['return_value']}"]
    raise ValueError(f"unsupported Main operation: {kind}")


def _render_operation(operation: Mapping[str, Any]) -> list[str]:
    kind = operation["kind"]
    if kind == "MapGet":
        source = _render_source_expr(operation)
        return [
            f"local keys = {source}.keys_value",
            f"local values = {source}.values_value",
            f"if {operation['key']} == null {{",
            "    return null",
            "}",
            f"local key = {operation['key']}",
            "local i = 0",
            f"loop(i < keys.length()) {{",
            "    if BoxHelpers.array_get(keys, i) == key {",
            "        return BoxHelpers.array_get(values, i)",
            "    }",
            "    i = i + 1",
            "}",
            "return null",
        ]
    if kind == "MapHas":
        source = _render_source_expr(operation)
        return [
            f"local keys = {source}.keys_value",
            f"if {operation['key']} == null {{",
            "    return 0",
            "}",
            f"local key = {operation['key']}",
            "local i = 0",
            f"loop(i < keys.length()) {{",
            "    if BoxHelpers.array_get(keys, i) == key {",
            "        return 1",
            "    }",
            "    i = i + 1",
            "}",
            "return 0",
        ]
    if kind == "MapLength":
        source = _render_source_expr(operation)
        return [
            f"local keys = {source}.keys_value",
            "return keys.length()",
        ]
    if kind == "MapIsEmpty":
        source = _render_source_expr(operation)
        return [
            f"local keys = {source}.keys_value",
            "if keys.length() == 0 {",
            "    return 1",
            "}",
            "return 0",
        ]
    if kind == "MapSet":
        source = _render_source_expr(operation)
        return [
            f"if {operation['key']} == null {{",
            "    return 0",
            "}",
            f"local key = {operation['key']}",
            "local i = 0",
            f"loop(i < {source}.keys_value.length()) {{",
            f"    if BoxHelpers.array_get({source}.keys_value, i) == key {{",
            f"        {source}.values_value.set(i, {operation['value']})",
            "        return 1",
            "    }",
            "    i = i + 1",
            "}",
            f"{source}.keys_value.push(key)",
            f"{source}.values_value.push({operation['value']})",
            f"local pos = {source}.keys_value.length() - 1",
            "loop(pos > 0) {",
            "    local prev = pos - 1",
            f"    local a = BoxHelpers.array_get({source}.keys_value, prev)",
            f"    local b = BoxHelpers.array_get({source}.keys_value, pos)",
            "    if a < b or a == b {",
            "        return 1",
            "    }",
            f"    local av = BoxHelpers.array_get({source}.values_value, prev)",
            f"    local bv = BoxHelpers.array_get({source}.values_value, pos)",
            f"    {source}.keys_value.set(prev, b)",
            f"    {source}.keys_value.set(pos, a)",
            f"    {source}.values_value.set(prev, bv)",
            f"    {source}.values_value.set(pos, av)",
            "    pos = pos - 1",
            "}",
            "return 1",
        ]
    if kind == "MapRemove":
        source = _render_source_expr(operation)
        return [
            f"return {source}.remove({operation['key']})",
        ]
    if kind == "MapClear":
        source = _render_source_expr(operation)
        return [
            f"{source}.keys_value = new ArrayBox()",
            f"{source}.values_value = new ArrayBox()",
            "return 1",
        ]
    if kind == "CloneOwnedMap":
        source = _render_source_expr(operation)
        return [f"return {source}.clone_owned()"]
    if kind == "ReplaceOwnedMap":
        source = _render_source_expr(operation)
        return [f"{source} = {operation['value']}.clone_owned()"]
    if kind == "ReturnSource":
        return [f"return {_render_source_expr(operation)}"]
    if kind == "CarrierSnapshotFromOwnedMap":
        map_arg = operation["map_arg"]
        loop_var = operation["loop_var"]
        carrier_names = operation["carrier_names_arg"]
        carrier_host_ids = operation["carrier_host_ids_arg"]
        return [
            f"local snapshot_total = {map_arg}.keys_value.length()",
            "local loop_var_id = null",
            "local carrier_count = 0",
            "local i = 0",
            f"loop(i < snapshot_total) {{",
            f"    local key = BoxHelpers.array_get({map_arg}.keys_value, i)",
            f"    local value = BoxHelpers.array_get({map_arg}.values_value, i)",
            f"    if key == {loop_var} {{",
            "        loop_var_id = value",
            "    } else {",
            f"        {carrier_names}.push(key)",
            f"        {carrier_host_ids}.push(value)",
            "        carrier_count = carrier_count + 1",
            "    }",
            "    i = i + 1",
            "}",
            "",
            "return 0",
        ]
    if kind == "ExplicitCarrierSnapshotFromOwnedMap":
        map_arg = operation["map_arg"]
        loop_var_id = operation["loop_var_id"]
        requested_names = operation["requested_names"]
        carrier_names = operation["carrier_names_arg"]
        carrier_host_ids = operation["carrier_host_ids_arg"]
        return [
            f"local snapshot_total = {map_arg}.keys_value.length()",
            "local loop_var_name = null",
            "local requested_name_copy = new ArrayBox()",
            "local requested_name_total = " + f"{requested_names}.length()",
            "local requested_index = 0",
            f"loop(requested_index < requested_name_total) {{",
            f"    local requested_name = BoxHelpers.array_get({requested_names}, requested_index)",
            "    requested_name_copy.push(requested_name)",
            "    requested_index = requested_index + 1",
            "}",
            "",
            "local carrier_count = 0",
            "local i = 0",
            f"loop(i < snapshot_total) {{",
            f"    local key = BoxHelpers.array_get({map_arg}.keys_value, i)",
            f"    local value = BoxHelpers.array_get({map_arg}.values_value, i)",
            f"    if value == {loop_var_id} {{",
            "        loop_var_name = key",
            "    } else {",
                "        local requested_match = 0",
                "        local name_index = 0",
                "        loop(name_index < requested_name_total) {",
            "            if BoxHelpers.array_get(requested_name_copy, name_index) == key {",
                    "                requested_match = 1",
                    "            }",
                    "            name_index = name_index + 1",
                "        }",
            "        if requested_match != 0 {",
            f"            {carrier_names}.push(key)",
            f"            {carrier_host_ids}.push(value)",
            "            carrier_count = carrier_count + 1",
            "        }",
            "    }",
            "    i = i + 1",
            "}",
            "",
            "if carrier_count != requested_name_total {",
            "    print(\"explicit_carrier_snapshot_missing_requested_carrier=fail\")",
            "    return 1",
            "}",
            "",
            "return 0",
        ]
    if kind == "ReturnI64":
        return [f"return {operation['return_value']}"]
    if kind == "AllFieldsMapIsEmpty":
        source = _render_source_expr(operation)
        fields = operation.get("fields", [])
        if not fields:
            return ["return 1"]
        checks = [f"{source}.{field}.is_empty()" for field in fields]
        return [
            f"if {' && '.join(checks)} {{",
            "    return 1",
            "}",
            "return 0",
        ]
    raise ValueError(f"unsupported Hako operation: {kind}")


def _render_method_body(method: Mapping[str, Any]) -> list[str]:
    if "operations" not in method:
        raise ValueError(f"method has no operations: {method['signature']}")
    lines: list[str] = []
    for operation in method["operations"]:
        lines.extend(_render_operation(operation))
    return lines


def _render_main_body(main: Mapping[str, Any]) -> list[str]:
    if "operations" not in main:
        raise ValueError("main has no operations")
    lines: list[str] = []
    for operation in main["operations"]:
        lines.extend(_render_main_operation(operation))
    return lines


def _render_static_box(name: str, methods: Sequence[Mapping[str, Any]], trailing_blank_line: bool) -> list[str]:
    lines = [f"static box {name} {{"]
    for index, method in enumerate(methods):
        lines.append(f"    {method['signature']} {{")
        lines.extend(_indent(_render_method_body(method), 8))
        lines.append("    }")
        if index != len(methods) - 1 or trailing_blank_line:
            lines.append("")
    lines.append("}")
    return lines


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
    lines.extend([f"using {using_module} as OrderedMap", ""])
    lines.extend(["using selfhost.shared.common.box_helpers as BoxHelpers", ""])

    box = verified_ir["box"]
    box_fields = box.get("fields")
    if box_fields is not None:
        lines.append(f"box {box['name']} {{")
        for field in box_fields:
            lines.append(f"    {field['name']}: {field['field_type']}")
        lines.append("")
        lines.append("    birth() {")
        for field in box_fields:
            lines.append(f"        me.{field['name']} = {_render_field_initializer(field)}")
        lines.append("    }")
        lines.append("}")
        lines.append("")
    else:
        field_name = box["field_name"]
        lines.extend(
            [
                f"box {box['name']} {{",
                f"    {field_name}: {box['field_type']}",
                "",
                "    birth() {",
                f"        me.{field_name} = {_render_initializer(box)}",
                "    }",
                "}",
                "",
            ]
        )

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
            *_indent(_render_main_body(main), 8),
            "    }",
            "}",
            "",
        ]
    )
    return "\n".join(lines)

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


def _render_source_expr(operation: Mapping[str, Any]) -> str:
    source = operation.get("source")
    if source is not None:
        return str(source)
    field = operation.get("field")
    if field is not None:
        return f"ctx.{field}"
    raise ValueError(f"operation is missing source/field: {operation.get('kind')}")


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
            "    if keys.get(i) == key {",
            "        return values.get(i)",
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
            "    if keys.get(i) == key {",
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
            f"    if {source}.keys_value.get(i) == key {{",
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
            f"    local a = {source}.keys_value.get(prev)",
            f"    local b = {source}.keys_value.get(pos)",
            "    if a < b or a == b {",
            "        return 1",
            "    }",
            f"    local av = {source}.values_value.get(prev)",
            f"    local bv = {source}.values_value.get(pos)",
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
        output = operation["output_arg"]
        return [
            f"local snapshot_total = {map_arg}.keys_value.length()",
            "local carrier_names = new ArrayBox()",
            "local carrier_host_ids = new ArrayBox()",
            "local loop_var_id = null",
            "local carrier_count = 0",
            "local i = 0",
            f"loop(i < snapshot_total) {{",
            f"    local key = {map_arg}.keys_value.get(i)",
            f"    local value = {map_arg}.values_value.get(i)",
            f"    if key == {loop_var} {{",
            "        loop_var_id = value",
            "    } else {",
            "        carrier_names.push(key)",
            "        carrier_host_ids.push(value)",
            "        carrier_count = carrier_count + 1",
            "    }",
            "    i = i + 1",
            "}",
            "",
            f"{output}.set(\"loop_var_name\", {loop_var})",
            f"{output}.set(\"loop_var_id\", loop_var_id)",
            f"{output}.set(\"carrier_names\", carrier_names)",
            f"{output}.set(\"carrier_host_ids\", carrier_host_ids)",
            f"{output}.set(\"carrier_count\", carrier_count)",
            "return 0",
        ]
    if kind == "ExplicitCarrierSnapshotFromOwnedMap":
        map_arg = operation["map_arg"]
        loop_var = operation["loop_var"]
        loop_var_id = operation["loop_var_id"]
        requested_names = operation["requested_names"]
        output = operation["output_arg"]
        return [
            f"local snapshot_total = {map_arg}.keys_value.length()",
            "local requested_name_copy = new ArrayBox()",
            "local requested_name_total = " + f"{requested_names}.length()",
            "local requested_index = 0",
            f"loop(requested_index < requested_name_total) {{",
            f"    local requested_name = {requested_names}.get(requested_index)",
            "    requested_name_copy.push(requested_name)",
            "    requested_index = requested_index + 1",
            "}",
            "",
            f"{output}.set(\"loop_var_name\", {loop_var})",
            f"{output}.set(\"loop_var_id\", {loop_var_id})",
            f"{output}.set(\"requested_names\", requested_name_copy)",
            "",
            "local carrier_names = new ArrayBox()",
            "local carrier_host_ids = new ArrayBox()",
            "local carrier_count = 0",
            "local i = 0",
            f"loop(i < snapshot_total) {{",
            f"    local key = {map_arg}.keys_value.get(i)",
            f"    local value = {map_arg}.values_value.get(i)",
            f"    if key != {loop_var} {{",
            "        local requested_match = 0",
            "        local name_index = 0",
            "        loop(name_index < requested_name_total) {",
            "            if requested_name_copy.get(name_index) == key {",
            "                requested_match = 1",
            "            }",
            "            name_index = name_index + 1",
            "        }",
            "        if requested_match != 0 {",
            "            carrier_names.push(key)",
            "            carrier_host_ids.push(value)",
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
            f"{output}.set(\"carrier_count\", carrier_count)",
            f"{output}.set(\"carrier_names\", carrier_names)",
            f"{output}.set(\"carrier_host_ids\", carrier_host_ids)",
            f"{output}.set(\"requested_name_count\", requested_name_total)",
            "return 0",
        ]
    if kind == "ReturnI64":
        return [f"return {operation['return_value']}"]
    raise ValueError(f"unsupported Hako operation: {kind}")


def _render_method_body(method: Mapping[str, Any]) -> list[str]:
    if "operations" not in method:
        raise ValueError(f"method has no operations: {method['signature']}")
    lines: list[str] = []
    for operation in method["operations"]:
        lines.extend(_render_operation(operation))
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

    box = verified_ir["box"]
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
            *_indent(main["lines"], 8),
            "    }",
            "}",
            "",
        ]
    )
    return "\n".join(lines)

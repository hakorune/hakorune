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


def _render_operation(operation: Mapping[str, Any]) -> list[str]:
    kind = operation["kind"]
    field = operation.get("field")
    if kind == "MapGet":
        return [f"return ctx.{field}.get({operation['key']})"]
    if kind == "MapHas":
        return [
            f"if ctx.{field}.has({operation['key']}) == true {{",
            "    return 1",
            "}",
            "return 0",
        ]
    if kind == "MapLength":
        return [f"return ctx.{field}.length()"]
    if kind == "MapIsEmpty":
        return [
            f"if ctx.{field}.length() == 0 {{",
            "    return 1",
            "}",
            "return 0",
        ]
    if kind == "MapSet":
        return [f"ctx.{field}.set({operation['key']}, {operation['value']})"]
    if kind == "MapRemove":
        return [f"return ctx.{field}.remove({operation['key']})"]
    if kind == "MapClear":
        return [f"ctx.{field}.clear()"]
    if kind == "CloneOwnedMap":
        return [f"return ctx.{field}.clone_owned()"]
    if kind == "ReplaceOwnedMap":
        return [f"ctx.{field} = {operation['value']}.clone_owned()"]
    if kind == "CarrierSnapshotFromOwnedMap":
        map_arg = operation["map_arg"]
        loop_var = operation["loop_var"]
        output = operation["output_arg"]
        return [
            f"{output}.set(\"loop_var_name\", {loop_var})",
            f"{output}.set(\"loop_var_id\", {map_arg}.get({loop_var}))",
            "",
            "local carrier_names = new ArrayBox()",
            "local carrier_host_ids = new ArrayBox()",
            "local i = 0",
            f"loop(i < {map_arg}.length()) {{",
            f"    local key = {map_arg}.key_at(i)",
            f"    if key != {loop_var} {{",
            "        carrier_names.push(key)",
            f"        carrier_host_ids.push({map_arg}.get(key))",
            "    }",
            "    i = i + 1",
            "}",
            "",
            f"{output}.set(\"carrier_names\", carrier_names)",
            f"{output}.set(\"carrier_host_ids\", carrier_host_ids)",
            f"{output}.set(\"carrier_count\", carrier_names.length())",
            "return 0",
        ]
    if kind == "ExplicitCarrierSnapshotFromOwnedMap":
        map_arg = operation["map_arg"]
        loop_var = operation["loop_var"]
        loop_var_id = operation["loop_var_id"]
        requested_names = operation["requested_names"]
        output = operation["output_arg"]
        return [
            f"{output}.set(\"loop_var_name\", {loop_var})",
            f"{output}.set(\"loop_var_id\", {loop_var_id})",
            "",
            "local requested_name_copy = new ArrayBox()",
            "local requested_index = 0",
            f"loop(requested_index < {requested_names}.length()) {{",
            f"    local requested_name = {requested_names}.get(requested_index)",
            "    requested_name_copy.push(requested_name)",
            "    requested_index = requested_index + 1",
            "}",
            f"{output}.set(\"requested_names\", requested_name_copy)",
            "",
            "local requested_name_map = OrderedMap.create()",
            "local name_index = 0",
            "loop(name_index < requested_name_copy.length()) {",
            "    local requested_name = requested_name_copy.get(name_index)",
            "    requested_name_map.set(requested_name, 1)",
            "    name_index = name_index + 1",
            "}",
            "",
            "local carrier_names = new ArrayBox()",
            "local carrier_host_ids = new ArrayBox()",
            "local i = 0",
            f"loop(i < {map_arg}.length()) {{",
            f"    local key = {map_arg}.key_at(i)",
            f"    if key != {loop_var} {{",
            "        local requested_match = requested_name_map.get(key)",
            "        if requested_match != null {",
            "            carrier_names.push(key)",
            f"            carrier_host_ids.push({map_arg}.get(key))",
            "        }",
            "    }",
            "    i = i + 1",
            "}",
            "",
            "if carrier_names.length() != requested_name_copy.length() {",
            "    print(\"explicit_carrier_snapshot_missing_requested_carrier=fail\")",
            "    return 1",
            "}",
            "",
            f"{output}.set(\"carrier_names\", carrier_names)",
            f"{output}.set(\"carrier_host_ids\", carrier_host_ids)",
            f"{output}.set(\"carrier_count\", carrier_names.length())",
            f"{output}.set(\"requested_name_count\", requested_name_copy.length())",
            "return 0",
        ]
    if kind == "ReturnI64":
        return [f"return {operation['return_value']}"]
    raise ValueError(f"unsupported Hako operation: {kind}")


def _render_method_body(method: Mapping[str, Any]) -> list[str]:
    if "operations" not in method:
        return list(method["body_lines"])
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

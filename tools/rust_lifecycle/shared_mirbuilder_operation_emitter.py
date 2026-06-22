#!/usr/bin/env python3
"""Operation body renderer for shared MirBuilder Hako artifacts."""

from __future__ import annotations

from collections.abc import Mapping
from typing import Any

from shared_mirbuilder_emitter_common import (
    render_main_value,
    render_replacement_expr,
    render_source_expr,
    render_string_literal,
)


def _render_map_lookup(kind: str, operation: Mapping[str, Any]) -> list[str]:
    source = render_source_expr(operation)
    if kind == "MapGet":
        return [
            f"local keys = {source}.keys_value",
            f"local values = {source}.values_value",
            f"if {operation['key']} == null {{",
            "    return null",
            "}",
            f"local key = {operation['key']}",
            "local i = 0",
            "loop(i < keys.length()) {",
            "    if BoxHelpers.array_get(keys, i) == key {",
            "        return BoxHelpers.array_get(values, i)",
            "    }",
            "    i = i + 1",
            "}",
            "return null",
        ]
    return [
        f"local keys = {source}.keys_value",
        f"if {operation['key']} == null {{",
        "    return 0",
        "}",
        f"local key = {operation['key']}",
        "local i = 0",
        "loop(i < keys.length()) {",
        "    if BoxHelpers.array_get(keys, i) == key {",
        "        return 1",
        "    }",
        "    i = i + 1",
        "}",
        "return 0",
    ]


def _render_ordered_map_set(operation: Mapping[str, Any]) -> list[str]:
    source = render_source_expr(operation)
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


def _render_carrier_snapshot(operation: Mapping[str, Any]) -> list[str]:
    map_arg = operation["map_arg"]
    loop_var = operation["loop_var"]
    carrier_names = operation["carrier_names_arg"]
    carrier_host_ids = operation["carrier_host_ids_arg"]
    return [
        f"local snapshot_total = {map_arg}.keys_value.length()",
        "local loop_var_id = null",
        "local carrier_count = 0",
        "local i = 0",
        "loop(i < snapshot_total) {",
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


def _render_explicit_carrier_snapshot(operation: Mapping[str, Any]) -> list[str]:
    map_arg = operation["map_arg"]
    loop_var_id = operation["loop_var_id"]
    requested_names = operation["requested_names"]
    carrier_names = operation["carrier_names_arg"]
    carrier_host_ids = operation["carrier_host_ids_arg"]
    return [
        f"local snapshot_total = {map_arg}.keys_value.length()",
        "local loop_var_name = null",
        "local requested_name_copy = new ArrayBox()",
        f"local requested_name_total = {requested_names}.length()",
        "local requested_index = 0",
        "loop(requested_index < requested_name_total) {",
        f"    local requested_name = BoxHelpers.array_get({requested_names}, requested_index)",
        "    requested_name_copy.push(requested_name)",
        "    requested_index = requested_index + 1",
        "}",
        "",
        "local carrier_count = 0",
        "local i = 0",
        "loop(i < snapshot_total) {",
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
        '    print("explicit_carrier_snapshot_missing_requested_carrier=fail")',
        "    return 1",
        "}",
        "",
        "return 0",
    ]


def render_operation(operation: Mapping[str, Any]) -> list[str]:
    kind = operation["kind"]
    if kind == "MapGetOption":
        source = render_source_expr(operation)
        target = operation.get("target")
        if target is None:
            return [
                f"if {source}.has({operation['key']}) {{",
                f"    return Option::Some({source}.get({operation['key']}))",
                "}",
                "return Option::None()",
            ]
        return [
            f"local {target}: Option<MirValueKind> = Option::None()",
            f"if {source}.has({operation['key']}) {{",
            f"    {target} = Option::Some({source}.get({operation['key']}))",
            "}",
        ]
    if kind == "ReturnDefaultIfMissing":
        return [
            f"guard let Option::Some(value) = {operation['source']} else {{",
            f"    return {operation['default']}",
            "}",
            "return value",
        ]
    if kind == "NewLocalBox":
        target = operation.get("target")
        box_name = operation.get("box")
        if target is None or box_name is None:
            raise ValueError("NewLocalBox requires target and box")
        return [f"local {target} = new {box_name}()"]
    if kind == "MoveFieldAndResetSource":
        replacement = operation.get("replacement")
        if not isinstance(replacement, Mapping):
            raise ValueError("MoveFieldAndResetSource requires replacement")
        return [
            f"{operation['target_owner']}.{operation['target_field']} = {operation['source_owner']}.{operation['source_field']}",
            f"{operation['source_owner']}.{operation['source_field']} = {render_replacement_expr(replacement)}",
        ]
    if kind == "AssertNotConsumed":
        source = render_source_expr(operation)
        fail_message = operation.get("fail_message", "[snapshot:already-consumed]")
        fail_code = operation.get("fail_code", 1)
        return [
            f"if {source} != 0 {{",
            f"    print({render_string_literal(fail_message)})",
            f"    return {fail_code}",
            "}",
        ]
    if kind == "MarkConsumed":
        return [f"{render_source_expr(operation)} = 1", "return 0"]
    if kind in {"MapGet", "MapHas"}:
        return _render_map_lookup(kind, operation)
    if kind == "MapLength":
        return [f"local keys = {render_source_expr(operation)}.keys_value", "return keys.length()"]
    if kind == "MapIsEmpty":
        source = render_source_expr(operation)
        return [
            f"local keys = {source}.keys_value",
            "if keys.length() == 0 {",
            "    return 1",
            "}",
            "return 0",
        ]
    if kind == "MapSet":
        source = render_source_expr(operation)
        if operation.get("storage") == "MapBox":
            return [f"{source}.set({operation['key']}, {operation['value']})", "return 1"]
        return _render_ordered_map_set(operation)
    if kind == "MapBoxGet":
        return [f"return {render_source_expr(operation)}.get({operation['key']})"]
    if kind == "MapBoxHas":
        return [f"return {render_source_expr(operation)}.has({operation['key']})"]
    if kind == "MapRemove":
        return [f"return {render_source_expr(operation)}.remove({operation['key']})"]
    if kind == "MapClear":
        source = render_source_expr(operation)
        if operation.get("storage") == "MapBox":
            if "field" in operation:
                return [f"{source} = new MapBox()", "return 1"]
            return [f"{source}.clear()", "return 1"]
        return [
            f"{source}.keys_value = new ArrayBox()",
            f"{source}.values_value = new ArrayBox()",
            "return 1",
        ]
    if kind == "CloneOwnedMap":
        return [f"return {render_source_expr(operation)}.clone_owned()"]
    if kind == "ReplaceOwnedMap":
        return [f"{render_source_expr(operation)} = {operation['value']}.clone_owned()"]
    if kind == "NewBoxWithFieldValues":
        target = operation.get("target")
        box_name = operation.get("box")
        field_values = operation.get("field_values", {})
        if target is None or box_name is None or not isinstance(field_values, Mapping):
            raise ValueError("NewBoxWithFieldValues requires target, box, and field_values")
        lines = [f"local {target} = new {box_name}()"]
        for field, value in field_values.items():
            lines.append(f"{target}.{field} = {render_main_value(value)}")
        lines.append(f"return {target}")
        return lines
    if kind == "FieldGet":
        return [f"return {render_source_expr(operation)}"]
    if kind == "FieldSet":
        return [f"{render_source_expr(operation)} = {operation['value']}", "return 1"]
    if kind == "SetSome":
        return [f"{render_source_expr(operation)} = Option::Some({operation['value']})", "return 1"]
    if kind == "ClearOption":
        return [f"{render_source_expr(operation)} = Option::None()", "return 1"]
    if kind == "CloneImmutableString":
        return [f"return {render_source_expr(operation)}"]
    if kind == "ReturnSource":
        return [f"return {render_source_expr(operation)}"]
    if kind == "CarrierSnapshotFromOwnedMap":
        return _render_carrier_snapshot(operation)
    if kind == "ExplicitCarrierSnapshotFromOwnedMap":
        return _render_explicit_carrier_snapshot(operation)
    if kind == "ReturnI64":
        return [f"return {operation['return_value']}"]
    if kind == "AllFieldsMapIsEmpty":
        source = render_source_expr(operation)
        fields = operation.get("fields", [])
        if not fields:
            return ["return 1"]
        checks = [f"{source}.{field}.is_empty()" for field in fields]
        return [f"if {' && '.join(checks)} {{", "    return 1", "}", "return 0"]
    if kind == "TakeThenSaturatingIncrementU32":
        source = render_source_expr(operation)
        return [
            f"local id = {source}",
            f"if {source} < 4294967295 {{",
            f"    {source} = {source} + 1",
            "}",
            "return id",
        ]
    raise ValueError(f"unsupported Hako operation: {kind}")


def render_method_body(method: Mapping[str, Any]) -> list[str]:
    if "operations" not in method:
        raise ValueError(f"method has no operations: {method['signature']}")
    lines: list[str] = []
    for operation in method["operations"]:
        lines.extend(render_operation(operation))
    return lines

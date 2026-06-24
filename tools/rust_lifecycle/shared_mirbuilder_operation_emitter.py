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


def _render_expr(expr: Any) -> str:
    if isinstance(expr, str):
        return expr
    if not isinstance(expr, Mapping):
        return render_main_value(expr)
    kind = expr.get("kind")
    if kind == "Var":
        name = expr.get("name")
        if not isinstance(name, str):
            raise ValueError("Var expression requires name")
        return name
    if kind == "I64":
        value = expr.get("value")
        if not isinstance(value, int):
            raise ValueError("I64 expression requires integer value")
        return str(value)
    if kind == "ArrayLength":
        source = expr.get("source")
        if not isinstance(source, str):
            raise ValueError("ArrayLength expression requires source")
        return f"{source}.length()"
    if kind == "ArrayGet":
        source = expr.get("source")
        index = expr.get("index")
        if not isinstance(source, str) or index is None:
            raise ValueError("ArrayGet expression requires source and index")
        return f"BoxHelpers.array_get({source}, {_render_expr(index)})"
    if kind in {"AddI64", "LtI64", "EqI64"}:
        left = expr.get("left")
        right = expr.get("right")
        if left is None or right is None:
            raise ValueError(f"{kind} expression requires left and right")
        op = {"AddI64": "+", "LtI64": "<", "EqI64": "=="}[kind]
        return f"{_render_expr(left)} {op} {_render_expr(right)}"
    raise ValueError(f"unsupported Hako expression: {kind}")


def _render_statement_operation(operation: Mapping[str, Any]) -> list[str]:
    kind = operation["kind"]
    if kind == "LocalI64":
        target = operation.get("target")
        if not isinstance(target, str):
            raise ValueError("LocalI64 requires target")
        return [f"local {target} = {_render_expr(operation.get('value', {'kind': 'I64', 'value': 0}))}"]
    if kind == "Assign":
        target = operation.get("target")
        value = operation.get("value")
        if not isinstance(target, str) or value is None:
            raise ValueError("Assign requires target and value")
        return [f"{target} = {_render_expr(value)}"]
    if kind == "ArrayPush":
        target = operation.get("target")
        value = operation.get("value")
        if not isinstance(target, str) or value is None:
            raise ValueError("ArrayPush requires target and value")
        return [f"{target}.push({_render_expr(value)})"]
    if kind == "StructuredLoop":
        condition = operation.get("condition")
        body = operation.get("body")
        if condition is None or not isinstance(body, list):
            raise ValueError("StructuredLoop requires condition and body")
        lines = [f"loop({_render_expr(condition)}) {{"]
        for item in body:
            if not isinstance(item, Mapping):
                raise ValueError("StructuredLoop body entries must be operations")
            lines.extend("    " + line if line else "" for line in _render_statement_operation(item))
        lines.append("}")
        return lines
    if kind == "ExplicitPhiI64":
        target = operation.get("target")
        condition = operation.get("condition")
        true_value = operation.get("true_value")
        false_value = operation.get("false_value")
        if not isinstance(target, str) or condition is None or true_value is None or false_value is None:
            raise ValueError("ExplicitPhiI64 requires target, condition, true_value, and false_value")
        return [
            f"local {target} = 0",
            f"if {_render_expr(condition)} {{",
            f"    {target} = {_render_expr(true_value)}",
            "} else {",
            f"    {target} = {_render_expr(false_value)}",
            "}",
        ]
    if kind == "ExplicitMultiExitPhiI64Array":
        target = operation.get("target")
        selector = operation.get("selector")
        exits = operation.get("exits")
        if not isinstance(target, str) or selector is None or not isinstance(exits, list) or not exits:
            raise ValueError("ExplicitMultiExitPhiI64Array requires target, selector, and exits")
        lines = [f"local {target} = new ArrayBox()"]
        for index, exit_case in enumerate(exits):
            if not isinstance(exit_case, Mapping):
                raise ValueError("ExplicitMultiExitPhiI64Array exits must be mappings")
            condition = exit_case.get("condition")
            values = exit_case.get("values")
            if condition is None or not isinstance(values, list):
                raise ValueError("ExplicitMultiExitPhiI64Array exit requires condition and values")
            prefix = "if" if index == 0 else "} else if"
            lines.append(f"{prefix} {_render_expr(condition)} {{")
            for value in values:
                lines.append(f"    {target}.push({_render_expr(value)})")
        lines.append("} else {")
        lines.append(f"    print({render_string_literal(operation.get('fail_message', 'multi_exit_phi_unknown_exit=fail'))})")
        lines.append(f"    return {operation.get('fail_code', 1)}")
        lines.append("}")
        return lines
    raise ValueError(f"unsupported statement operation: {kind}")


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


def _render_ordered_map_set_statement(source: str, key: str, value: str) -> list[str]:
    return [
        "local found = 0",
        "local set_i = 0",
        f"loop(set_i < {source}.keys_value.length()) {{",
        f"    if BoxHelpers.array_get({source}.keys_value, set_i) == {key} {{",
        f"        {source}.values_value.set(set_i, {value})",
        "        found = 1",
        f"        set_i = {source}.keys_value.length()",
        "    }",
        "    set_i = set_i + 1",
        "}",
        "if found == 0 {",
        f"    {source}.keys_value.push({key})",
        f"    {source}.values_value.push({value})",
        f"    local pos = {source}.keys_value.length() - 1",
        "    local keep_swapping = 1",
        "    loop(pos > 0 && keep_swapping != 0) {",
        "        local prev = pos - 1",
        f"        local a = BoxHelpers.array_get({source}.keys_value, prev)",
        f"        local b = BoxHelpers.array_get({source}.keys_value, pos)",
        "        if a < b or a == b {",
        "            keep_swapping = 0",
        "        } else {",
        f"            local av = BoxHelpers.array_get({source}.values_value, prev)",
        f"            local bv = BoxHelpers.array_get({source}.values_value, pos)",
        f"            {source}.keys_value.set(prev, b)",
        f"            {source}.keys_value.set(pos, a)",
        f"            {source}.values_value.set(prev, bv)",
        f"            {source}.values_value.set(pos, av)",
        "            pos = pos - 1",
        "        }",
        "    }",
        "}",
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


def _render_read_fold_statement(operation: Mapping[str, Any]) -> list[str]:
    kind = operation["kind"]
    if kind == "MapLookupOption":
        source = operation.get("source")
        key = operation.get("key")
        target = operation.get("target")
        raw_target = operation.get("raw_target", f"raw_{target}")
        if not all(isinstance(value, str) for value in (source, key, target, raw_target)):
            raise ValueError("MapLookupOption statement requires source, key, target, and raw_target")
        return [
            f"local {raw_target} = {source}.get({key})",
            f"local {target} = Option::None()",
            f"if {raw_target} != null {{",
            f"    {target} = Option::Some({raw_target})",
            "}",
        ]
    if kind == "CallStatic":
        target = operation.get("target")
        callee = operation.get("callee")
        args = operation.get("args", [])
        if not isinstance(target, str) or not isinstance(callee, str) or not isinstance(args, list):
            raise ValueError("CallStatic statement requires target, callee, and args")
        if not all(isinstance(arg, str) for arg in args):
            raise ValueError("CallStatic statement args must be local names")
        return [f"local {target} = {callee}({', '.join(args)})"]
    if kind == "ConstructOwnedProduct":
        target = operation.get("target")
        box_name = operation.get("box")
        fields = operation.get("fields")
        if not isinstance(target, str) or not isinstance(box_name, str) or not isinstance(fields, Mapping):
            raise ValueError("ConstructOwnedProduct requires target, box, and fields")
        lines = [f"local {target} = new {box_name}()"]
        for field, value in fields.items():
            if not isinstance(field, str) or not isinstance(value, str):
                raise ValueError("ConstructOwnedProduct fields must map field names to local names")
            lines.append(f"{target}.{field} = {value}")
        return lines
    if kind == "SequencePush":
        target = operation.get("target")
        value = operation.get("value")
        if not isinstance(target, str) or not isinstance(value, str):
            raise ValueError("SequencePush statement requires target and value")
        return [f"{target}.push({value})"]
    if kind == "MapSet":
        source = operation.get("source")
        key = operation.get("key")
        value = operation.get("value")
        if not isinstance(source, str) or not isinstance(key, str) or not isinstance(value, str):
            raise ValueError("MapSet statement requires source, key, and value")
        storage = operation.get("storage")
        if storage is None:
            raise ValueError("MapSet statement requires explicit storage")
        if storage in {"MapBox", "OrderedMapBox", "ValueIdOrderedMapBox"}:
            return [f"{source}.set({key}, {value})"]
        raise ValueError(f"unsupported MapSet statement storage: {storage}")
    raise ValueError(f"unsupported read-fold statement operation: {kind}")


def _render_for_each_map_entry(operation: Mapping[str, Any]) -> list[str]:
    source = operation.get("source")
    key_binding = operation.get("key_binding")
    value_binding = operation.get("value_binding")
    body = operation.get("body")
    if not isinstance(source, str) or not isinstance(key_binding, str) or not isinstance(value_binding, str):
        raise ValueError("ForEachMapEntry requires source, key_binding, and value_binding")
    if not isinstance(body, list):
        raise ValueError("ForEachMapEntry requires body")
    source_storage = operation.get("source_storage")
    if source_storage is None:
        raise ValueError("ForEachMapEntry requires explicit source_storage")
    if source_storage == "ValueIdOrderedMapBox":
        lines = [
            f"local total = {source}.length()",
            "local i = 0",
            "loop(i < total) {",
            f"    local {key_binding} = {source}.key_at(i)",
            f"    local {value_binding} = {source}.value_at(i)",
        ]
    elif source_storage == "OrderedMapBox":
        lines = [
            f"local total = {source}.length()",
            "local i = 0",
            "loop(i < total) {",
            f"    local {key_binding} = {source}.key_at(i)",
            f"    local {value_binding} = {source}.get({key_binding})",
        ]
    elif source_storage == "MapBox":
        lines = [
            f"local keys = {source}.keys()",
            "local i = 0",
            "loop(i < keys.length()) {",
            f"    local {key_binding} = BoxHelpers.array_get(keys, i)",
            f"    local {value_binding} = {source}.get({key_binding})",
        ]
    else:
        raise ValueError(f"unsupported ForEachMapEntry source storage: {source_storage}")
    for item in body:
        if not isinstance(item, Mapping):
            raise ValueError("ForEachMapEntry body entries must be operations")
        lines.extend("    " + line if line else "" for line in _render_read_fold_statement(item))
    lines.extend(["    i = i + 1", "}"])
    return lines


def _render_for_each_ordered_map_entry(operation: Mapping[str, Any]) -> list[str]:
    source = operation.get("source")
    key_binding = operation.get("key_binding")
    value_binding = operation.get("value_binding")
    body = operation.get("body")
    if not isinstance(source, str) or not isinstance(key_binding, str) or not isinstance(value_binding, str):
        raise ValueError("ForEachOrderedMapEntry requires source, key_binding, and value_binding")
    if not isinstance(body, list):
        raise ValueError("ForEachOrderedMapEntry requires body")
    lines = [
        f"local total = {source}.keys_value.length()",
        "local i = 0",
        "loop(i < total) {",
        f"    local {key_binding} = {source}.keys_value.get(i)",
        f"    local {value_binding} = {source}.values_value.get(i)",
    ]
    for item in body:
        if not isinstance(item, Mapping):
            raise ValueError("ForEachOrderedMapEntry body entries must be operations")
        lines.extend("    " + line if line else "" for line in _render_read_fold_statement(item))
    lines.extend(["    i = i + 1", "}"])
    return lines


def render_operation(operation: Mapping[str, Any]) -> list[str]:
    kind = operation["kind"]
    if kind in {"LocalI64", "Assign", "ArrayPush", "StructuredLoop", "ExplicitPhiI64", "ExplicitMultiExitPhiI64Array"}:
        return _render_statement_operation(operation)
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
    if kind == "NewLocalArray":
        target = operation.get("target")
        if target is None:
            raise ValueError("NewLocalArray requires target")
        return [f"local {target} = new ArrayBox()"]
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
        storage = operation.get("storage")
        if storage is None:
            raise ValueError("MapSet requires explicit storage")
        if storage == "OrderedMapBox":
            return _render_ordered_map_set(operation)
        if storage in {"MapBox", "ValueIdOrderedMapBox"}:
            return [
                f"{source}.set({operation['key']}, {operation['value']})",
                "return 1",
            ]
        raise ValueError(f"unsupported MapSet storage: {storage}")
    if kind == "ForEachMapEntry":
        return _render_for_each_map_entry(operation)
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
    if kind == "SequencePush":
        return [f"{render_source_expr(operation)}.push({operation['value']})", "return 1"]
    if kind == "SequencePopOption":
        source = render_source_expr(operation)
        return [
            f"local n = {source}.length()",
            "if n == 0 {",
            "    return Option::None()",
            "}",
            f"local value = {source}.pop()",
            "return Option::Some(value)",
        ]
    if kind == "SequenceLastOption":
        source = render_source_expr(operation)
        value_expr = f"BoxHelpers.array_get({source}, n - 1)"
        if operation.get("element_transport") == "i64":
            value_expr = f"({value_expr} + 0)"
        return [
            f"local n = {source}.length()",
            "if n == 0 {",
            "    return Option::None()",
            "}",
            f"return Option::Some({value_expr})",
        ]
    if kind == "ClassifyEnumVariants":
        type_source = operation.get("type_source")
        source_enum = operation.get("source_enum")
        missing_value = operation.get("missing_value_fallback")
        variant_groups = operation.get("variant_groups")
        default_return = operation.get("default_return")
        if not isinstance(type_source, str) or not isinstance(source_enum, str):
            raise ValueError("ClassifyEnumVariants requires type_source and source_enum")
        if not isinstance(missing_value, dict) or not isinstance(variant_groups, list) or not isinstance(default_return, str):
            raise ValueError("ClassifyEnumVariants requires missing_value_fallback, variant_groups, and default_return")
        name_source = missing_value.get("input")
        string_set = missing_value.get("string_set")
        matched = missing_value.get("matched")
        unmatched = missing_value.get("unmatched")
        if not isinstance(name_source, str) or not isinstance(string_set, list) or not isinstance(matched, str) or not isinstance(unmatched, str):
            raise ValueError("ClassifyEnumVariants missing-value fallback is malformed")
        fallback_check = " or ".join(f"{name_source} == {render_string_literal(name)}" for name in string_set)
        cases = []
        for group in variant_groups:
            variants = group.get("variants") if isinstance(group, dict) else None
            returns = group.get("returns") if isinstance(group, dict) else None
            if not isinstance(variants, list) or not isinstance(returns, str):
                raise ValueError("ClassifyEnumVariants variant group is malformed")
            for variant in variants:
                if isinstance(variant, str):
                    cases.append({"name": variant, "payload_var": None, "returns": returns})
                elif isinstance(variant, dict) and isinstance(variant.get("name"), str):
                    cases.append({
                        "name": variant["name"],
                        "payload_var": variant.get("payload_var"),
                        "returns": returns,
                    })
                else:
                    raise ValueError("ClassifyEnumVariants variant entry is malformed")

        def render_cases(index: int, indent: str = "") -> list[str]:
            if index >= len(cases):
                return [f"{indent}return {default_return}"]
            case = cases[index]
            name = case["name"]
            returns = case["returns"]
            payload_var = case.get("payload_var")
            if isinstance(payload_var, str):
                lines = [f"{indent}guard let {source_enum}::{name}({payload_var}) = ty else {{"]
                lines.extend(render_cases(index + 1, indent + "    "))
                lines.append(f"{indent}}}")
                lines.append(f"{indent}return {returns}")
                return lines
            lines = [f"{indent}if ty == {source_enum}::{name}() {{"]
            lines.append(f"{indent}    return {returns}")
            lines.append(f"{indent}}}")
            lines.extend(render_cases(index + 1, indent))
            return lines

        return [
            f"guard let Option::Some(ty) = {type_source} else {{",
            f"    if {fallback_check} {{",
            f"        return {matched}",
            "    }",
            f"    return {unmatched}",
            "}",
        ] + render_cases(0)
    if kind == "ForEachOrderedMapEntry":
        return _render_for_each_ordered_map_entry(operation)
    if kind == "ArrayElementFieldGet":
        source = operation.get("array")
        index = operation.get("index")
        field = operation.get("field_name")
        if not isinstance(source, str) or not isinstance(index, str) or not isinstance(field, str):
            raise ValueError("ArrayElementFieldGet requires array, index, and field_name")
        return [f"return {source}.get({index}).{field}"]
    if kind == "CloneOwnedMap":
        target = operation.get("target")
        if isinstance(target, str):
            source = render_source_expr(operation)
            target_storage = operation.get("target_storage")
            if target_storage == "ValueIdOrderedMapBox":
                key_var = f"{target}_clone_key"
                value_var = f"{target}_clone_value"
                index_var = f"{target}_clone_i"
                total_var = f"{target}_clone_total"
                return [
                    "local " + target + " = OrderedMap.create()",
                    f"local {total_var} = {source}.length()",
                    f"local {index_var} = 0",
                    f"loop({index_var} < {total_var}) {{",
                    f"    local {key_var} = {source}.key_at({index_var})",
                    f"    local {value_var} = {source}.value_at({index_var})",
                    f"    {target}.set({key_var}, {value_var})",
                    f"    {index_var} = {index_var} + 1",
                    "}",
                ]
            if target_storage == "OrderedMapBox":
                constructor = "OrderedMap.create()"
                key_var = f"{target}_clone_key"
                keys_var = f"{target}_clone_keys"
                value_var = f"{target}_clone_value"
                index_var = f"{target}_clone_i"
                lines = [
                    f"local {target} = {constructor}",
                    f"local {keys_var} = {source}.keys_value",
                    f"local {index_var} = 0",
                    f"loop({index_var} < {keys_var}.length()) {{",
                    f"    local {key_var} = BoxHelpers.array_get({keys_var}, {index_var})",
                    f"    local {value_var} = BoxHelpers.array_get({source}.values_value, {index_var})",
                ]
                lines.extend(
                    "    " + line
                    for line in _render_ordered_map_set_statement(target, key_var, value_var)
                )
                lines.extend([f"    {index_var} = {index_var} + 1", "}"])
                return lines
            raise ValueError(f"unsupported CloneOwnedMap target storage: {target_storage}")
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

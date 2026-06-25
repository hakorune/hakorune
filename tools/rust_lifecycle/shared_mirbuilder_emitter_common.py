#!/usr/bin/env python3
"""Small rendering helpers shared by MirBuilder Hako emitters."""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from typing import Any


def indent(lines: Sequence[str], spaces: int) -> list[str]:
    prefix = " " * spaces
    return [prefix + line if line else "" for line in lines]


def render_initializer(box: Mapping[str, Any]) -> str:
    operation = box.get("initializer_operation")
    if operation is None:
        return str(box["initializer"])
    if operation.get("kind") == "NewOrderedMap":
        return "OrderedMap.create()"
    if operation.get("kind") == "NewValueIdOrderedMap":
        return "ValueIdOrderedMap.create()"
    raise ValueError(f"unsupported initializer operation: {operation.get('kind')}")


def render_field_initializer(field: Mapping[str, Any]) -> str:
    operation = field.get("initializer_operation")
    if operation is None:
        return str(field["initializer"])
    if operation.get("kind") == "NewOrderedMap":
        return "OrderedMap.create()"
    if operation.get("kind") == "NewValueIdOrderedMap":
        return "ValueIdOrderedMap.create()"
    if operation.get("kind") == "NewMap":
        return "new MapBox()"
    raise ValueError(f"unsupported initializer operation: {operation.get('kind')}")


def render_source_expr(operation: Mapping[str, Any]) -> str:
    source = operation.get("source")
    if source is not None:
        return str(source)
    field = operation.get("field")
    if field is not None:
        return f"ctx.{field}"
    raise ValueError(f"operation is missing source/field: {operation.get('kind')}")


def render_replacement_expr(replacement: Mapping[str, Any]) -> str:
    kind = replacement.get("kind")
    if kind == "NewMap":
        return "new MapBox()"
    raise ValueError(f"unsupported replacement operation: {kind}")


def render_string_literal(value: Any) -> str:
    text = str(value)
    escaped = text.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def render_main_value(value: Any) -> str:
    if isinstance(value, Mapping):
        if "literal" in value:
            return render_string_literal(value["literal"])
        if "expr" in value:
            return str(value["expr"])
        raise ValueError(f"unsupported Main value object: {value}")
    if value is None:
        return "null"
    return str(value)


def render_call_args(args: Any) -> str:
    if args is None:
        return ""
    if isinstance(args, str):
        return args
    if isinstance(args, Sequence):
        return ", ".join(render_main_value(arg) for arg in args)
    return str(args)

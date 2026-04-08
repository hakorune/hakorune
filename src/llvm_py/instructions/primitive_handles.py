import llvmlite.ir as ir
from typing import Any, Optional

from type_facts import is_box_handle_fact


def _ensure_module_function(module, name: str, return_type, arg_types):
    for func in module.functions:
        if func.name == name:
            return func
    return ir.Function(module, ir.FunctionType(return_type, arg_types), name=name)


def resolver_value_type(resolver, value_id: int) -> Optional[Any]:
    if resolver is None or not hasattr(resolver, "value_types"):
        return None
    value_types = getattr(resolver, "value_types", None)
    if not isinstance(value_types, dict):
        return None
    return value_types.get(value_id)


def primitive_numeric_meta_kind(meta: Any) -> Optional[str]:
    if meta == "i64" or meta == "Integer" or (
        isinstance(meta, dict) and meta.get("kind") in ("i64", "Integer")
    ):
        return "Integer"
    if is_box_handle_fact(meta, "IntegerBox") or is_box_handle_fact(meta, "BoolBox"):
        return "Integer"
    if meta == "f64" or meta == "Float" or (
        isinstance(meta, dict) and meta.get("kind") in ("f64", "Float")
    ):
        return "Float"
    return None


def primitive_unbox_helper_name(meta: Any) -> Optional[str]:
    if is_box_handle_fact(meta, "IntegerBox"):
        return "nyash.integer.get_h"
    if is_box_handle_fact(meta, "BoolBox"):
        return "nyash.bool.get_h"
    return None


def unbox_primitive_handle_if_needed(
    builder: ir.IRBuilder,
    value,
    meta: Any,
    *,
    name_hint: str,
):
    helper_name = primitive_unbox_helper_name(meta)
    if helper_name is None or value is None:
        return value

    i64 = ir.IntType(64)
    try:
        vtype = value.type
    except Exception:
        vtype = None

    if isinstance(vtype, ir.PointerType):
        value = builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    elif isinstance(vtype, ir.IntType):
        if vtype.width < 64:
            value = builder.zext(value, i64, name=f"{name_hint}_zext")
        elif vtype.width > 64:
            value = builder.trunc(value, i64, name=f"{name_hint}_trunc")

    callee = _ensure_module_function(builder.module, helper_name, i64, [i64])
    return builder.call(callee, [value], name=f"{name_hint}_unbox")

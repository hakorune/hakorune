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
    if is_box_handle_fact(meta, "FloatBox"):
        return "Float"
    if meta == "f64" or meta == "Float" or (
        isinstance(meta, dict) and meta.get("kind") in ("f64", "Float")
    ):
        return "Float"
    return None


def primitive_unbox_helper_spec(meta: Any):
    if is_box_handle_fact(meta, "IntegerBox"):
        return ("nyash.integer.get_h", ir.IntType(64))
    if is_box_handle_fact(meta, "BoolBox"):
        return ("nyash.bool.get_h", ir.IntType(64))
    if is_box_handle_fact(meta, "FloatBox"):
        return ("nyash.float.unbox_to_f64", ir.DoubleType())
    return None


def primitive_unbox_helper_name(meta: Any) -> Optional[str]:
    spec = primitive_unbox_helper_spec(meta)
    if spec is None:
        return None
    return spec[0]


def primitive_unbox_return_type(meta: Any):
    spec = primitive_unbox_helper_spec(meta)
    if spec is None:
        return None
    return spec[1]


def _canonical_unbox_helper_arg(
    builder: ir.IRBuilder,
    value,
    *,
    name_hint: str,
):
    i64 = ir.IntType(64)
    try:
        vtype = value.type
    except Exception:
        vtype = None

    if isinstance(vtype, ir.PointerType):
        return builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    if isinstance(vtype, ir.IntType):
        if vtype.width < 64:
            return builder.zext(value, i64, name=f"{name_hint}_zext")
        if vtype.width > 64:
            return builder.trunc(value, i64, name=f"{name_hint}_trunc")
        return value
    return None


def unbox_primitive_handle_if_needed(
    builder: ir.IRBuilder,
    value,
    meta: Any,
    *,
    name_hint: str,
):
    helper_name = primitive_unbox_helper_name(meta)
    return_type = primitive_unbox_return_type(meta)
    if helper_name is None or return_type is None or value is None:
        return value

    try:
        vtype = value.type
    except Exception:
        vtype = None

    if isinstance(vtype, ir.DoubleType) and isinstance(return_type, ir.DoubleType):
        return value

    arg_value = _canonical_unbox_helper_arg(builder, value, name_hint=name_hint)
    if arg_value is None:
        return value

    i64 = ir.IntType(64)
    callee = _ensure_module_function(builder.module, helper_name, return_type, [i64])
    return builder.call(callee, [arg_value], name=f"{name_hint}_unbox")

from typing import Any, Dict, Optional

import llvmlite.ir as ir

from instructions.mir_call.runtime_data_dispatch import lower_runtime_data_field_call
from instructions.primitive_handles import resolver_value_type, unbox_primitive_handle_if_needed
from instructions.thin_entry_selection import thin_entry_prefers_inline_scalar_field
from instructions.user_box_local import (
    lower_local_user_box_field_get,
    lower_local_user_box_field_set,
)
from type_facts import is_box_handle_fact
from utils.resolver_helpers import mark_as_handle
from utils.values import resolve_i64_strict
from .field_access_helpers_typed_exact import (
    _exact_field_plan_for_receiver,
    _lower_exact_object_field_get,
    _lower_exact_object_field_set,
    _receiver_box_type,
    _selected_typed_object_exact_slot_native_direct_route_decision,
    _selected_typed_object_exact_slot_route_decision,
)

from instructions.field_access_helpers_common import (
    _canonical_i64,
    _declare,
    _ensure_handle,
    _field_ptr,
    _mark_bool_immediate,
    _mark_float_immediate,
    _mark_integer_immediate,
    _resolve_receiver,
)

_SAFE_TYPED_FIELD_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)

def _declared_type_matches_box_type(declared_type: Any, expected_box_type: str) -> bool:
    return is_box_handle_fact(declared_type, expected_box_type) or declared_type == expected_box_type


def _lookup_user_box_field_decl(
    user_box_decls: Any,
    receiver_box_type: Optional[str],
    field_name: str,
) -> Optional[Dict[str, Any]]:
    if not isinstance(receiver_box_type, str):
        return None
    for box_decl in user_box_decls or []:
        if not isinstance(box_decl, dict) or box_decl.get("name") != receiver_box_type:
            continue
        for field_decl in box_decl.get("field_decls", []) or []:
            if isinstance(field_decl, dict) and field_decl.get("name") == field_name:
                return field_decl
        return None
    return None


def _typed_user_box_field_enabled(
    *,
    box_vid: Optional[int],
    field_name: str,
    declared_type: Any,
    user_box_decls: Any,
    resolver,
    expected_box_type: str,
    thin_entry_surface: str,
    selection_value_id: Optional[int] = None,
) -> bool:
    selector_pref = thin_entry_prefers_inline_scalar_field(
        resolver=resolver,
        surface=thin_entry_surface,
        box_vid=box_vid,
        field_name=field_name,
        selection_value_id=selection_value_id,
    )
    if selector_pref is False:
        return False
    if selector_pref is True and _declared_type_matches_box_type(
        declared_type, expected_box_type
    ):
        return True

    receiver_box_type = _receiver_box_type(resolver, box_vid)
    field_decl = _lookup_user_box_field_decl(user_box_decls, receiver_box_type, field_name)
    if not isinstance(field_decl, dict):
        return False
    if bool(field_decl.get("is_weak")):
        return False
    if field_decl.get("declared_type") != expected_box_type:
        return False
    if declared_type is not None and not _declared_type_matches_box_type(
        declared_type, expected_box_type
    ):
        return False
    return True


def _typed_integer_field_enabled(
    *,
    box_vid: Optional[int],
    field_name: str,
    declared_type: Any,
    user_box_decls: Any,
    resolver,
    thin_entry_surface: str = "user_box_field_get",
    selection_value_id: Optional[int] = None,
) -> bool:
    return _typed_user_box_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        expected_box_type="IntegerBox",
        thin_entry_surface=thin_entry_surface,
        selection_value_id=selection_value_id,
    )


def _typed_bool_field_enabled(
    *,
    box_vid: Optional[int],
    field_name: str,
    declared_type: Any,
    user_box_decls: Any,
    resolver,
    thin_entry_surface: str = "user_box_field_get",
    selection_value_id: Optional[int] = None,
) -> bool:
    return _typed_user_box_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        expected_box_type="BoolBox",
        thin_entry_surface=thin_entry_surface,
        selection_value_id=selection_value_id,
    )


def _typed_float_field_enabled(
    *,
    box_vid: Optional[int],
    field_name: str,
    declared_type: Any,
    user_box_decls: Any,
    resolver,
    thin_entry_surface: str = "user_box_field_get",
    selection_value_id: Optional[int] = None,
) -> bool:
    return _typed_user_box_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        expected_box_type="FloatBox",
        thin_entry_surface=thin_entry_surface,
        selection_value_id=selection_value_id,
    )


def _is_bool_immediate_meta(meta: Any) -> bool:
    if meta in ("i1", "Bool"):
        return True
    if isinstance(meta, dict) and meta.get("kind") in ("i1", "Bool"):
        return True
    return False


def _is_float_immediate_meta(meta: Any) -> bool:
    if meta in ("f64", "Float"):
        return True
    if isinstance(meta, dict) and meta.get("kind") in ("f64", "Float"):
        return True
    return False


def _boolish_llvm_value(vmap: Dict[int, Any], value_vid: Optional[int]) -> bool:
    if not isinstance(value_vid, int):
        return False
    value = vmap.get(value_vid)
    if value is None:
        return False
    try:
        vtype = value.type
    except _SAFE_TYPED_FIELD_EXC:
        return False
    return isinstance(vtype, ir.IntType) and vtype.width == 1


def _floatish_llvm_value(vmap: Dict[int, Any], value_vid: Optional[int]) -> bool:
    if not isinstance(value_vid, int):
        return False
    value = vmap.get(value_vid)
    if value is None:
        return False
    try:
        vtype = value.type
    except _SAFE_TYPED_FIELD_EXC:
        return False
    return isinstance(vtype, ir.DoubleType)


def _typed_bool_field_set_enabled(
    *,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    declared_type: Any,
    user_box_decls: Any,
    vmap: Dict[int, Any],
    resolver,
) -> bool:
    if not _typed_bool_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        thin_entry_surface="user_box_field_set",
    ):
        return False
    value_meta = (
        resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
    )
    if _is_bool_immediate_meta(value_meta) or is_box_handle_fact(value_meta, "BoolBox"):
        return True
    return _boolish_llvm_value(vmap, value_vid)


def _typed_float_field_set_enabled(
    *,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    declared_type: Any,
    user_box_decls: Any,
    vmap: Dict[int, Any],
    resolver,
) -> bool:
    if not _typed_float_field_enabled(
        box_vid=box_vid,
        field_name=field_name,
        declared_type=declared_type,
        user_box_decls=user_box_decls,
        resolver=resolver,
        thin_entry_surface="user_box_field_set",
    ):
        return False
    value_meta = (
        resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
    )
    if is_box_handle_fact(value_meta, "FloatBox"):
        return True
    if _is_float_immediate_meta(value_meta):
        return _floatish_llvm_value(vmap, value_vid)
    return _floatish_llvm_value(vmap, value_vid)


def _lower_typed_integer_field_get(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    dst_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    callee = _declare(module, "nyash.instance.get_i64_field_h", i64, [i64, i8p])
    result = builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name)],
        name="typed_field_get_i64",
    )
    if dst_vid is not None:
        vmap[dst_vid] = result
        _mark_integer_immediate(resolver, int(dst_vid))
    return result


def _lower_typed_integer_field_set(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    value_val = _resolve_receiver(
        builder, value_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    value_val = unbox_primitive_handle_if_needed(
        builder,
        _canonical_i64(builder, value_val, name_hint="typed_field_set_i64_value"),
        resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None,
        name_hint=f"typed_field_set_i64_{value_vid}",
    )
    value_val = _canonical_i64(builder, value_val, name_hint="typed_field_set_i64_final")
    callee = _declare(module, "nyash.instance.set_i64_field_h", i64, [i64, i8p, i64])
    return builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name), value_val],
        name="typed_field_set_i64",
    )


def _canonical_bool_i64(builder: ir.IRBuilder, value, *, name_hint: str):
    i64 = ir.IntType(64)
    if value is None:
        return ir.Constant(i64, 0)
    try:
        vtype = value.type
    except _SAFE_TYPED_FIELD_EXC:
        vtype = None
    if isinstance(vtype, ir.PointerType):
        value = builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    elif isinstance(vtype, ir.IntType):
        if vtype.width == 1:
            return builder.zext(value, i64, name=f"{name_hint}_zext")
        if vtype.width < 64:
            value = builder.zext(value, i64, name=f"{name_hint}_zext")
        elif vtype.width > 64:
            value = builder.trunc(value, i64, name=f"{name_hint}_trunc")
        else:
            zero = ir.Constant(i64, 0)
            as_i1 = builder.icmp_unsigned("!=", value, zero, name=f"{name_hint}_i1")
            return builder.zext(as_i1, i64, name=f"{name_hint}_i64")
    zero = ir.Constant(i64, 0)
    as_i1 = builder.icmp_unsigned("!=", value, zero, name=f"{name_hint}_i1")
    return builder.zext(as_i1, i64, name=f"{name_hint}_i64")


def _lower_typed_bool_field_get(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    dst_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    callee = _declare(module, "nyash.instance.get_bool_field_h", i64, [i64, i8p])
    result = builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name)],
        name="typed_field_get_bool",
    )
    if dst_vid is not None:
        vmap[dst_vid] = result
        _mark_bool_immediate(resolver, int(dst_vid))
    return result


def _lower_typed_bool_field_set(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    value_val = _resolve_receiver(
        builder, value_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    value_meta = (
        resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
    )
    value_val = unbox_primitive_handle_if_needed(
        builder,
        _canonical_i64(builder, value_val, name_hint="typed_field_set_bool_value"),
        value_meta,
        name_hint=f"typed_field_set_bool_{value_vid}",
    )
    value_val = _canonical_bool_i64(builder, value_val, name_hint="typed_field_set_bool_final")
    callee = _declare(module, "nyash.instance.set_bool_field_h", i64, [i64, i8p, i64])
    return builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name), value_val],
        name="typed_field_set_bool",
    )


def _resolve_typed_float_value(
    builder: ir.IRBuilder,
    value,
    value_meta: Any,
    *,
    name_hint: str,
) -> ir.Value:
    f64 = ir.DoubleType()
    if value is None:
        return ir.Constant(f64, 0.0)
    try:
        vtype = value.type
    except _SAFE_TYPED_FIELD_EXC:
        vtype = None
    if isinstance(vtype, ir.DoubleType):
        return value
    value = unbox_primitive_handle_if_needed(
        builder,
        value,
        value_meta,
        name_hint=name_hint,
    )
    try:
        vtype = value.type
    except _SAFE_TYPED_FIELD_EXC:
        vtype = None
    if isinstance(vtype, ir.DoubleType):
        return value
    raise RuntimeError(
        f"[field_access] typed float setter expected FloatBox handle or f64, got {vtype}"
    )


def _lower_typed_float_field_get(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    dst_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    f64 = ir.DoubleType()
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    callee = _declare(module, "nyash.instance.get_float_field_h", f64, [i64, i8p])
    result = builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name)],
        name="typed_field_get_float",
    )
    if dst_vid is not None:
        vmap[dst_vid] = result
        _mark_float_immediate(resolver, int(dst_vid))
    return result


def _lower_typed_float_field_set(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    f64 = ir.DoubleType()
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    value_meta = (
        resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
    )
    value_val = vmap.get(value_vid) if isinstance(value_vid, int) else None
    if value_val is None:
        value_val = _resolve_receiver(
            builder, value_vid, vmap, resolver, preds, block_end_values, bb_map
        )
    value_val = _resolve_typed_float_value(
        builder,
        value_val,
        value_meta,
        name_hint=f"typed_field_set_float_{value_vid}",
    )
    callee = _declare(module, "nyash.instance.set_float_field_h", i64, [i64, i8p, f64])
    return builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name), value_val],
        name="typed_field_set_float",
    )

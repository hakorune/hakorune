from typing import Any, Dict, Optional, Set, Tuple
import hashlib

import llvmlite.ir as ir

from instructions.llvm_decl import declare_function as _declare
from instructions.primitive_handles import resolver_value_type, unbox_primitive_handle_if_needed
from instructions.thin_entry_selection import (
    thin_entry_prefers_inline_scalar_subject,
)
from utils.values import resolve_i64_strict

_SAFE_USER_BOX_LOCAL_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)


_UNSET_LOCAL_FIELD = object()

_LOCAL_LAYOUT_NAMES = {
    "integer": "inline_i64",
    "int": "inline_i64",
    "i64": "inline_i64",
    "integerbox": "inline_i64",
    "bool": "inline_bool",
    "boolean": "inline_bool",
    "boolbox": "inline_bool",
    "float": "inline_f64",
    "f64": "inline_f64",
    "floatbox": "inline_f64",
}


def _field_ptr(builder: ir.IRBuilder, module: ir.Module, field_name: str) -> ir.Value:
    i8 = ir.IntType(8)
    i32 = ir.IntType(32)
    text = str(field_name or "")
    digest = hashlib.sha1(text.encode("utf-8")).hexdigest()[:12]
    global_name = f".field_lit_{digest}"
    data = (text + "\0").encode("utf-8")
    arr_ty = ir.ArrayType(i8, len(data))

    existing = None
    for global_value in module.global_values:
        if global_value.name == global_name:
            existing = global_value
            break

    if existing is None:
        global_var = ir.GlobalVariable(module, arr_ty, name=global_name)
        global_var.linkage = "private"
        global_var.global_constant = True
        global_var.initializer = ir.Constant(arr_ty, bytearray(data))
    else:
        global_var = existing

    c0 = ir.Constant(i32, 0)
    return builder.gep(global_var, [c0, c0], inbounds=True)


def _canonical_i64(builder: ir.IRBuilder, value, *, name_hint: str):
    i64 = ir.IntType(64)
    if value is None:
        return ir.Constant(i64, 0)
    try:
        vtype = value.type
    except _SAFE_USER_BOX_LOCAL_EXC:
        vtype = None
    if isinstance(vtype, ir.PointerType):
        return builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    if isinstance(vtype, ir.IntType):
        if vtype.width < 64:
            return builder.zext(value, i64, name=f"{name_hint}_zext")
        if vtype.width > 64:
            return builder.trunc(value, i64, name=f"{name_hint}_trunc")
    return value


def _canonical_bool_i1(builder: ir.IRBuilder, value, *, name_hint: str):
    i1 = ir.IntType(1)
    if value is None:
        return ir.Constant(i1, 0)
    try:
        vtype = value.type
    except _SAFE_USER_BOX_LOCAL_EXC:
        vtype = None
    if isinstance(vtype, ir.IntType):
        if vtype.width == 1:
            return value
        zero = ir.Constant(vtype, 0)
        return builder.icmp_unsigned("!=", value, zero, name=f"{name_hint}_i1")
    if isinstance(vtype, ir.PointerType):
        i64 = ir.IntType(64)
        as_i64 = builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
        return builder.icmp_unsigned("!=", as_i64, ir.Constant(i64, 0), name=f"{name_hint}_i1")
    i64 = ir.IntType(64)
    as_i64 = _canonical_i64(builder, value, name_hint=f"{name_hint}_coerce")
    return builder.icmp_unsigned("!=", as_i64, ir.Constant(i64, 0), name=f"{name_hint}_i1")


def _canonical_bool_i64(builder: ir.IRBuilder, value, *, name_hint: str):
    i64 = ir.IntType(64)
    if value is None:
        return ir.Constant(i64, 0)
    bool_i1 = _canonical_bool_i1(builder, value, name_hint=name_hint)
    return builder.zext(bool_i1, i64, name=f"{name_hint}_i64")


def _canonical_f64(builder: ir.IRBuilder, value, *, name_hint: str):
    f64 = ir.DoubleType()
    if value is None:
        return ir.Constant(f64, 0.0)
    try:
        vtype = value.type
    except _SAFE_USER_BOX_LOCAL_EXC:
        vtype = None
    if isinstance(vtype, ir.DoubleType):
        return value
    raise RuntimeError(
        f"[user_box_local] expected f64 for {name_hint}, got {vtype}"
    )


def _new_user_box_handle(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_type: str,
    *,
    name_hint: str,
) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    new_i64x = _declare(
        module,
        "nyash.env.box.new_i64x",
        i64,
        [i8p, i64, i64, i64, i64, i64],
    )

    type_bytes = (box_type + "\0").encode("utf-8")
    arr_ty = ir.ArrayType(ir.IntType(8), len(type_bytes))
    global_name = f".user_box_ty_{hashlib.sha1(box_type.encode('utf-8')).hexdigest()[:12]}"
    existing = None
    for global_value in module.global_values:
        if global_value.name == global_name:
            existing = global_value
            break
    if existing is None:
        global_var = ir.GlobalVariable(module, arr_ty, name=global_name)
        global_var.linkage = "private"
        global_var.global_constant = True
        global_var.initializer = ir.Constant(arr_ty, bytearray(type_bytes))
    else:
        global_var = existing

    c0 = ir.Constant(ir.IntType(32), 0)
    ptr = builder.gep(global_var, [c0, c0], inbounds=True)
    zero = ir.Constant(i64, 0)
    return builder.call(
        new_i64x,
        [ptr, zero, zero, zero, zero, zero],
        name=f"{name_hint}_newbox",
    )


def _set_i64_field(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    field_name: str,
    value,
    *,
    name_hint: str,
) -> None:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    callee = _declare(module, "nyash.instance.set_i64_field_h", i64, [i64, i8p, i64])
    builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name), _canonical_i64(builder, value, name_hint=name_hint)],
        name=f"{name_hint}_set_i64",
    )


def _set_bool_field(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    field_name: str,
    value,
    *,
    name_hint: str,
) -> None:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    callee = _declare(module, "nyash.instance.set_bool_field_h", i64, [i64, i8p, i64])
    builder.call(
        callee,
        [
            recv_h,
            _field_ptr(builder, module, field_name),
            _canonical_bool_i64(builder, value, name_hint=name_hint),
        ],
        name=f"{name_hint}_set_bool",
    )


def _set_float_field(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    field_name: str,
    value,
    *,
    name_hint: str,
) -> None:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    f64 = ir.DoubleType()
    callee = _declare(module, "nyash.instance.set_float_field_h", i64, [i64, i8p, f64])
    builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name), _canonical_f64(builder, value, name_hint=name_hint)],
        name=f"{name_hint}_set_float",
    )


def _layout_store(resolver) -> Dict[int, Dict[str, Any]]:
    layouts = getattr(resolver, "user_box_local_aggregate_layouts", None)
    if isinstance(layouts, dict):
        return layouts
    layouts = {}
    setattr(resolver, "user_box_local_aggregate_layouts", layouts)
    return layouts


def _is_local_user_box_aggregate(value: Any) -> bool:
    return isinstance(value, dict) and value.get("kind") == "local_user_box_aggregate"


def uses_local_user_box_aggregate(resolver, value_id: Optional[int], box_type: Optional[str] = None) -> bool:
    if not isinstance(value_id, int):
        return False
    layout = _layout_store(resolver).get(int(value_id))
    if not isinstance(layout, dict):
        return False
    if isinstance(box_type, str) and layout.get("box_name") != box_type:
        return False
    return True


def build_local_user_box_aggregate_for_newbox(
    resolver,
    dst_vid: Optional[int],
    box_type: Optional[str],
):
    if not isinstance(dst_vid, int) or not isinstance(box_type, str):
        return None
    layout = _layout_store(resolver).get(int(dst_vid))
    if not isinstance(layout, dict) or layout.get("box_name") != box_type:
        return None
    return {
        "kind": "local_user_box_aggregate",
        "box_name": box_type,
        "field_order": list(layout.get("field_order", [])),
        "field_layouts": dict(layout.get("field_layouts", {})),
        "fields": {
            field_name: _UNSET_LOCAL_FIELD
            for field_name in layout.get("field_order", [])
        },
    }


def _resolve_local_user_box_aggregate(value_vid: int, vmap: Dict[int, Any], resolver):
    direct = vmap.get(int(value_vid))
    if _is_local_user_box_aggregate(direct):
        return direct

    try:
        global_vmap = getattr(resolver, "global_vmap", None)
        if isinstance(global_vmap, dict):
            global_value = global_vmap.get(int(value_vid))
            if _is_local_user_box_aggregate(global_value):
                return global_value
    except _SAFE_USER_BOX_LOCAL_EXC:
        pass

    try:
        current_bid = getattr(resolver, "current_block_id", None)
        ctx = getattr(resolver, "context", None)
        if current_bid is not None and ctx is not None and hasattr(ctx, "get_block_snapshot"):
            snapshot = ctx.get_block_snapshot(int(current_bid))
            snap_value = snapshot.get(int(value_vid))
            if _is_local_user_box_aggregate(snap_value):
                return snap_value
    except _SAFE_USER_BOX_LOCAL_EXC:
        pass

    return None


def _copy_local_user_box_metadata_alias(resolver, src_vid: int, dst_vid: int) -> None:
    if resolver is None:
        return
    layouts = _layout_store(resolver)
    if int(src_vid) in layouts:
        layouts[int(dst_vid)] = layouts[int(src_vid)]


def _resolve_i64_value(
    builder: ir.IRBuilder,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
    *,
    hot_scope: str,
):
    if not isinstance(value_vid, int):
        return ir.Constant(ir.IntType(64), 0)
    return resolve_i64_strict(
        resolver,
        int(value_vid),
        builder.block,
        preds,
        block_end_values,
        vmap,
        bb_map,
        hot_scope=hot_scope,
    )


def _resolve_f64_value(
    builder: ir.IRBuilder,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
):
    if not isinstance(value_vid, int):
        return ir.Constant(ir.DoubleType(), 0.0)
    value = vmap.get(int(value_vid))
    if value is None:
        value = _resolve_i64_value(
            builder,
            int(value_vid),
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
            hot_scope="user_box_field_float",
        )
    value_meta = resolver_value_type(resolver, int(value_vid))
    value = unbox_primitive_handle_if_needed(
        builder,
        value,
        value_meta,
        name_hint=f"user_box_local_float_{value_vid}",
    )
    return _canonical_f64(builder, value, name_hint=f"user_box_local_float_{value_vid}")


def lower_local_user_box_field_get(
    builder: ir.IRBuilder,
    box_vid: Optional[int],
    field_name: str,
    dst_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    mark_integer,
    mark_bool,
    mark_float,
):
    if not isinstance(box_vid, int):
        return None
    local_box = _resolve_local_user_box_aggregate(int(box_vid), vmap, resolver)
    if local_box is None:
        return None

    field_layouts = local_box.get("field_layouts", {})
    if field_name not in field_layouts:
        raise RuntimeError(
            f"[user_box_local] missing field layout for {local_box.get('box_name')}.{field_name}"
        )

    stored = local_box.get("fields", {}).get(field_name, _UNSET_LOCAL_FIELD)
    if stored is _UNSET_LOCAL_FIELD:
        raise RuntimeError(
            f"[user_box_local] local aggregate read before initialization: {local_box.get('box_name')}.{field_name}"
        )

    layout_name = field_layouts[field_name]
    if layout_name == "inline_i64":
        result = _canonical_i64(builder, stored, name_hint=f"user_box_local_get_{field_name}")
        if dst_vid is not None:
            vmap[int(dst_vid)] = result
            mark_integer(resolver, int(dst_vid))
        return result
    if layout_name == "inline_bool":
        result = _canonical_bool_i1(builder, stored, name_hint=f"user_box_local_get_{field_name}")
        if dst_vid is not None:
            vmap[int(dst_vid)] = result
            mark_bool(resolver, int(dst_vid))
        return result
    if layout_name == "inline_f64":
        result = _canonical_f64(builder, stored, name_hint=f"user_box_local_get_{field_name}")
        if dst_vid is not None:
            vmap[int(dst_vid)] = result
            mark_float(resolver, int(dst_vid))
        return result
    raise RuntimeError(f"[user_box_local] unsupported local field layout: {layout_name}")


def lower_local_user_box_field_set(
    builder: ir.IRBuilder,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
):
    if not isinstance(box_vid, int):
        return False
    local_box = _resolve_local_user_box_aggregate(int(box_vid), vmap, resolver)
    if local_box is None:
        return False

    field_layouts = local_box.get("field_layouts", {})
    if field_name not in field_layouts:
        raise RuntimeError(
            f"[user_box_local] missing field layout for {local_box.get('box_name')}.{field_name}"
        )

    layout_name = field_layouts[field_name]
    if layout_name == "inline_i64":
        raw_value = _resolve_i64_value(
            builder,
            value_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
            hot_scope="user_box_field_i64",
        )
        value_meta = resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
        raw_value = unbox_primitive_handle_if_needed(
            builder,
            _canonical_i64(builder, raw_value, name_hint=f"user_box_local_set_{field_name}"),
            value_meta,
            name_hint=f"user_box_local_set_i64_{value_vid}",
        )
        local_box["fields"][field_name] = _canonical_i64(
            builder,
            raw_value,
            name_hint=f"user_box_local_store_{field_name}",
        )
        return True

    if layout_name == "inline_bool":
        raw_value = vmap.get(int(value_vid)) if isinstance(value_vid, int) else None
        if raw_value is None:
            raw_value = _resolve_i64_value(
                builder,
                value_vid,
                vmap,
                resolver,
                preds,
                block_end_values,
                bb_map,
                hot_scope="user_box_field_bool",
            )
        value_meta = resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
        raw_value = unbox_primitive_handle_if_needed(
            builder,
            raw_value,
            value_meta,
            name_hint=f"user_box_local_set_bool_{value_vid}",
        )
        local_box["fields"][field_name] = _canonical_bool_i1(
            builder,
            raw_value,
            name_hint=f"user_box_local_store_{field_name}",
        )
        return True

    if layout_name == "inline_f64":
        local_box["fields"][field_name] = _resolve_f64_value(
            builder,
            value_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
        return True

    raise RuntimeError(f"[user_box_local] unsupported local field layout: {layout_name}")


def materialize_local_user_box_aggregate(
    builder: ir.IRBuilder,
    module: ir.Module,
    local_box,
    *,
    name_hint: str,
):
    if not _is_local_user_box_aggregate(local_box):
        return local_box
    box_name = local_box.get("box_name")
    if not isinstance(box_name, str) or not box_name:
        raise RuntimeError("[user_box_local] local aggregate is missing box_name")

    recv_h = _new_user_box_handle(builder, module, box_name, name_hint=name_hint)
    field_layouts = local_box.get("field_layouts", {})
    fields = local_box.get("fields", {})
    for field_name in local_box.get("field_order", []):
        value = fields.get(field_name, _UNSET_LOCAL_FIELD)
        if value is _UNSET_LOCAL_FIELD:
            raise RuntimeError(
                f"[user_box_local] attempted to materialize {box_name}.{field_name} before initialization"
            )
        layout_name = field_layouts.get(field_name)
        if layout_name == "inline_i64":
            _set_i64_field(
                builder,
                module,
                recv_h,
                field_name,
                value,
                name_hint=f"{name_hint}_{field_name}",
            )
        elif layout_name == "inline_bool":
            _set_bool_field(
                builder,
                module,
                recv_h,
                field_name,
                value,
                name_hint=f"{name_hint}_{field_name}",
            )
        elif layout_name == "inline_f64":
            _set_float_field(
                builder,
                module,
                recv_h,
                field_name,
                value,
                name_hint=f"{name_hint}_{field_name}",
            )
        else:
            raise RuntimeError(
                f"[user_box_local] unsupported local field layout during materialization: {layout_name}"
            )
    return recv_h


def materialize_user_box_escape_value_if_needed(
    builder: ir.IRBuilder,
    module: ir.Module,
    value_id: Optional[int],
    vmap: Dict[int, Any],
    resolver=None,
    *,
    name_hint: str = "user_box_escape",
):
    if not isinstance(value_id, int):
        return None
    local_box = _resolve_local_user_box_aggregate(int(value_id), vmap, resolver)
    if local_box is None:
        return None
    return materialize_local_user_box_aggregate(
        builder,
        module,
        local_box,
        name_hint=name_hint,
    )

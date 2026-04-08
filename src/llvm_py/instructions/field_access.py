from typing import Any, Dict, Optional
import hashlib

import llvmlite.ir as ir

from instructions.mir_call.runtime_data_dispatch import lower_runtime_data_field_call
from utils.resolver_helpers import mark_as_handle
from utils.values import resolve_i64_strict


def _declare(module: ir.Module, name: str, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


def _ensure_handle(builder: ir.IRBuilder, module: ir.Module, value: ir.Value) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    if hasattr(value, "type"):
        if isinstance(value.type, ir.IntType) and value.type.width == 64:
            return value
        if isinstance(value.type, ir.PointerType):
            callee = _declare(module, "nyash.box.from_i8_string", i64, [i8p])
            return builder.call(callee, [value], name="field_ptr2h")
        if isinstance(value.type, ir.IntType):
            return (
                builder.zext(value, i64)
                if value.type.width < 64
                else builder.trunc(value, i64)
            )
    return ir.Constant(i64, 0)


def _field_ptr(builder: ir.IRBuilder, module: ir.Module, field_name: str) -> ir.Value:
    i8 = ir.IntType(8)
    i32 = ir.IntType(32)
    text = str(field_name or "")
    digest = hashlib.sha1(text.encode("utf-8")).hexdigest()[:12]
    global_name = f".field_lit_{digest}"
    data = (text + "\0").encode("utf-8")
    arr_ty = ir.ArrayType(i8, len(data))

    existing = None
    for g in module.global_values:
        if g.name == global_name:
            existing = g
            break

    if existing is None:
        g = ir.GlobalVariable(module, arr_ty, name=global_name)
        g.linkage = "private"
        g.global_constant = True
        g.initializer = ir.Constant(arr_ty, bytearray(data))
    else:
        g = existing

    c0 = ir.Constant(i32, 0)
    return builder.gep(g, [c0, c0], inbounds=True)


def _boxed_field_key(builder: ir.IRBuilder, module: ir.Module, field_name: str) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    callee = _declare(module, "nyash.box.from_i8_string", i64, [i8p])
    return builder.call(
        callee,
        [_field_ptr(builder, module, field_name)],
        name="field_name_h",
    )


def _resolve_receiver(
    builder: ir.IRBuilder,
    box_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> ir.Value:
    i64 = ir.IntType(64)
    if not isinstance(box_vid, int):
        return ir.Constant(i64, 0)
    return resolve_i64_strict(
        resolver,
        box_vid,
        builder.block,
        preds,
        block_end_values,
        vmap,
        bb_map,
        hot_scope="field",
    )


def lower_field_get(
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
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    key_h = _boxed_field_key(builder, module, field_name)
    result = lower_runtime_data_field_call(
        builder=builder,
        declare=lambda name, ret, args: _declare(module, name, ret, args),
        box_name="RuntimeDataBox",
        method="getField",
        recv_h=recv_h,
        args=[key_h],
        resolve_arg=None,
        ensure_handle=None,
    )
    if result is None:
        result = ir.Constant(i64, 0)
    if dst_vid is not None:
        vmap[dst_vid] = result
        mark_as_handle(resolver, int(dst_vid))
    return result


def lower_field_set(
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
    recv_val = _resolve_receiver(
        builder, box_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    recv_h = _ensure_handle(builder, module, recv_val)
    key_h = _boxed_field_key(builder, module, field_name)
    value_val = _resolve_receiver(
        builder, value_vid, vmap, resolver, preds, block_end_values, bb_map
    )
    value_h = _ensure_handle(builder, module, value_val)
    result = lower_runtime_data_field_call(
        builder=builder,
        declare=lambda name, ret, args: _declare(module, name, ret, args),
        box_name="RuntimeDataBox",
        method="setField",
        recv_h=recv_h,
        args=[key_h, value_h],
        resolve_arg=None,
        ensure_handle=None,
    )
    if result is None:
        result = ir.Constant(i64, 0)
    return result

from typing import Any, Dict, Optional
import hashlib
import os
import sys

import llvmlite.ir as ir

from instructions.llvm_decl import declare_function as _declare
from instructions.mir_call.runtime_data_dispatch import lower_runtime_data_field_call
from instructions.primitive_handles import resolver_value_type, unbox_primitive_handle_if_needed
from instructions.thin_entry_selection import thin_entry_prefers_inline_scalar_field
from instructions.typed_object_exact import (
    exact_field_plan_for_box,
    is_handle_storage,
    is_signed_storage,
    is_unsigned_storage,
)
from instructions.user_box_local import (
    lower_local_user_box_field_get,
    lower_local_user_box_field_set,
)
from instructions.typeop import _emit_trap
from type_facts import is_box_handle_fact
from utils.resolver_helpers import mark_as_handle
from utils.values import resolve_i64_strict

DIRECT_SLOT_NATIVEDIRECT_SELECTED_METHOD = "HakoAllocPageModel.acquire_usize/1"
DIRECT_SLOT_OBJECT_HEADER_BYTES = 24
DIRECT_SLOT_CELL_BYTES = 16
DIRECT_SLOT_CELL_PAYLOAD_OFFSET_BYTES = 8


def _mark_integer_immediate(resolver, vid: int) -> None:
    try:
        if not hasattr(resolver, "value_types") or not isinstance(resolver.value_types, dict):
            resolver.value_types = {}
        resolver.value_types[int(vid)] = "i64"
    except Exception:
        pass
    try:
        integerish_ids = getattr(resolver, "integerish_ids", None)
        if isinstance(integerish_ids, set):
            integerish_ids.add(int(vid))
    except Exception:
        pass


def _mark_bool_immediate(resolver, vid: int) -> None:
    try:
        if not hasattr(resolver, "value_types") or not isinstance(resolver.value_types, dict):
            resolver.value_types = {}
        resolver.value_types[int(vid)] = "i1"
    except Exception:
        pass


def _mark_float_immediate(resolver, vid: int) -> None:
    try:
        if not hasattr(resolver, "value_types") or not isinstance(resolver.value_types, dict):
            resolver.value_types = {}
        resolver.value_types[int(vid)] = "Float"
    except Exception:
        pass


def _exact_slot_helper_enabled() -> bool:
    return (
        os.environ.get("HAKO_TYPED_OBJECT_STORE") == "single_thread_exact"
        and os.environ.get("HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER") == "1"
    )


def _is_exact_slot_u64_storage(storage: Any) -> bool:
    normalized = str(storage).strip().lower() if isinstance(storage, str) else None
    return normalized == "u64" or (
        normalized == "usize" and sys.maxsize > 2**32
    )


def _current_function_name(builder: ir.IRBuilder) -> Optional[str]:
    block = getattr(builder, "block", None)
    function = getattr(block, "function", None)
    name = getattr(function, "name", None)
    return str(name) if name is not None else None


def _direct_slot_nativedirect_selected(builder: ir.IRBuilder) -> bool:
    return (
        os.environ.get("HAKO_TYPED_OBJECT_STORE") == "direct_slot_exact"
        and _current_function_name(builder) == DIRECT_SLOT_NATIVEDIRECT_SELECTED_METHOD
    )


def _direct_slot_nativedirect_storage_supported(storage: Any) -> bool:
    return (
        _is_exact_slot_u64_storage(storage)
        or storage == "i64"
        or is_handle_storage(storage)
    )


def _direct_slot_payload_ptr(
    builder: ir.IRBuilder,
    recv_h: ir.Value,
    slot: int,
) -> ir.Value:
    i64 = ir.IntType(64)
    if slot < 0:
        raise RuntimeError("[direct-slot/nativedirect] negative field slot")
    object_base = builder.and_(
        recv_h,
        ir.Constant(i64, -2),
        name="direct_slot_object_base",
    )
    payload_offset = (
        DIRECT_SLOT_OBJECT_HEADER_BYTES
        + slot * DIRECT_SLOT_CELL_BYTES
        + DIRECT_SLOT_CELL_PAYLOAD_OFFSET_BYTES
    )
    payload_addr = builder.add(
        object_base,
        ir.Constant(i64, payload_offset),
        name="direct_slot_payload_addr",
    )
    return builder.inttoptr(
        payload_addr,
        i64.as_pointer(),
        name="direct_slot_payload_ptr",
    )


def _lower_direct_slot_nativedirect_get(
    builder: ir.IRBuilder,
    recv_h: ir.Value,
    slot: int,
    storage: Any,
    dst_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
) -> Optional[ir.Value]:
    if not _direct_slot_nativedirect_selected(builder):
        return None
    if not _direct_slot_nativedirect_storage_supported(storage):
        raise RuntimeError(
            f"[direct-slot/nativedirect] unsupported storage in selected method: {storage}"
        )
    ptr = _direct_slot_payload_ptr(builder, recv_h, slot)
    result = builder.load(ptr, name="direct_slot_payload_load")
    if dst_vid is not None:
        vmap[int(dst_vid)] = result
        if is_handle_storage(storage):
            mark_as_handle(resolver, int(dst_vid))
        else:
            _mark_integer_immediate(resolver, int(dst_vid))
    return result


def _lower_direct_slot_nativedirect_set(
    builder: ir.IRBuilder,
    recv_h: ir.Value,
    slot: int,
    storage: Any,
    value_val: ir.Value,
) -> Optional[ir.Value]:
    if not _direct_slot_nativedirect_selected(builder):
        return None
    if not _direct_slot_nativedirect_storage_supported(storage):
        raise RuntimeError(
            f"[direct-slot/nativedirect] unsupported storage in selected method: {storage}"
        )
    i64 = ir.IntType(64)
    ptr = _direct_slot_payload_ptr(builder, recv_h, slot)
    builder.store(value_val, ptr)
    return ir.Constant(i64, 1)


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


def _canonical_i64(builder: ir.IRBuilder, value, *, name_hint: str):
    i64 = ir.IntType(64)
    if value is None:
        return ir.Constant(i64, 0)
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

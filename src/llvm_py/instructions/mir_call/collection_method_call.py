"""
Shared collection-style method lowering for MIR call lowerers.

This module owns the common `get/push/set/has` route order shared by
`method_call.py` and `mir_call_legacy.py`.
"""

from typing import Callable, List, Optional

from llvmlite import ir

from .auto_specialize import (
    prefer_array_i64_key_i64_value_route,
    prefer_array_i64_key_route,
)
from .runtime_data_dispatch import lower_runtime_data_method_call


def _resolve_or_zero(
    resolve_arg: Callable[[int], Optional[ir.Value]], arg_ids: List[int], index: int, zero
):
    if index >= len(arg_ids):
        return zero
    return resolve_arg(arg_ids[index]) or zero


def _lower_array_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)

    if method_name == "get":
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        if not arg_ids:
            return zero
        if prefer_array_i64_key_route(method_name, resolver, arg_ids):
            callee = declare("nyash.array.slot_load_hi", i64, [i64, i64])
            return builder.call(callee, [recv_h, key], name="unified_array_slot_load_hi")
        callee = declare("nyash.runtime_data.get_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name="unified_runtime_data_get_hh")

    if method_name == "push":
        value = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        if not arg_ids:
            return recv_h
        callee = declare("nyash.array.slot_append_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, value], name="unified_array_slot_append_hh")

    if method_name == "set":
        if len(arg_ids) < 2:
            return recv_h
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        value = _resolve_or_zero(resolve_arg, arg_ids, 1, zero)
        if prefer_array_i64_key_route(method_name, resolver, arg_ids):
            if prefer_array_i64_key_i64_value_route(method_name, resolver, arg_ids):
                callee = declare("nyash.array.set_hii", i64, [i64, i64, i64])
                return builder.call(callee, [recv_h, key, value], name="unified_array_set_hii")
            callee = declare("nyash.array.set_hih", i64, [i64, i64, i64])
            return builder.call(callee, [recv_h, key, value], name="unified_array_set_hih")
        callee = declare("nyash.runtime_data.set_hhh", i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, key, value], name="unified_runtime_data_set_hhh")

    if method_name == "has":
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        if not arg_ids:
            return zero
        callee = declare("nyash.runtime_data.has_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name="unified_runtime_data_has_hh")

    return None


def _lower_map_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)

    if method_name == "get":
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        if not arg_ids:
            return zero
        callee = declare("nyash.map.slot_load_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name="unified_map_slot_load_hh")

    if method_name == "push":
        value = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        if not arg_ids:
            return recv_h
        callee = declare("nyash.array.slot_append_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, value], name="unified_array_slot_append_hh")

    if method_name == "set":
        if len(arg_ids) < 2:
            return recv_h
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        value = _resolve_or_zero(resolve_arg, arg_ids, 1, zero)
        callee = declare("nyash.map.slot_store_hhh", i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, key, value], name="unified_map_slot_store_hhh")

    if method_name == "has":
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        if not arg_ids:
            return zero
        callee = declare("nyash.map.probe_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name="unified_map_probe_hh")

    return None


def lower_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    box_name,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    resolver=None,
    receiver_vid=None,
    prefer_array_mono_route=None,
):
    runtime_result = lower_runtime_data_method_call(
        builder=builder,
        declare=declare,
        box_name=box_name,
        method=method_name,
        recv_h=recv_h,
        args=[resolve_arg(arg_id) for arg_id in arg_ids],
        resolver=resolver,
        receiver_vid=receiver_vid,
        arg_vids=arg_ids,
        prefer_array_mono_route=prefer_array_mono_route,
    )
    if runtime_result is not None:
        return runtime_result

    if str(box_name or "") == "ArrayBox":
        return _lower_array_collection_method_call(
            builder=builder,
            declare=declare,
            method_name=method_name,
            recv_h=recv_h,
            arg_ids=arg_ids,
            resolve_arg=resolve_arg,
            resolver=resolver,
        )

    return _lower_map_collection_method_call(
        builder=builder,
        declare=declare,
        method_name=method_name,
        recv_h=recv_h,
        arg_ids=arg_ids,
        resolve_arg=resolve_arg,
    )

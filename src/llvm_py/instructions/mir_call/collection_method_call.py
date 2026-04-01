"""
Shared collection-style method lowering for MIR call lowerers.

This module owns the common `get/push/set/has/clear` route order shared by
`method_call.py` and `mir_call_legacy.py`.
"""

from typing import Callable, List, Optional

from llvmlite import ir

from .runtime_data_dispatch import (
    lower_runtime_data_method_call,
    select_array_collection_call_spec,
)


def _resolve_or_zero(
    resolve_arg: Callable[[int], Optional[ir.Value]], arg_ids: List[int], index: int, zero
):
    if index >= len(arg_ids):
        return zero
    return resolve_arg(arg_ids[index]) or zero


def _lower_call_spec(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    spec,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)
    symbol, call_name, arity = spec
    if arity == 1:
        if not arg_ids:
            return zero
        arg0 = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        callee = declare(symbol, i64, [i64, i64])
        return builder.call(callee, [recv_h, arg0], name=call_name)
    if arity == 2:
        if len(arg_ids) < 2:
            return recv_h
        arg0 = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        arg1 = _resolve_or_zero(resolve_arg, arg_ids, 1, zero)
        callee = declare(symbol, i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, arg0, arg1], name=call_name)
    return None


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

    # Preserve the existing fail-safe return shape for missing arguments.
    if method_name in ("get", "has") and not arg_ids:
        return zero
    if method_name in ("push", "set") and (
        (method_name == "push" and not arg_ids) or (method_name == "set" and len(arg_ids) < 2)
    ):
        return recv_h

    # Keep ArrayBox and RuntimeDataBox(array-specialized) on the same canonical
    # RawArray symbol table so lowering truth cannot drift across entrypoints.
    spec = select_array_collection_call_spec(
        method=method_name,
        resolver=resolver,
        arg_vids=arg_ids,
    )
    if spec is None:
        return None
    return _lower_call_spec(
        builder=builder,
        declare=declare,
        spec=spec,
        recv_h=recv_h,
        arg_ids=arg_ids,
        resolve_arg=resolve_arg,
    )


def _lower_map_collection_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    box_name,
    method_name: str,
    recv_h,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
):
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)

    if method_name == "clear":
        if str(box_name or "") != "MapBox":
            return None
        if arg_ids:
            return zero
        callee = declare("nyash.map.clear_h", i64, [i64])
        return builder.call(callee, [recv_h], name="unified_map_clear_h")

    if method_name == "delete":
        if str(box_name or "") != "MapBox":
            return None
        if len(arg_ids) < 1:
            return zero
        key = _resolve_or_zero(resolve_arg, arg_ids, 0, zero)
        callee = declare("nyash.map.delete_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name="unified_map_delete_hh")

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
        box_name=box_name,
        method_name=method_name,
        recv_h=recv_h,
        arg_ids=arg_ids,
        resolve_arg=resolve_arg,
    )

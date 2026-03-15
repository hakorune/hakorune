"""
Shared collection-style method lowering for MIR call lowerers.

This module owns the common `get/push/set/has` route order shared by
`method_call.py` and `mir_call_legacy.py`.
"""

from typing import Callable, List, Optional

from llvmlite import ir

from .runtime_data_dispatch import lower_runtime_data_method_call


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
    i64 = ir.IntType(64)
    zero = ir.Constant(i64, 0)

    if method_name == "get":
        if not arg_ids:
            return zero
        key = resolve_arg(arg_ids[0]) or zero
        runtime_result = lower_runtime_data_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            method=method_name,
            recv_h=recv_h,
            args=[key],
            resolver=resolver,
            receiver_vid=receiver_vid,
            arg_vids=arg_ids,
            prefer_array_mono_route=prefer_array_mono_route,
        )
        if runtime_result is not None:
            return runtime_result
        callee = declare("nyash.map.get_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name="unified_map_get")

    if method_name == "push":
        if not arg_ids:
            return recv_h
        value = resolve_arg(arg_ids[0]) or zero
        runtime_result = lower_runtime_data_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            method=method_name,
            recv_h=recv_h,
            args=[value],
            resolver=resolver,
            receiver_vid=receiver_vid,
            arg_vids=arg_ids,
            prefer_array_mono_route=prefer_array_mono_route,
        )
        if runtime_result is not None:
            return runtime_result
        callee = declare("nyash.array.push_h", i64, [i64, i64])
        return builder.call(callee, [recv_h, value], name="unified_array_push")

    if method_name == "set":
        if len(arg_ids) < 2:
            return recv_h
        key = resolve_arg(arg_ids[0]) or zero
        value = resolve_arg(arg_ids[1]) or zero
        runtime_result = lower_runtime_data_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            method=method_name,
            recv_h=recv_h,
            args=[key, value],
            resolver=resolver,
            receiver_vid=receiver_vid,
            arg_vids=arg_ids,
            prefer_array_mono_route=prefer_array_mono_route,
        )
        if runtime_result is not None:
            return runtime_result
        callee = declare("nyash.map.set_hh", i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, key, value], name="unified_map_set")

    if method_name == "has":
        if not arg_ids:
            return zero
        key = resolve_arg(arg_ids[0]) or zero
        runtime_result = lower_runtime_data_method_call(
            builder=builder,
            declare=declare,
            box_name=box_name,
            method=method_name,
            recv_h=recv_h,
            args=[key],
            resolver=resolver,
            receiver_vid=receiver_vid,
            arg_vids=arg_ids,
            prefer_array_mono_route=prefer_array_mono_route,
        )
        if runtime_result is not None:
            return runtime_result
        callee = declare("nyash.map.has_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, key], name="unified_map_has")

    return None

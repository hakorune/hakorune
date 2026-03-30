"""
RuntimeData/collection method helpers for BoxCall lowering.

This module owns the ArrayBox/MapBox/RuntimeData-like collection routes used by
the generic BoxCall path so `boxcall.py` can stay focused on route order.
"""

from typing import Callable, Optional

from llvmlite import ir

from instructions.mir_call.auto_specialize import (
    prefer_runtime_data_array_i64_key_i64_value_route,
    prefer_runtime_data_array_i64_key_route,
    receiver_is_arrayish,
    receiver_is_mapish,
    receiver_is_stringish,
)
from utils.resolver_helpers import get_box_type


def try_lower_collection_boxcall(
    *,
    builder,
    module,
    method_name: str,
    recv_val,
    box_vid: int,
    args,
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    declare: Callable,
    resolver=None,
):
    i64 = ir.IntType(64)
    recv_h = ensure_handle(recv_val)

    if method_name == "size":
        known_box_name = get_box_type(resolver, box_vid)
        if receiver_is_stringish(resolver, box_vid):
            callee = declare(module, "nyash.string.len_h", i64, [i64])
            return builder.call(callee, [recv_h], name="string_size_h")
        if receiver_is_arrayish(resolver, box_vid):
            callee = declare(module, "nyash.array.slot_len_h", i64, [i64])
            return builder.call(callee, [recv_h], name="array_size_h")
        if known_box_name == "MapBox" or receiver_is_mapish(resolver, box_vid):
            callee = declare(module, "nyash.map.entry_count_h", i64, [i64])
            return builder.call(callee, [recv_h], name="map_entry_count_h")
        callee = declare(module, "nyash.any.length_h", i64, [i64])
        return builder.call(callee, [recv_h], name="any_size_h")

    if method_name == "get":
        k = resolve_arg(args[0]) if args else ir.Constant(i64, 0)
        if k is None:
            k = ir.Constant(i64, 0)
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "ArrayBox" or receiver_is_arrayish(resolver, box_vid):
            if prefer_runtime_data_array_i64_key_route(
                method=method_name,
                resolver=resolver,
                arg_vids=args,
            ):
                callee = declare(module, "nyash.array.slot_load_hi", i64, [i64, i64])
                return builder.call(callee, [recv_h, k], name="array_slot_load_hi")
            callee = declare(module, "nyash.runtime_data.get_hh", i64, [i64, i64])
            return builder.call(callee, [recv_h, k], name="runtime_data_get_hh")
        callee = declare(module, "nyash.map.slot_load_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, k], name="map_slot_load_hh")

    if method_name == "push":
        v0 = resolve_arg(args[0]) if args else ir.Constant(i64, 0)
        if v0 is None:
            v0 = ir.Constant(i64, 0)
        callee = declare(module, "nyash.array.slot_append_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, v0], name="arr_slot_append_hh")

    if method_name == "set":
        k = resolve_arg(args[0]) if len(args) > 0 else ir.Constant(i64, 0)
        if k is None:
            k = ir.Constant(i64, 0)
        v = resolve_arg(args[1]) if len(args) > 1 else ir.Constant(i64, 0)
        if v is None:
            v = ir.Constant(i64, 0)
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "ArrayBox" or receiver_is_arrayish(resolver, box_vid):
            if prefer_runtime_data_array_i64_key_route(
                method=method_name,
                resolver=resolver,
                arg_vids=args,
            ):
                if prefer_runtime_data_array_i64_key_i64_value_route(
                    method=method_name,
                    resolver=resolver,
                    arg_vids=args,
                ):
                    callee = declare(module, "nyash.array.slot_store_hii", i64, [i64, i64, i64])
                    return builder.call(callee, [recv_h, k, v], name="array_slot_store_hii")
                callee = declare(module, "nyash.array.slot_store_hih", i64, [i64, i64, i64])
                return builder.call(callee, [recv_h, k, v], name="array_slot_store_hih")
            callee = declare(module, "nyash.runtime_data.set_hhh", i64, [i64, i64, i64])
            return builder.call(callee, [recv_h, k, v], name="runtime_data_set_hhh")
        callee = declare(module, "nyash.map.slot_store_hhh", i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, k, v], name="map_slot_store_hhh")

    if method_name == "has":
        k = resolve_arg(args[0]) if args else ir.Constant(i64, 0)
        if k is None:
            k = ir.Constant(i64, 0)
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "ArrayBox" or receiver_is_arrayish(resolver, box_vid):
            callee = declare(module, "nyash.runtime_data.has_hh", i64, [i64, i64])
            return builder.call(callee, [recv_h, k], name="runtime_data_has_hh")
        callee = declare(module, "nyash.map.probe_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, k], name="map_probe_hh")

    return None


def declare(module, name: str, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)

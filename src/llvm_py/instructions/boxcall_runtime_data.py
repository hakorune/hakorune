"""
RuntimeData/collection method helpers for BoxCall lowering.

This module owns the ArrayBox/MapBox/RuntimeData-like collection routes used by
the generic BoxCall path so `boxcall.py` can stay focused on route order.
"""

from typing import Callable, Optional

from llvmlite import ir

from instructions.mir_call.auto_specialize import (
    receiver_is_arrayish,
    receiver_is_mapish,
    receiver_is_stringish,
)
from instructions.mir_call.runtime_data_dispatch import select_array_collection_call_spec
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

    def _call_from_spec(spec):
        if spec is None:
            return None
        symbol, call_name, arity = spec
        if arity == 1:
            value = resolve_arg(args[0]) if args else ir.Constant(i64, 0)
            if value is None:
                value = ir.Constant(i64, 0)
            callee = declare(module, symbol, i64, [i64, i64])
            return builder.call(callee, [recv_h, value], name=call_name)
        if arity == 2:
            key = resolve_arg(args[0]) if len(args) > 0 else ir.Constant(i64, 0)
            if key is None:
                key = ir.Constant(i64, 0)
            value = resolve_arg(args[1]) if len(args) > 1 else ir.Constant(i64, 0)
            if value is None:
                value = ir.Constant(i64, 0)
            callee = declare(module, symbol, i64, [i64, i64, i64])
            return builder.call(callee, [recv_h, key, value], name=call_name)
        return None

    if method_name == "size":
        known_box_name = get_box_type(resolver, box_vid)
        if receiver_is_stringish(resolver, box_vid):
            callee = declare(module, "nyash.string.len_h", i64, [i64])
            return builder.call(callee, [recv_h], name="string_size_h")
        if receiver_is_arrayish(resolver, box_vid):
            callee = declare(module, "nyash.array.slot_len_h", i64, [i64])
            return builder.call(callee, [recv_h], name="array_size_h")
        if known_box_name == "MapBox" or receiver_is_mapish(resolver, box_vid):
            callee = declare(module, "nyash.map.entry_count_i64", i64, [i64])
            return builder.call(callee, [recv_h], name="map_entry_count_i64")
        callee = declare(module, "nyash.any.length_h", i64, [i64])
        return builder.call(callee, [recv_h], name="any_size_h")

    if method_name == "get":
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "ArrayBox" or receiver_is_arrayish(resolver, box_vid):
            return _call_from_spec(
                select_array_collection_call_spec(
                    method=method_name,
                    resolver=resolver,
                    arg_vids=args,
                )
            )
        k = resolve_arg(args[0]) if args else ir.Constant(i64, 0)
        if k is None:
            k = ir.Constant(i64, 0)
        callee = declare(module, "nyash.map.slot_load_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, k], name="map_slot_load_hh")

    if method_name == "push":
        return _call_from_spec(
            select_array_collection_call_spec(
                method=method_name,
                resolver=resolver,
                arg_vids=args,
            )
        )

    if method_name == "set":
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "ArrayBox" or receiver_is_arrayish(resolver, box_vid):
            return _call_from_spec(
                select_array_collection_call_spec(
                    method=method_name,
                    resolver=resolver,
                    arg_vids=args,
                )
            )
        k = resolve_arg(args[0]) if len(args) > 0 else ir.Constant(i64, 0)
        if k is None:
            k = ir.Constant(i64, 0)
        v = resolve_arg(args[1]) if len(args) > 1 else ir.Constant(i64, 0)
        if v is None:
            v = ir.Constant(i64, 0)
        callee = declare(module, "nyash.map.slot_store_hhh", i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, k, v], name="map_slot_store_hhh")

    if method_name == "has":
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "ArrayBox" or receiver_is_arrayish(resolver, box_vid):
            return _call_from_spec(
                select_array_collection_call_spec(
                    method=method_name,
                    resolver=resolver,
                    arg_vids=args,
                )
            )
        k = resolve_arg(args[0]) if args else ir.Constant(i64, 0)
        if k is None:
            k = ir.Constant(i64, 0)
        callee = declare(module, "nyash.map.probe_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, k], name="map_probe_hh")

    if method_name == "clear":
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "MapBox" or receiver_is_mapish(resolver, box_vid):
            if args:
                return ir.Constant(i64, 0)
            callee = declare(module, "nyash.map.clear_h", i64, [i64])
            return builder.call(callee, [recv_h], name="map_clear_h")

    if method_name == "delete":
        known_box_name = get_box_type(resolver, box_vid)
        if known_box_name == "MapBox" or receiver_is_mapish(resolver, box_vid):
            if not args:
                return ir.Constant(i64, 0)
            key = resolve_arg(args[0]) if len(args) > 0 else ir.Constant(i64, 0)
            if key is None:
                key = ir.Constant(i64, 0)
            callee = declare(module, "nyash.map.delete_hh", i64, [i64, i64])
            return builder.call(callee, [recv_h, key], name="map_delete_hh")

    return None


def declare(module, name: str, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)

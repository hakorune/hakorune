"""
Shared string/console method lowering for MIR call lowerers.

This module owns the common string search/slice route plus console/log
compat wrapper shared by `method_call.py` and `mir_call_legacy.py`.
"""

from typing import Callable, List, Optional

from llvmlite import ir

from .intrinsic_registry import lookup_intrinsic_spec


def lower_string_search_or_slice_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    method_name: Optional[str],
    recv_h,
    recv_ptr=None,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    needle_ptr_for_value: Optional[Callable[[int], Optional[ir.Value]]] = None,
    mark_receiver_stringish: Optional[Callable[[], None]] = None,
    box_string_ptr: Optional[Callable[[ir.Value], ir.Value]] = None,
    store_result_string_ptr: Optional[Callable[[ir.Value], None]] = None,
):
    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    zero = ir.Constant(i64, 0)

    spec = lookup_intrinsic_spec(method_name, len(arg_ids))
    if spec is None or spec.symbol not in (
        "nyash.string.substring_hii",
        "nyash.string.indexOf_hh",
        "nyash.string.lastIndexOf_hh",
    ):
        return None

    if mark_receiver_stringish is not None:
        mark_receiver_stringish()

    if spec.symbol == "nyash.string.substring_hii":
        start = resolve_arg(arg_ids[0]) or zero
        stop = resolve_arg(arg_ids[1]) or zero
        if recv_ptr is not None and box_string_ptr is not None:
            callee = declare("nyash.string.substring_sii", i8p, [i8p, i64, i64])
            out_ptr = builder.call(callee, [recv_ptr, start, stop], name="unified_substring_sii")
            if store_result_string_ptr is not None:
                store_result_string_ptr(out_ptr)
            return box_string_ptr(out_ptr)
        callee = declare(spec.symbol, i64, [i64, i64, i64])
        return builder.call(callee, [recv_h, start, stop], name="unified_substring")

    needle = resolve_arg(arg_ids[0]) or zero

    if recv_ptr is not None and needle_ptr_for_value is not None:
        needle_ptr = needle_ptr_for_value(arg_ids[0])
        if needle_ptr is not None:
            if spec.symbol == "nyash.string.indexOf_hh":
                callee = declare("nyash.string.indexOf_ss", i64, [i8p, i8p])
                return builder.call(callee, [recv_ptr, needle_ptr], name="unified_indexOf_ss")
            if spec.symbol == "nyash.string.lastIndexOf_hh":
                callee = declare("nyash.string.lastIndexOf_ss", i64, [i8p, i8p])
                return builder.call(callee, [recv_ptr, needle_ptr], name="unified_lastIndexOf_ss")

    needle_h = ensure_handle(needle)
    callee = declare(spec.symbol, i64, [i64, i64])
    call_name = "unified_indexOf" if spec.symbol == "nyash.string.indexOf_hh" else "unified_lastIndexOf"
    return builder.call(callee, [recv_h, needle_h], name=call_name)


def lower_string_or_console_method_call(
    *,
    builder: ir.IRBuilder,
    declare: Callable,
    method_name: Optional[str],
    recv_h,
    recv_ptr=None,
    arg_ids: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    needle_ptr_for_value: Optional[Callable[[int], Optional[ir.Value]]] = None,
    mark_receiver_stringish: Optional[Callable[[], None]] = None,
    box_string_ptr: Optional[Callable[[ir.Value], ir.Value]] = None,
    store_result_string_ptr: Optional[Callable[[ir.Value], None]] = None,
):
    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    zero = ir.Constant(i64, 0)

    result = lower_string_search_or_slice_method_call(
        builder=builder,
        declare=declare,
        method_name=method_name,
        recv_h=recv_h,
        recv_ptr=recv_ptr,
        arg_ids=arg_ids,
        resolve_arg=resolve_arg,
        ensure_handle=ensure_handle,
        needle_ptr_for_value=needle_ptr_for_value,
        mark_receiver_stringish=mark_receiver_stringish,
        box_string_ptr=box_string_ptr,
        store_result_string_ptr=store_result_string_ptr,
    )
    if result is not None:
        return result

    if method_name == "log":
        if arg_ids:
            arg0 = resolve_arg(arg_ids[0]) or zero
            if isinstance(arg0.type, ir.IntType) and arg0.type.width == 64:
                bridge = declare("nyash.string.to_i8p_h", i8p, [i64])
                ptr = builder.call(bridge, [arg0], name="unified_str_h2p")
                callee = declare("nyash.console.log", i64, [i8p])
                return builder.call(callee, [ptr], name="unified_console_log")
            callee = declare("nyash.console.log", i64, [i8p])
            return builder.call(callee, [arg0], name="unified_console_log")
        return zero

    return None

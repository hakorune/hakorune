"""
Shared string/console method lowering for MIR call lowerers.

This module owns the common `substring/indexOf/lastIndexOf/log` route order
shared by `method_call.py` and `mir_call_legacy.py`.
"""

from typing import Callable, List, Optional

from llvmlite import ir


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
    mark_receiver_stringish: Optional[Callable[[], None]] = None,
    box_string_ptr: Optional[Callable[[ir.Value], ir.Value]] = None,
    store_result_string_ptr: Optional[Callable[[ir.Value], None]] = None,
):
    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    zero = ir.Constant(i64, 0)

    def _mark_receiver():
        if mark_receiver_stringish is not None:
            mark_receiver_stringish()

    if method_name == "substring":
        if len(arg_ids) >= 2:
            _mark_receiver()
            start = resolve_arg(arg_ids[0]) or zero
            stop = resolve_arg(arg_ids[1]) or zero
            if recv_ptr is not None and box_string_ptr is not None:
                callee = declare("nyash.string.substring_sii", i8p, [i8p, i64, i64])
                out_ptr = builder.call(callee, [recv_ptr, start, stop], name="unified_substring_sii")
                if store_result_string_ptr is not None:
                    store_result_string_ptr(out_ptr)
                return box_string_ptr(out_ptr)
            callee = declare("nyash.string.substring_hii", i64, [i64, i64, i64])
            return builder.call(callee, [recv_h, start, stop], name="unified_substring")
        return recv_h

    if method_name == "lastIndexOf":
        if arg_ids:
            _mark_receiver()
            needle = resolve_arg(arg_ids[0]) or zero
            needle_h = ensure_handle(needle)
            callee = declare("nyash.string.lastIndexOf_hh", i64, [i64, i64])
            return builder.call(callee, [recv_h, needle_h], name="unified_lastIndexOf")
        return ir.Constant(i64, -1)

    if method_name == "indexOf" and len(arg_ids) == 1:
        _mark_receiver()
        needle = resolve_arg(arg_ids[0]) or zero
        needle_h = ensure_handle(needle)
        callee = declare("nyash.string.indexOf_hh", i64, [i64, i64])
        return builder.call(callee, [recv_h, needle_h], name="unified_indexOf")

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

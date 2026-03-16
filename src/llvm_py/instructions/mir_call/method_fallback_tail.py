"""
Shared method-call fallback tail for MIR call lowerers.

This module owns the final "direct known-box or generic by-name plugin"
dispatch tail so `method_call.py` and `mir_call_legacy.py` do not keep
duplicate route-order logic.

Contract:
- plugin fallback sees i64 handles for both receiver and args
- pointer/string args must be boxed before the generic plugin invoke entrypoint
"""

from typing import Callable, List, Optional

from llvmlite import ir

from instructions.by_name_method import lower_plugin_invoke_by_name
from instructions.direct_box_method import try_lower_known_box_method_call


def lower_direct_or_plugin_method_call(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    box_name: Optional[str],
    method_name: Optional[str],
    recv_h,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    direct_call_name: str,
    plugin_call_name: str,
    receiver_literal: Optional[str] = None,
):
    direct_result = try_lower_known_box_method_call(
        builder=builder,
        module=module,
        box_name=box_name,
        method_name=method_name,
        recv_h=recv_h,
        args=args,
        resolve_arg=resolve_arg,
        ensure_handle=ensure_handle,
        call_name=direct_call_name,
        receiver_literal=receiver_literal,
    )
    if direct_result is not None:
        return direct_result

    i64 = ir.IntType(64)
    argc = ir.Constant(i64, len(args))
    a1 = resolve_arg(args[0]) if args else ir.Constant(i64, 0)
    a2 = resolve_arg(args[1]) if len(args) > 1 else ir.Constant(i64, 0)
    if a1 is None:
        a1 = ir.Constant(i64, 0)
    else:
        a1 = ensure_handle(a1)
    if a2 is None:
        a2 = ir.Constant(i64, 0)
    else:
        a2 = ensure_handle(a2)
    return lower_plugin_invoke_by_name(
        builder=builder,
        module=module,
        recv_h=recv_h,
        method_name=method_name,
        argc_value=argc,
        arg1_value=a1,
        arg2_value=a2,
        call_name=plugin_call_name,
    )

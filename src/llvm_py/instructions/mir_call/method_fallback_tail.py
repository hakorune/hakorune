"""
Shared method-call fallback orchestrator for MIR call lowerers.

This module owns the direct-known-box check so `method_call.py` and
`mir_call_legacy.py` do not keep duplicate route-order logic. Unsupported
unknown method calls now fail fast instead of falling back to by-name.
"""

from typing import Callable, List, Optional

from llvmlite import ir

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

    raise NotImplementedError(
        f"Unsupported MIR method call: box={box_name!r} method={method_name!r}"
    )

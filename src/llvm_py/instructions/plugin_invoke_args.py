"""
Generic plugin invoke argument packing helpers.

This module keeps argc / arg boxing separate from by-name lowering so shared
callers can reuse the same policy without owning invoke mechanics.
"""

from typing import Optional

from llvmlite import ir


def build_plugin_invoke_args(
    *,
    args,
    resolve_arg,
    ensure_handle,
    argc_cap: Optional[int] = None,
):
    i64 = ir.IntType(64)
    argc_len = len(args)
    if argc_cap is not None:
        argc_len = min(argc_len, argc_cap)
    argc = ir.Constant(i64, argc_len)
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
    return argc, a1, a2

"""
Direct lowering helpers for known user-box method calls.

When the receiver box type is statically known and the lowered module already
contains a matching `Box.method/arity` function, prefer a direct call over the
generic plugin invoke shim. This keeps user-defined box methods executable on
native LLVM/AOT routes without relying on plugin-host method resolution.
"""

from typing import Callable, List, Optional, Tuple

from llvmlite import ir

from naming_helper import encode_static_method


def resolve_known_box_method(
    module: ir.Module,
    box_name: Optional[str],
    method_name: Optional[str],
    arities: Tuple[int, ...],
):
    """Return the matching lowered function for a known box method, if present."""
    if not box_name or not method_name:
        return None

    for arity in arities:
        candidates = [
            encode_static_method(box_name, method_name, arity),
            f"{box_name}.{method_name}/{arity}",
        ]
        for candidate in candidates:
            for func in module.functions:
                if func.name == candidate:
                    return func
    plain_candidate = f"{box_name}.{method_name}"
    for func in module.functions:
        if func.name == plain_candidate:
            return func
    return None


def try_lower_known_box_method_call(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    box_name: Optional[str],
    method_name: Optional[str],
    recv_h: ir.Value,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
):
    """Lower to a direct `Box.method/arity` call when the target exists."""
    i64 = ir.IntType(64)
    callee = resolve_known_box_method(module, box_name, method_name, (len(args) + 1, len(args)))
    if callee is None:
        return None

    want_receiver = len(callee.args) == len(args) + 1
    argv = [recv_h] if want_receiver else []
    for arg_vid in args:
        arg_val = resolve_arg(arg_vid)
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)
        argv.append(ensure_handle(arg_val))
    return builder.call(callee, argv, name=call_name)

"""
Direct MirBuilder lowering helpers.

This module owns the native LLVM route for the narrow MirBuilder direct-route
slice so direct route logic does not grow back into generic known-box lowering.
"""

from typing import Callable, List, Optional, Tuple
import os

from llvmlite import ir

from naming_helper import encode_static_method


def _resolve_direct_callee(
    module: ir.Module,
    method_name: str,
    arities: Tuple[int, ...],
):
    for arity in arities:
        candidates = [
            encode_static_method("MirBuilderBox", method_name, arity),
            f"MirBuilderBox.{method_name}/{arity}",
        ]
        for candidate in candidates:
            for func in module.functions:
                if func.name == candidate:
                    return func

    plain_candidate = f"MirBuilderBox.{method_name}"
    for func in module.functions:
        if func.name == plain_candidate:
            return func
    return None


def _strict_direct_arg_or_zero(method_name: str, arg_index: int):
    if os.environ.get("NYASH_LLVM_STRICT") == "1":
        raise RuntimeError(
            f"[LLVM_PY/STRICT] MirBuilderBox direct arg unresolved: "
            f"{method_name} arg{arg_index}"
        )
    return ir.Constant(ir.IntType(64), 0)


def _lower_mir_builder_direct(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    method_name: str,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
):
    if len(args) != 2:
        return None

    i64 = ir.IntType(64)
    callee = _resolve_direct_callee(module, method_name, (len(args),))
    if callee is None:
        return None

    argv = []
    for index, arg_vid in enumerate(args):
        arg_val = resolve_arg(arg_vid)
        if arg_val is None:
            arg_val = _strict_direct_arg_or_zero(method_name, index)
        argv.append(ensure_handle(arg_val))
    return builder.call(callee, argv, name=call_name)


def lower_mir_builder_emit_from_program_json_direct(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
):
    return _lower_mir_builder_direct(
        builder=builder,
        module=module,
        method_name="emit_from_program_json_v0",
        args=args,
        resolve_arg=resolve_arg,
        ensure_handle=ensure_handle,
        call_name=call_name,
    )


def lower_mir_builder_emit_from_source_direct(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
):
    return _lower_mir_builder_direct(
        builder=builder,
        module=module,
        method_name="emit_from_source_v0",
        args=args,
        resolve_arg=resolve_arg,
        ensure_handle=ensure_handle,
        call_name=call_name,
    )

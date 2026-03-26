"""
Direct FileBox lowering helpers.

This module owns the native LLVM route for the narrow FileBox hard-retire
execution slices so direct route logic does not grow back into generic
known-box lowering.
"""

from typing import Callable, List, Optional

from llvmlite import ir

from instructions.string_fast import string_const_boxer_symbol


def _declare(module: ir.Module, name: str, ret, args):
    for func in module.functions:
        if func.name == name:
            return func
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


def _unique_global_name(module: ir.Module, base: str) -> str:
    existing = {value.name for value in module.global_values}
    name = base
    suffix = 1
    while name in existing:
        name = f"{base}.{suffix}"
        suffix += 1
    return name


def _declare_const_string_handle(
    builder: ir.IRBuilder,
    module: ir.Module,
    text: str,
    call_name: str,
):
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    i64 = ir.IntType(64)
    data = text.encode("utf-8") + b"\0"
    arr_ty = ir.ArrayType(i8, len(data))
    gname = _unique_global_name(module, ".filebox.const")
    global_value = ir.GlobalVariable(module, arr_ty, name=gname)
    global_value.initializer = ir.Constant(arr_ty, bytearray(data))
    global_value.linkage = "private"
    global_value.global_constant = True
    c0 = ir.Constant(ir.IntType(32), 0)
    ptr = builder.gep(global_value, [c0, c0], inbounds=True)
    boxer = _declare(module, string_const_boxer_symbol(), i64, [i8p])
    return builder.call(boxer, [ptr], name=call_name)


def lower_filebox_open_direct(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
):
    if len(args) not in (1, 2):
        return None

    i64 = ir.IntType(64)
    path_arg = resolve_arg(args[0])
    if path_arg is None:
        path_arg = ir.Constant(i64, 0)
    path_h = ensure_handle(path_arg)

    if len(args) == 2:
        mode_arg = resolve_arg(args[1])
        if mode_arg is None:
            mode_arg = ir.Constant(i64, 0)
        mode_h = ensure_handle(mode_arg)
    else:
        mode_h = _declare_const_string_handle(
            builder,
            module,
            "r",
            f"{call_name}_mode_h",
        )

    callee = _declare(module, "nyash.file.open_hhh", i64, [i64, i64, i64])
    return builder.call(callee, [recv_h, path_h, mode_h], name=call_name)

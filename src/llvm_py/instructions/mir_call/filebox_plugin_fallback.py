"""
Explicit FileBox compat fallback for by-name plugin invoke.

This isolates the `nyash.plugin.invoke_by_name_i64` emission away from
the method-call direct route so the remaining compat leaf stays visible
and cannot silently widen back into a generic fallback path.
"""

from typing import Callable, List, Optional

from llvmlite import ir


FILEBOX_PLUGIN_FALLBACK_METHODS = frozenset(
    ("open", "read", "readBytes", "close")
)


def _declare(module: ir.Module, name: str, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


def _unique_global_name(module: ir.Module, base: str) -> str:
    existing = {g.name for g in module.global_values}
    name = base
    suffix = 1
    while name in existing:
        name = f"{base}.{suffix}"
        suffix += 1
    return name


def _declare_method_string(builder: ir.IRBuilder, module: ir.Module, method_name: str):
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    arr_bytes = method_name.encode("utf-8") + b"\0"
    arr_ty = ir.ArrayType(i8, len(arr_bytes))
    gname = _unique_global_name(module, f".plugin.meth.{method_name}")
    g = ir.GlobalVariable(module, arr_ty, name=gname)
    g.initializer = ir.Constant(arr_ty, bytearray(arr_bytes))
    g.linkage = "private"
    g.global_constant = True
    c0 = ir.Constant(ir.IntType(32), 0)
    return builder.gep(g, [c0, c0], inbounds=True)


def lower_filebox_plugin_invoke_by_name(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    method_name: str,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
):
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    method_cstr = _declare_method_string(builder, module, method_name)
    argc = ir.Constant(i64, len(args))
    a1 = ir.Constant(i64, 0)
    a2 = ir.Constant(i64, 0)
    if len(args) > 0:
        arg_val = resolve_arg(args[0])
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)
        a1 = ensure_handle(arg_val)
    if len(args) > 1:
        arg_val = resolve_arg(args[1])
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)
        a2 = ensure_handle(arg_val)
    callee = _declare(module, "nyash.plugin.invoke_by_name_i64", i64, [i64, i8p, i64, i64, i64])
    return builder.call(callee, [recv_h, method_cstr, argc, a1, a2], name=call_name)

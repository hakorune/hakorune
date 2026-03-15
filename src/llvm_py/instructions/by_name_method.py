"""
Shared by-name method invocation helpers.

This module keeps the generic plugin-method fallback in one place so opcode
lowerers do not each own their own `nyash.plugin.invoke_by_name_i64` wiring.
"""

from typing import Callable, Optional

from llvmlite import ir

from instructions.mir_call.intrinsic_registry import produces_string_result


def _get_or_create_method_global(module: ir.Module, method_name: str):
    i8 = ir.IntType(8)
    method_str = method_name.encode("utf-8") + b"\0"
    method_gname = f"unified_method_{method_name}"
    if method_gname in module.globals:
        return module.get_global(method_gname)

    method_global = ir.GlobalVariable(
        module,
        ir.ArrayType(i8, len(method_str)),
        name=method_gname,
    )
    method_global.initializer = ir.Constant(
        ir.ArrayType(i8, len(method_str)),
        bytearray(method_str),
    )
    method_global.global_constant = True
    return method_global


def lower_plugin_invoke_by_name(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h,
    method_name: str,
    argc_value,
    arg1_value,
    arg2_value,
    call_name: str,
):
    i64 = ir.IntType(64)
    method_global = _get_or_create_method_global(module, method_name)
    mptr = builder.gep(
        method_global,
        [ir.Constant(ir.IntType(32), 0), ir.Constant(ir.IntType(32), 0)],
    )
    callee = _declare(module, "nyash.plugin.invoke_by_name_i64", i64, [i64, mptr.type, i64, i64, i64])
    return builder.call(callee, [recv_h, mptr, argc_value, arg1_value, arg2_value], name=call_name)


def mark_string_result_if_needed(resolver, dst_vid: Optional[int], method_name: Optional[str]) -> None:
    if dst_vid is None or resolver is None or not hasattr(resolver, "mark_string"):
        return
    if produces_string_result(method_name):
        resolver.mark_string(dst_vid)


def _declare(module: ir.Module, name: str, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)

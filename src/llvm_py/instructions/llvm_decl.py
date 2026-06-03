"""LLVM declaration helpers shared by instruction lowerers."""

import llvmlite.ir as ir


def declare_function(module: ir.Module, name: str, ret_ty, arg_tys):
    for fn in module.functions:
        if fn.name == name:
            return fn
    return ir.Function(module, ir.FunctionType(ret_ty, arg_tys), name=name)

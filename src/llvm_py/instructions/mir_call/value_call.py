"""
Dynamic function value call lowering for MIR Call instruction.

Handles lowering of first-class function calls (calling through a function value)
to LLVM IR.
"""

from typing import Dict, Any, Optional
from llvmlite import ir
from .arg_resolver import make_call_arg_resolver


def lower_value_call(builder, module, func_vid, args, dst_vid, vmap, resolver, owner):
    """
    Lower dynamic function value call - TRUE UNIFIED IMPLEMENTATION

    Args:
        builder: LLVM IR builder
        module: LLVM Module
        func_vid: Value ID holding the function/closure to call
        args: List of argument value IDs
        dst_vid: Destination value ID (register)
        vmap: Value mapping dict
        resolver: Value resolver instance
        owner: NyashLLVMBuilder instance

    Effects:
        - Dispatches to appropriate dynamic call function based on argument count
        - Handles both function handles and closure handles
        - Stores result in vmap[dst_vid]
    """
    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()

    # Helper to resolve arguments
    _resolve_arg = make_call_arg_resolver(builder, vmap, resolver, owner)

    # Helper to declare function
    def _declare(name: str, ret, args_types):
        for f in module.functions:
            if f.name == name:
                return f
        fnty = ir.FunctionType(ret, args_types)
        return ir.Function(module, fnty, name=name)

    # Resolve the function value (handle to function or closure)
    func_val = _resolve_arg(func_vid)
    if func_val is None:
        func_val = ir.Constant(i64, 0)

    # Resolve arguments
    arg_vals = []
    for arg_id in args:
        arg_val = _resolve_arg(arg_id) or ir.Constant(i64, 0)
        arg_vals.append(arg_val)

    # Dynamic dispatch based on function value type
    # This could be a function handle, closure handle, or method handle

    if len(arg_vals) == 0:
        # No arguments - simple function call
        callee = _declare("nyash.dynamic.call_0", i64, [i64])
        result = builder.call(callee, [func_val], name="unified_dynamic_call_0")

    elif len(arg_vals) == 1:
        # One argument
        callee = _declare("nyash.dynamic.call_1", i64, [i64, i64])
        result = builder.call(callee, [func_val, arg_vals[0]], name="unified_dynamic_call_1")

    elif len(arg_vals) == 2:
        # Two arguments
        callee = _declare("nyash.dynamic.call_2", i64, [i64, i64, i64])
        result = builder.call(callee, [func_val, arg_vals[0], arg_vals[1]], name="unified_dynamic_call_2")

    else:
        # Generic variadic call
        argc = ir.Constant(i64, len(arg_vals))

        # Create argument array for variadic call
        if len(arg_vals) <= 4:
            # Use direct argument passing for small argument lists
            arg_types = [i64] * (2 + len(arg_vals))  # func_val, argc, ...args
            callee = _declare("nyash.dynamic.call_n", i64, arg_types)
            call_args = [func_val, argc] + arg_vals
            result = builder.call(callee, call_args, name="unified_dynamic_call_n")
        else:
            # For large argument lists, use array-based approach
            callee = _declare("nyash.dynamic.call_array", i64, [i64, i64, i8p])

            # Create temporary array for arguments
            array_type = ir.ArrayType(i64, len(arg_vals))
            array_alloca = builder.alloca(array_type, name="unified_arg_array")

            # Store arguments in array
            for i, arg_val in enumerate(arg_vals):
                gep = builder.gep(array_alloca, [ir.Constant(ir.IntType(32), 0), ir.Constant(ir.IntType(32), i)])
                builder.store(arg_val, gep)

            # Cast array to i8*
            array_ptr = builder.bitcast(array_alloca, i8p, name="unified_arg_array_ptr")
            result = builder.call(callee, [func_val, argc, array_ptr], name="unified_dynamic_call_array")

    # Store result
    if dst_vid is not None:
        vmap[dst_vid] = result

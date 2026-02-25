"""
External C ABI function call lowering for MIR Call instruction.

Handles lowering of extern function calls to LLVM IR.
Supports C ABI calling conventions with proper type conversions.
"""

from typing import Dict, Any, Optional
from llvmlite import ir
import os
from .arg_resolver import make_call_arg_resolver


def lower_extern_call(builder, module, extern_name, args, dst_vid, vmap, resolver, owner):
    """
    Lower external C ABI call - TRUE UNIFIED IMPLEMENTATION

    Args:
        builder: LLVM IR builder
        module: LLVM Module
        extern_name: Name of the external function
        args: List of argument value IDs
        dst_vid: Destination value ID (register)
        vmap: Value mapping dict
        resolver: Value resolver instance
        owner: NyashLLVMBuilder instance

    Effects:
        - Inserts automatic safepoint
        - Normalizes extern function names
        - Creates C ABI function declaration if needed
        - Performs handle-to-pointer conversions for string arguments
        - Stores result in vmap[dst_vid]
    """
    from instructions.safepoint import insert_automatic_safepoint
    from instructions.extern_normalize import normalize_extern_name

    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()

    # Insert automatic safepoint
    if os.environ.get('NYASH_LLVM_AUTO_SAFEPOINT', '1') == '1':
        insert_automatic_safepoint(builder, module, "externcall")

    # Helper to resolve arguments
    _resolve_arg = make_call_arg_resolver(builder, vmap, resolver, owner)

    # Normalize extern target names via shared normalizer
    extern_name = normalize_extern_name(extern_name)

    # Look up extern function in module
    func = None
    for f in module.functions:
        if f.name == extern_name:
            func = f
            break

    if not func:
        # Create C ABI function declaration
        if extern_name == "nyash.console.log":
            func_type = ir.FunctionType(i64, [i8p])
        elif extern_name in ["print", "panic", "error"]:
            func_type = ir.FunctionType(ir.VoidType(), [i8p])
        else:
            # Generic extern: i64 return, i64 args
            arg_types = [i64] * len(args)
            func_type = ir.FunctionType(i64, arg_types)

        func = ir.Function(module, func_type, name=extern_name)

    # Prepare arguments with C ABI type conversion
    call_args = []
    for i, arg_id in enumerate(args):
        arg_val = _resolve_arg(arg_id)
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)

        # Type conversion for C ABI
        if i < len(func.args):
            expected_type = func.args[i].type

            if expected_type.is_pointer:
                # Convert i64 handle to i8* for string parameters
                if isinstance(arg_val.type, ir.IntType) and arg_val.type.width == 64:
                    # Use string handle-to-pointer conversion
                    try:
                        to_i8p = None
                        for f in module.functions:
                            if f.name == "nyash.string.to_i8p_h":
                                to_i8p = f
                                break
                        if not to_i8p:
                            to_i8p_type = ir.FunctionType(i8p, [i64])
                            to_i8p = ir.Function(module, to_i8p_type, name="nyash.string.to_i8p_h")

                        arg_val = builder.call(to_i8p, [arg_val], name=f"unified_extern_h2p_{i}")
                    except (AttributeError, TypeError, ValueError, KeyError):
                        # Fallback: inttoptr conversion
                        arg_val = builder.inttoptr(arg_val, expected_type, name=f"unified_extern_i2p_{i}")
                elif not arg_val.type.is_pointer:
                    arg_val = builder.inttoptr(arg_val, expected_type, name=f"unified_extern_i2p_{i}")

            elif isinstance(expected_type, ir.IntType):
                # Convert to expected integer width
                if arg_val.type.is_pointer:
                    arg_val = builder.ptrtoint(arg_val, expected_type, name=f"unified_extern_p2i_{i}")
                elif isinstance(arg_val.type, ir.IntType) and arg_val.type.width != expected_type.width:
                    if arg_val.type.width < expected_type.width:
                        arg_val = builder.zext(arg_val, expected_type, name=f"unified_extern_zext_{i}")
                    else:
                        arg_val = builder.trunc(arg_val, expected_type, name=f"unified_extern_trunc_{i}")

        call_args.append(arg_val)

    # Make the C ABI call - TRUE UNIFIED
    if len(call_args) == len(func.args):
        result = builder.call(func, call_args, name=f"unified_extern_{extern_name}")
    else:
        # Truncate args to match function signature
        result = builder.call(func, call_args[:len(func.args)], name=f"unified_extern_{extern_name}_trunc")

    # Store result
    if dst_vid is not None:
        ret_type = func.function_type.return_type
        if isinstance(ret_type, ir.VoidType):
            vmap[dst_vid] = ir.Constant(i64, 0)
        else:
            vmap[dst_vid] = result

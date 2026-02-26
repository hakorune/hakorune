"""
Global function call lowering for MIR Call instruction.

Handles lowering of global function calls (e.g., print, panic, user-defined functions)
to LLVM IR.
"""

from typing import Dict, Any, Optional
from llvmlite import ir
import os
from utils.resolver_helpers import is_handle_type, is_stringish_legacy
from .arg_resolver import make_call_arg_resolver


def lower_global_call(builder, module, func_name, args, dst_vid, vmap, resolver, owner):
    """
    Lower global function call - TRUE UNIFIED IMPLEMENTATION

    Args:
        builder: LLVM IR builder
        module: LLVM Module
        func_name: Name of the global function to call
        args: List of argument value IDs
        dst_vid: Destination value ID (register)
        vmap: Value mapping dict
        resolver: Value resolver instance
        owner: NyashLLVMBuilder instance

    Effects:
        - Inserts automatic safepoint
        - Creates function declaration if needed
        - Emits LLVM call instruction
        - Stores result in vmap[dst_vid]
        - Marks string-producing functions for type tracking
    """
    from instructions.safepoint import insert_automatic_safepoint

    # Insert automatic safepoint
    if os.environ.get('NYASH_LLVM_AUTO_SAFEPOINT', '1') == '1':
        insert_automatic_safepoint(builder, module, "function_call")

    # Resolver helper
    _resolve_arg = make_call_arg_resolver(builder, vmap, resolver, owner)

    # Look up function in module
    func = None
    for f in module.functions:
        if f.name == func_name:
            func = f
            break

    if not func:
        # Create function declaration with appropriate signature
        # Phase 131-15-P1: Handle C ABI extern functions (print, panic, error)
        i8p = ir.IntType(8).as_pointer()
        if func_name in ["print", "panic", "error"]:
            # C ABI: void(i8*)
            func_type = ir.FunctionType(ir.VoidType(), [i8p])
        elif func_name == "nyash.console.log":
            # C ABI: i64(i8*)
            func_type = ir.FunctionType(ir.IntType(64), [i8p])
        else:
            # Default: i64(...i64)
            ret_type = ir.IntType(64)
            arg_types = [ir.IntType(64)] * len(args)
            func_type = ir.FunctionType(ret_type, arg_types)
        func = ir.Function(module, func_type, name=func_name)

    # Prepare arguments with type conversion
    call_args = []
    for i, arg_id in enumerate(args):
        arg_val = _resolve_arg(arg_id)
        if arg_val is None:
            arg_val = ir.Constant(ir.IntType(64), 0)

        # Type conversion for function signature matching
        if i < len(func.args):
            expected_type = func.args[i].type
            if expected_type.is_pointer and isinstance(arg_val.type, ir.IntType):
                # Convert i64 to i8* for C ABI-style functions (print/panic/error).
                #
                # IMPORTANT: `print()` in AOT must not interpret unboxed integers as handles
                # (handle collision prints wrong strings). Use type facts to decide:
                # - stringish -> treat as handle and bridge via nyash.string.to_i8p_h
                # - otherwise -> box integer (nyash.box.from_i64) then bridge
                if arg_val.type.width == 64:
                    i8p = ir.IntType(8).as_pointer()
                    to_i8p = None
                    for f in module.functions:
                        if f.name == "nyash.string.to_i8p_h":
                            to_i8p = f
                            break
                    if not to_i8p:
                        to_i8p_type = ir.FunctionType(i8p, [ir.IntType(64)])
                        to_i8p = ir.Function(module, to_i8p_type, name="nyash.string.to_i8p_h")

                    # Phase 285LLVM-1.5: Unified type checking via resolver_helpers
                    is_stringish = is_stringish_legacy(resolver, int(arg_id))
                    is_handle = is_handle_type(resolver, int(arg_id))
                    integerish_ids = getattr(resolver, "integerish_ids", set()) if resolver is not None else set()
                    is_integerish = int(arg_id) in integerish_ids

                    # Debug logging: handle detection
                    if is_handle and os.environ.get('NYASH_CLI_VERBOSE') == '1':
                        import sys
                        print(f"[llvm-py/types] print arg %{arg_id}: is_handle=True, skip boxing", file=sys.stderr)

                    v_to_print = arg_val
                    # Phase 285LLVM-1.4: Only box if NOT stringish AND NOT already a handle
                    if func_name == "print" and not is_stringish and (not is_handle or is_integerish):
                        # Raw i64 value: box it before printing
                        # Debug logging: raw i64 boxing
                        if os.environ.get('NYASH_CLI_VERBOSE') == '1':
                            import sys
                            print(
                                f"[llvm-py/types] print arg %{arg_id}: raw/integerish i64, box.from_i64 called",
                                file=sys.stderr,
                            )
                        boxer = None
                        for f in module.functions:
                            if f.name == "nyash.box.from_i64":
                                boxer = f
                                break
                        if boxer is None:
                            boxer = ir.Function(module, ir.FunctionType(ir.IntType(64), [ir.IntType(64)]), name="nyash.box.from_i64")
                        v_to_print = builder.call(boxer, [arg_val], name=f"global_box_i64_{i}")

                    arg_val = builder.call(to_i8p, [v_to_print], name=f"global_h2p_{i}")
                else:
                    arg_val = builder.inttoptr(arg_val, expected_type, name=f"global_i2p_{i}")
            elif isinstance(expected_type, ir.IntType) and arg_val.type.is_pointer:
                arg_val = builder.ptrtoint(arg_val, expected_type, name=f"global_p2i_{i}")

        call_args.append(arg_val)

    # Make the call - TRUE UNIFIED
    result = builder.call(func, call_args, name=f"unified_global_{func_name}")

    # Store result
    if dst_vid is not None:
        vmap[dst_vid] = result
        # Mark string-producing functions
        if resolver and hasattr(resolver, 'mark_string'):
            if any(key in func_name for key in ['esc_json', 'node_json', 'dirname', 'join', 'read_all', 'toJson']):
                resolver.mark_string(dst_vid)

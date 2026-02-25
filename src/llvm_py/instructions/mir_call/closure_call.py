"""
Closure creation lowering for MIR Call instruction.

Handles lowering of closure creation (NewClosure) to LLVM IR.
"""

from typing import Dict, Any, Optional
from llvmlite import ir
from .arg_resolver import make_call_arg_resolver


def lower_closure_creation(builder, module, params, captures, me_capture, dst_vid, vmap, resolver, owner):
    """
    Lower closure creation - TRUE UNIFIED IMPLEMENTATION

    Args:
        builder: LLVM IR builder
        module: LLVM Module
        params: List of parameter definitions for the closure
        captures: List of captured variable value IDs
        me_capture: Value ID of captured 'me' reference (if any)
        dst_vid: Destination value ID (register)
        vmap: Value mapping dict
        resolver: Value resolver instance
        owner: NyashLLVMBuilder instance

    Effects:
        - Creates closure object with captured values
        - Stores closure handle in vmap[dst_vid]
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

    # Create closure metadata structure
    num_captures = len(captures) if captures else 0
    num_params = len(params) if params else 0

    # Resolve captured values
    capture_vals = []
    if captures:
        for capture in captures:
            if isinstance(capture, dict) and 'id' in capture:
                cap_val = _resolve_arg(capture['id']) or ir.Constant(i64, 0)
            elif isinstance(capture, int):
                cap_val = _resolve_arg(capture) or ir.Constant(i64, 0)
            else:
                cap_val = ir.Constant(i64, 0)
            capture_vals.append(cap_val)

    # Add me_capture if present
    if me_capture is not None:
        me_val = _resolve_arg(me_capture) if isinstance(me_capture, int) else ir.Constant(i64, 0)
        capture_vals.append(me_val)
        num_captures += 1

    # Call closure creation function
    if num_captures > 0:
        # Closure with captures
        callee = _declare("nyash.closure.new_with_captures", i64, [i64, i64] + [i64] * num_captures)
        args = [ir.Constant(i64, num_params), ir.Constant(i64, num_captures)] + capture_vals
        result = builder.call(callee, args, name="unified_closure_with_captures")
    else:
        # Simple closure without captures
        callee = _declare("nyash.closure.new", i64, [i64])
        result = builder.call(callee, [ir.Constant(i64, num_params)], name="unified_closure_simple")

    # Store result
    if dst_vid is not None:
        vmap[dst_vid] = result

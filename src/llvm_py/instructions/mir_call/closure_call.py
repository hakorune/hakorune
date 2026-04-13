"""
Closure creation lowering for MIR Call instruction.

Handles lowering of closure creation (NewClosure) to LLVM IR.
"""

from typing import Dict, Any, Optional
from llvmlite import ir
from .arg_resolver import make_call_arg_resolver
from builders.closure_split_contract import build_closure_split_contract


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

    contract = build_closure_split_contract(params, captures, me_capture)
    num_params = int(contract["proof"]["param_count"])
    capture_ids = contract["proof"]["env_capture_value_ids"]
    num_captures = int(contract["proof"]["env_capture_count"])
    capture_vals = [_resolve_arg(capture_id) or ir.Constant(i64, 0) for capture_id in capture_ids]

    # Call closure creation function
    if contract["lowering"]["use_capture_ctor"]:
        # Closure with captures
        callee = _declare(contract["lowering"]["ctor_name"], i64, [i64, i64] + [i64] * num_captures)
        args = [ir.Constant(i64, num_params), ir.Constant(i64, num_captures)] + capture_vals
        result = builder.call(callee, args, name="unified_closure_with_captures")
    else:
        # Simple closure without captures
        callee = _declare(contract["lowering"]["ctor_name"], i64, [i64])
        result = builder.call(callee, [ir.Constant(i64, num_params)], name="unified_closure_simple")

    # Store result
    if dst_vid is not None:
        vmap[dst_vid] = result

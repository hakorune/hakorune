"""
Box constructor call lowering for MIR Call instruction.

Handles lowering of box constructor calls (NewBox) to LLVM IR.
Supports built-in boxes (StringBox, ArrayBox, etc.) and plugin boxes.
"""

from functools import partial
from typing import Dict, Any, Optional
from llvmlite import ir
from instructions.llvm_decl import declare_function
from .arg_resolver import make_call_arg_resolver
from .direct_array_birth import (
    DIRECT_ARRAY_I64_BIRTH_SYMBOL,
    PUBLIC_ARRAY_BIRTH_SYMBOL,
    mark_direct_array_i64_origin,
    should_use_direct_array_i64_birth,
)


def lower_constructor_call(builder, module, box_type, args, dst_vid, vmap, resolver, owner):
    """
    Lower box constructor - TRUE UNIFIED IMPLEMENTATION

    Args:
        builder: LLVM IR builder
        module: LLVM Module
        box_type: Type of box to construct (e.g., "StringBox", "ArrayBox")
        args: List of constructor argument value IDs
        dst_vid: Destination value ID (register)
        vmap: Value mapping dict
        resolver: Value resolver instance
        owner: NyashLLVMBuilder instance

    Effects:
        - Emits specialized constructor calls for built-in boxes
        - Falls back to generic constructor for plugin boxes
        - Stores result handle in vmap[dst_vid]
    """
    i64 = ir.IntType(64)
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()

    # Helper to resolve arguments
    _resolve_arg = make_call_arg_resolver(builder, vmap, resolver, owner)

    declare = partial(declare_function, module)

    # TRUE UNIFIED CONSTRUCTOR DISPATCH
    if box_type == "StringBox":
        if args and len(args) > 0:
            # String constructor with initial value
            arg0 = _resolve_arg(args[0])
            if arg0 and isinstance(arg0.type, ir.IntType) and arg0.type.width == 64:
                # Already a handle, return as-is
                result = arg0
            elif arg0 and arg0.type.is_pointer:
                # Convert i8* to string handle
                callee = declare("nyash.box.from_i8_string", i64, [i8p])
                result = builder.call(callee, [arg0], name="unified_str_new")
            else:
                # Create empty string
                callee = declare("nyash.string.new", i64, [])
                result = builder.call(callee, [], name="unified_str_empty")
        else:
            # Empty string constructor
            callee = declare("nyash.string.new", i64, [])
            result = builder.call(callee, [], name="unified_str_empty")

    elif box_type in ("ArrayBox", "DirectArrayI64"):
        # Default construction remains public ArrayBox. The DirectArray symbol
        # is exact-lane only and produces a distinct direct-buffer handle kind.
        direct_array_birth = should_use_direct_array_i64_birth(box_type)
        callee = declare(
            DIRECT_ARRAY_I64_BIRTH_SYMBOL if direct_array_birth else PUBLIC_ARRAY_BIRTH_SYMBOL,
            i64,
            [],
        )
        result = builder.call(callee, [], name="unified_arr_new")
        if direct_array_birth:
            mark_direct_array_i64_origin(resolver, dst_vid)

    elif box_type == "MapBox":
        # Align with kernel export (birth_h)
        callee = declare("nyash.map.birth_h", i64, [])
        result = builder.call(callee, [], name="unified_map_new")

    elif box_type == "IntegerBox":
        if args and len(args) > 0:
            arg0 = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            callee = declare("nyash.integer.new", i64, [i64])
            result = builder.call(callee, [arg0], name="unified_int_new")
        else:
            callee = declare("nyash.integer.new", i64, [i64])
            result = builder.call(callee, [ir.Constant(i64, 0)], name="unified_int_zero")

    elif box_type == "BoolBox":
        if args and len(args) > 0:
            arg0 = _resolve_arg(args[0]) or ir.Constant(i64, 0)
            callee = declare("nyash.bool.new", i64, [i64])
            result = builder.call(callee, [arg0], name="unified_bool_new")
        else:
            callee = declare("nyash.bool.new", i64, [i64])
            result = builder.call(callee, [ir.Constant(i64, 0)], name="unified_bool_false")

    else:
        # Generic box constructor or plugin box
        # Defensive: ensure box_type is never None
        if box_type is None:
            # Fallback to generic box if type is missing
            box_type = "Box"
        box_type_lower = box_type.lower() if hasattr(box_type, 'lower') else str(box_type).lower()
        constructor_name = f"nyash.{box_type_lower}.new"
        if args:
            arg_vals = [_resolve_arg(arg_id) or ir.Constant(i64, 0) for arg_id in args]
            arg_types = [i64] * len(arg_vals)
            callee = declare(constructor_name, i64, arg_types)
            result = builder.call(callee, arg_vals, name=f"unified_{box_type_lower}_new")
        else:
            callee = declare(constructor_name, i64, [])
            result = builder.call(callee, [], name=f"unified_{box_type_lower}_new")

    # Store result
    if dst_vid is not None:
        vmap[dst_vid] = result

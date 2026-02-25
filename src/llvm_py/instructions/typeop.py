"""
TypeOp instruction lowering (Phase 274 P2)
Handles type conversions and type checks with SSOT alignment to Rust VM
"""

import llvmlite.ir as ir
from typing import Dict, Optional, Any
from utils.values import safe_vmap_write

# Type aliases for frontend normalization (shared with MIR builder)
TYPE_ALIASES = {
    "Int": "IntegerBox",
    "Integer": "IntegerBox",
    "String": "StringBox",
    "Bool": "BoolBox",
    "Float": "FloatBox",
    "Array": "ArrayBox",
    "Void": "VoidBox",
}

def normalize_type_name(ty: str) -> str:
    """Normalize frontend type aliases to Box names"""
    return TYPE_ALIASES.get(ty, ty)

def lower_typeop(
    builder: ir.IRBuilder,
    op: str,
    src_vid: int,
    dst_vid: int,
    target_type: Optional[str],
    vmap: Dict[int, ir.Value],
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None,
) -> None:
    """
    Lower MIR TypeOp instruction with SSOT alignment to Rust VM

    Phase 274 P2: Implements runtime type introspection using kernel helper
    - Primitives (Integer, String, Bool): immediate check via resolver.value_types
    - Boxes: runtime check via kernel nyash.any.is_type_h
    - Fail-fast: TypeOp::Cast emits trap on mismatch

    Operations:
    - cast: Type conversion (fail-fast on mismatch)
    - is: Type check (returns 0/1)
    - as: Safe cast (alias for cast)

    Args:
        builder: Current LLVM IR builder
        op: Operation type (cast, is, as)
        src_vid: Source value ID
        dst_vid: Destination value ID
        target_type: Target type name (e.g., "StringBox", "IntegerBox")
        vmap: Value map
        resolver: Optional resolver for type handling
    """
    # Prefer BuildCtx maps when provided
    if ctx is not None:
        try:
            if getattr(ctx, 'resolver', None) is not None:
                resolver = ctx.resolver
            if getattr(ctx, 'preds', None) is not None and preds is None:
                preds = ctx.preds
            if getattr(ctx, 'block_end_values', None) is not None and block_end_values is None:
                block_end_values = ctx.block_end_values
            if getattr(ctx, 'bb_map', None) is not None and bb_map is None:
                bb_map = ctx.bb_map
        except Exception:
            pass

    if resolver is not None and preds is not None and block_end_values is not None and bb_map is not None:
        src_val = resolver.resolve_i64(src_vid, builder.block, preds, block_end_values, vmap, bb_map)
    else:
        src_val = vmap.get(src_vid, ir.Constant(ir.IntType(64), 0))

    # Normalize type name
    normalized_type = normalize_type_name(target_type) if target_type else "Unknown"

    op_l = (op or "").lower()

    if op_l in ("is", "check"):
        # Type check operation - returns i64 (0 or 1)
        _lower_typeop_is(builder, src_vid, dst_vid, src_val, normalized_type, vmap, resolver)

    elif op_l in ("cast", "as"):
        # Cast/as operation - fail-fast on mismatch
        _lower_typeop_cast(builder, src_vid, dst_vid, src_val, normalized_type, vmap, resolver)

    else:
        # Unknown operation - return 0
        safe_vmap_write(vmap, dst_vid, ir.Constant(ir.IntType(64), 0), "typeop_unknown")

def _lower_typeop_is(
    builder: ir.IRBuilder,
    src_vid: int,
    dst_vid: int,
    src_val: ir.Value,
    normalized_type: str,
    vmap: Dict[int, ir.Value],
    resolver=None,
) -> None:
    """
    Lower TypeOp::Check (is operation)
    Returns 1 if type matches, 0 otherwise

    Strategy:
    1. Check resolver.value_types for raw vs handle discrimination
    2. Primitives (Integer, String, Bool): immediate check (no kernel call)
    3. Boxes: call kernel nyash.any.is_type_h
    """
    # Step 1: Check resolver.value_types for raw vs handle discrimination
    mir_type = None
    if resolver is not None and hasattr(resolver, 'value_types') and isinstance(resolver.value_types, dict):
        mir_type = resolver.value_types.get(src_vid)
        # Debug: log type information
        import os
        if os.environ.get('NYASH_TYPEOP_DEBUG') == '1':
            print(f"[TypeOp is] src_vid={src_vid}, mir_type={mir_type}, target={normalized_type}")

    # Step 2: Primitive immediate check (raw i64)
    if _is_primitive_type(mir_type, 'Integer'):
        # Raw i64: immediate check
        if normalized_type in ("Integer", "IntegerBox"):
            result = ir.Constant(ir.IntType(64), 1)  # Match
        else:
            result = ir.Constant(ir.IntType(64), 0)  # No match
        safe_vmap_write(vmap, dst_vid, result, "typeop_is_primitive")
        return

    if _is_primitive_type(mir_type, 'String'):
        # String check
        if normalized_type in ("String", "StringBox"):
            result = ir.Constant(ir.IntType(64), 1)
        else:
            result = ir.Constant(ir.IntType(64), 0)
        safe_vmap_write(vmap, dst_vid, result, "typeop_is_string")
        return

    if _is_primitive_type(mir_type, 'Bool'):
        # Bool check
        if normalized_type in ("Bool", "BoolBox"):
            result = ir.Constant(ir.IntType(64), 1)
        else:
            result = ir.Constant(ir.IntType(64), 0)
        safe_vmap_write(vmap, dst_vid, result, "typeop_is_bool")
        return

    # Step 3: Handle (Box) → call kernel is_type_h
    result = _emit_kernel_type_check(builder, src_val, normalized_type)
    safe_vmap_write(vmap, dst_vid, result, "typeop_is_handle")

def _lower_typeop_cast(
    builder: ir.IRBuilder,
    src_vid: int,
    dst_vid: int,
    src_val: ir.Value,
    normalized_type: str,
    vmap: Dict[int, ir.Value],
    resolver=None,
) -> None:
    """
    Lower TypeOp::Cast (cast/as operation)
    Returns value if type matches, traps on mismatch (fail-fast)

    Strategy:
    1. Check resolver.value_types for raw vs handle discrimination
    2. Primitives: immediate check, trap on mismatch
    3. Boxes: kernel check, trap on mismatch
    """
    # Step 1: Check resolver.value_types for raw vs handle discrimination
    mir_type = None
    if resolver is not None and hasattr(resolver, 'value_types') and isinstance(resolver.value_types, dict):
        mir_type = resolver.value_types.get(src_vid)

    # Step 2: Primitive immediate check (raw i64)
    if _is_primitive_type(mir_type, 'Integer'):
        # Raw i64: check target type
        if normalized_type in ("Integer", "IntegerBox"):
            # Match: pass through
            safe_vmap_write(vmap, dst_vid, src_val, "typeop_cast_primitive_ok")
            return
        else:
            # Mismatch: fail-fast (trap)
            _emit_trap(builder)
            # Unreachable after trap, but emit defensive value
            safe_vmap_write(vmap, dst_vid, ir.Constant(ir.IntType(64), 0), "typeop_cast_primitive_trap")
            return

    if _is_primitive_type(mir_type, 'String'):
        # String check
        if normalized_type in ("String", "StringBox"):
            safe_vmap_write(vmap, dst_vid, src_val, "typeop_cast_string_ok")
            return
        else:
            _emit_trap(builder)
            safe_vmap_write(vmap, dst_vid, ir.Constant(ir.IntType(64), 0), "typeop_cast_string_trap")
            return

    if _is_primitive_type(mir_type, 'Bool'):
        # Bool check
        if normalized_type in ("Bool", "BoolBox"):
            safe_vmap_write(vmap, dst_vid, src_val, "typeop_cast_bool_ok")
            return
        else:
            _emit_trap(builder)
            safe_vmap_write(vmap, dst_vid, ir.Constant(ir.IntType(64), 0), "typeop_cast_bool_trap")
            return

    # Step 3: Handle (Box) → call kernel is_type_h for runtime check
    check_result = _emit_kernel_type_check(builder, src_val, normalized_type)

    # Check if type matches
    zero = ir.Constant(ir.IntType(64), 0)
    is_match = builder.icmp_unsigned('!=', check_result, zero, name=f"typeop_cast_match_{dst_vid}")

    # Fail-fast: if mismatch, trap
    # Get current function for block allocation
    fn = builder.function
    trap_bb = fn.append_basic_block(name=f"typeop_cast_fail_{dst_vid}")
    ok_bb = fn.append_basic_block(name=f"typeop_cast_ok_{dst_vid}")

    builder.cbranch(is_match, ok_bb, trap_bb)

    # Trap block
    builder.position_at_end(trap_bb)
    _emit_trap(builder)

    # OK block
    builder.position_at_end(ok_bb)
    safe_vmap_write(vmap, dst_vid, src_val, "typeop_cast_handle_ok")

def _is_primitive_type(mir_type, expected: str) -> bool:
    """
    Check if MIR type metadata indicates a primitive type

    Args:
        mir_type: Value from resolver.value_types (str or dict)
        expected: Expected primitive name ("Integer", "String", "Bool")

    Returns:
        True if mir_type indicates the expected primitive
    """
    if mir_type is None:
        return False

    # JSON metadata vocabulary (from Rust runner):
    # - "i64" for integers
    # - "i1" for booleans
    # - {"kind":"string"} for strings (even when runtime value is a StringBox handle)
    if expected == "Integer":
        if mir_type in ("i64", "Integer"):
            return True
        if isinstance(mir_type, dict) and mir_type.get("kind") in ("i64", "Integer"):
            return True
        return False

    if expected == "Bool":
        if mir_type in ("i1", "Bool"):
            return True
        if isinstance(mir_type, dict) and mir_type.get("kind") in ("i1", "Bool"):
            return True
        return False

    if expected == "String":
        if mir_type == "String":
            return True
        if isinstance(mir_type, dict):
            k = mir_type.get("kind")
            if k == "string":
                return True
            if k == "handle" and mir_type.get("box_type") == "StringBox":
                return True
            if mir_type.get("box_type") == "StringBox":
                return True
        return False

    return False

def _emit_kernel_type_check(builder: ir.IRBuilder, src_val: ir.Value, type_name: str) -> ir.Value:
    """
    Emit call to kernel nyash.any.is_type_h for runtime type check

    Args:
        builder: LLVM IR builder
        src_val: Source value (i64 handle)
        type_name: Normalized type name (e.g., "IntegerBox")

    Returns:
        i64 result (1 if match, 0 otherwise)
    """
    # Create global string for type name
    type_name_bytes = bytearray(type_name.encode('utf-8') + b'\0')
    type_name_str = ir.GlobalVariable(
        builder.module,
        ir.ArrayType(ir.IntType(8), len(type_name_bytes)),
        name=f"type_name_{type_name}"
    )
    type_name_str.initializer = ir.Constant(
        ir.ArrayType(ir.IntType(8), len(type_name_bytes)),
        type_name_bytes
    )
    type_name_str.linkage = 'internal'
    type_name_str.global_constant = True

    # Get pointer to string
    type_name_ptr = builder.gep(
        type_name_str,
        [ir.Constant(ir.IntType(32), 0), ir.Constant(ir.IntType(32), 0)],
        name=f"type_name_ptr_{type_name}"
    )

    # Declare kernel function (check if already exists to avoid duplicates)
    func_name = "nyash.any.is_type_h"
    try:
        is_type_h = builder.module.get_global(func_name)
    except KeyError:
        # Function doesn't exist, create it
        is_type_h_ty = ir.FunctionType(ir.IntType(64), [ir.IntType(64), ir.IntType(8).as_pointer()])
        is_type_h = ir.Function(builder.module, is_type_h_ty, name=func_name)

    # Call kernel helper
    result = builder.call(is_type_h, [src_val, type_name_ptr], name=f"is_{type_name}")

    return result

def _emit_trap(builder: ir.IRBuilder) -> None:
    """
    Emit unreachable instruction for fail-fast TypeOp error

    Args:
        builder: LLVM IR builder

    Note: Uses unreachable directly instead of llvm.trap intrinsic
    for simplicity and llvmlite compatibility
    """
    builder.unreachable()

def lower_convert(
    builder: ir.IRBuilder,
    src_vid: int,
    dst_vid: int,
    from_type: str,
    to_type: str,
    vmap: Dict[int, ir.Value],
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None,
) -> None:
    """
    Lower type conversion between primitive types

    Args:
        builder: Current LLVM IR builder
        src_vid: Source value ID
        dst_vid: Destination value ID
        from_type: Source type (i32, i64, f64, ptr)
        to_type: Target type
        vmap: Value map
    """
    if ctx is not None:
        try:
            if getattr(ctx, 'resolver', None) is not None:
                resolver = ctx.resolver
            if getattr(ctx, 'preds', None) is not None and preds is None:
                preds = ctx.preds
            if getattr(ctx, 'block_end_values', None) is not None and block_end_values is None:
                block_end_values = ctx.block_end_values
            if getattr(ctx, 'bb_map', None) is not None and bb_map is None:
                bb_map = ctx.bb_map
        except Exception:
            pass
    if resolver is not None and preds is not None and block_end_values is not None and bb_map is not None:
        # Choose resolution based on from_type
        if from_type == "ptr":
            src_val = resolver.resolve_ptr(src_vid, builder.block, preds, block_end_values, vmap)
        else:
            src_val = resolver.resolve_i64(src_vid, builder.block, preds, block_end_values, vmap, bb_map)
    else:
        src_val = vmap.get(src_vid)
    if not src_val:
        # Default based on target type
        if to_type == "f64":
            safe_vmap_write(vmap, dst_vid, ir.Constant(ir.DoubleType(), 0.0), "convert_default_f64")
        elif to_type == "ptr":
            i8 = ir.IntType(8)
            safe_vmap_write(vmap, dst_vid, ir.Constant(i8.as_pointer(), None), "convert_default_ptr")
        else:
            safe_vmap_write(vmap, dst_vid, ir.Constant(ir.IntType(64), 0), "convert_default_i64")
        return

    # Perform conversion
    if from_type == "i64" and to_type == "f64":
        # int to float
        result = builder.sitofp(src_val, ir.DoubleType())
    elif from_type == "f64" and to_type == "i64":
        # float to int
        result = builder.fptosi(src_val, ir.IntType(64))
    elif from_type == "i64" and to_type == "ptr":
        # int to pointer
        i8 = ir.IntType(8)
        result = builder.inttoptr(src_val, i8.as_pointer(), name=f"conv_i2p_{dst_vid}")
    elif from_type == "ptr" and to_type == "i64":
        # pointer to int
        result = builder.ptrtoint(src_val, ir.IntType(64), name=f"conv_p2i_{dst_vid}")
    elif from_type == "i32" and to_type == "i64":
        # sign extend
        result = builder.sext(src_val, ir.IntType(64))
    elif from_type == "i64" and to_type == "i32":
        # truncate
        result = builder.trunc(src_val, ir.IntType(32))
    else:
        # Unknown conversion - pass through
        result = src_val

    safe_vmap_write(vmap, dst_vid, result, f"convert_{from_type}_to_{to_type}")

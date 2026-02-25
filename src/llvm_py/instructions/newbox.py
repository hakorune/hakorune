"""
NewBox instruction lowering
Handles box creation (new StringBox(), new IntegerBox(), etc.)
"""

import llvmlite.ir as ir
import os
from typing import Dict, List, Optional, Any
from instructions.string_fast import can_reuse_literal_string_handle

def lower_newbox(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_type: str,
    args: List[int],
    dst_vid: int,
    vmap: Dict[int, ir.Value],
    resolver=None,
    ctx: Optional[Any] = None
) -> None:
    """
    Lower MIR NewBox instruction
    
    Creates a new box instance and returns its handle.
    
    Args:
        builder: Current LLVM IR builder
        module: LLVM module
        box_type: Box type name (e.g., "StringBox", "IntegerBox")
        args: Constructor arguments
        dst_vid: Destination value ID for box handle
        vmap: Value map
        resolver: Optional resolver for type handling
    """
    # Use NyRT shim: prefer dedicated core-box paths, otherwise env.box.new_i64x
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    fast_on = os.environ.get('NYASH_LLVM_FAST') == '1'

    def _declare(name: str, ret_ty, arg_tys):
        for f in module.functions:
            if f.name == name:
                return f
        return ir.Function(module, ir.FunctionType(ret_ty, arg_tys), name=name)

    def _resolve_arg(vid: int):
        try:
            v = vmap.get(vid)
            if v is not None:
                return v
        except (AttributeError, TypeError):
            pass
        if resolver is not None and hasattr(resolver, "resolve_i64") and ctx is not None:
            try:
                return resolver.resolve_i64(
                    vid,
                    builder.block,
                    getattr(ctx, "preds", {}),
                    getattr(ctx, "block_end_values", {}),
                    vmap,
                    getattr(ctx, "bb_map", {}),
                )
            except (AttributeError, TypeError, ValueError, KeyError):
                pass
        return None

    def _track_stringbox_origin():
        # Track StringBox creation for FAST path optimization
        # If newbox(StringBox, [string_arg]), store dst_vid -> string_arg mapping
        if box_type == "StringBox" and args and resolver is not None:
            try:
                if not hasattr(resolver, 'newbox_string_args'):
                    resolver.newbox_string_args = {}
                resolver.newbox_string_args[dst_vid] = args[0]
                if hasattr(resolver, 'mark_string'):
                    try:
                        resolver.mark_string(int(dst_vid))
                    except (TypeError, ValueError):
                        resolver.mark_string(dst_vid)
            except (AttributeError, TypeError, KeyError):
                pass

    if box_type == "StringBox":
        arg0_vid = args[0] if args else None
        arg0 = _resolve_arg(arg0_vid) if arg0_vid is not None else None

        # FAST path (opt-in): literal-backed constructor can reuse the
        # existing handle directly (const side already interns with NyRT helper).
        if fast_on and can_reuse_literal_string_handle(resolver, arg0_vid, arg0):
            vmap[dst_vid] = arg0
            _track_stringbox_origin()
            return

        if arg0 is not None:
            if isinstance(arg0.type, ir.IntType) and arg0.type.width == 64:
                # Handle -> UTF-8 pointer -> fresh StringBox handle
                to_i8p = _declare("nyash.string.to_i8p_h", i8p, [i64])
                from_i8 = _declare("nyash.box.from_i8_string", i64, [i8p])
                p = builder.call(to_i8p, [arg0], name="newbox_str_h2p")
                handle = builder.call(from_i8, [p], name="newbox_string_from_h")
            elif arg0.type.is_pointer:
                from_i8 = _declare("nyash.box.from_i8_string", i64, [i8p])
                p = arg0 if arg0.type == i8p else builder.bitcast(arg0, i8p, name="newbox_str_cast")
                handle = builder.call(from_i8, [p], name="newbox_string_from_p")
            else:
                new_str = _declare("nyash.string.new", i64, [])
                handle = builder.call(new_str, [], name="newbox_string_empty")
        else:
            new_str = _declare("nyash.string.new", i64, [])
            handle = builder.call(new_str, [], name="newbox_string_empty")

        vmap[dst_vid] = handle
        _track_stringbox_origin()
        return

    # Core fast paths
    if box_type in ("ArrayBox", "MapBox"):
        birth_name = "nyash.array.birth_h" if box_type == "ArrayBox" else "nyash.map.birth_h"
        birth = None
        for f in module.functions:
            if f.name == birth_name:
                birth = f
                break
        if not birth:
            birth = ir.Function(module, ir.FunctionType(i64, []), name=birth_name)
        handle = builder.call(birth, [], name=f"birth_{box_type}")
        vmap[dst_vid] = handle
        return
    # Prefer variadic shim: nyash.env.box.new_i64x(type_name, argc, a1, a2, a3, a4)
    new_i64x = None
    for f in module.functions:
        if f.name == "nyash.env.box.new_i64x":
            new_i64x = f
            break
    if not new_i64x:
        new_i64x = ir.Function(module, ir.FunctionType(i64, [i8p, i64, i64, i64, i64, i64]), name="nyash.env.box.new_i64x")

    # Build C-string for type name (unique global per function)
    sbytes = (box_type + "\0").encode('utf-8')
    arr_ty = ir.ArrayType(ir.IntType(8), len(sbytes))
    try:
        fn = builder.block.parent
        fn_name = getattr(fn, 'name', 'fn')
    except Exception:
        fn_name = 'fn'
    base = f".box_ty_{fn_name}_{dst_vid}"
    existing = {g.name for g in module.global_values}
    name = base
    n = 1
    while name in existing:
        name = f"{base}.{n}"; n += 1
    g = ir.GlobalVariable(module, arr_ty, name=name)
    g.linkage = 'private'
    g.global_constant = True
    g.initializer = ir.Constant(arr_ty, bytearray(sbytes))
    c0 = ir.Constant(ir.IntType(32), 0)
    ptr = builder.gep(g, [c0, c0], inbounds=True)
    zero = ir.Constant(i64, 0)
    handle = builder.call(new_i64x, [ptr, zero, zero, zero, zero, zero], name=f"new_{box_type}")
    vmap[dst_vid] = handle

    _track_stringbox_origin()

def lower_newbox_generic(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst_vid: int,
    vmap: Dict[int, ir.Value]
) -> None:
    """
    Create a generic box with runtime allocation
    
    This is used when box type is not statically known.
    """
    # Look up generic allocation function
    alloc_func = None
    for f in module.functions:
        if f.name == "ny_alloc_box":
            alloc_func = f
            break
    
    if not alloc_func:
        # Declare ny_alloc_box(size: i64) -> i64
        i64 = ir.IntType(64)
        func_type = ir.FunctionType(i64, [i64])
        alloc_func = ir.Function(module, func_type, name="ny_alloc_box")
    
    # Default box size (e.g., 64 bytes)
    size = ir.Constant(ir.IntType(64), 64)
    handle = builder.call(alloc_func, [size], name="new_box")
    
    vmap[dst_vid] = handle

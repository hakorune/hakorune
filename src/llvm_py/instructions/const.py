"""
Const instruction lowering
Handles integer, float, string, and void constants
"""

import llvmlite.ir as ir
import os
from typing import Dict, Any, Optional, Tuple
from utils.values import safe_vmap_write
from instructions.string_fast import string_const_boxer_symbol, llvm_fast_enabled

_I64 = ir.IntType(64)
_I64_CONST_CACHE: Dict[int, ir.Constant] = {}


def _const_i64(value: Any) -> ir.Constant:
    """Return cached i64 constant for hot const paths."""
    ival = int(value)
    cached = _I64_CONST_CACHE.get(ival)
    if cached is None:
        cached = ir.Constant(_I64, ival)
        _I64_CONST_CACHE[ival] = cached
    return cached


def _unique_global_name(module: ir.Module, base: str) -> str:
    existing = {g.name for g in module.global_values}
    name = base
    n = 1
    while name in existing:
        name = f"{base}.{n}"
        n += 1
    return name


def _entry_builder(entry_bb: Optional[ir.Block]) -> Optional[ir.IRBuilder]:
    if entry_bb is None:
        return None
    ib = ir.IRBuilder(entry_bb)
    term = getattr(entry_bb, "terminator", None)
    if term is not None:
        ib.position_before(term)
    else:
        ib.position_at_end(entry_bb)
    return ib


def _try_get_or_create_hoisted_string_const(
    builder: ir.IRBuilder,
    module: ir.Module,
    resolver,
    str_val: str,
) -> Optional[Tuple[ir.Value, ir.Value]]:
    if resolver is None or not llvm_fast_enabled():
        return None
    handles = getattr(resolver, "hoisted_string_handles", None)
    ptrs = getattr(resolver, "hoisted_string_ptrs", None)
    entry_bb = getattr(resolver, "entry_block", None)
    if not isinstance(handles, dict) or not isinstance(ptrs, dict) or entry_bb is None:
        return None
    if builder.block is None:
        return None

    # Dominance guard: only reuse entry-hoisted const for blocks reachable from entry.
    reachable = getattr(resolver, "reachable_block_ids", None)
    if isinstance(reachable, set):
        cur_bid = None
        try:
            cur_bid = int(str(builder.block.name).replace("bb", ""))
        except Exception:
            cur_bid = None
        if cur_bid is not None and cur_bid not in reachable:
            return None

    boxer_name = string_const_boxer_symbol()
    key = f"{boxer_name}:{str_val}"
    cached_h = handles.get(key)
    cached_p = ptrs.get(key)
    if cached_h is not None and cached_p is not None:
        return cached_h, cached_p

    ib = _entry_builder(entry_bb)
    if ib is None:
        return None

    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    str_bytes = str_val.encode("utf-8") + b"\0"
    arr_ty = ir.ArrayType(i8, len(str_bytes))
    str_const = ir.Constant(arr_ty, bytearray(str_bytes))
    try:
        fn = entry_bb.parent
        fn_name = getattr(fn, "name", "fn")
    except Exception:
        fn_name = "fn"
    gname = _unique_global_name(module, f".strhoist.{fn_name}")
    g = ir.GlobalVariable(module, arr_ty, name=gname)
    g.initializer = str_const
    g.linkage = "private"
    g.global_constant = True

    i32 = ir.IntType(32)
    c0 = ir.Constant(i32, 0)
    gep = ib.gep(g, [c0, c0], inbounds=True)
    boxer_ty = ir.FunctionType(ir.IntType(64), [i8p])
    boxer = None
    for f in module.functions:
        if f.name == boxer_name:
            boxer = f
            break
    if boxer is None:
        boxer = ir.Function(module, boxer_ty, name=boxer_name)
    handle = ib.call(boxer, [gep], name=f"const_str_h_hoist_{len(handles)}")

    handles[key] = handle
    ptrs[key] = gep
    return handle, gep

def lower_const(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst: int,
    value: Dict[str, Any],
    vmap: Dict[int, ir.Value],
    resolver=None
) -> None:
    """
    Lower MIR Const instruction
    
    Args:
        builder: Current LLVM IR builder
        module: LLVM module
        dst: Destination value ID
        value: Const value dict with 'type' and 'value' fields
        vmap: Value map (value_id -> llvm value)
    """
    const_type = value.get('type', 'void')
    const_val = value.get('value')
    
    if const_type == 'i64':
        # Integer constant
        llvm_val = _const_i64(const_val)
        # Phase 131-12-P1: Object identity trace before write
        import sys
        if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
            print(f"[vmap/id] const dst={dst} vmap id={id(vmap)} before_write", file=sys.stderr)
        safe_vmap_write(vmap, dst, llvm_val, "const_i64", resolver=resolver)

    elif const_type == 'f64':
        # Float constant
        f64 = ir.DoubleType()
        llvm_val = ir.Constant(f64, float(const_val))
        safe_vmap_write(vmap, dst, llvm_val, "const_f64", resolver=resolver)
        
    elif const_type == 'string' or (isinstance(const_type, dict) and const_type.get('kind') in ('handle','ptr') and const_type.get('box_type') == 'StringBox'):
        str_val = str(const_val)
        hoisted = _try_get_or_create_hoisted_string_const(builder, module, resolver, str_val)
        if hoisted is not None:
            handle, gep = hoisted
            safe_vmap_write(vmap, dst, handle, "const_string_hoist", resolver=resolver)
            if resolver is not None:
                if hasattr(resolver, 'string_literals'):
                    resolver.string_literals[dst] = str_val
                if hasattr(resolver, 'mark_string'):
                    resolver.mark_string(dst)
                try:
                    resolver.string_ptrs[dst] = gep
                except Exception:
                    pass
            return

        # String constant - create global and immediately box to i64 handle
        i8 = ir.IntType(8)
        str_bytes = str_val.encode('utf-8') + b'\0'
        arr_ty = ir.ArrayType(i8, len(str_bytes))
        str_const = ir.Constant(arr_ty, bytearray(str_bytes))
        try:
            fn = builder.block.parent
            fn_name = getattr(fn, 'name', 'fn')
        except Exception:
            fn_name = 'fn'
        name = _unique_global_name(module, f".str.{fn_name}.{dst}")
        g = ir.GlobalVariable(module, arr_ty, name=name)
        g.initializer = str_const
        g.linkage = 'private'
        g.global_constant = True
        # GEP to first element and box to handle immediately
        i32 = ir.IntType(32)
        c0 = ir.Constant(i32, 0)
        gep = builder.gep(g, [c0, c0], inbounds=True)
        i8p = i8.as_pointer()
        boxer_ty = ir.FunctionType(ir.IntType(64), [i8p])
        boxer_name = string_const_boxer_symbol()
        boxer = None
        for f in module.functions:
            if f.name == boxer_name:
                boxer = f
                break
        if boxer is None:
            boxer = ir.Function(module, boxer_ty, name=boxer_name)
        handle = builder.call(boxer, [gep], name=f"const_str_h_{dst}")
        safe_vmap_write(vmap, dst, handle, "const_string", resolver=resolver)
        if resolver is not None:
            if hasattr(resolver, 'string_literals'):
                resolver.string_literals[dst] = str_val
            # Mark this value-id as string-ish to guide '+' and '==' lowering
            if hasattr(resolver, 'mark_string'):
                resolver.mark_string(dst)
            # Keep raw pointer for potential pointer-API sites (e.g., console.log)
            try:
                resolver.string_ptrs[dst] = gep
            except Exception:
                pass

    elif const_type == 'void':
        # Void/null constant - use i64 zero
        safe_vmap_write(vmap, dst, _const_i64(0), "const_void", resolver=resolver)

    else:
        # Unknown type - default to i64 zero
        safe_vmap_write(vmap, dst, _const_i64(0), "const_unknown", resolver=resolver)

"""
Phase 134-B: StringBox LLVM Bridge - StringBox 統合モジュール

目的:
- StringBox メソッド (length/len/substring/lastIndexOf) の LLVM IR 変換を1箇所に集約
- BoxCall lowering 側の分岐を削除し、箱化モジュール化を実現

設計原則:
- Phase 133 ConsoleLlvmBridge パターンを継承
- 複雑な最適化パス (NYASH_LLVM_FAST, NYASH_STR_CP) を統合
- literal folding, length_cache 等の高度な最適化を含む
"""

import llvmlite.ir as ir
from typing import Dict, List, Optional, Any
import os


# StringBox method mapping (TypeRegistry slots 410-413)
STRINGBOX_METHODS = {
    "length": 410,
    "len": 410,  # Alias for length
    "substring": 411,
    "lastIndexOf": 412,
    "indexOf": 413,
}


def _declare(module: ir.Module, name: str, ret, args):
    """Declare or get existing function"""
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


def _ensure_handle(builder: ir.IRBuilder, module: ir.Module, v: ir.Value) -> ir.Value:
    """Coerce a value to i64 handle. If pointer, box via nyash.box.from_i8_string."""
    i64 = ir.IntType(64)
    if hasattr(v, 'type'):
        if isinstance(v.type, ir.IntType) and v.type.width == 64:
            return v
        if isinstance(v.type, ir.PointerType):
            # call nyash.box.from_i8_string(i8*) -> i64
            i8p = ir.IntType(8).as_pointer()
            # If pointer-to-array, GEP to first element
            try:
                if isinstance(v.type.pointee, ir.ArrayType):
                    c0 = ir.IntType(32)(0)
                    v = builder.gep(v, [c0, c0], name="sb_str_gep")
            except Exception:
                pass
            callee = _declare(module, "nyash.box.from_i8_string", i64, [i8p])
            return builder.call(callee, [v], name="str_ptr2h_sb")
        if isinstance(v.type, ir.IntType):
            # extend/trunc to i64
            return builder.zext(v, i64) if v.type.width < 64 else builder.trunc(v, i64)
    return ir.Constant(i64, 0)


def emit_stringbox_call(
    builder: ir.IRBuilder,
    module: ir.Module,
    method_name: str,
    recv_val: ir.Value,
    args: List[int],
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    box_vid: int,
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None,
) -> bool:
    """
    Emit StringBox method call to LLVM IR.

    Returns:
        True if method was handled, False if not a StringBox method

    Args:
        builder: LLVM IR builder
        module: LLVM module
        method_name: StringBox method name (length/len/substring/lastIndexOf/indexOf)
        recv_val: Receiver value (StringBox instance)
        args: Argument value IDs
        dst_vid: Destination value ID
        vmap: Value map
        box_vid: Box value ID
        resolver: Optional type resolver
        preds: Predecessor map
        block_end_values: Block end values
        bb_map: Basic block map
        ctx: Build context
    """
    # Check if this is a StringBox method
    if method_name not in STRINGBOX_METHODS:
        return False

    i64 = ir.IntType(64)

    # Extract resolver/preds from ctx if available
    r = resolver
    p = preds
    bev = block_end_values
    bbm = bb_map
    if ctx is not None:
        try:
            r = getattr(ctx, 'resolver', r)
            p = getattr(ctx, 'preds', p)
            bev = getattr(ctx, 'block_end_values', bev)
            bbm = getattr(ctx, 'bb_map', bbm)
        except Exception:
            pass

    def _res_i64(vid: int):
        """Resolve value ID to i64 via resolver or vmap"""
        if r is not None and p is not None and bev is not None and bbm is not None:
            try:
                return r.resolve_i64(vid, builder.block, p, bev, vmap, bbm)
            except Exception:
                return None
        return vmap.get(vid)

    # Dispatch to method-specific handlers
    if method_name in ("length", "len"):
        return _emit_length(
            builder, module, recv_val, args, dst_vid, vmap, box_vid, r, p, bev, bbm
        )
    elif method_name == "substring":
        return _emit_substring(
            builder, module, recv_val, args, dst_vid, vmap, r, p, bev, bbm, _res_i64
        )
    elif method_name == "lastIndexOf":
        return _emit_lastindexof(
            builder, module, recv_val, args, dst_vid, vmap, r, p, bev, bbm, _res_i64
        )
    elif method_name == "indexOf":
        return _emit_indexof(
            builder, module, recv_val, args, dst_vid, vmap, r, p, bev, bbm, _res_i64
        )

    return False


def _emit_length(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_val: ir.Value,
    args: List[int],
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    box_vid: int,
    resolver,
    preds,
    block_end_values,
    bb_map,
) -> bool:
    """
    Emit StringBox.length() / StringBox.len() to LLVM IR.

    Supports:
    - NYASH_LLVM_FAST: Fast path optimization
    - literal folding: "hello".length() -> 5
    - length_cache: cache computed lengths
    """
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()

    # Check NYASH_LLVM_FAST flag
    fast_on = os.environ.get('NYASH_LLVM_FAST') == '1'
    force_bridge = os.environ.get('NYASH_LEN_FORCE_BRIDGE') == '1'

    def _cache_len(val):
        if not fast_on or resolver is None or dst_vid is None or box_vid is None:
            return
        cache = getattr(resolver, 'length_cache', None)
        if cache is None:
            return
        try:
            cache[int(box_vid)] = val
        except Exception:
            pass

    # Fast path: check length_cache
    if fast_on and resolver is not None and dst_vid is not None and box_vid is not None:
        cache = getattr(resolver, 'length_cache', None)
        if cache is not None:
            try:
                cached = cache.get(int(box_vid))
            except Exception:
                cached = None
            if cached is not None:
                vmap[dst_vid] = cached
                return True

    # Ultra-fast: literal length folding
    if fast_on and dst_vid is not None and resolver is not None:
        try:
            lit = None
            arg_vid = None

            # Case A: newbox(StringBox, const)
            if hasattr(resolver, 'newbox_string_args'):
                arg_vid = resolver.newbox_string_args.get(int(box_vid))
            if arg_vid is not None and hasattr(resolver, 'string_literals'):
                lit = resolver.string_literals.get(int(arg_vid))

            # Case B: receiver itself is a literal-backed handle
            if lit is None and hasattr(resolver, 'string_literals'):
                lit = resolver.string_literals.get(int(box_vid))

            if isinstance(lit, str):
                # Compute length based on mode
                use_cp = _codepoint_mode()
                n = len(lit) if use_cp else len(lit.encode('utf-8'))
                const_len = ir.Constant(i64, n)
                vmap[dst_vid] = const_len
                _cache_len(const_len)
                return True
        except Exception:
            pass

    # Fast path: use string_ptrs for direct strlen
    if fast_on and resolver is not None and hasattr(resolver, 'string_ptrs'):
        try:
            ptr = resolver.string_ptrs.get(int(box_vid))
        except Exception:
            ptr = None

        # Fallback: check newbox_string_args
        if ptr is None and hasattr(resolver, 'newbox_string_args'):
            try:
                arg_vid = resolver.newbox_string_args.get(int(box_vid))
                if arg_vid is not None:
                    ptr = resolver.string_ptrs.get(int(arg_vid))
            except Exception:
                pass

        if ptr is not None:
            return _fast_strlen(builder, module, ptr, dst_vid, vmap, _cache_len)

    # Fast path: if receiver is known stringish handle, prefer direct len_h by default.
    # Legacy bridge path can be forced via NYASH_LEN_FORCE_BRIDGE=1.
    if fast_on and resolver is not None and hasattr(resolver, 'is_stringish'):
        try:
            if resolver.is_stringish(int(box_vid)):
                recv_h = _ensure_handle(builder, module, recv_val)
                if not force_bridge:
                    callee = _declare(module, "nyash.string.len_h", i64, [i64])
                    result = builder.call(callee, [recv_h], name="string_len_h")
                    if dst_vid is not None:
                        vmap[dst_vid] = result
                    _cache_len(result)
                    return True
                to_i8p = _declare(module, "nyash.string.to_i8p_h", i8p, [i64])
                ptr = builder.call(to_i8p, [recv_h], name="strlen_h2p")
                return _fast_strlen(builder, module, ptr, dst_vid, vmap, _cache_len)
        except Exception:
            pass

    # Default: Any.length_h(handle) -> i64
    recv_h = _ensure_handle(builder, module, recv_val)
    callee = _declare(module, "nyash.any.length_h", i64, [i64])
    result = builder.call(callee, [recv_h], name="any_length_h")
    if dst_vid is not None:
        vmap[dst_vid] = result
    return True


def _emit_substring(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_val: ir.Value,
    args: List[int],
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    resolver,
    preds,
    block_end_values,
    bb_map,
    _res_i64,
) -> bool:
    """
    Emit StringBox.substring(start, end) to LLVM IR.

    Supports:
    - NYASH_STR_CP: Code point vs UTF-8 byte mode
    """
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()

    # Get start and end indices
    s = _res_i64(args[0]) if args else ir.Constant(i64, 0)
    if s is None:
        s = vmap.get(args[0], ir.Constant(i64, 0)) if args else ir.Constant(i64, 0)

    e = _res_i64(args[1]) if len(args) > 1 else ir.Constant(i64, 0)
    if e is None:
        e = vmap.get(args[1], ir.Constant(i64, 0)) if len(args) > 1 else ir.Constant(i64, 0)

    # Handle-based path
    if hasattr(recv_val, 'type') and isinstance(recv_val.type, ir.IntType):
        callee = _declare(module, "nyash.string.substring_hii", i64, [i64, i64, i64])
        h = builder.call(callee, [recv_val, s, e], name="substring_h")
        if dst_vid is not None:
            vmap[dst_vid] = h
            try:
                if resolver is not None and hasattr(resolver, 'mark_string'):
                    resolver.mark_string(dst_vid)
            except Exception:
                pass
        return True

    # Pointer-based path
    recv_p = recv_val
    if hasattr(recv_p, 'type') and isinstance(recv_p.type, ir.PointerType):
        try:
            if isinstance(recv_p.type.pointee, ir.ArrayType):
                c0 = ir.Constant(ir.IntType(32), 0)
                recv_p = builder.gep(recv_p, [c0, c0], name="sb_gep_recv")
        except Exception:
            pass
    else:
        recv_p = ir.Constant(i8p, None)

    # Coerce indices
    if hasattr(s, 'type') and isinstance(s.type, ir.PointerType):
        s = builder.ptrtoint(s, i64)
    if hasattr(e, 'type') and isinstance(e.type, ir.PointerType):
        e = builder.ptrtoint(e, i64)

    callee = _declare(module, "nyash.string.substring_sii", i8p, [i8p, i64, i64])
    p = builder.call(callee, [recv_p, s, e], name="substring")
    conv = _declare(module, "nyash.box.from_i8_string", i64, [i8p])
    h = builder.call(conv, [p], name="str_ptr2h_sub")

    if dst_vid is not None:
        vmap[dst_vid] = h
        try:
            if resolver is not None and hasattr(resolver, 'mark_string'):
                resolver.mark_string(dst_vid)
            if resolver is not None and hasattr(resolver, 'string_ptrs'):
                resolver.string_ptrs[int(dst_vid)] = p
        except Exception:
            pass

    return True


def _emit_lastindexof(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_val: ir.Value,
    args: List[int],
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    resolver,
    preds,
    block_end_values,
    bb_map,
    _res_i64,
) -> bool:
    """
    Emit StringBox.lastIndexOf(needle) to LLVM IR.
    """
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()

    # Get needle argument
    n_i64 = _res_i64(args[0]) if args else ir.Constant(i64, 0)
    if n_i64 is None:
        n_i64 = vmap.get(args[0], ir.Constant(i64, 0)) if args else ir.Constant(i64, 0)

    # Handle-based path
    if hasattr(recv_val, 'type') and isinstance(recv_val.type, ir.IntType):
        callee = _declare(module, "nyash.string.lastIndexOf_hh", i64, [i64, i64])
        res = builder.call(callee, [recv_val, n_i64], name="lastIndexOf_hh")
        if dst_vid is not None:
            vmap[dst_vid] = res
        return True

    # Pointer-based path
    recv_p = recv_val
    if hasattr(recv_p, 'type') and isinstance(recv_p.type, ir.PointerType):
        try:
            if isinstance(recv_p.type.pointee, ir.ArrayType):
                c0 = ir.Constant(ir.IntType(32), 0)
                recv_p = builder.gep(recv_p, [c0, c0], name="sb_gep_recv2")
        except Exception:
            pass
    else:
        recv_p = ir.Constant(i8p, None)

    # Convert needle to pointer
    needle = n_i64
    if hasattr(needle, 'type') and isinstance(needle.type, ir.IntType):
        needle = builder.inttoptr(needle, i8p, name="sb_i2p_needle")
    elif hasattr(needle, 'type') and isinstance(needle.type, ir.PointerType):
        try:
            if isinstance(needle.type.pointee, ir.ArrayType):
                c0 = ir.Constant(ir.IntType(32), 0)
                needle = builder.gep(needle, [c0, c0], name="sb_gep_needle")
        except Exception:
            pass

    callee = _declare(module, "nyash.string.lastIndexOf_ss", i64, [i8p, i8p])
    res = builder.call(callee, [recv_p, needle], name="lastIndexOf")
    if dst_vid is not None:
        vmap[dst_vid] = res

    return True


def _emit_indexof(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_val: ir.Value,
    args: List[int],
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    resolver,
    preds,
    block_end_values,
    bb_map,
    _res_i64,
) -> bool:
    """
    Emit StringBox.indexOf(needle) to LLVM IR.
    """
    i64 = ir.IntType(64)

    # Get needle argument
    needle_val = _res_i64(args[0]) if args else ir.Constant(i64, 0)
    if needle_val is None:
        needle_val = vmap.get(args[0], ir.Constant(i64, 0)) if args else ir.Constant(i64, 0)

    recv_h = _ensure_handle(builder, module, recv_val)
    needle_h = _ensure_handle(builder, module, needle_val)

    callee = _declare(module, "nyash.string.indexOf_hh", i64, [i64, i64])
    res = builder.call(callee, [recv_h, needle_h], name="indexOf_hh")
    if dst_vid is not None:
        vmap[dst_vid] = res

    return True


# Helper functions

def _literal_fold_length(literal_str: str) -> int:
    """
    Compute literal StringBox length at compile-time.

    Example: "hello".length() -> 5
    """
    use_cp = _codepoint_mode()
    return len(literal_str) if use_cp else len(literal_str.encode('utf-8'))


def _fast_strlen(
    builder: ir.IRBuilder,
    module: ir.Module,
    ptr: ir.Value,
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    cache_callback,
) -> bool:
    """
    NYASH_LLVM_FAST path for optimized strlen implementation.
    """
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()

    mode = 1 if _codepoint_mode() else 0
    mode_c = ir.Constant(i64, mode)

    # Prefer neutral kernel symbol
    callee = _declare(module, "nyrt_string_length", i64, [i8p, i64])
    result = builder.call(callee, [ptr, mode_c], name="strlen_si")

    if dst_vid is not None:
        vmap[dst_vid] = result
        cache_callback(result)

    return True


def _codepoint_mode() -> bool:
    """
    Check NYASH_STR_CP flag to determine code point / UTF-8 byte mode.

    Returns:
        True if code point mode, False if UTF-8 byte mode
    """
    return os.environ.get('NYASH_STR_CP') == '1'


# Phase 134-B: Diagnostic helpers

def get_stringbox_method_info(method_name: str) -> Optional[Dict[str, Any]]:
    """
    Get StringBox method metadata for debugging/diagnostics.

    Returns:
        Dict with keys: slot, arity, is_alias
        None if not a StringBox method
    """
    if method_name not in STRINGBOX_METHODS:
        return None

    arity_map = {
        "length": 0,
        "len": 0,
        "substring": 2,
        "lastIndexOf": 1,
        "indexOf": 1,
    }

    return {
        "slot": STRINGBOX_METHODS[method_name],
        "arity": arity_map[method_name],
        "is_alias": method_name == "len",
    }

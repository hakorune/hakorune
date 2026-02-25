"""
Phase 133: Console LLVM Bridge - ConsoleBox 統合モジュール

目的:
- ConsoleBox メソッド (log/println/warn/error/clear) の LLVM IR 変換を1箇所に集約
- BoxCall lowering 側の分岐を削除し、箱化モジュール化を実現

設計原則:
- Rust VM の TypeRegistry (slot 400-403) と完全一致
- Phase 122 の println/log エイリアス統一を踏襲
- ABI は nyash.console.* シリーズで統一 (i8* ptr, i64 len)
"""

import llvmlite.ir as ir
from typing import Dict, List, Optional, Any

# Console method mapping (Phase 122 unified)
# slot 400: log/println (alias)
# slot 401: warn
# slot 402: error
# slot 403: clear

CONSOLE_METHODS = {
    "log": "nyash.console.log",
    "println": "nyash.console.log",  # Phase 122: log のエイリアス
    "warn": "nyash.console.warn",
    "error": "nyash.console.error",
    "clear": "nyash.console.clear",
}


def _declare(module: ir.Module, name: str, ret, args):
    """Declare or get existing function"""
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


def _ensure_i8_ptr(builder: ir.IRBuilder, module: ir.Module, v: ir.Value) -> ir.Value:
    """
    Convert any value to i8* for console output.

    Handles:
    - i64 handle → nyash.string.to_i8p_h(i64) → i8*
    - i8* → pass through
    - pointer to array → GEP to first element
    """
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()

    if hasattr(v, 'type'):
        # Already i8*, pass through
        if isinstance(v.type, ir.PointerType):
            pointee = v.type.pointee
            if isinstance(pointee, ir.IntType) and pointee.width == 8:
                return v
            # Pointer to array: GEP to first element
            if isinstance(pointee, ir.ArrayType):
                c0 = ir.IntType(32)(0)
                return builder.gep(v, [c0, c0], name="cons_arr_gep")

        # i64 handle: convert via bridge
        if isinstance(v.type, ir.IntType):
            if v.type.width == 64:
                bridge = _declare(module, "nyash.string.to_i8p_h", i8p, [i64])
                return builder.call(bridge, [v], name="str_h2p_cons")
            # Other int widths: extend to i64, then convert
            v_i64 = builder.zext(v, i64) if v.type.width < 64 else builder.trunc(v, i64)
            bridge = _declare(module, "nyash.string.to_i8p_h", i8p, [i64])
            return builder.call(bridge, [v_i64], name="str_h2p_cons")

    # Fallback: null pointer
    return ir.Constant(i8p, None)


def emit_console_call(
    builder: ir.IRBuilder,
    module: ir.Module,
    method_name: str,
    args: List[int],
    dst_vid: Optional[int],
    vmap: Dict[int, ir.Value],
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None,
) -> bool:
    """
    Emit ConsoleBox method call to LLVM IR.

    Returns:
        True if method was handled, False if not a Console method

    Args:
        builder: LLVM IR builder
        module: LLVM module
        method_name: Console method name (log/println/warn/error/clear)
        args: Argument value IDs
        dst_vid: Destination value ID (usually None for Console methods)
        vmap: Value map
        resolver: Optional type resolver
        preds: Predecessor map
        block_end_values: Block end values
        bb_map: Basic block map
        ctx: Build context
    """
    # Check if this is a Console method
    if method_name not in CONSOLE_METHODS:
        return False

    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()

    # Get target runtime function name
    runtime_fn_name = CONSOLE_METHODS[method_name]

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

    # clear() takes no arguments
    if method_name == "clear":
        callee = _declare(module, runtime_fn_name, ir.VoidType(), [])
        builder.call(callee, [], name="console_clear")
        if dst_vid is not None:
            vmap[dst_vid] = ir.Constant(i64, 0)
        return True

    # log/println/warn/error take 1 string argument
    if not args:
        # No argument provided, use empty string
        arg0_ptr = ir.Constant(i8p, None)
    else:
        arg0_vid = args[0]

        # Try to get pointer directly from resolver.string_ptrs (fast path)
        arg0_ptr = None
        if r is not None and hasattr(r, 'string_ptrs'):
            try:
                arg0_ptr = r.string_ptrs.get(int(arg0_vid))
            except Exception:
                pass

        # Fallback: resolve value and convert to i8*
        if arg0_ptr is None:
            arg0_val = vmap.get(arg0_vid)
            if arg0_val is None and r is not None:
                arg0_val = _res_i64(arg0_vid)
            if arg0_val is None:
                arg0_val = ir.Constant(i64, 0)

            arg0_ptr = _ensure_i8_ptr(builder, module, arg0_val)

    # Emit call: void @nyash.console.{log,warn,error}(i8* %ptr)
    # Note: Current ABI uses i8* only (null-terminated), not i8* + i64 len
    callee = _declare(module, runtime_fn_name, i64, [i8p])
    builder.call(callee, [arg0_ptr], name=f"console_{method_name}")

    # Console methods return void (treated as 0)
    if dst_vid is not None:
        vmap[dst_vid] = ir.Constant(i64, 0)

    return True


# Phase 133: Diagnostic helpers

def get_console_method_info(method_name: str) -> Optional[Dict[str, Any]]:
    """
    Get Console method metadata for debugging/diagnostics.

    Returns:
        Dict with keys: runtime_fn, slot, arity, is_alias
        None if not a Console method
    """
    if method_name not in CONSOLE_METHODS:
        return None

    # Slot mapping (from TypeRegistry)
    slot_map = {
        "log": 400,
        "println": 400,  # Alias
        "warn": 401,
        "error": 402,
        "clear": 403,
    }

    return {
        "runtime_fn": CONSOLE_METHODS[method_name],
        "slot": slot_map[method_name],
        "arity": 0 if method_name == "clear" else 1,
        "is_alias": method_name == "println",
    }


def validate_console_abi(module: ir.Module) -> List[str]:
    """
    Validate that Console runtime functions are correctly declared.

    Returns:
        List of validation errors (empty if all OK)
    """
    errors = []
    i8p = ir.IntType(8).as_pointer()
    i64 = ir.IntType(64)
    void = ir.VoidType()

    expected = {
        "nyash.console.log": (i64, [i8p]),
        "nyash.console.warn": (i64, [i8p]),
        "nyash.console.error": (i64, [i8p]),
        "nyash.console.clear": (void, []),
    }

    for fn_name, (ret_ty, arg_tys) in expected.items():
        found = None
        for f in module.functions:
            if f.name == fn_name:
                found = f
                break

        if found is None:
            continue  # Not yet declared, OK

        # Check signature
        if str(found.return_value.type) != str(ret_ty):
            errors.append(f"{fn_name}: return type mismatch (expected {ret_ty}, got {found.return_value.type})")

        if len(found.args) != len(arg_tys):
            errors.append(f"{fn_name}: arg count mismatch (expected {len(arg_tys)}, got {len(found.args)})")
        else:
            for i, (expected_ty, actual_arg) in enumerate(zip(arg_tys, found.args)):
                if str(actual_arg.type) != str(expected_ty):
                    errors.append(f"{fn_name}: arg {i} type mismatch (expected {expected_ty}, got {actual_arg.type})")

    return errors

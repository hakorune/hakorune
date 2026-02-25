"""
Lifecycle management instruction lowering (Phase 287)
KeepAlive and ReleaseStrong for reference counting semantics
"""

import llvmlite.ir as ir
from typing import Dict, List, Any, Optional

def lower_keepalive(
    builder: ir.IRBuilder,
    module: ir.Module,
    values: List[int],
    vmap: Dict[int, ir.Value],
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None
) -> None:
    """
    Lower MIR KeepAlive instruction

    KeepAlive is a no-op in LLVM backend - it only affects DCE/liveness
    analysis in MIR optimization passes.

    Args:
        builder: Current LLVM IR builder
        module: LLVM module
        values: List of value IDs to keep alive
        vmap: Value map
    """
    # No-op: KeepAlive only affects MIR DCE/liveness analysis
    pass

def lower_release_strong(
    builder: ir.IRBuilder,
    module: ir.Module,
    values: List[int],
    vmap: Dict[int, ir.Value],
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None
) -> None:
    """
    Lower MIR ReleaseStrong instruction

    Releases strong references to the specified values, potentially
    triggering deallocation if reference count drops to zero.

    Args:
        builder: Current LLVM IR builder
        module: LLVM module
        values: List of value IDs to release
        vmap: Value map
    """
    # Look up or declare ny_release_strong function
    release_func = None
    for f in module.functions:
        if f.name == "ny_release_strong":
            release_func = f
            break

    if not release_func:
        # Declare ny_release_strong(handle: i64) -> void
        i64 = ir.IntType(64)
        void = ir.VoidType()
        func_type = ir.FunctionType(void, [i64])
        release_func = ir.Function(module, func_type, name="ny_release_strong")

    # Release each value
    i64 = ir.IntType(64)
    for vid in values:
        # Resolve value (prefer BuildCtx if provided)
        r = resolver; p = preds; bev = block_end_values; bbm = bb_map
        if ctx is not None:
            try:
                r = getattr(ctx, 'resolver', r)
                p = getattr(ctx, 'preds', p)
                bev = getattr(ctx, 'block_end_values', bev)
                bbm = getattr(ctx, 'bb_map', bbm)
            except Exception:
                pass

        if r is not None and p is not None and bev is not None and bbm is not None:
            val = r.resolve_i64(vid, builder.block, p, bev, vmap, bbm)
        else:
            val = vmap.get(vid, ir.Constant(i64, 0))

        # Ensure i64 (handles are i64)
        if hasattr(val, 'type') and val.type.is_pointer:
            val = builder.ptrtoint(val, i64, name=f"release_p2i_{vid}")

        # Call release function
        builder.call(release_func, [val], name=f"release_{vid}")

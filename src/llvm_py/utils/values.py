"""
Value resolution helpers
Centralize policies like "prefer same-block SSA; otherwise resolve with dominance".
"""

from typing import Any, Dict, Optional
import llvmlite.ir as ir
import sys
import os
from trace import hot_count as trace_hot_count
from phi_wiring.debug_helper import is_phi_debug_enabled, is_phi_trace_enabled

def resolve_i64_strict(
    resolver,
    value_id: int,
    current_block: ir.Block,
    preds: Dict[int, list],
    block_end_values: Dict[int, Dict[int, Any]],
    vmap: Dict[int, Any],
    bb_map: Optional[Dict[int, ir.Block]] = None,
    *,
    prefer_local: bool = True,
    hot_scope: str = "",
) -> ir.Value:
    """Resolve i64 under policies:
    - If prefer_local and vmap has a same-block definition, reuse it.
    - Otherwise, delegate to resolver to localize with PHI/casts as needed.
    """
    debug = is_phi_debug_enabled()

    # Prefer current vmap SSA first (block-local map is passed in vmap)
    scope = hot_scope if hot_scope else "generic"
    val = vmap.get(value_id)
    if debug:
        val_type = type(val).__name__ if val is not None else "None"
        from trace import phi as trace_phi
        trace_phi(f"[resolve_i64_strict] v{value_id} vmap={val_type}")
    if prefer_local and val is not None:
        trace_hot_count(resolver, f"resolve_local_hit_{scope}")
        if debug:
            trace_phi(f"[resolve_i64_strict] v{value_id} -> local vmap")
        return val
    # If local map misses, try builder-global vmap (e.g., predeclared PHIs)
    try:
        if hasattr(resolver, 'global_vmap') and isinstance(resolver.global_vmap, dict):
            gval = resolver.global_vmap.get(value_id)
            if gval is not None:
                trace_hot_count(resolver, f"resolve_global_hit_{scope}")
                if debug:
                    trace_phi(f"[resolve_i64_strict] v{value_id} -> global_vmap")
                return gval
    except Exception:
        pass
    # Fallback to resolver
    if resolver is None:
        if debug:
            trace_phi(f"[resolve_i64_strict] v{value_id} -> 0 (no resolver)")
        return ir.Constant(ir.IntType(64), 0)
    if debug:
        trace_phi(f"[resolve_i64_strict] v{value_id} -> resolver.resolve_i64")
    trace_hot_count(resolver, f"resolve_fallback_{scope}")
    return resolver.resolve_i64(value_id, current_block, preds, block_end_values, vmap, bb_map)

def safe_vmap_write(vmap: Dict[int, Any], dst: int, value: Any, context: str = "", resolver=None, block_id: Optional[int] = None) -> None:
    """
    PHI overwrite protection for vmap writes + def_blocks registration (P0-1 unified).

    Implements fail-fast protection against ValueId namespace collisions.

    Args:
        vmap: Value map to write to
        dst: Destination ValueId
        value: LLVM IR value to write
        context: Context string for error messages (e.g., "const", "binop")
        resolver: Optional resolver for def_blocks tracking (P0-1)
        block_id: Optional block ID for def_blocks registration (P0-1)

    Behavior:
        - STRICT mode (NYASH_LLVM_STRICT=1): Raises error if overwriting PHI
        - TRACE mode (NYASH_LLVM_TRACE_VMAP=1): Logs warning but skips overwrite
        - Default: Silently skips PHI overwrite (SSOT: PHI defined once)
        - P0-1: If resolver and block_id provided, register in def_blocks
    """
    existing = vmap.get(dst)
    if existing is not None and hasattr(existing, 'add_incoming'):
        # PHI node detected - overwrite forbidden (SSOT principle)
        if os.environ.get('NYASH_LLVM_STRICT') == '1':
            raise RuntimeError(
                f"[LLVM_PY/{context}] Cannot overwrite PHI dst={dst}. "
                f"ValueId namespace collision detected. "
                f"Existing: PHI node, Attempted: {type(value).__name__}"
            )
        # STRICT not enabled - warn and skip
        if is_phi_trace_enabled():
            print(f"[vmap/warn] Skipping overwrite of PHI dst={dst} in context={context}", file=sys.stderr)
        return  # Do not overwrite PHI

    # Safe to write
    vmap[dst] = value

    # Phase 131-12-P1: Trace successful write
    if is_phi_trace_enabled():
        print(f"[vmap/write] dst={dst} written, vmap.keys()={sorted(vmap.keys())[:20]}", file=sys.stderr)

    # P0-1: Register definition in def_blocks for dominance tracking (SSOT for all instructions)
    if resolver is not None and hasattr(resolver, 'def_blocks'):
        # Auto-detect block_id from resolver if not provided explicitly
        bid = block_id
        if bid is None and hasattr(resolver, 'current_block_id'):
            bid = resolver.current_block_id

        if bid is not None:
            resolver.def_blocks.setdefault(dst, set()).add(bid)
            if os.environ.get('NYASH_LLVM_TRACE_VMAP') == '1':
                print(f"[vmap/def_blocks] Registered v{dst} in block {bid} (context={context})", file=sys.stderr)

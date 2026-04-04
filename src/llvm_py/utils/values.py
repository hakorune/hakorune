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


def _block_id_from_block_name(block: Any) -> Optional[int]:
    try:
        name = getattr(block, "name", None)
        if isinstance(name, bytes):
            name = name.decode()
        if isinstance(name, str) and name.startswith("bb"):
            return int(name[2:])
    except Exception:
        return None
    return None


def _phi_owner_name(candidate: Any) -> Optional[str]:
    try:
        owner = getattr(getattr(candidate, "basic_block", None), "name", None)
        if owner is None:
            owner = getattr(getattr(candidate, "parent", None), "name", None)
        if isinstance(owner, bytes):
            owner = owner.decode()
        return owner if isinstance(owner, str) else None
    except Exception:
        return None


def _block_name(block: Any) -> Optional[str]:
    try:
        name = getattr(block, "name", None)
        if isinstance(name, bytes):
            name = name.decode()
        return name if isinstance(name, str) else None
    except Exception:
        return None


def _same_block_phi(candidate: Any, current_block: ir.Block) -> bool:
    try:
        if candidate is None or not hasattr(candidate, "add_incoming"):
            return False
        return _phi_owner_name(candidate) == _block_name(current_block)
    except Exception:
        return False


def _defined_in_block(resolver: Any, value_id: int, block_id: Optional[int]) -> bool:
    try:
        if block_id is None:
            return False
        def_blocks = getattr(resolver, "def_blocks", {})
        return (
            isinstance(def_blocks, dict)
            and value_id in def_blocks
            and int(block_id) in def_blocks.get(value_id, set())
        )
    except Exception:
        return False


def _single_def_dominates_block(resolver: Any, value_id: int, block_id: Optional[int]) -> bool:
    try:
        if block_id is None:
            return False
        def_blocks = getattr(resolver, "def_blocks", {})
        defs = def_blocks.get(value_id, set()) if isinstance(def_blocks, dict) else set()
        if len(defs) != 1:
            return False
        def_bid = next(iter(defs))
        ctx = getattr(resolver, "context", None)
        if ctx is None or not hasattr(ctx, "dominates"):
            return False
        return bool(ctx.dominates(int(def_bid), int(block_id)))
    except Exception:
        return False


def _phi_owner_dominates_block(resolver: Any, candidate: Any, block_id: Optional[int]) -> bool:
    try:
        if block_id is None or candidate is None or not hasattr(candidate, "add_incoming"):
            return False
        phi_bid = _block_id_from_block_name(getattr(candidate, "basic_block", None))
        if phi_bid is None:
            return False
        ctx = getattr(resolver, "context", None)
        if ctx is None or not hasattr(ctx, "dominates"):
            return False
        return bool(ctx.dominates(phi_bid, int(block_id)))
    except Exception:
        return False


def _global_reuse_allowed(resolver: Any, value_id: int, candidate: Any, current_block: ir.Block) -> bool:
    try:
        if candidate is None:
            return False
        if isinstance(candidate, (ir.Argument, ir.Constant)):
            return True
    except Exception:
        pass

    current_bid = _block_id_from_block_name(current_block)
    if hasattr(candidate, "add_incoming"):
        return (
            _same_block_phi(candidate, current_block)
            or _phi_owner_dominates_block(
                resolver,
                candidate,
                current_bid,
            )
            or _single_def_dominates_block(resolver, value_id, current_bid)
        )

    if _defined_in_block(resolver, value_id, current_bid):
        return True
    return _single_def_dominates_block(resolver, value_id, current_bid)


def _declared_phi_in_current_block(resolver: Any, value_id: int, current_block: ir.Block) -> bool:
    try:
        block_phi_incomings = getattr(resolver, "block_phi_incomings", None)
        if not isinstance(block_phi_incomings, dict):
            return False
        current_bid = _block_id_from_block_name(current_block)
        if current_bid is None:
            return False
        dst_map = block_phi_incomings.get(int(current_bid))
        return isinstance(dst_map, dict) and int(value_id) in dst_map
    except Exception:
        return False


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

    # Prefer current-block SSA only when it is actually local to the block.
    scope = hot_scope if hot_scope else "generic"
    val = vmap.get(value_id)
    if debug:
        val_type = type(val).__name__ if val is not None else "None"
        from trace import phi as trace_phi
        trace_phi(f"[resolve_i64_strict] v{value_id} vmap={val_type}")
    if prefer_local and val is not None:
        if _same_block_phi(val, current_block) or _defined_in_block(
            resolver,
            value_id,
            _block_id_from_block_name(current_block),
        ) or (
            hasattr(val, "add_incoming")
            and _declared_phi_in_current_block(resolver, value_id, current_block)
        ):
            trace_hot_count(resolver, f"resolve_local_hit_{scope}")
            if debug:
                trace_phi(f"[resolve_i64_strict] v{value_id} -> local vmap")
            return val
    # If local map misses, try builder-global vmap only for values that are
    # known to safely dominate here. Raw global vmap reuse for ordinary SSA
    # values can pull a sibling-block definition into the current block and
    # trigger LLVM dominance failures.
    try:
        if hasattr(resolver, 'global_vmap') and isinstance(resolver.global_vmap, dict):
            gval = resolver.global_vmap.get(value_id)
            if gval is not None:
                allow_global = _global_reuse_allowed(resolver, value_id, gval, current_block)
                if allow_global:
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

def _is_global_vmap(vmap: Dict[int, Any], resolver: Any) -> bool:
    """Return True when the write target is resolver's global vmap SSOT."""
    try:
        gv = getattr(resolver, "global_vmap", None)
        return isinstance(gv, dict) and (vmap is gv)
    except Exception:
        return False


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
        if _is_global_vmap(vmap, resolver):
            # Global vmap is the PHI SSOT; overwrite is forbidden.
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
        # Block-local vmap snapshots may legally shadow PHIs with concrete values.

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

"""
Copy instruction lowering
MIR13 PHI-off uses explicit copies along edges/blocks to model merges.
"""

import llvmlite.ir as ir
import os
import sys
from typing import Dict, Optional, Any
from utils.values import resolve_i64_strict, safe_vmap_write
from utils.resolver_helpers import safe_get_type_tag, safe_set_type_tag

def lower_copy(
    builder: ir.IRBuilder,
    dst: int,
    src: int,
    vmap: Dict[int, ir.Value],
    resolver=None,
    current_block=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None,
):
    """Lower a copy by mapping dst to src value in the current block scope.

    Prefer same-block SSA from vmap; fallback to resolver to preserve
    dominance and to localize values across predecessors.
    """
    # If BuildCtx is provided, prefer its maps for consistency.
    if ctx is not None:
        try:
            if getattr(ctx, 'resolver', None) is not None:
                resolver = ctx.resolver
            if getattr(ctx, 'vmap', None) is not None and vmap is None:
                vmap = ctx.vmap
            if getattr(ctx, 'preds', None) is not None and preds is None:
                preds = ctx.preds
            if getattr(ctx, 'block_end_values', None) is not None and block_end_values is None:
                block_end_values = ctx.block_end_values
            if getattr(ctx, 'bb_map', None) is not None and bb_map is None:
                bb_map = ctx.bb_map
        except Exception:
            pass
    # Prefer local SSA directly in FAST lane to avoid resolver round-trip overhead
    # on dense copy chains (numeric_mixed_medium hotspot).
    val = None
    if os.environ.get('NYASH_LLVM_FAST') == '1':
        try:
            val = vmap.get(src)
        except Exception:
            val = None
    # Resolve otherwise to preserve dominance
    if val is None:
        val = resolve_i64_strict(resolver, src, current_block, preds, block_end_values, vmap, bb_map)
    if val is None:
        val = ir.Constant(ir.IntType(64), 0)
    # Phase 131-12-P1: Object identity trace before write
    if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
        print(f"[vmap/id] copy dst={dst} src={src} vmap id={id(vmap)} before_write", file=sys.stderr)
    safe_vmap_write(vmap, dst, val, "copy", resolver=resolver)

    # TypeFacts propagation (SSOT): preserve type tags across Copy.
    # Many MIR patterns materialize a temp then Copy into a local; without this,
    # string equality/concat may incorrectly fall back to integer/handle ops.
    #
    # Phase 285LLVM-1.5: Unified type tag propagation via resolver_helpers
    try:
        # Propagate type tag if source has one
        src_tag = safe_get_type_tag(resolver, src)
        if src_tag is not None:
            # Copy dict to avoid aliasing
            safe_set_type_tag(resolver, dst, src_tag.copy())
            # Debug logging: type tag propagation
            if os.environ.get('NYASH_CLI_VERBOSE') == '1':
                print(f"[llvm-py/copy] %{src} → %{dst}: {src_tag} propagated", file=sys.stderr)

        # Legacy stringish tagging (fallback for code not yet using value_types)
        elif resolver is not None and hasattr(resolver, "is_stringish") and resolver.is_stringish(src):
            if hasattr(resolver, "mark_string"):
                resolver.mark_string(dst)
    except Exception:
        pass

    # Propagate literal StringBox origin metadata for fast length/len lowering.
    try:
        if resolver is not None and hasattr(resolver, "newbox_string_args"):
            src_map = resolver.newbox_string_args
            if isinstance(src_map, dict) and src in src_map:
                src_map[dst] = src_map[src]
    except Exception:
        pass

    # Propagate literal string table through Copy so PHI/call routes can fold.
    try:
        if resolver is not None and hasattr(resolver, "string_literals"):
            lit_map = resolver.string_literals
            if isinstance(lit_map, dict) and src in lit_map:
                lit_map[dst] = lit_map[src]
    except Exception:
        pass

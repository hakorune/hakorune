"""
Branch instruction lowering
Conditional branch based on condition value
"""

import llvmlite.ir as ir
from typing import Dict
from utils.values import resolve_i64_strict

def lower_branch(
    builder: ir.IRBuilder,
    cond_vid: int,
    then_bid: int,
    else_bid: int,
    vmap: Dict[int, ir.Value],
    bb_map: Dict[int, ir.Block],
    resolver=None,
    preds=None,
    block_end_values=None
) -> None:
    """
    Lower MIR Branch instruction
    
    Args:
        builder: Current LLVM IR builder
        cond_vid: Condition value ID
        then_bid: Then block ID
        else_bid: Else block ID
        vmap: Value map
        bb_map: Block map
    """
    # Get condition value with preference to same-block SSA
    # Phase 131-7 debug
    try:
        import os, sys
        if os.environ.get('NYASH_CLI_VERBOSE') == '1':
            print(f"[branch] cond_vid={cond_vid} in vmap={cond_vid in vmap} (vmap id={id(vmap)})", file=sys.stderr)
            if cond_vid in vmap:
                print(f"[branch] vmap[{cond_vid}] = {vmap[cond_vid]}", file=sys.stderr)
    except Exception:
        pass
    cond = resolve_i64_strict(resolver, cond_vid, builder.block, preds, block_end_values, vmap, bb_map)
    try:
        import os, sys
        if os.environ.get('NYASH_CLI_VERBOSE') == '1':
            print(f"[branch] resolved cond={cond}", file=sys.stderr)
    except Exception:
        pass
    if cond is None:
        # Default to false if missing
        cond = ir.Constant(ir.IntType(1), 0)
    
    # Phase 275 A1: Check MIR type facts for Void discrimination (fail-fast)
    mir_type = None
    if resolver is not None and hasattr(resolver, 'value_types') and isinstance(resolver.value_types, dict):
        mir_type = resolver.value_types.get(cond_vid)

    # Void → TypeError (trap) - Phase 275 A1: Fail-fast for Void in boolean context
    if mir_type == 'Void' or (isinstance(mir_type, dict) and mir_type.get('kind') == 'Void'):
        print(f"⚠️  [branch/CRITICAL] Void in boolean context! v{cond_vid}", file=sys.stderr)
        builder.unreachable()
        return

    # VoidBox → TypeError (trap)
    if isinstance(mir_type, dict) and mir_type.get('box_type') == 'VoidBox':
        print(f"⚠️  [branch/CRITICAL] VoidBox in boolean context! v{cond_vid}", file=sys.stderr)
        builder.unreachable()
        return

    # Convert to i1 if needed (existing logic for non-Void types)
    if hasattr(cond, 'type'):
        # If we already have an i1 (canonical compare result), use it directly.
        if isinstance(cond.type, ir.IntType) and cond.type.width == 1:
            pass
        elif isinstance(cond.type, ir.IntType) and cond.type.width == 64:
            # i64 to i1: compare != 0
            zero = ir.Constant(ir.IntType(64), 0)
            cond = builder.icmp_unsigned('!=', cond, zero, name="cond_i1")
        elif isinstance(cond.type, ir.PointerType):
            # Pointer to i1: compare != null
            null = ir.Constant(cond.type, None)
            cond = builder.icmp_unsigned('!=', cond, null, name="cond_p1")
    
    # Get target blocks
    then_bb = bb_map.get(then_bid)
    else_bb = bb_map.get(else_bid)
    
    if then_bb and else_bb:
        builder.cbranch(cond, then_bb, else_bb)

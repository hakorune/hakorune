"""
Phase 256 P1.5: Select Instruction (ternary conditional) LLVM lowering
Equivalent to: dst = cond ? then_val : else_val
"""

from typing import Dict, Any, Optional
from llvmlite import ir
from trace import debug as trace_debug


def lower_select(
    builder: ir.IRBuilder,
    resolver,
    cond_vid: int,
    then_val_vid: int,
    else_val_vid: int,
    dst_vid: int,
    vmap_ctx: Dict[int, ir.Value],
    preds: Optional[Dict[int, list]] = None,
    block_end_values: Optional[Dict[int, Dict[int, ir.Value]]] = None,
    bb_map: Optional[Dict[int, ir.Block]] = None,
    ctx: Optional[Any] = None,
):
    """
    Lower Select instruction to LLVM IR.

    Select is a ternary conditional: dst = cond ? then_val : else_val

    Implementation: Use llvmlite's builder.select() which generates:
        %dst = select i1 %cond, <type> %then_val, <type> %else_val

    Args:
        builder: LLVM IR builder
        resolver: Value resolver for loading values
        cond_vid: Condition ValueId (must be i1/bool)
        then_val_vid: Value when condition is true
        else_val_vid: Value when condition is false
        dst_vid: Destination ValueId
        vmap_ctx: Value map context
        preds: CFG predecessors map (for value resolution)
        block_end_values: Block end values map (for value resolution)
        bb_map: BasicBlock map (for value resolution)
        ctx: Optional build context
    """
    trace_debug(f"[select] lowering: dst={dst_vid}, cond={cond_vid}, then={then_val_vid}, else={else_val_vid}")

    def _resolve_vid(vid: int, label: str):
        # Same-block values (fresh compare/binop/select results) live in current vmap.
        # Snapshot-only lookup misses these and can collapse ternary to constants.
        val = vmap_ctx.get(vid)
        if val is not None:
            return val
        val = resolver._value_at_end_i64(
            vid,
            int(str(builder.block.name).replace('bb', '')),
            preds,
            block_end_values,
            vmap_ctx,
            bb_map,
        )
        if val is None:
            trace_debug(f"[select] {label} fallback to 0")
            return ir.Constant(ir.IntType(64), 0)
        return val

    # Load condition value (must be i1/bool)
    cond = _resolve_vid(cond_vid, "cond")

    # Convert cond to i1 (boolean) if needed
    if cond.type != ir.IntType(1):
        # Extract lowest bit: i64 -> i1
        cond = builder.icmp_unsigned("!=", cond, ir.Constant(ir.IntType(64), 0), name=f"cond_to_i1_{cond_vid}")
        trace_debug(f"[select] converted cond to i1: {cond}")

    # Load then_val and else_val
    then_val = _resolve_vid(then_val_vid, "then_val")
    else_val = _resolve_vid(else_val_vid, "else_val")

    # Ensure both branches have same type
    if then_val.type != else_val.type:
        # If types differ, cast else_val to then_val's type
        if else_val.type == ir.IntType(64) and then_val.type == ir.IntType(64):
            pass  # Same integer type, no cast needed
        else:
            # Fallback: use i64 for both
            if then_val.type != ir.IntType(64):
                then_val = builder.zext(then_val, ir.IntType(64), name=f"then_zext_{then_val_vid}")
            if else_val.type != ir.IntType(64):
                else_val = builder.zext(else_val, ir.IntType(64), name=f"else_zext_{else_val_vid}")
            trace_debug(f"[select] type mismatch, cast to i64")

    # Emit select instruction
    # LLVM: select i1 %cond, <type> %then_val, <type> %else_val
    result = builder.select(cond, then_val, else_val, name=f"select_{dst_vid}")
    trace_debug(f"[select] emitted: %{dst_vid} = select %{cond_vid}, %{then_val_vid}, %{else_val_vid} -> {result}")

    # Store result in vmap
    vmap_ctx[dst_vid] = result
    trace_debug(f"[select] stored result in vmap[{dst_vid}]")

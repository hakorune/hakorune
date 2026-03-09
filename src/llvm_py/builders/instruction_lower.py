from typing import Dict, Any
from llvmlite import ir
from trace import debug as trace_debug

# Import instruction handlers
from instructions.const import lower_const
from instructions.binop import lower_binop
from instructions.compare import lower_compare
from instructions.unop import lower_unop
from instructions.controlflow.jump import lower_jump
from instructions.controlflow.branch import lower_branch
from instructions.ret import lower_return
from instructions.copy import lower_copy
from instructions.call import lower_call
from instructions.boxcall import lower_boxcall
from instructions.externcall import lower_externcall
from instructions.typeop import lower_typeop
from instructions.newbox import lower_newbox
from instructions.safepoint import lower_safepoint
from instructions.barrier import lower_barrier
from instructions.select import lower_select  # Phase 256 P1.5: Select instruction
from instructions.loopform import lower_while_loopform
from instructions.controlflow.while_ import lower_while_regular
from instructions.mir_call import lower_mir_call  # New unified handler
from instructions.weak import lower_weak_new, lower_weak_load  # Phase 285LLVM-1: WeakRef
from instructions.lifecycle import lower_keepalive, lower_release_strong  # Phase 287: Lifecycle

SUPPORTED_OPS = {
    "const",
    "binop",
    "jump",
    "copy",
    "branch",
    "ret",
    "phi",
    "compare",
    "unop",
    "mir_call",
    "call",
    "boxcall",
    "externcall",
    "newbox",
    "typeop",
    "safepoint",
    "barrier",
    "keepalive",
    "release_strong",
    "select",
    "weak_new",
    "weak_load",
    "while",
}


def lower_instruction(owner, builder: ir.IRBuilder, inst: Dict[str, Any], func: ir.Function):
    """Dispatch a single MIR instruction to appropriate lowering helper.

    owner is the NyashLLVMBuilder instance to access module, resolver, maps, and ctx.
    """
    op = inst.get("op")
    if op not in SUPPORTED_OPS:
        msg = f"[llvm/lower:unsupported_op] unknown instruction: {op}"
        trace_debug(msg)
        raise RuntimeError(msg)

    # Pick current vmap context (per-block context during lowering)
    vmap_ctx = getattr(owner, '_current_vmap', owner.vmap)
    # Phase 131-12-P1: Object identity trace for vmap_ctx investigation
    import os, sys
    if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
        print(f"[vmap/id] instruction op={op} vmap_ctx id={id(vmap_ctx)}", file=sys.stderr)

    if op == "const":
        dst = inst.get("dst")
        value = inst.get("value")
        lower_const(builder, owner.module, dst, value, vmap_ctx, owner.resolver)

    elif op == "binop":
        operation = inst.get("operation")
        lhs = inst.get("lhs")
        rhs = inst.get("rhs")
        dst = inst.get("dst")
        dst_type = inst.get("dst_type")
        lower_binop(builder, owner.resolver, operation, lhs, rhs, dst,
                    vmap_ctx, builder.block, owner.preds, owner.block_end_values, owner.bb_map,
                    dst_type=dst_type)

    elif op == "jump":
        target = inst.get("target")
        lower_jump(builder, target, owner.bb_map)

    elif op == "copy":
        dst = inst.get("dst")
        src = inst.get("src")
        lower_copy(builder, dst, src, vmap_ctx, owner.resolver, builder.block, owner.preds, owner.block_end_values, owner.bb_map, getattr(owner, 'ctx', None))

    elif op == "branch":
        cond = inst.get("cond")
        then_bid = inst.get("then")
        else_bid = inst.get("else")
        lower_branch(builder, cond, then_bid, else_bid, vmap_ctx, owner.bb_map, owner.resolver, owner.preds, owner.block_end_values)

    elif op == "ret":
        value = inst.get("value")
        lower_return(builder, value, vmap_ctx, func.function_type.return_type,
                     owner.resolver, owner.preds, owner.block_end_values, owner.bb_map, getattr(owner, 'ctx', None))

    elif op == "phi":
        # No-op here: プレースホルダは前処理（setup_phi_placeholders）で一元管理。
        return

    elif op == "compare":
        # Dedicated compare op
        operation = inst.get("cmp") or inst.get("operation") or inst.get("op")
        lhs = inst.get("lhs")
        rhs = inst.get("rhs")
        dst = inst.get("dst")
        cmp_kind = inst.get("cmp_kind")
        lower_compare(builder, operation, lhs, rhs, dst, vmap_ctx,
                      owner.resolver, builder.block, owner.preds, owner.block_end_values, owner.bb_map,
                      meta={"cmp_kind": cmp_kind} if cmp_kind else None,
                      ctx=getattr(owner, 'ctx', None))

    elif op == "unop":
        # Unary op: kind in {'neg','not','bitnot'}; src is operand
        kind_raw = inst.get("kind") or inst.get("operation") or ""
        # Defensive: ensure kind_raw is never None before calling .lower()
        if kind_raw is None:
            kind_raw = ""
        kind = kind_raw.lower() if hasattr(kind_raw, 'lower') else str(kind_raw).lower()
        srcv = inst.get("src") or inst.get("operand")
        dst = inst.get("dst")
        lower_unop(builder, owner.resolver, kind, srcv, dst, vmap_ctx, builder.block,
                   owner.preds, owner.block_end_values, owner.bb_map, ctx=getattr(owner, 'ctx', None))

    elif op == "mir_call":
        # Unified MIR Call handling (accept both nested and flat shapes)
        mir_call = inst.get("mir_call")
        if mir_call is None:
            mir_call = {
                'callee': inst.get('callee'),
                'args': inst.get('args', []),
                'flags': inst.get('flags', {}),
                'effects': inst.get('effects', {}),
            }
        dst = inst.get("dst")
        lower_mir_call(owner, builder, mir_call, dst, vmap_ctx, owner.resolver)

    elif op == "call":
        func_name = inst.get("func")
        if func_name is None:
            func_name = inst.get("callee")
        args = inst.get("args", [])
        dst = inst.get("dst")
        lower_call(builder, owner.module, func_name, args, dst, vmap_ctx, owner.resolver, owner.preds, owner.block_end_values, owner.bb_map, getattr(owner, 'ctx', None))

    elif op == "boxcall":
        box_vid = inst.get("box")
        method = inst.get("method")
        method_id = inst.get("method_id")  # Phase 287 P4: Extract method_id for universal slots
        args = inst.get("args", [])
        dst = inst.get("dst")
        lower_boxcall(builder, owner.module, box_vid, method, args, dst,
                      vmap_ctx, owner.resolver, owner.preds, owner.block_end_values, owner.bb_map, getattr(owner, 'ctx', None), method_id)
        # Optional: honor explicit dst_type for tagging (string handle)
        try:
            dst_type = inst.get("dst_type")
            if dst is not None and isinstance(dst_type, dict):
                if dst_type.get("kind") == "handle" and dst_type.get("box_type") == "StringBox":
                    if hasattr(owner.resolver, 'mark_string'):
                        owner.resolver.mark_string(int(dst))
            # Track last substring for optional esc_json fallback
            try:
                if isinstance(method, str) and method == 'substring' and isinstance(dst, int):
                    owner._last_substring_vid = int(dst)
            except Exception:
                pass
        except Exception:
            pass

    elif op == "externcall":
        func_name = inst.get("func")
        args = inst.get("args", [])
        dst = inst.get("dst")
        lower_externcall(builder, owner.module, func_name, args, dst,
                         vmap_ctx, owner.resolver, owner.preds, owner.block_end_values, owner.bb_map, getattr(owner, 'ctx', None))

    elif op == "newbox":
        box_type = inst.get("type")
        args = inst.get("args", [])
        dst = inst.get("dst")
        lower_newbox(builder, owner.module, box_type, args, dst,
                     vmap_ctx, owner.resolver, getattr(owner, 'ctx', None))

    elif op == "typeop":
        operation = inst.get("operation")
        src = inst.get("src")
        dst = inst.get("dst")
        target_type = inst.get("target_type")
        lower_typeop(builder, operation, src, dst, target_type,
                     vmap_ctx, owner.resolver, owner.preds, owner.block_end_values, owner.bb_map, getattr(owner, 'ctx', None))

    elif op == "safepoint":
        live = inst.get("live", [])
        lower_safepoint(builder, owner.module, live, vmap_ctx,
                        resolver=owner.resolver, preds=owner.preds,
                        block_end_values=owner.block_end_values, bb_map=owner.bb_map,
                        ctx=getattr(owner, 'ctx', None))

    elif op == "barrier":
        barrier_type = inst.get("type", "memory")
        lower_barrier(builder, barrier_type, ctx=getattr(owner, 'ctx', None))

    elif op == "keepalive":
        # Phase 287: KeepAlive (no-op in LLVM, affects DCE/liveness only)
        values = inst.get("values", [])
        lower_keepalive(builder, owner.module, values, vmap_ctx,
                        resolver=owner.resolver, preds=owner.preds,
                        block_end_values=owner.block_end_values, bb_map=owner.bb_map,
                        ctx=getattr(owner, 'ctx', None))

    elif op == "release_strong":
        # Phase 287: ReleaseStrong (variable overwrite semantics)
        values = inst.get("values", [])
        lower_release_strong(builder, owner.module, values, vmap_ctx,
                             resolver=owner.resolver, preds=owner.preds,
                             block_end_values=owner.block_end_values, bb_map=owner.bb_map,
                             ctx=getattr(owner, 'ctx', None))

    elif op == "select":
        # Phase 256 P1.5: Select instruction (ternary conditional)
        cond = inst.get("cond")
        then_val = inst.get("then_val")
        else_val = inst.get("else_val")
        dst = inst.get("dst")
        lower_select(builder, owner.resolver, cond, then_val, else_val, dst,
                     vmap_ctx, owner.preds, owner.block_end_values, owner.bb_map,
                     ctx=getattr(owner, 'ctx', None))

    elif op == "weak_new":
        # Phase 285LLVM-1: WeakRef(New) instruction (strong → weak)
        dst = inst.get("dst")
        box_val = inst.get("box_val")
        lower_weak_new(builder, owner.module, dst, box_val, vmap_ctx, ctx=getattr(owner, 'ctx', None))

    elif op == "weak_load":
        # Phase 285LLVM-1: WeakRef(Load) instruction (weak → strong or 0/Void)
        dst = inst.get("dst")
        weak_ref = inst.get("weak_ref")
        lower_weak_load(builder, owner.module, dst, weak_ref, vmap_ctx, ctx=getattr(owner, 'ctx', None))

    elif op == "while":
        # Experimental LoopForm lowering inside a block
        cond = inst.get("cond")
        body = inst.get("body", [])
        owner.loop_count += 1
        if not lower_while_loopform(builder, func, cond, body,
                                    owner.loop_count, owner.vmap, owner.bb_map,
                                    owner.resolver, owner.preds, owner.block_end_values,
                                    getattr(owner, 'ctx', None)):
            # Fallback to regular while (structured)
            try:
                owner.resolver._owner_lower_instruction = owner.lower_instruction
            except Exception:
                pass
            lower_while_regular(builder, func, cond, body,
                                owner.loop_count, owner.vmap, owner.bb_map,
                                owner.resolver, owner.preds, owner.block_end_values)
    else:
        # Defensive path (should be unreachable due to pre-check)
        msg = f"[llvm/lower:unsupported_op] unknown instruction: {op}"
        trace_debug(msg)
        raise RuntimeError(msg)

    # Record per-inst definition for lifetime hinting as soon as available
    try:
        dst_maybe = inst.get("dst")
        if isinstance(dst_maybe, int) and dst_maybe in owner.vmap:
            cur_bid = None
            try:
                cur_bid = int(str(builder.block.name).replace('bb',''))
            except Exception:
                pass
            if cur_bid is not None:
                owner.def_blocks.setdefault(dst_maybe, set()).add(cur_bid)
    except Exception:
        pass

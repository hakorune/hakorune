from __future__ import annotations
from typing import Dict, List, Any, Optional, Tuple

import llvmlite.ir as ir

from .common import trace
from .debug_helper import (
    is_phi_debug_enabled,
    is_phi_strict_enabled,
    is_phi_trace_enabled,
)
from .error_helpers import PhiStrictError, PhiDebugMessage
from .fact_propagation import mark_arrayish_handle, should_mark_phi_arrayish

def _const_i64(builder, n: int) -> ir.Constant:
    try:
        return ir.Constant(builder.i64, int(n))
    except Exception:
        # Failsafe: llvmlite requires a Module-bound type; fallback to 64-bit 0
        return ir.Constant(ir.IntType(64), int(n) if isinstance(n, int) else 0)


def ensure_phi(builder, block_id: int, dst_vid: int, bb: ir.Block, dst_type=None) -> ir.Instruction:
    """Ensure a PHI placeholder exists at the block head for dst_vid and return it.

    Phase 132 Fix: Ensure PHI instructions are ALWAYS at block head, before any
    other instructions. This is critical for LLVM IR validity.

    Phase 275 P0: Support Float type PHIs (double) based on dst_type from MIR JSON.
    """
    # Always place PHI at block start to keep LLVM invariant "PHI nodes at top"

    # Check if PHI already exists in vmap for this block
    cur = builder.vmap.get(dst_vid)
    try:
        if cur is not None and hasattr(cur, "add_incoming"):
            cur_bb_name = getattr(getattr(cur, "basic_block", None), "name", None)
            bb_name = getattr(bb, "name", None)
            try:
                if isinstance(cur_bb_name, bytes):
                    cur_bb_name = cur_bb_name.decode()
            except Exception:
                pass
            try:
                if isinstance(bb_name, bytes):
                    bb_name = bb_name.decode()
            except Exception:
                pass
            if cur_bb_name == bb_name:
                import sys
                if is_phi_debug_enabled():
                    print(f"[phi_wiring/reuse] v{dst_vid} existing PHI found, type={cur.type}", file=sys.stderr)
                return cur
    except Exception:
        pass

    # Phase 275 P0: Check predeclared PHI with type compatibility verification
    predecl = getattr(builder, "predeclared_ret_phis", {}) if hasattr(builder, "predeclared_ret_phis") else {}
    phi = predecl.get((int(block_id), int(dst_vid))) if predecl else None
    if phi is not None:
        # Phase 275 P0: Verify type compatibility
        import sys
        expected_type = ir.DoubleType() if (dst_type == 'f64' or dst_type == 'double') else builder.i64
        if phi.type == expected_type:
            builder.vmap[dst_vid] = phi
            trace({"phi": "ensure_predecl", "block": int(block_id), "dst": int(dst_vid)})
            if is_phi_debug_enabled():
                print(f"[phi_wiring/reuse] v{dst_vid} predeclared PHI type matches: {phi.type}", file=sys.stderr)
            return phi
        else:
            # Phase 275 P0: 型不一致の古いPHIを発見 → CRITICAL警告
            print(f"⚠️  [phi_wiring/CRITICAL] PHI type mismatch! "
                  f"v{dst_vid}: predeclared={phi.type} expected={expected_type}",
                  file=sys.stderr)

            # PhiManager に古いPHI無効化を通知（あれば）
            try:
                if hasattr(builder, 'phi_manager'):
                    builder.phi_manager.invalidate_phi(int(block_id), int(dst_vid))
            except Exception:
                pass

            # 詳細デバッグ
            if is_phi_debug_enabled():
                print(f"[phi_wiring/type_mismatch] Creating new PHI with correct type", file=sys.stderr)

    # Phase 132 Critical Fix: Check if block already has a terminator
    # If so, we're trying to add PHI too late - this is a bug
    block_has_terminator = False
    try:
        if bb.terminator is not None:
            block_has_terminator = True
    except Exception:
        pass

    if block_has_terminator:
        # This should not happen - PHIs must be created before terminators
        error = PhiStrictError(
            message=f"PHI v{dst_vid} created after terminator in bb{block_id}! This violates LLVM IR PHI-first invariant.",
            next_file="phi_placement.py::verify_phi_ordering",
            block_id=block_id,
            dst_vid=dst_vid,
        )
        error.raise_if_strict()

        # Default: warning only (preserve existing behavior)
        if is_phi_debug_enabled():
            import sys
            print(f"[phi_wiring] WARNING: {error.message}", file=sys.stderr)
        # Try to create PHI anyway at the start, but log the issue

    # Create PHI at block start
    b = ir.IRBuilder(bb)
    try:
        # Phase 132: Explicitly position BEFORE the first instruction
        # This ensures PHI is at the very start
        instrs = list(bb.instructions)
        if instrs:
            b.position_before(instrs[0])
        else:
            b.position_at_start(bb)
    except Exception:
        try:
            b.position_at_start(bb)
        except Exception:
            pass

    # Phase 277 P1: Use type_helper SSOT for PHI type determination
    from .type_helper import dst_type_to_llvm_type
    phi_type = dst_type_to_llvm_type(dst_type, builder)

    import sys
    if is_phi_debug_enabled():
        print(f"[phi_wiring/create] v{dst_vid} dst_type={dst_type} -> phi_type={phi_type}", file=sys.stderr)

    ph = b.phi(phi_type, name=f"phi_{dst_vid}")
    # Phase 132 Debug: Check if basic_block is set correctly
    if is_phi_debug_enabled() or is_phi_trace_enabled():
        phi_bb = getattr(ph, 'basic_block', None)
        phi_bb_name = getattr(phi_bb, 'name', None) if phi_bb is not None else None
        bb_name = getattr(bb, 'name', None)
        print(f"[phi_wiring/create] v{dst_vid} PHI created: phi.basic_block={phi_bb_name} expected={bb_name}", file=sys.stderr)
    builder.vmap[dst_vid] = ph
    trace({"phi": "ensure_create", "block": int(block_id), "dst": int(dst_vid), "after_term": block_has_terminator})
    return ph


def phi_at_block_head(block: ir.Block, ty: ir.Type, name: str | None = None) -> ir.Instruction:
    """Create a PHI at the very start of `block` and return it.
    Keeps LLVM's requirement that PHI nodes are grouped at the top of a block.
    """
    b = ir.IRBuilder(block)
    try:
        b.position_at_start(block)
    except Exception:
        pass
    return b.phi(ty, name=name) if name is not None else b.phi(ty)


def build_succs(preds: Dict[int, List[int]]) -> Dict[int, List[int]]:
    succs: Dict[int, List[int]] = {}
    for to_bid, from_list in (preds or {}).items():
        for fr in from_list:
            succs.setdefault(fr, []).append(to_bid)
    return succs


def nearest_pred_on_path(
    succs: Dict[int, List[int]], preds_list: List[int], decl_b: int, target_bid: int
) -> Optional[int]:
    from collections import deque

    q = deque([decl_b])
    visited = set([decl_b])
    parent: Dict[int, Any] = {decl_b: None}
    while q:
        cur = q.popleft()
        if cur == target_bid:
            par = parent.get(target_bid)
            return par if par in preds_list else None
        for nx in succs.get(cur, []):
            if nx not in visited:
                visited.add(nx)
                parent[nx] = cur
                q.append(nx)
    return None


def wire_incomings(builder, block_id: int, dst_vid: int, incoming: List[Tuple[int, int]], context=None):
    """Wire PHI incoming edges for (block_id, dst_vid) using declared (decl_b, v_src) pairs.

    Phase 132-P1: Use context Box for function-local state isolation.

    SSOT: duplicate incoming per predecessor
    ----------------------------------------
    LLVM PHI wiring may observe *multiple* incoming candidates that map to the same actual
    predecessor (`pred_match`). This can happen legitimately because:
    - jump-only / trampoline blocks: multiple declared blocks collapse to the same CFG predecessor,
    - alias/self-carry handling: placeholder `dst_vid` appears in the incoming list and is rewritten,
    - prepass placeholders: additional metadata can create redundant candidates.

    Selection contract:
    - Once a predecessor’s incoming value is successfully chosen, do not overwrite it later
      (avoid last-wins instability).
    - A synthesized `0` is treated as an "unresolved sentinel" (fallback path). If we later obtain
      a successfully-resolved non-zero value for the same predecessor, we may replace `0`.
    - In STRICT mode, `0` should be rare; a `0` incoming indicates "missing snapshot / unresolved"
      and should be caught by stricter checks upstream.
    """
    bb = builder.bb_map.get(block_id)
    if bb is None:
        return
    # Prefer an existing PHI already materialized in this block (e.g., by resolver)
    phi = None
    try:
        # Phase 132-P1: Use context.get_block_snapshot (simple block_id key)
        if context is not None:
            snapshot = context.get_block_snapshot(int(block_id))
            cur = snapshot.get(int(dst_vid))
        else:
            # Fallback for backward compatibility
            snap = getattr(builder, 'block_end_values', {}) or {}
            cur = snap.get(int(block_id), {}).get(int(dst_vid))
        if cur is not None and hasattr(cur, 'add_incoming'):
            # Ensure it belongs to the same block
            cur_bb_name = getattr(getattr(cur, 'basic_block', None), 'name', None)
            bb_name = getattr(bb, 'name', None)
            try:
                if isinstance(cur_bb_name, bytes):
                    cur_bb_name = cur_bb_name.decode()
            except Exception:
                pass
            try:
                if isinstance(bb_name, bytes):
                    bb_name = bb_name.decode()
            except Exception:
                pass
            if cur_bb_name == bb_name:
                phi = cur
                # Mirror to global vmap for downstream lookups
                try:
                    builder.vmap[dst_vid] = phi
                except Exception:
                    pass
    except Exception:
        phi = None
    if phi is None:
        # Phase 275 P0: Get dst_type from resolver's value_types (SSOT)
        from .type_helper import get_phi_dst_type
        dst_type = get_phi_dst_type(builder, dst_vid)
        import sys
        if is_phi_debug_enabled():
            print(f"[phi_wiring] v{dst_vid} -> dst_type='{dst_type}'", file=sys.stderr)
        phi = ensure_phi(builder, block_id, dst_vid, bb, dst_type=dst_type)
    preds_raw = [p for p in builder.preds.get(block_id, []) if p != block_id]
    seen = set()
    preds_list: List[int] = []
    for p in preds_raw:
        if p not in seen:
            preds_list.append(p)
            seen.add(p)
    succs = build_succs(builder.preds)
    init_src_vid = None
    for (_bd0, vs0) in incoming:
        try:
            vi = int(vs0)
        except Exception:
            continue
        if vi != int(dst_vid):
            init_src_vid = vi
            break
    chosen: Dict[int, ir.Value] = {}

    def _is_zero_const(v: ir.Value) -> bool:
        try:
            if isinstance(v, ir.Constant) and isinstance(v.type, ir.IntType) and v.type.width == 64:
                return int(getattr(v, "constant", 1)) == 0
        except Exception:
            pass
        return False

    for (b_decl, v_src) in incoming:
        try:
            bd = int(b_decl)
            vs = int(v_src)
        except Exception:
            continue
        trace({"phi": "wire_process", "dst": int(dst_vid), "decl_b": bd, "v_src": vs, "init_src_vid": init_src_vid})
        pred_match = nearest_pred_on_path(succs, preds_list, bd, block_id)
        trace({"phi": "wire_pred_match", "decl_b": bd, "pred_match": pred_match})
        if pred_match is None:
            trace({"phi": "wire_skip_no_path", "decl_b": bd, "target": int(block_id), "src": vs})
            continue
        original_vs = vs
        if vs == int(dst_vid) and init_src_vid is not None:
            trace({"phi": "wire_self_carry", "dst": int(dst_vid), "vs": vs, "init_src_vid": init_src_vid})
            vs = int(init_src_vid)
        if original_vs != vs:
            trace({"phi": "wire_replaced_src", "original": original_vs, "replaced": vs})
        try:
            # P0-4: Use resolve_incoming for PHI incoming values
            # Phase 132-P1: Pass context for function-local state isolation
            val = builder.resolver.resolve_incoming(pred_match, vs, context=context)
            trace({"phi": "wire_resolved", "vs": vs, "pred": pred_match, "val_type": type(val).__name__})
        except Exception as e:
            trace({"phi": "wire_resolve_fail", "vs": vs, "pred": pred_match, "error": str(e)})
            val = None
        resolved_ok = val is not None
        # Phase 277 P1: Strict mode forbids silent fallback to 0
        if val is None:
            error = PhiStrictError(
                message=f"PHI v{dst_vid} incoming from bb{pred_match} could not be resolved (vs={vs}). Silent fallback to 0 is forbidden in strict mode.",
                next_file="llvm_builder.py::_value_at_end_i64",
                block_id=block_id,
                dst_vid=dst_vid,
                context=f"pred={pred_match}",
            )
            error.raise_if_strict()
            # Default: silent fallback (existing behavior)
            val = _const_i64(builder, 0)
        else:
            try:
                # Some paths can accidentally pass plain integers; coerce to i64 const
                if not hasattr(val, 'type'):
                    val = _const_i64(builder, int(val))
            except Exception as e:
                error = PhiStrictError(
                    message=f"PHI v{dst_vid} incoming type coercion failed (vs={vs}, pred={pred_match}): {e}",
                    next_file="phi_wiring.py::get_phi_operand_type",
                    block_id=block_id,
                    dst_vid=dst_vid,
                )
                error.raise_if_strict()
                # Default: silent fallback (existing behavior)
                val = _const_i64(builder, 0)
        # SSOT for ambiguous PHI incoming (same pred_match multiple times):
        # - prefer a non-zero / successfully-resolved value over a synthesized zero,
        # - otherwise keep the first choice to avoid last-wins "overwrite to 0".
        prev = chosen.get(pred_match)
        if prev is None:
            chosen[pred_match] = val
        else:
            if _is_zero_const(prev) and not _is_zero_const(val) and resolved_ok:
                chosen[pred_match] = val
        trace({"phi": "wire_choose", "pred": int(pred_match), "dst": int(dst_vid), "src": int(vs)})
    wired = 0
    for pred_bid, val in chosen.items():
        pred_bb = builder.bb_map.get(pred_bid)
        if pred_bb is None:
            continue
        # llvmlite requires (value, block) of correct types
        phi.add_incoming(val, pred_bb)
        trace({"phi": "add_incoming", "dst": int(dst_vid), "pred": int(pred_bid)})
        wired += 1
    return wired


def _mark_phi_stringish(builder, dst_vid: int, incoming: List[Tuple[int, int]]) -> None:
    try:
        resolver = getattr(builder, "resolver", None)
        if not (
            resolver is not None
            and hasattr(resolver, "is_stringish")
            and hasattr(resolver, "mark_string")
        ):
            return
        for (_decl_b, v_src) in incoming or []:
            try:
                if resolver.is_stringish(int(v_src)):
                    resolver.mark_string(int(dst_vid))
                    return
            except Exception:
                continue
    except Exception:
        pass


def _mark_phi_arrayish(builder, dst_vid: int, incoming: List[Tuple[int, int]]) -> None:
    try:
        resolver = getattr(builder, "resolver", None)
        if should_mark_phi_arrayish(resolver, None, incoming):
            mark_arrayish_handle(resolver, int(dst_vid))
    except Exception:
        pass


def _consensus_incoming_mapping(
    mapping: Any,
    dst_vid: int,
    incoming: List[Tuple[int, int]],
    value_ok,
):
    if not isinstance(mapping, dict):
        return None

    candidate = None
    conflict = False
    for (_decl_b, v_src) in incoming or []:
        try:
            v_src_i = int(v_src)
        except Exception:
            continue
        mapped = candidate if v_src_i == int(dst_vid) and candidate is not None else mapping.get(v_src_i)
        if not value_ok(mapped):
            continue
        if candidate is None:
            candidate = mapped
        elif candidate != mapped:
            conflict = True
            break
    if conflict:
        return None
    return candidate


def _propagate_phi_origin_maps(builder, dst_vid: int, incoming: List[Tuple[int, int]]) -> None:
    resolver = getattr(builder, "resolver", None)
    if resolver is None:
        return

    try:
        src_map = getattr(resolver, "newbox_string_args", None)
        candidate = _consensus_incoming_mapping(src_map, dst_vid, incoming, lambda value: value is not None)
        if candidate is not None and isinstance(src_map, dict):
            src_map[int(dst_vid)] = candidate
    except Exception:
        pass

    try:
        lit_map = getattr(resolver, "string_literals", None)
        candidate = _consensus_incoming_mapping(lit_map, dst_vid, incoming, lambda value: isinstance(value, str))
        if isinstance(candidate, str) and isinstance(lit_map, dict):
            lit_map[int(dst_vid)] = candidate
    except Exception:
        pass


def _propagate_finalized_phi_facts(builder, dst_vid: int, incoming: List[Tuple[int, int]]) -> None:
    _mark_phi_stringish(builder, dst_vid, incoming)
    _mark_phi_arrayish(builder, dst_vid, incoming)
    _propagate_phi_origin_maps(builder, dst_vid, incoming)


def finalize_phis(builder, context):
    """Finalize PHI nodes by wiring their incoming edges.
    Phase 132-P1: Use context Box for function-local state isolation.

    Args:
        context: FunctionLowerContext Box containing function-local state
    """
    total_blocks = 0
    total_dsts = 0
    total_wired = 0
    alias_map = getattr(context, "phi_trivial_aliases", None)
    if not isinstance(alias_map, dict):
        alias_map = {}
    for block_id, dst_map in (context.block_phi_incomings or {}).items():
        total_blocks += 1
        for dst_vid, incoming in (dst_map or {}).items():
            total_dsts += 1
            if (int(block_id), int(dst_vid)) in alias_map:
                trace({
                    "phi": "finalize_skip_trivial_alias",
                    "block": int(block_id),
                    "dst": int(dst_vid),
                    "src": int(alias_map[(int(block_id), int(dst_vid))]),
                })
                continue
            wired = wire_incomings(builder, int(block_id), int(dst_vid), incoming, context=context)
            total_wired += int(wired or 0)
            _propagate_finalized_phi_facts(builder, int(dst_vid), incoming)
            trace({"phi": "finalize", "block": int(block_id), "dst": int(dst_vid), "wired": int(wired or 0)})
    trace({"phi": "finalize_summary", "blocks": int(total_blocks), "dsts": int(total_dsts), "incoming_wired": int(total_wired)})

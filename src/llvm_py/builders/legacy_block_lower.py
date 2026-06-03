from typing import Dict, Any, Optional, List

from llvmlite import ir
from trace import debug as trace_debug
from trace import phi as trace_phi
from trace import phi_json as trace_phi_json
from phi_wiring import ensure_phi as _ensure_phi


def _safe_int(value) -> Optional[int]:
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def _safe_trace_phi_json(payload: Dict[str, Any]) -> None:
    try:
        trace_phi_json(payload)
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        pass


def _safe_trace_phi(message: str) -> None:
    try:
        trace_phi(message)
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        pass


def _is_stringish_type(inst: Dict[str, Any]) -> bool:
    opx = inst.get("op")
    if opx == "const":
        v = inst.get("value", {}) or {}
        t = v.get("type")
        return t == "string" or (
            isinstance(t, dict)
            and t.get("kind") in ("handle", "ptr")
            and t.get("box_type") == "StringBox"
        )
    if opx in ("binop", "boxcall", "externcall"):
        t = inst.get("dst_type")
        return (
            isinstance(t, dict)
            and t.get("kind") == "handle"
            and t.get("box_type") == "StringBox"
        )
    return False


def _record_block_phi_incoming(builder, bid0: int, dst0: int, incoming0) -> None:
    pairs = []
    for (v, b) in incoming0:
        v_i = _safe_int(v)
        b_i = _safe_int(b)
        if v_i is None or b_i is None:
            continue
        pairs.append((b_i, v_i))
    builder.block_phi_incomings.setdefault(bid0, {})[dst0] = pairs


def _register_phi_placeholder(builder, bid0: int, dst0: int, ph0) -> None:
    try:
        builder.phi_manager.register_phi(int(bid0), int(dst0), ph0)
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        builder.vmap[dst0] = ph0


def _mark_string_dst_if_needed(builder, dst0: int, incoming0, produced_str: Dict[int, bool], inst: Dict[str, Any]) -> None:
    resolver = getattr(builder, "resolver", None)
    dst_type0 = inst.get("dst_type")
    mark_str = (
        isinstance(dst_type0, dict)
        and dst_type0.get("kind") == "handle"
        and dst_type0.get("box_type") == "StringBox"
    )
    if not mark_str:
        for (v_id, _b_id) in incoming0:
            if produced_str.get(int(v_id)):
                mark_str = True
                break
    if mark_str and resolver is not None and hasattr(resolver, "mark_string"):
        resolver.mark_string(int(dst0))


def _safe_position_at_start(builder_ir, bb0) -> None:
    builder_ir.position_at_start(bb0)


def _safe_resolver_bind(builder, builder_ir) -> None:
    if hasattr(builder, "resolver"):
        builder.resolver.builder = builder_ir
        builder.resolver.module = builder.module


def _is_phi_value(value) -> bool:
    try:
        return hasattr(value, "add_incoming")
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        return False


def _copy_visible_vmap(builder, bb) -> Dict[int, ir.Value]:
    try:
        visible: Dict[int, ir.Value] = {}
        for vid, val in (builder.vmap or {}).items():
            if _is_phi_value(val):
                bb_of = getattr(getattr(val, "basic_block", None), "name", None)
                if bb_of != bb.name:
                    continue
            visible[vid] = val
        return visible
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        return dict(builder.vmap)


def _record_block_snapshot(builder, bid: int, snap: Dict[int, ir.Value]) -> None:
    builder.block_end_values[bid] = snap


def _cleanup_current_vmap(builder) -> None:
    try:
        delattr(builder, "_current_vmap")
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        pass


def _next_pred_on_path(succs: Dict[int, List[int]], preds_list: List[int], decl_b: int, target_bid: int) -> Optional[int]:
    from collections import deque

    q = deque([decl_b])
    visited = {decl_b}
    parent: Dict[int, Optional[int]] = {decl_b: None}
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


def _safe_value_at_end(builder, src_vid: int, pred_match: int):
    try:
        return builder.resolver._value_at_end_i64(
            src_vid,
            pred_match,
            builder.preds,
            builder.block_end_values,
            builder.vmap,
            builder.bb_map,
        )
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        return None


def _safe_add_incoming(phi, value, pred_bb) -> None:
    try:
        phi.add_incoming(value, pred_bb)
    except (AttributeError, KeyError, RuntimeError, TypeError, ValueError):
        pass


def setup_phi_placeholders(builder, blocks: List[Dict[str, Any]]):
    """Predeclare PHIs and collect incoming metadata for finalize_phis.

    This pass is function-local and must be invoked after basic blocks are
    created and before lowering individual blocks. It also tags string-ish
    values eagerly to help downstream resolvers choose correct intrinsics.
    """
    produced_str: Dict[int, bool] = {}
    for block_data in blocks:
        for inst in block_data.get("instructions", []) or []:
            dstx = _safe_int(inst.get("dst"))
            if dstx is None:
                continue
            if _is_stringish_type(inst):
                produced_str[dstx] = True

    builder.block_phi_incomings = {}
    for block_data in blocks:
        bid0 = _safe_int(block_data.get("id", 0))
        if bid0 is None:
            continue
        bb0 = builder.bb_map.get(bid0)
        for inst in block_data.get("instructions", []) or []:
            if inst.get("op") != "phi":
                continue
            dst0 = _safe_int(inst.get("dst"))
            incoming0 = inst.get("incoming", []) or []
            if dst0 is None:
                continue
            _record_block_phi_incoming(builder, bid0, dst0, incoming0)
            if bb0 is None:
                continue
            b0 = ir.IRBuilder(bb0)
            _safe_position_at_start(b0, bb0)
            existing = builder.vmap.get(dst0)
            if not _is_phi_value(existing):
                ph0 = b0.phi(builder.i64, name=f"phi_{dst0}")
                _register_phi_placeholder(builder, bid0, dst0, ph0)
            _mark_string_dst_if_needed(builder, dst0, incoming0, produced_str, inst)
            if hasattr(builder, "def_blocks"):
                builder.def_blocks.setdefault(int(dst0), set()).add(int(bid0))

    if hasattr(builder, "resolver"):
        builder.resolver.block_phi_incomings = builder.block_phi_incomings


def lower_block(builder, bb: ir.Block, block_data: Dict[str, Any], func: ir.Function):
    """Lower a single basic block.

    Emit all non-terminator ops first, then control-flow terminators
    (branch/jump/ret). This avoids generating IR after a terminator.
    """
    builder_ir = ir.IRBuilder(bb)
    trace_debug(f"[llvm-py] === lower_block bb{block_data.get('id')} ===")
    _safe_resolver_bind(builder, builder_ir)
    instructions = block_data.get("instructions", [])
    # JSON-declared PHIs are not materialized here; placeholders are created uniformly
    # via ensure_phi in finalize_phis to keep PHIs grouped at block head.
    # Partition into body ops and terminators
    body_ops: List[Dict[str, Any]] = []
    term_ops: List[Dict[str, Any]] = []
    for inst in (instructions or []):
        opx = inst.get("op")
        if opx in ("branch", "jump", "ret"):
            term_ops.append(inst)
        elif opx == "phi":
            continue
        else:
            body_ops.append(inst)
    # Per-block SSA map (avoid cross-block vmap pollution)
    # Seed with non-PHI globals and PHIs that belong to this block only.
    vmap_cur: Dict[int, ir.Value] = {}
    vmap_cur = _copy_visible_vmap(builder, bb)
    # Expose to lower_instruction users (e.g., while_ regular lowering)
    builder._current_vmap = vmap_cur
    created_ids: List[int] = []
    # Compute ids defined in this block to help with copy/PHI decisions
    defined_here_all: set = set()
    for _inst in body_ops:
        d = _safe_int(_inst.get("dst"))
        if d is not None:
            defined_here_all.add(d)
    # Keep PHI synthesis on-demand in resolver; avoid predeclaring here to reduce clashes.
    # Lower body ops first in-order
    for i_idx, inst in enumerate(body_ops):
        trace_debug(
            f"[llvm-py] body op: {inst.get('op')} dst={inst.get('dst')} cond={inst.get('cond')}"
        )
        if bb.terminator is not None:
            break
        builder_ir.position_at_end(bb)
        # Special-case copy: avoid forward self-block dependencies only when src is defined later in this block
        if inst.get("op") == "copy":
            src_i = inst.get("src")
            skip_now = False
            if isinstance(src_i, int):
                # Check if src will be defined in a subsequent instruction
                for _rest in body_ops[i_idx + 1 :]:
                    if _safe_int(_rest.get("dst")) == int(src_i):
                        skip_now = True
                        break
            if skip_now:
                # Skip now; a later copy will remap after src becomes available
                pass
            else:
                builder.lower_instruction(builder_ir, inst, func)
        else:
            builder.lower_instruction(builder_ir, inst, func)
        # Phase 131-7: Bidirectional sync between per-block vmap and global vmap
        # This ensures values are available for subsequent instructions (e.g., branch using unop result)
        dst = _safe_int(inst.get("dst"))
        if dst is not None:
            if dst in vmap_cur:
                builder.vmap[dst] = vmap_cur[dst]
            elif dst in builder.vmap:
                _gval = builder.vmap[dst]
                if _is_phi_value(_gval):
                    bb_of = getattr(getattr(_gval, "basic_block", None), "name", None)
                    if bb_of != bb.name:
                        continue
                vmap_cur[dst] = _gval
            created_ids.append(dst)
    # Save block-end snapshot
    bid = block_data.get("id", 0)
    # values that were not redefined in this block (but remain live)
    # are available to PHI finalize wiring. This avoids omissions of
    # phi-dst/cyclic and carry-over values.
    snap: Dict[int, ir.Value] = dict(vmap_cur)
    keys = sorted(list(snap.keys()))
    _safe_trace_phi_json({"phi": "snapshot", "block": int(bid), "keys": [int(k) for k in keys[:20]]})
    # Record block-local definitions for lifetime hinting
    for vid in created_ids:
        if vid in vmap_cur:
            builder.def_blocks.setdefault(vid, set()).add(block_data.get("id", 0))
    builder.block_end_values[bid] = snap
    # Clear current vmap context
    _cleanup_current_vmap(builder)


def finalize_phis(builder):
    """Finalize PHIs declared in JSON by wiring incoming edges at block heads.
    Uses resolver._value_at_end_i64 to materialize values at predecessor ends,
    ensuring casts/boxing are inserted in predecessor blocks (dominance-safe).
    """
    # Iterate JSON-declared PHIs per block
    # Build succ map for nearest-predecessor mapping
    succs: Dict[int, List[int]] = {}
    for to_bid, from_list in (builder.preds or {}).items():
        for fr in from_list:
            succs.setdefault(fr, []).append(to_bid)
    for block_id, dst_map in (
        getattr(builder, "block_phi_incomings", {}) or {}
    ).items():
        _safe_trace_phi_json(
            {
                "phi": "finalize_begin",
                "block": int(block_id),
                "dsts": [int(k) for k in (dst_map or {}).keys()],
            }
        )
        bb = builder.bb_map.get(block_id)
        if bb is None:
            continue
        for dst_vid, incoming in (dst_map or {}).items():
            _safe_trace_phi_json(
                {
                    "phi": "finalize_dst",
                    "block": int(block_id),
                    "dst": int(dst_vid),
                    "incoming": [
                        (int(v), int(b))
                        for (b, v) in [(b, v) for (v, b) in (incoming or [])]
                    ],
                }
            )
            # Phase 275 P0: Get dst_type from resolver's value_types (SSOT)
            from phi_wiring.type_helper import get_phi_dst_type

            dst_type = get_phi_dst_type(builder, dst_vid)
            # Ensure placeholder exists at block head with common helper
            phi = _ensure_phi(
                builder, int(block_id), int(dst_vid), bb, dst_type=dst_type
            )
            builder.vmap[int(dst_vid)] = phi
            n = (
                getattr(phi, "name", b"").decode()
                if hasattr(getattr(phi, "name", None), "decode")
                else str(getattr(phi, "name", ""))
            )
            _safe_trace_phi_json(
                {
                    "phi": "finalize_target",
                    "block": int(block_id),
                    "dst": int(dst_vid),
                    "ir": str(n),
                }
            )
            # Wire incoming per CFG predecessor; map src_vid when provided
            preds_raw = [p for p in builder.preds.get(block_id, []) if p != block_id]
            # Deduplicate while preserving order
            seen = set()
            preds_list: List[int] = []
            for p in preds_raw:
                if p not in seen:
                    preds_list.append(p)
                    seen.add(p)
            # Precompute a non-self initial source (if present) to use for self-carry cases
            init_src_vid: Optional[int] = None
            for (b_decl0, v_src0) in incoming:
                vs0 = _safe_int(v_src0)
                if vs0 is None:
                    continue
                if vs0 != int(dst_vid):
                    init_src_vid = vs0
                    break
            # Pre-resolve declared incomings to nearest immediate predecessors
            chosen: Dict[int, ir.Value] = {}
            for (b_decl, v_src) in incoming:
                bd = _safe_int(b_decl)
                vs = _safe_int(v_src)
                if bd is None or vs is None:
                    continue
                _safe_trace_phi(
                    f"[finalize_phis] Processing incoming: dst_vid={dst_vid}, b_decl={bd}, v_src={vs}"
                )
                pred_match = _next_pred_on_path(succs, preds_list, bd, block_id)
                _safe_trace_phi(f"[finalize_phis]   nearest_pred_on_path({bd}) = {pred_match}")
                if pred_match is None:
                    continue
                # If self-carry is specified (vs == dst_vid), map to init_src_vid when available
                if vs == int(dst_vid) and init_src_vid is not None:
                    _safe_trace_phi(
                        f"[finalize_phis]   SELF-CARRY DETECTED: vs={vs} == dst_vid={dst_vid}, replacing with init_src_vid={init_src_vid}"
                    )
                    vs = int(init_src_vid)
                val = _safe_value_at_end(builder, vs, pred_match)
                if val is not None:
                    _safe_trace_phi(
                        f"[finalize_phis]   _value_at_end_i64({vs}, {pred_match}) = {val}"
                    )
                else:
                    _safe_trace_phi(
                        f"[finalize_phis]   _value_at_end_i64({vs}, {pred_match}) FAILED"
                    )
                if val is None:
                    _safe_trace_phi("[finalize_phis]   Value resolution failed, using fallback 0")
                    val = ir.Constant(builder.i64, 0)
                chosen[pred_match] = val
                _safe_trace_phi(f"[finalize_phis]   CHOSEN: pred_bid={pred_match} -> val={val}")
            # Fill remaining predecessors with dst carry or (optionally) a synthesized default
            _safe_trace_phi(
                f"[finalize_phis] Filling remaining preds: preds_list={preds_list}, chosen_keys={list(chosen.keys())}"
            )
            for pred in preds_list:
                if pred in chosen:
                    continue
                val = _safe_value_at_end(builder, int(dst_vid), pred)
                if val is None:
                    val = ir.Constant(builder.i64, 0)
                chosen[pred] = val
            for pred in preds_list:
                _safe_add_incoming(phi, chosen[pred], builder.bb_map[pred])

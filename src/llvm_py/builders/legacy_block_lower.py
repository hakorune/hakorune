from typing import Dict, Any, Optional, List

from llvmlite import ir
from trace import debug as trace_debug
from trace import phi as trace_phi
from trace import phi_json as trace_phi_json
from phi_wiring import ensure_phi as _ensure_phi


def setup_phi_placeholders(builder, blocks: List[Dict[str, Any]]):
    """Predeclare PHIs and collect incoming metadata for finalize_phis.

    This pass is function-local and must be invoked after basic blocks are
    created and before lowering individual blocks. It also tags string-ish
    values eagerly to help downstream resolvers choose correct intrinsics.
    """
    try:
        # Pass A: collect producer stringish hints per value-id
        produced_str: Dict[int, bool] = {}
        for block_data in blocks:
            for inst in block_data.get("instructions", []) or []:
                try:
                    opx = inst.get("op")
                    dstx = inst.get("dst")
                    if dstx is None:
                        continue
                    is_str = False
                    if opx == "const":
                        v = inst.get("value", {}) or {}
                        t = v.get("type")
                        if t == "string" or (
                            isinstance(t, dict)
                            and t.get("kind") in ("handle", "ptr")
                            and t.get("box_type") == "StringBox"
                        ):
                            is_str = True
                    elif opx in ("binop", "boxcall", "externcall"):
                        t = inst.get("dst_type")
                        if (
                            isinstance(t, dict)
                            and t.get("kind") == "handle"
                            and t.get("box_type") == "StringBox"
                        ):
                            is_str = True
                    if is_str:
                        produced_str[int(dstx)] = True
                except Exception:
                    pass
        # Pass B: materialize PHI placeholders and record incoming metadata
        builder.block_phi_incomings = {}
        for block_data in blocks:
            bid0 = block_data.get("id", 0)
            bb0 = builder.bb_map.get(bid0)
            for inst in block_data.get("instructions", []) or []:
                if inst.get("op") == "phi":
                    try:
                        dst0 = int(inst.get("dst"))
                        incoming0 = inst.get("incoming", []) or []
                    except Exception:
                        dst0 = None
                        incoming0 = []
                    if dst0 is None:
                        continue
                    # Record incoming metadata for finalize_phis
                    try:
                        builder.block_phi_incomings.setdefault(bid0, {})[dst0] = [
                            (int(b), int(v)) for (v, b) in incoming0
                        ]
                    except Exception:
                        pass
                    # Ensure placeholder exists at block head
                    if bb0 is not None:
                        b0 = ir.IRBuilder(bb0)
                        try:
                            b0.position_at_start(bb0)
                        except Exception:
                            pass
                        existing = builder.vmap.get(dst0)
                        is_phi = False
                        try:
                            is_phi = hasattr(existing, "add_incoming")
                        except Exception:
                            is_phi = False
                        if not is_phi:
                            ph0 = b0.phi(builder.i64, name=f"phi_{dst0}")
                            # Phase 132-P0: Store PHI in phi_manager ONLY, not in global vmap
                            # This prevents ValueId collisions when different blocks use the same ValueId
                            try:
                                builder.phi_manager.register_phi(int(bid0), int(dst0), ph0)
                            except Exception:
                                # Fallback: store in global vmap (legacy behavior)
                                builder.vmap[dst0] = ph0
                        # Tag propagation: if explicit dst_type marks string or any incoming was produced as string-ish, tag dst
                        try:
                            dst_type0 = inst.get("dst_type")
                            mark_str = (
                                isinstance(dst_type0, dict)
                                and dst_type0.get("kind") == "handle"
                                and dst_type0.get("box_type") == "StringBox"
                            )
                            if not mark_str:
                                for (v_id, _b_id) in incoming0:
                                    try:
                                        if produced_str.get(int(v_id)):
                                            mark_str = True
                                            break
                                    except Exception:
                                        pass
                            if mark_str and hasattr(builder.resolver, "mark_string"):
                                builder.resolver.mark_string(int(dst0))
                        except Exception:
                            pass
                        # Definition hint: PHI defines dst in this block
                        try:
                            builder.def_blocks.setdefault(int(dst0), set()).add(int(bid0))
                        except Exception:
                            pass
        # Sync to resolver
        try:
            builder.resolver.block_phi_incomings = builder.block_phi_incomings
        except Exception:
            pass
    except Exception:
        pass


def lower_block(builder, bb: ir.Block, block_data: Dict[str, Any], func: ir.Function):
    """Lower a single basic block.

    Emit all non-terminator ops first, then control-flow terminators
    (branch/jump/ret). This avoids generating IR after a terminator.
    """
    builder_ir = ir.IRBuilder(bb)
    try:
        trace_debug(f"[llvm-py] === lower_block bb{block_data.get('id')} ===")
    except Exception:
        pass
    # Provide builder/module to resolver for PHI/casts insertion
    try:
        builder.resolver.builder = builder_ir
        builder.resolver.module = builder.module
    except Exception:
        pass
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
    try:
        for _vid, _val in (builder.vmap or {}).items():
            keep = True
            try:
                if hasattr(_val, "add_incoming"):
                    bb_of = getattr(getattr(_val, "basic_block", None), "name", None)
                    keep = bb_of == bb.name
            except Exception:
                keep = False
            if keep:
                vmap_cur[_vid] = _val
    except Exception:
        vmap_cur = dict(builder.vmap)
    # Expose to lower_instruction users (e.g., while_ regular lowering)
    builder._current_vmap = vmap_cur
    created_ids: List[int] = []
    # Compute ids defined in this block to help with copy/PHI decisions
    defined_here_all: set = set()
    for _inst in body_ops:
        try:
            d = _inst.get("dst")
            if isinstance(d, int):
                defined_here_all.add(d)
        except Exception:
            pass
    # Keep PHI synthesis on-demand in resolver; avoid predeclaring here to reduce clashes.
    # Lower body ops first in-order
    for i_idx, inst in enumerate(body_ops):
        try:
            trace_debug(
                f"[llvm-py] body op: {inst.get('op')} dst={inst.get('dst')} cond={inst.get('cond')}"
            )
        except Exception:
            pass
        try:
            if bb.terminator is not None:
                break
        except Exception:
            pass
        builder_ir.position_at_end(bb)
        # Special-case copy: avoid forward self-block dependencies only when src is defined later in this block
        if inst.get("op") == "copy":
            src_i = inst.get("src")
            skip_now = False
            if isinstance(src_i, int):
                try:
                    # Check if src will be defined in a subsequent instruction
                    for _rest in body_ops[i_idx + 1 :]:
                        try:
                            if int(_rest.get("dst")) == int(src_i):
                                skip_now = True
                                break
                        except Exception:
                            pass
                except Exception:
                    pass
            if skip_now:
                # Skip now; a later copy will remap after src becomes available
                pass
            else:
                builder.lower_instruction(builder_ir, inst, func)
        else:
            builder.lower_instruction(builder_ir, inst, func)
        # Phase 131-7: Bidirectional sync between per-block vmap and global vmap
        # This ensures values are available for subsequent instructions (e.g., branch using unop result)
        try:
            dst = inst.get("dst")
            if isinstance(dst, int):
                # First, check if the instruction wrote to vmap_cur (local lowering result)
                if dst in vmap_cur:
                    # Sync vmap_cur[dst] -> builder.vmap[dst] -> vmap_cur[dst]
                    # (bidirectional to ensure both maps have the value)
                    builder.vmap[dst] = vmap_cur[dst]
                    # Explicitly update vmap_cur reference in case it's a different object
                    # (though it should be the same, this ensures consistency)
                # Then check if global vmap has a value (e.g., from resolver)
                elif dst in builder.vmap:
                    _gval = builder.vmap[dst]
                    # Avoid syncing PHIs that belong to other blocks (placeholders)
                    try:
                        if hasattr(_gval, "add_incoming"):
                            bb_of = getattr(getattr(_gval, "basic_block", None), "name", None)
                            if bb_of != bb.name:
                                continue
                    except Exception:
                        pass
                    vmap_cur[dst] = _gval
                # Record if defined in this block to avoid over-localization
                created_ids.append(dst)
        except Exception:
            pass
    # Save block-end snapshot
    bid = block_data.get("id", 0)
    # values that were not redefined in this block (but remain live)
    # are available to PHI finalize wiring. This avoids omissions of
    # phi-dst/cyclic and carry-over values.
    snap: Dict[int, ir.Value] = dict(vmap_cur)
    try:
        keys = sorted(list(snap.keys()))
        # Emit structured snapshot event for up to first 20 keys
        try:
            trace_phi_json(
                {"phi": "snapshot", "block": int(bid), "keys": [int(k) for k in keys[:20]]}
            )
        except Exception:
            pass
    except Exception:
        pass
    # Record block-local definitions for lifetime hinting
    for vid in created_ids:
        if vid in vmap_cur:
            builder.def_blocks.setdefault(vid, set()).add(block_data.get("id", 0))
    builder.block_end_values[bid] = snap
    # Clear current vmap context
    try:
        delattr(builder, "_current_vmap")
    except Exception:
        pass


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
        try:
            trace_phi_json(
                {
                    "phi": "finalize_begin",
                    "block": int(block_id),
                    "dsts": [int(k) for k in (dst_map or {}).keys()],
                }
            )
        except Exception:
            pass
        bb = builder.bb_map.get(block_id)
        if bb is None:
            continue
        for dst_vid, incoming in (dst_map or {}).items():
            try:
                trace_phi_json(
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
            except Exception:
                pass
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
            try:
                trace_phi_json(
                    {
                        "phi": "finalize_target",
                        "block": int(block_id),
                        "dst": int(dst_vid),
                        "ir": str(n),
                    }
                )
            except Exception:
                pass
            # Wire incoming per CFG predecessor; map src_vid when provided
            preds_raw = [p for p in builder.preds.get(block_id, []) if p != block_id]
            # Deduplicate while preserving order
            seen = set()
            preds_list: List[int] = []
            for p in preds_raw:
                if p not in seen:
                    preds_list.append(p)
                    seen.add(p)
            # Helper: find the nearest immediate predecessor on a path decl_b -> ... -> block_id
            def nearest_pred_on_path(decl_b: int) -> Optional[int]:
                # BFS from decl_b to block_id; return the parent of block_id on that path.
                from collections import deque

                q = deque([decl_b])
                visited = set([decl_b])
                parent: Dict[int, Optional[int]] = {decl_b: None}
                while q:
                    cur = q.popleft()
                    if cur == block_id:
                        par = parent.get(block_id)
                        return par if par in preds_list else None
                    for nx in succs.get(cur, []):
                        if nx not in visited:
                            visited.add(nx)
                            parent[nx] = cur
                            q.append(nx)
                return None

            # Precompute a non-self initial source (if present) to use for self-carry cases
            init_src_vid: Optional[int] = None
            for (b_decl0, v_src0) in incoming:
                try:
                    vs0 = int(v_src0)
                except Exception:
                    continue
                if vs0 != int(dst_vid):
                    init_src_vid = vs0
                    break
            # Pre-resolve declared incomings to nearest immediate predecessors
            chosen: Dict[int, ir.Value] = {}
            for (b_decl, v_src) in incoming:
                try:
                    bd = int(b_decl)
                    vs = int(v_src)
                except Exception:
                    continue
                try:
                    trace_phi(
                        f"[finalize_phis] Processing incoming: dst_vid={dst_vid}, b_decl={bd}, v_src={vs}"
                    )
                except Exception:
                    pass
                pred_match = nearest_pred_on_path(bd)
                try:
                    trace_phi(
                        f"[finalize_phis]   nearest_pred_on_path({bd}) = {pred_match}"
                    )
                except Exception:
                    pass
                if pred_match is None:
                    continue
                # If self-carry is specified (vs == dst_vid), map to init_src_vid when available
                if vs == int(dst_vid) and init_src_vid is not None:
                    try:
                        trace_phi(
                            f"[finalize_phis]   SELF-CARRY DETECTED: vs={vs} == dst_vid={dst_vid}, replacing with init_src_vid={init_src_vid}"
                        )
                    except Exception:
                        pass
                    vs = int(init_src_vid)
                try:
                    val = builder.resolver._value_at_end_i64(
                        vs,
                        pred_match,
                        builder.preds,
                        builder.block_end_values,
                        builder.vmap,
                        builder.bb_map,
                    )
                    try:
                        trace_phi(
                            f"[finalize_phis]   _value_at_end_i64({vs}, {pred_match}) = {val}"
                        )
                    except Exception:
                        pass
                except Exception as e:
                    val = None
                    try:
                        trace_phi(
                            f"[finalize_phis]   _value_at_end_i64({vs}, {pred_match}) FAILED: {e}"
                        )
                    except Exception:
                        pass
                if val is None:
                    try:
                        trace_phi(
                            "[finalize_phis]   Value resolution failed, using fallback 0"
                        )
                    except Exception:
                        pass
                    val = ir.Constant(builder.i64, 0)
                chosen[pred_match] = val
                try:
                    trace_phi(
                        f"[finalize_phis]   CHOSEN: pred_bid={pred_match} -> val={val}"
                    )
                except Exception:
                    pass
            # Fill remaining predecessors with dst carry or (optionally) a synthesized default
            try:
                trace_phi(
                    f"[finalize_phis] Filling remaining preds: preds_list={preds_list}, chosen_keys={list(chosen.keys())}"
                )
            except Exception:
                pass
            for pred in preds_list:
                if pred in chosen:
                    continue
                try:
                    val = builder.resolver._value_at_end_i64(
                        int(dst_vid),
                        pred,
                        builder.preds,
                        builder.block_end_values,
                        builder.vmap,
                        builder.bb_map,
                    )
                except Exception:
                    val = None
                if val is None:
                    val = ir.Constant(builder.i64, 0)
                chosen[pred] = val
            for pred in preds_list:
                try:
                    phi.add_incoming(chosen[pred], builder.bb_map[pred])
                except Exception:
                    pass

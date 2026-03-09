from typing import Dict, Any, List, Tuple, NamedTuple
import os
import sys
from llvmlite import ir
from trace import debug as trace_debug
from trace import phi_json as trace_phi_json
from phi_manager import PhiManager


def is_jump_only_block(block_info: Dict) -> bool:
    """Phase 131-14-B: Detect pure jump-only blocks (trampoline blocks).

    A pure jump-only block has:
    - NO PHI instructions (PHI blocks do meaningful work - value merging)
    - NO other instructions except a single terminator (jump/branch/ret)
    - Acts as a pure trampoline/routing block

    Blocks with PHI instructions are NOT jump-only because they perform value
    merging and must compute their own snapshots.
    """
    instructions = block_info.get("instructions", [])

    # Check if block has any PHI instructions
    has_phi = any(i.get("op") == "phi" for i in instructions)
    if has_phi:
        # PHI blocks are NOT jump-only - they do value merging
        return False

    # Check if block has only terminator instructions
    non_term = [
        i for i in instructions
        if i.get("op") not in ("ret", "jump", "branch")
    ]
    return len(non_term) == 0


def get_predecessors(bid: int, preds: Dict[int, List[int]]) -> List[int]:
    """Phase 131-14 P0-3: Get predecessors for a block.

    Args:
        bid: Block ID
        preds: Predecessor map (bid -> [predecessor_bids])

    Returns:
        List of predecessor block IDs
    """
    return preds.get(bid, [])


class DeferredTerminator(NamedTuple):
    """Phase 131-12-P1: Deferred terminator with vmap snapshot.

    This structure captures the terminator operations along with the vmap state
    at the end of Pass A, ensuring Pass C uses the correct SSA context.
    """
    bb: ir.Block
    term_ops: List[Dict[str, Any]]
    vmap_snapshot: Dict[int, ir.Value]


def resolve_jump_only_snapshots(builder, block_by_id: Dict[int, Dict[str, Any]], context):
    """Phase 131-14-B P0-2: Resolve jump-only block snapshots (Pass B).
    Phase 132-P1: Use context Box for function-local state isolation.

    This function runs AFTER all blocks have been lowered (Pass A) but BEFORE
    PHI finalization. It resolves snapshots for jump-only blocks by following
    the CFG to find the nearest non-jump-only predecessor.

    Uses path compression to efficiently handle chains of jump-only blocks.

    SSOT: Snapshots are based on CFG structure, not processing order.

    Args:
        context: FunctionLowerContext Box containing function-local state
    """
    import sys

    strict_mode = os.environ.get('NYASH_LLVM_STRICT') == '1'
    trace_vmap = os.environ.get('NYASH_LLVM_TRACE_VMAP') == '1'

    jump_only = context.jump_only_blocks
    if not jump_only:
        if trace_vmap:
            print("[vmap/resolve/passB] No jump-only blocks to resolve", file=sys.stderr)
        return

    if trace_vmap:
        print(f"[vmap/resolve/passB] Resolving {len(jump_only)} jump-only blocks: {sorted(jump_only.keys())}", file=sys.stderr)

    resolved = {}  # bid -> snapshot dict

    def resolve(bid: int, visited: set | None = None) -> Dict[int, Any]:
        """Recursively resolve snapshot for a block, with cycle detection."""
        if visited is None:
            visited = set()

        # Cycle detection
        if bid in visited:
            if strict_mode:
                raise RuntimeError(
                    f"[LLVM_PY/STRICT] Phase 131-14-B: Cycle detected in jump-only chain: "
                    f"{visited} -> {bid}"
                )
            if trace_vmap:
                print(f"[vmap/resolve/passB] WARNING: Cycle at bb{bid}, returning empty", file=sys.stderr)
            return {}

        visited.add(bid)

        # Already resolved (path compression cache)
        if bid in resolved:
            if trace_vmap:
                print(f"[vmap/resolve/passB] bb{bid} already resolved (cached)", file=sys.stderr)
            return resolved[bid]

        # Normal block - already has snapshot from Pass A
        # Phase 132-P1: Use context.block_end_values (simple block_id key)
        snapshot = context.get_block_snapshot(bid)
        if snapshot:
            if trace_vmap:
                print(
                    f"[vmap/resolve/passB] bb{bid} is normal block with snapshot "
                    f"({len(snapshot)} values)",
                    file=sys.stderr
                )
            return snapshot

        # Jump-only block - resolve from predecessor
        if bid in jump_only:
            pred_bid = jump_only[bid]
            if trace_vmap:
                print(f"[vmap/resolve/passB] bb{bid} is jump-only, resolving from pred bb{pred_bid}", file=sys.stderr)

            # Recursively resolve predecessor
            pred_snapshot = resolve(pred_bid, visited)

            if not pred_snapshot:
                if strict_mode:
                    raise RuntimeError(
                        f"[LLVM_PY/STRICT] Phase 131-14-B: jump-only block bb{bid} "
                        f"cannot resolve snapshot from predecessor bb{pred_bid} "
                        f"(predecessor has no snapshot)"
                    )
                if trace_vmap:
                    print(
                        f"[vmap/resolve/passB] WARNING: bb{bid} pred bb{pred_bid} has no snapshot, "
                        f"using empty dict",
                        file=sys.stderr
                    )
                pred_snapshot = {}

            # Cache the result (path compression)
            resolved[bid] = dict(pred_snapshot)
            if trace_vmap:
                print(
                    f"[vmap/resolve/passB] bb{bid} resolved from bb{pred_bid}: "
                    f"{len(resolved[bid])} values",
                    file=sys.stderr
                )
            return resolved[bid]

        # Unknown block (should not happen if Pass A worked correctly)
        if strict_mode:
            raise RuntimeError(
                f"[LLVM_PY/STRICT] Phase 131-14-B: block bb{bid} is neither normal "
                f"nor jump-only (invalid state)"
            )

        if trace_vmap:
            print(f"[vmap/resolve/passB] WARNING: bb{bid} unknown state, returning empty", file=sys.stderr)
        return {}

    # Resolve all jump-only blocks
    # Phase 132-P1: Use context.set_block_snapshot (simple block_id key)
    for bid in sorted(jump_only.keys()):
        snapshot = resolve(bid)
        context.set_block_snapshot(bid, snapshot)

        if trace_vmap:
            print(
                f"[vmap/resolve/passB] ✅ bb{bid} final snapshot: "
                f"{len(snapshot)} values, keys={sorted(snapshot.keys())[:10]}",
                file=sys.stderr
            )

    if trace_vmap:
        print(f"[vmap/resolve/passB] Pass B complete: resolved {len(jump_only)} jump-only blocks", file=sys.stderr)


def lower_blocks(builder, func: ir.Function, block_by_id: Dict[int, Dict[str, Any]], order: List[int], loop_plan: Dict[str, Any] | None, context):
    """Lower blocks in multi-pass to ensure PHIs are always before terminators.

    Phase 131-4: Multi-pass block lowering architecture
    Phase 131-14-B: Two-pass snapshot resolution
    Phase 132-P1: Use context Box for function-local state isolation
    - Pass A: Lower non-terminator instructions only (terminators deferred)
      - jump-only blocks: record metadata only, NO snapshot resolution
    - Pass B: PHI finalization happens in function_lower.py
      - resolve_jump_only_snapshots() called BEFORE PHI finalization
    - Pass C: Lower terminators (happens after PHI finalization)

    This ensures LLVM IR invariant: PHI nodes must be at block head before any
    other instructions, and terminators must be last.

    Args:
        context: FunctionLowerContext Box containing function-local state
    """
    skipped: set[int] = set()
    if loop_plan is not None:
        try:
            for bskip in loop_plan.get('skip_blocks', []):
                if bskip != loop_plan.get('header'):
                    skipped.add(int(bskip))
        except Exception:
            pass
    for bid in order:
        block_data = block_by_id.get(bid)
        if block_data is None:
            continue
        # If loop prepass applies, lower while once at header and skip loop-internal blocks
        if loop_plan is not None and bid == loop_plan.get('header'):
            bb = builder.bb_map[bid]
            ib = ir.IRBuilder(bb)
            try:
                builder.resolver.builder = ib
                builder.resolver.module = builder.module
                # P0-1: Set current block_id for def_blocks tracking
                builder.resolver.current_block_id = bid
            except Exception:
                pass
            builder.loop_count += 1
            body_insts = loop_plan.get('body_insts', [])
            cond_vid = loop_plan.get('cond')
            from instructions.loopform import lower_while_loopform
            ok = False
            try:
                builder._current_vmap = dict(builder.vmap)
                ok = lower_while_loopform(
                    ib,
                    func,
                    cond_vid,
                    body_insts,
                    builder.loop_count,
                    builder.vmap,
                    builder.bb_map,
                    builder.resolver,
                    builder.preds,
                    builder.block_end_values,
                    getattr(builder, 'ctx', None),
                )
            except Exception:
                ok = False
            if not ok:
                try:
                    builder.resolver._owner_lower_instruction = builder.lower_instruction
                except Exception:
                    pass
                from instructions.controlflow.while_ import lower_while_regular
                lower_while_regular(ib, func, cond_vid, body_insts,
                                    builder.loop_count, builder.vmap, builder.bb_map,
                                    builder.resolver, builder.preds, builder.block_end_values)
            try:
                delattr(builder, '_current_vmap')
            except Exception:
                pass
            for bskip in loop_plan.get('skip_blocks', []):
                skipped.add(bskip)
            # Ensure skipped original blocks have a valid terminator: branch to while exit
            try:
                exit_name = f"while{builder.loop_count}_exit"
                exit_bb = None
                for bbf in func.blocks:
                    try:
                        if str(bbf.name) == exit_name:
                            exit_bb = bbf
                            break
                    except Exception:
                        pass
                if exit_bb is not None:
                    try:
                        orig_exit_bb = builder.bb_map.get(loop_plan.get('exit'))
                        if orig_exit_bb is not None and exit_bb.terminator is None:
                            ibx = ir.IRBuilder(exit_bb)
                            ibx.branch(orig_exit_bb)
                    except Exception:
                        pass
                    for bskip in loop_plan.get('skip_blocks', []):
                        if bskip == loop_plan.get('header'):
                            continue
                        bb_skip = builder.bb_map.get(bskip)
                        if bb_skip is None:
                            continue
                        try:
                            if bb_skip.terminator is None:
                                ib = ir.IRBuilder(bb_skip)
                                if orig_exit_bb is not None:
                                    ib.branch(orig_exit_bb)
                        except Exception:
                            pass
            except Exception:
                pass
            continue

        if bid in skipped:
            continue
        bb = builder.bb_map[bid]
        ib = ir.IRBuilder(bb)
        try:
            builder.resolver.builder = ib
            builder.resolver.module = builder.module
            # P0-1: Set current block_id for def_blocks tracking
            builder.resolver.current_block_id = bid
        except Exception:
            pass
        block_data = block_by_id.get(bid, {})
        insts = block_data.get('instructions', []) or []
        # Split into body and terminator ops
        body_ops: List[Dict[str, Any]] = []
        term_ops: List[Dict[str, Any]] = []
        for inst in insts:
            try:
                opx = inst.get('op')
            except Exception:
                opx = None
            if opx in ("ret","jump","branch"):
                term_ops.append(inst)
            elif opx == "phi":
                continue
            else:
                body_ops.append(inst)
        # Per-block SSA map
        # Phase 132-P1: Use context.phi_manager for PHI filtering (Box-First principle)
        vmap_cur: Dict[int, ir.Value] = {}
        try:
            vmap_cur = context.phi_manager.filter_vmap_preserve_phis(
                builder.vmap or {},
                int(bid),
                context,
            )
            # Trace output for debugging (only if env var set)
            if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
                phi_count = sum(1 for v in vmap_cur.values() if hasattr(v, 'add_incoming'))
                print(f"[vmap/phi_filter] bb{bid} filtered vmap: {len(vmap_cur)} values, {phi_count} PHIs", file=sys.stderr)
        except Exception:
            # Fallback: copy all values without filtering
            vmap_cur = dict(builder.vmap)
        builder._current_vmap = vmap_cur
        # Phase 131-12-P1: Object identity trace for vmap_cur investigation
        import os, sys
        if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
            print(f"[vmap/id] bb{bid} vmap_cur id={id(vmap_cur)} keys={sorted(vmap_cur.keys())[:10]}", file=sys.stderr)
        created_ids: List[int] = []
        defined_here_all: set = set()
        for _inst in body_ops:
            try:
                d = _inst.get('dst')
                if isinstance(d, int):
                    defined_here_all.add(d)
            except Exception:
                pass
        # Lower body ops
        for i_idx, inst in enumerate(body_ops):
            try:
                trace_debug(f"[llvm-py] body op: {inst.get('op')} dst={inst.get('dst')} cond={inst.get('cond')}")
            except Exception:
                pass
            try:
                if bb.terminator is not None:
                    break
            except Exception:
                pass
            ib.position_at_end(bb)
            if inst.get('op') == 'copy':
                src_i = inst.get('src')
                skip_now = False
                if isinstance(src_i, int):
                    try:
                        for _rest in body_ops[i_idx+1:]:
                            try:
                                if int(_rest.get('dst')) == int(src_i):
                                    skip_now = True
                                    break
                            except Exception:
                                pass
                    except Exception:
                        pass
                if skip_now:
                    pass
                else:
                    builder.lower_instruction(ib, inst, func)
            else:
                builder.lower_instruction(ib, inst, func)
            try:
                dst = inst.get("dst")
                if isinstance(dst, int):
                    # Prefer current vmap context (_current_vmap) updates; fallback to global vmap
                    _gval = None
                    try:
                        _gval = vmap_cur.get(dst)
                    except Exception:
                        _gval = None
                    if _gval is None and dst in builder.vmap:
                        _gval = builder.vmap[dst]
                    if _gval is not None:
                        try:
                            if hasattr(_gval, 'add_incoming'):
                                bb_of = getattr(getattr(_gval, 'basic_block', None), 'name', None)
                                if bb_of == bb.name:
                                    vmap_cur[dst] = _gval
                            else:
                                vmap_cur[dst] = _gval
                        except Exception:
                            vmap_cur[dst] = _gval
                    if dst not in created_ids and dst in vmap_cur:
                        created_ids.append(dst)
                        # P0-1.5: Update def_blocks IMMEDIATELY after instruction lowering
                        # This ensures resolver can detect defined_here for same-block uses
                        # Phase 132-P1: Use context.add_def_block
                        try:
                            context.add_def_block(dst, block_data.get("id", 0))
                        except Exception:
                            pass
            except Exception:
                pass
        # Materialize trivial PHI aliases for this block into vmap_cur so snapshots
        # carry alias destinations even when not explicitly used in block body.
        try:
            alias_map = getattr(context, "phi_trivial_aliases", None)
            if isinstance(alias_map, dict):
                for (alias_bid, dst_vid), src_vid in alias_map.items():
                    if int(alias_bid) != int(bid):
                        continue
                    if int(dst_vid) in vmap_cur:
                        continue
                    alias_val = builder.resolver.resolve_i64(
                        int(src_vid),
                        bb,
                        builder.preds,
                        builder.block_end_values,
                        vmap_cur,
                        builder.bb_map,
                    )
                    if alias_val is not None:
                        vmap_cur[int(dst_vid)] = alias_val
                        if int(dst_vid) not in created_ids:
                            created_ids.append(int(dst_vid))
                        context.add_def_block(int(dst_vid), block_data.get("id", 0))
        except Exception:
            pass
        # Phase 131-4 Pass A: DEFER terminators until after PHI finalization
        # Phase 131-12-P1 P0-2: Store terminators WITH vmap_cur snapshot for Pass C
        if not hasattr(builder, '_deferred_terminators'):
            builder._deferred_terminators = {}
        if term_ops:
            # CRITICAL: dict(vmap_cur) creates a snapshot copy to prevent mutation issues
            vmap_snapshot = dict(vmap_cur)
            builder._deferred_terminators[bid] = DeferredTerminator(bb, term_ops, vmap_snapshot)
            # Phase 131-12-P1: Trace snapshot creation
            import os, sys
            if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
                print(f"[vmap/id] Pass A bb{bid} snapshot id={id(vmap_snapshot)} keys={sorted(vmap_snapshot.keys())[:10]}", file=sys.stderr)
        # Phase 131-7: Sync ALL created values to global vmap (not just PHIs)
        # This ensures Pass C (deferred terminators) can access values from Pass A
        # Phase 132-P1: Use context.phi_manager for PHI protection (Box-First principle)
        try:
            # Create sync dict from created values only
            sync_dict = {vid: vmap_cur[vid] for vid in created_ids if vid in vmap_cur}
            # PhiManager.sync_protect_phis ensures PHIs are never overwritten (SSOT)
            context.phi_manager.sync_protect_phis(builder.vmap, sync_dict)
            if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
                print(f"[vmap/sync] bb{bid} synced {len(sync_dict)} values to builder.vmap (PHIs protected)", file=sys.stderr)
        except Exception:
            pass
        # End-of-block snapshot
        # Phase 131-14-B P0-1: Jump-only blocks - record metadata only (Pass A)
        strict_mode = os.environ.get('NYASH_LLVM_STRICT') == '1'
        trace_vmap = os.environ.get('NYASH_LLVM_TRACE_VMAP') == '1'

        is_jump_only = is_jump_only_block(block_data)
        if trace_vmap:
            print(
                f"[vmap/snapshot] bb{bid} is_jump_only={is_jump_only} "
                f"instructions={[i.get('op') for i in block_data.get('instructions', [])]}",
                file=sys.stderr
            )

        if is_jump_only:
            # Phase 131-14-B: Jump-only blocks - record metadata, defer snapshot resolution to Pass B
            preds_list = get_predecessors(bid, builder.preds)

            if len(preds_list) == 0:
                # No predecessors - error in STRICT mode
                if strict_mode:
                    raise RuntimeError(
                        f"[LLVM_PY/STRICT] Phase 131-14-B: jump-only block bb{bid} "
                        f"has no predecessors (orphan trampoline)"
                    )
                # Non-STRICT: use current vmap_cur (defensive fallback)
                snap = dict(vmap_cur)
                if trace_vmap:
                    print(
                        f"[vmap/snapshot] bb{bid} jump-only with 0 preds: "
                        f"using vmap_cur keys={sorted(snap.keys())}",
                        file=sys.stderr
                    )
            elif len(preds_list) == 1:
                # Single predecessor - record metadata for Pass B resolution
                pred_bid = preds_list[0]
                context.register_jump_only_block(bid, pred_bid)

                # DO NOT create snapshot here - will be resolved in Pass B
                # Set snap to None to indicate "skip storing in block_end_values"
                snap = None

                if trace_vmap:
                    print(
                        f"[vmap/snapshot/passA] bb{bid} jump-only: recorded pred=bb{pred_bid}, "
                        f"snapshot deferred to Pass B",
                        file=sys.stderr
                    )
            else:
                # Multiple predecessors - error in STRICT mode (merge rules not yet defined)
                if strict_mode:
                    raise RuntimeError(
                        f"[LLVM_PY/STRICT] Phase 131-14-B: jump-only block bb{bid} "
                        f"has multiple predecessors: {preds_list} "
                        f"(merge propagation not implemented)"
                    )
                # Non-STRICT: use current vmap_cur (defensive fallback)
                snap = dict(vmap_cur)
                if trace_vmap:
                    print(
                        f"[vmap/snapshot] bb{bid} jump-only with multiple preds {preds_list}: "
                        f"using vmap_cur keys={sorted(snap.keys())}",
                        file=sys.stderr
                    )
        else:
            # Normal block: use its own vmap_cur
            snap = dict(vmap_cur)

        # Phase 131-14-B: Only store snapshot if not deferred (snap is not None)
        # Phase 132-P1: Use context.set_block_snapshot (simple block_id key)
        if snap is not None:
            try:
                keys = sorted(list(snap.keys()))
            except Exception:
                keys = list(snap.keys())
            trace_phi_json({"phi": "snapshot", "block": int(bid), "keys": [int(k) for k in keys[:20]]})
            for vid in created_ids:
                if vid in vmap_cur:
                    context.add_def_block(vid, block_data.get("id", 0))
            context.set_block_snapshot(bid, snap)
        else:
            # Jump-only block with deferred snapshot - don't store yet
            if trace_vmap:
                print(
                    f"[vmap/snapshot/passA] bb{bid} snapshot deferred (not stored in block_end_values)",
                    file=sys.stderr
                )
        try:
            delattr(builder, '_current_vmap')
        except Exception:
            pass


def lower_terminators(builder, func: ir.Function):
    """Phase 131-4 Pass C: Lower deferred terminators after PHI finalization.
    Phase 131-12-P1 P0-3: Restore vmap_cur snapshot for each block's terminator lowering.

    This ensures PHI nodes are always at block heads before terminators are added,
    maintaining LLVM IR's invariant: PHIs first, then other instructions, then terminators.
    The vmap snapshot ensures terminators see the SSA context from Pass A, not later mutations.
    """
    if not hasattr(builder, '_deferred_terminators'):
        return

    deferred = builder._deferred_terminators
    trace_debug(f"[llvm-py/pass-c] Lowering {len(deferred)} blocks with deferred terminators")

    import os, sys
    strict_mode = os.environ.get('NYASH_LLVM_STRICT') == '1'

    for bid, deferred_term in deferred.items():
        # Phase 131-12-P1: Unpack DeferredTerminator with vmap snapshot
        bb = deferred_term.bb
        term_ops = deferred_term.term_ops
        vmap_snapshot = deferred_term.vmap_snapshot

        # Phase 131-12-P1 P0-4: STRICT mode assertion
        if strict_mode:
            assert vmap_snapshot is not None, f"STRICT: vmap_snapshot must exist for bb{bid}"
            trace_debug(f"[llvm-py/pass-c/strict] bb{bid} vmap_snapshot id={id(vmap_snapshot)}")

        # Phase 131-12-P1 P0-3: Save and restore _current_vmap
        old_current_vmap = getattr(builder, '_current_vmap', None)
        builder._current_vmap = vmap_snapshot

        # Trace snapshot restoration
        if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
            print(f"[vmap/id] Pass C bb{bid} restored snapshot id={id(vmap_snapshot)} keys={sorted(vmap_snapshot.keys())[:10]}", file=sys.stderr)

        # Phase 131-12-P1 P0-4: STRICT mode verification
        if strict_mode:
            assert hasattr(builder, '_current_vmap'), f"STRICT: _current_vmap must be set for bb{bid} terminator lowering"
            assert id(builder._current_vmap) == id(vmap_snapshot), f"STRICT: _current_vmap must match snapshot for bb{bid}"

        try:
            ib = ir.IRBuilder(bb)
            try:
                builder.resolver.builder = ib
                builder.resolver.module = builder.module
                # Phase 131-4: Disable PHI synthesis during terminator lowering
                # Terminators should only use values that already exist (from Pass A/B)
                builder.resolver._disable_phi_synthesis = True
            except Exception:
                pass

            for inst in term_ops:
                try:
                    trace_debug(f"[llvm-py/pass-c] term op: {inst.get('op')} dst={inst.get('dst')} in bb{bid}")
                except Exception:
                    pass
                try:
                    if bb.terminator is not None:
                        # Terminator already exists (e.g., from loop lowering), skip
                        trace_debug(f"[llvm-py/pass-c] bb{bid} already has terminator, skipping")
                        break
                except Exception:
                    pass
                ib.position_at_end(bb)
                builder.lower_instruction(ib, inst, func)
        finally:
            # Phase 131-12-P1 P0-3: Restore previous _current_vmap state (prevent side effects)
            if old_current_vmap is None:
                if hasattr(builder, '_current_vmap'):
                    delattr(builder, '_current_vmap')
            else:
                builder._current_vmap = old_current_vmap

    # Clean up deferred state
    try:
        delattr(builder, '_deferred_terminators')
    except Exception:
        pass

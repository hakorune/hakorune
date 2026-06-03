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


def _bind_resolver_block(
    builder,
    ir_builder: ir.IRBuilder,
    block_id: int,
    *,
    disable_phi_synthesis: bool = False,
) -> None:
    resolver = getattr(builder, "resolver", None)
    if resolver is None:
        return
    resolver.builder = ir_builder
    resolver.module = builder.module
    resolver.current_block_id = block_id
    if disable_phi_synthesis:
        resolver._disable_phi_synthesis = True


def _bind_resolver_instruction(builder, block_id: int, instruction_index: int) -> None:
    resolver = getattr(builder, "resolver", None)
    if resolver is None:
        return
    resolver.current_block_id = block_id
    resolver.current_instruction_index = int(instruction_index)


def _set_current_vmap(builder, vmap):
    old_current_vmap = getattr(builder, '_current_vmap', None)
    builder._current_vmap = vmap
    return old_current_vmap


def _restore_current_vmap(builder, old_current_vmap) -> None:
    if old_current_vmap is None:
        if hasattr(builder, '_current_vmap'):
            delattr(builder, '_current_vmap')
    else:
        builder._current_vmap = old_current_vmap


def _annotate_instruction(inst: Dict[str, Any], block_id: int, instruction_index: int):
    op = inst.get('op')
    inst["__block_id"] = block_id
    inst["__instruction_index"] = instruction_index
    return op


def _split_block_ops(insts: List[Dict[str, Any]], block_id: int):
    body_ops: List[Dict[str, Any]] = []
    term_ops: List[Dict[str, Any]] = []
    for original_instruction_index, inst in enumerate(insts):
        op = _annotate_instruction(inst, block_id, original_instruction_index)
        if op in ("ret", "jump", "branch"):
            term_ops.append(inst)
        elif op == "phi":
            continue
        else:
            body_ops.append(inst)
    return body_ops, term_ops


def _record_created_id(context, created_ids: List[int], vid: int, block_id: int) -> None:
    if vid not in created_ids:
        created_ids.append(vid)
    add_def_block = getattr(context, "add_def_block", None)
    if callable(add_def_block):
        add_def_block(vid, block_id)


def _find_function_block(func: ir.Function, name: str):
    for block in func.blocks:
        if str(block.name) == name:
            return block
    return None


def _branch_to_if_open(block, target) -> None:
    if block is not None and target is not None and block.terminator is None:
        ir.IRBuilder(block).branch(target)


def _patch_loop_prepass_skipped_blocks(builder, func: ir.Function, loop_plan, loop_count: int) -> None:
    exit_bb = _find_function_block(func, f"while{loop_count}_exit")
    if exit_bb is None:
        return

    orig_exit_bb = builder.bb_map.get(loop_plan.get('exit'))
    _branch_to_if_open(exit_bb, orig_exit_bb)

    for bskip in loop_plan.get('skip_blocks', []):
        if bskip == loop_plan.get('header'):
            continue
        _branch_to_if_open(builder.bb_map.get(bskip), orig_exit_bb)


def _body_redefines_value(body_ops: List[Dict[str, Any]], start_index: int, value_id: int) -> bool:
    for rest in body_ops[start_index:]:
        dst = rest.get('dst')
        if isinstance(dst, int) and int(dst) == int(value_id):
            return True
    return False


def _sync_lowered_dst(context, builder, vmap_cur, created_ids, block_id: int, bb, inst) -> None:
    dst = inst.get("dst")
    if not isinstance(dst, int):
        return

    lowered_value = vmap_cur.get(dst)
    if lowered_value is None:
        lowered_value = (getattr(builder, "vmap", {}) or {}).get(dst)
    if lowered_value is None:
        return

    if hasattr(lowered_value, 'add_incoming'):
        value_block_name = getattr(getattr(lowered_value, 'basic_block', None), 'name', None)
        if value_block_name != bb.name:
            return

    vmap_cur[dst] = lowered_value
    if dst in vmap_cur:
        # P0-1.5: Update def_blocks IMMEDIATELY after instruction lowering.
        _record_created_id(context, created_ids, dst, block_id)


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

    resolved = {}  # bid -> (value snapshot, string_ptr snapshot)

    def resolve(bid: int, visited: set | None = None) -> tuple[Dict[int, Any], Dict[int, Any]]:
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
            return {}, {}

        visited.add(bid)

        # Already resolved (path compression cache)
        if bid in resolved:
            if trace_vmap:
                print(f"[vmap/resolve/passB] bb{bid} already resolved (cached)", file=sys.stderr)
            return resolved[bid]

        # Normal block - already has snapshot from Pass A
        # Phase 132-P1: Use context.block_end_values (simple block_id key)
        snapshot = context.get_block_snapshot(bid)
        ptr_snapshot = context.get_block_string_ptr_snapshot(bid)
        if snapshot or ptr_snapshot:
            if trace_vmap:
                print(
                    f"[vmap/resolve/passB] bb{bid} is normal block with snapshot "
                    f"({len(snapshot)} values)",
                    file=sys.stderr
                )
            return snapshot, ptr_snapshot

        # Jump-only block - resolve from predecessor
        if bid in jump_only:
            pred_bid = jump_only[bid]
            if trace_vmap:
                print(f"[vmap/resolve/passB] bb{bid} is jump-only, resolving from pred bb{pred_bid}", file=sys.stderr)

            # Recursively resolve predecessor
            pred_snapshot, pred_ptr_snapshot = resolve(pred_bid, visited)

            if not pred_snapshot and not pred_ptr_snapshot:
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
                pred_ptr_snapshot = {}

            # Cache the result (path compression)
            resolved[bid] = (dict(pred_snapshot), dict(pred_ptr_snapshot))
            if trace_vmap:
                print(
                    f"[vmap/resolve/passB] bb{bid} resolved from bb{pred_bid}: "
                    f"{len(resolved[bid][0])} values / {len(resolved[bid][1])} ptr mirrors",
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
        return {}, {}

    # Resolve all jump-only blocks
    # Phase 132-P1: Use context.set_block_snapshot (simple block_id key)
    for bid in sorted(jump_only.keys()):
        snapshot, ptr_snapshot = resolve(bid)
        context.set_block_snapshot(bid, snapshot)
        context.set_block_string_ptr_snapshot(bid, ptr_snapshot)

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
        for bskip in loop_plan.get('skip_blocks', []):
            if bskip != loop_plan.get('header'):
                skipped.add(int(bskip))
    for bid in order:
        block_data = block_by_id.get(bid)
        if block_data is None:
            continue
        # If loop prepass applies, lower while once at header and skip loop-internal blocks
        if loop_plan is not None and bid == loop_plan.get('header'):
            bb = builder.bb_map[bid]
            ib = ir.IRBuilder(bb)
            _bind_resolver_block(builder, ib, bid)
            builder.loop_count += 1
            body_insts = loop_plan.get('body_insts', [])
            cond_vid = loop_plan.get('cond')
            loop_simd_contract = None
            header_bid = int(loop_plan.get("header"))
            ctx = getattr(builder, "ctx", None)
            if ctx is not None:
                loop_simd_contract = getattr(ctx, "loop_simd_contracts", {}).get(header_bid)
            from instructions.loopform import lower_while_loopform
            ok = False
            try:
                _set_current_vmap(builder, dict(builder.vmap))
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
                    loop_simd_contract,
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
                                    builder.resolver, builder.preds, builder.block_end_values,
                                    loop_simd_contract)
            _restore_current_vmap(builder, None)
            for bskip in loop_plan.get('skip_blocks', []):
                skipped.add(bskip)
            # Ensure skipped original blocks have a valid terminator: branch to while exit
            _patch_loop_prepass_skipped_blocks(builder, func, loop_plan, builder.loop_count)
            continue

        if bid in skipped:
            continue
        bb = builder.bb_map[bid]
        ib = ir.IRBuilder(bb)
        _bind_resolver_block(builder, ib, bid)
        block_data = block_by_id.get(bid, {})
        insts = block_data.get('instructions', []) or []
        body_ops, term_ops = _split_block_ops(insts, bid)
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
        _set_current_vmap(builder, vmap_cur)
        # Phase 131-12-P1: Object identity trace for vmap_cur investigation
        import os, sys
        if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
            print(f"[vmap/id] bb{bid} vmap_cur id={id(vmap_cur)} keys={sorted(vmap_cur.keys())[:10]}", file=sys.stderr)
        created_ids: List[int] = []
        # Lower body ops
        for i_idx, inst in enumerate(body_ops):
            trace_debug(f"[llvm-py] body op: {inst.get('op')} dst={inst.get('dst')} cond={inst.get('cond')}")
            if bb.terminator is not None:
                break
            ib.position_at_end(bb)
            _bind_resolver_instruction(builder, bid, inst.get("__instruction_index", i_idx))
            if inst.get('op') == 'copy':
                src_i = inst.get('src')
                if isinstance(src_i, int) and _body_redefines_value(body_ops, i_idx + 1, src_i):
                    pass
                else:
                    builder.lower_instruction(ib, inst, func)
            else:
                builder.lower_instruction(ib, inst, func)
            _sync_lowered_dst(context, builder, vmap_cur, created_ids, block_data.get("id", 0), bb, inst)
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
                        _record_created_id(
                            context,
                            created_ids,
                            int(dst_vid),
                            block_data.get("id", 0),
                        )
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
                ptr_snap = dict(getattr(builder.resolver, "string_ptrs", {}) or {})
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
                ptr_snap = None

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
                ptr_snap = dict(getattr(builder.resolver, "string_ptrs", {}) or {})
                if trace_vmap:
                    print(
                        f"[vmap/snapshot] bb{bid} jump-only with multiple preds {preds_list}: "
                        f"using vmap_cur keys={sorted(snap.keys())}",
                        file=sys.stderr
                    )
        else:
            # Normal block: use its own vmap_cur
            snap = dict(vmap_cur)
            ptr_snap = dict(getattr(builder.resolver, "string_ptrs", {}) or {})

        # Phase 131-14-B: Only store snapshot if not deferred (snap is not None)
        # Phase 132-P1: Use context.set_block_snapshot (simple block_id key)
        if snap is not None:
            keys = sorted(list(snap.keys()))
            trace_phi_json({"phi": "snapshot", "block": int(bid), "keys": [int(k) for k in keys[:20]]})
            for vid in created_ids:
                if vid in vmap_cur:
                    _record_created_id(context, created_ids, vid, block_data.get("id", 0))
            context.set_block_snapshot(bid, snap)
            context.set_block_string_ptr_snapshot(bid, ptr_snap or {})
        else:
            # Jump-only block with deferred snapshot - don't store yet
            if trace_vmap:
                print(
                    f"[vmap/snapshot/passA] bb{bid} snapshot deferred (not stored in block_end_values)",
                    file=sys.stderr
                )
        _restore_current_vmap(builder, None)


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
        old_current_vmap = _set_current_vmap(builder, vmap_snapshot)

        # Trace snapshot restoration
        if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
            print(f"[vmap/id] Pass C bb{bid} restored snapshot id={id(vmap_snapshot)} keys={sorted(vmap_snapshot.keys())[:10]}", file=sys.stderr)

        # Phase 131-12-P1 P0-4: STRICT mode verification
        if strict_mode:
            assert hasattr(builder, '_current_vmap'), f"STRICT: _current_vmap must be set for bb{bid} terminator lowering"
            assert id(builder._current_vmap) == id(vmap_snapshot), f"STRICT: _current_vmap must match snapshot for bb{bid}"

        try:
            ib = ir.IRBuilder(bb)
            # Phase 131-4: Disable PHI synthesis during terminator lowering.
            # Terminators should only use values that already exist (from Pass A/B).
            _bind_resolver_block(builder, ib, bid, disable_phi_synthesis=True)

            for inst in term_ops:
                trace_debug(f"[llvm-py/pass-c] term op: {inst.get('op')} dst={inst.get('dst')} in bb{bid}")
                if bb.terminator is not None:
                    # Terminator already exists (e.g., from loop lowering), skip
                    trace_debug(f"[llvm-py/pass-c] bb{bid} already has terminator, skipping")
                    break
                ib.position_at_end(bb)
                _bind_resolver_instruction(builder, bid, inst.get("__instruction_index", -1))
                builder.lower_instruction(ib, inst, func)
        finally:
            # Phase 131-12-P1 P0-3: Restore previous _current_vmap state (prevent side effects)
            _restore_current_vmap(builder, old_current_vmap)

    # Clean up deferred state
    if hasattr(builder, '_deferred_terminators'):
        delattr(builder, '_deferred_terminators')

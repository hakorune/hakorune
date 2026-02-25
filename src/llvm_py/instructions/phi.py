"""
PHI instruction lowering
Critical for SSA form - handles value merging from different control flow paths
"""

import llvmlite.ir as ir
from phi_wiring import phi_at_block_head
from phi_wiring.debug_helper import is_phi_debug_enabled
from typing import Dict, List, Tuple, Optional
from utils.values import safe_vmap_write

def lower_phi(
    builder: ir.IRBuilder,
    dst_vid: int,
    incoming: List[Tuple[int, int]],  # [(value_id, block_id), ...]
    vmap: Dict[int, ir.Value],
    bb_map: Dict[int, ir.Block],
    current_block: ir.Block,
    resolver=None,  # Resolver instance (optional)
    block_end_values: Optional[Dict[int, Dict[int, ir.Value]]] = None,
    preds_map: Optional[Dict[int, List[int]]] = None
) -> None:
    """
    Lower MIR PHI instruction

    Args:
        builder: Current LLVM IR builder
        dst_vid: Destination value ID
        incoming: List of (value_id, block_id) pairs
        vmap: Value map
        bb_map: Block map
        current_block: Current basic block
        resolver: Optional resolver for advanced type handling
    """
    # Phase 275 DEBUG
    import sys
    sys.stderr.write(f"[PHI_ENTER] dst={dst_vid} incoming={incoming}\n")
    sys.stderr.flush()

    if not incoming:
        # No incoming edges - use zero
        vmap[dst_vid] = ir.Constant(ir.IntType(64), 0)
        return
    
    # Phase 275 P0: Detect PHI type from incoming values
    # Default is i64 for handles, but use double if any incoming is double
    phi_type = ir.IntType(64)
    has_double = False

    # First pass: check if any incoming value is double type
    import sys
    phi_debug = is_phi_debug_enabled()
    if phi_debug:
        sys.stderr.write(f"[PHI_TYPE] Processing dst={dst_vid} with {len(incoming)} incoming edges\n")
        sys.stderr.flush()
    for val_id, block_id in incoming:
        block = bb_map.get(block_id)
        if block is None:
            if phi_debug:
                sys.stderr.write(f"[PHI_TYPE] dst={dst_vid} val={val_id} block={block_id} -> block not found\n")
            continue
        if block_end_values is not None:
            pred_snapshot = block_end_values.get(block_id, {})
            val = pred_snapshot.get(val_id) if val_id is not None else None
            if phi_debug:
                val_type = str(val.type) if val is not None and hasattr(val, 'type') else 'None'
                sys.stderr.write(f"[PHI_TYPE] dst={dst_vid} val={val_id} block={block_id} -> type={val_type}\n")
            if val is not None:
                try:
                    if isinstance(val.type, ir.DoubleType):
                        has_double = True
                        break
                except Exception:
                    pass
        else:
            if phi_debug:
                sys.stderr.write(f"[PHI_TYPE] dst={dst_vid} block_end_values is None\n")

    if has_double:
        phi_type = ir.DoubleType()
        if phi_debug:
            sys.stderr.write(f"[PHI_TYPE] dst={dst_vid} -> using DoubleType\n")
    else:
        if phi_debug:
            sys.stderr.write(f"[PHI_TYPE] dst={dst_vid} -> using IntType(64)\n")
    
    # Build map from provided incoming
    incoming_map: Dict[int, int] = {}
    for val_id, block_id in incoming:
        incoming_map[block_id] = val_id

    # Resolve actual predecessor set
    cur_bid = None
    try:
        cur_bid = int(str(current_block.name).replace('bb',''))
    except Exception:
        pass
    actual_preds = []
    if preds_map is not None and cur_bid is not None:
        actual_preds = [p for p in preds_map.get(cur_bid, []) if p != cur_bid]
    else:
        # Fallback: use blocks in incoming list
        actual_preds = [b for _, b in incoming]

    # P1: Collect incoming values from corresponding snapshots (SSOT)
    incoming_pairs: List[Tuple[ir.Block, ir.Value]] = []
    used_default_zero = False

    import os
    strict_mode = os.environ.get('NYASH_LLVM_STRICT') == '1'

    for block_id in actual_preds:
        block = bb_map.get(block_id)
        vid = incoming_map.get(block_id)
        if block is None:
            continue

        # P1: Get value from pred_bid's snapshot (SSOT - no global search)
        if block_end_values is not None:
            pred_snapshot = block_end_values.get(block_id, {})
            val = pred_snapshot.get(vid) if vid is not None else None
        else:
            val = None

        if val is None:
            # P1: STRICT mode - fail fast on snapshot miss
            if strict_mode:
                available_keys = sorted(list(pred_snapshot.keys())) if block_end_values is not None else []
                raise RuntimeError(
                    f"[LLVM_PY/STRICT] PHI incoming miss:\n"
                    f"  Source ValueId: v{vid}\n"
                    f"  Predecessor Block: bb{block_id}\n"
                    f"  PHI destination: v{dst_vid}\n"
                    f"  Available in snapshot: {available_keys}\n"
                    f"  Hint: Value v{vid} should be in block_end_values[{block_id}]"
                )
            # Non-STRICT: fallback to 0
            val = ir.Constant(phi_type, 0)
            used_default_zero = True
        else:
            # Coerce types at predecessor end if needed
            if hasattr(val, 'type') and val.type != phi_type:
                pb = ir.IRBuilder(block)
                try:
                    term = block.terminator
                    if term is not None:
                        pb.position_before(term)
                    else:
                        pb.position_at_end(block)
                except Exception:
                    pb.position_at_end(block)

                # Phase 275 P0: Handle type conversions for mixed PHI
                if isinstance(phi_type, ir.DoubleType) and isinstance(val.type, ir.IntType):
                    # i64 → double: number promotion
                    val = pb.sitofp(val, phi_type, name=f"phi_i2f_{vid}")
                elif isinstance(phi_type, ir.IntType) and isinstance(val.type, ir.DoubleType):
                    # double → i64: convert back (shouldn't happen if type detection works)
                    val = pb.fptosi(val, phi_type, name=f"phi_f2i_{vid}")
                elif isinstance(phi_type, ir.IntType) and val.type.is_pointer:
                    i8p = ir.IntType(8).as_pointer()
                    try:
                        if hasattr(val.type, 'pointee') and isinstance(val.type.pointee, ir.ArrayType):
                            c0 = ir.Constant(ir.IntType(32), 0)
                            val = pb.gep(val, [c0, c0], name=f"phi_gep_{vid}")
                    except Exception:
                        pass
                    boxer = None
                    for f in builder.module.functions:
                        if f.name == 'nyash.box.from_i8_string':
                            boxer = f
                            break
                    if boxer is None:
                        boxer = ir.Function(builder.module, ir.FunctionType(ir.IntType(64), [i8p]), name='nyash.box.from_i8_string')
                    val = pb.call(boxer, [val], name=f"phi_ptr2h_{vid}")
        incoming_pairs.append((block, val))

    # If nothing collected, use zero constant and bail out
    if not incoming_pairs:
        vmap[dst_vid] = ir.Constant(phi_type, 0)
        return

    # Create PHI instruction at the block head and add incoming
    phi = phi_at_block_head(current_block, phi_type, name=f"phi_{dst_vid}")
    for block, val in incoming_pairs:
        phi.add_incoming(val, block)
    
    # Store PHI result
    vmap[dst_vid] = phi

    # Register PHI definition in def_blocks (critical for resolver dominance tracking)
    if resolver is not None and hasattr(resolver, 'def_blocks') and cur_bid is not None:
        resolver.def_blocks.setdefault(dst_vid, set()).add(cur_bid)
        if is_phi_debug_enabled():
            try:
                from trace import phi as trace_phi_debug
                trace_phi_debug(f"[PHI_DEBUG] Registered dst_vid={dst_vid} in def_blocks for block={cur_bid}")
            except Exception:
                pass

    # Strict mode: fail fast on synthesized zeros (indicates incomplete incoming or dominance issue)
    from phi_wiring.debug_helper import is_phi_strict_enabled
    if used_default_zero and is_phi_strict_enabled():
        raise RuntimeError(f"[LLVM_PY] PHI dst={dst_vid} used synthesized zero; check preds/incoming")
    try:
        from trace import phi as trace_phi
        try:
            blkname = str(current_block.name)
        except Exception:
            blkname = '<blk>'
        trace_phi(f"[PHI] {blkname} v{dst_vid} incoming={len(incoming_pairs)} zero={1 if used_default_zero else 0}")
    except Exception:
        pass
    # Propagate string-ness: if any incoming value-id is tagged string-ish, mark dst as string-ish.
    try:
        if resolver is not None and hasattr(resolver, 'is_stringish') and hasattr(resolver, 'mark_string'):
            for val_id, _b in incoming:
                try:
                    if resolver.is_stringish(val_id):
                        resolver.mark_string(dst_vid)
                        break
                except Exception:
                    pass
    except Exception:
        pass

    # Propagate literal StringBox origin metadata when incoming origins agree.
    # This enables loop PHI vars to keep const-string provenance for FAST length/len.
    try:
        src_map = getattr(resolver, "newbox_string_args", None) if resolver is not None else None
        if isinstance(src_map, dict):
            candidate = None
            conflict = False
            for val_id, _b in incoming:
                mapped = None
                if val_id == dst_vid and candidate is not None:
                    mapped = candidate
                else:
                    mapped = src_map.get(val_id)
                if mapped is None:
                    continue
                if candidate is None:
                    candidate = mapped
                elif candidate != mapped:
                    conflict = True
                    break
            if not conflict and candidate is not None:
                src_map[dst_vid] = candidate
    except Exception:
        pass

    # Propagate literal string table for PHI destination when incoming literals agree.
    try:
        lit_map = getattr(resolver, "string_literals", None) if resolver is not None else None
        if isinstance(lit_map, dict):
            candidate = None
            conflict = False
            for val_id, _b in incoming:
                mapped = lit_map.get(val_id)
                if not isinstance(mapped, str):
                    continue
                if candidate is None:
                    candidate = mapped
                elif candidate != mapped:
                    conflict = True
                    break
            if not conflict and candidate is not None:
                lit_map[dst_vid] = candidate
    except Exception:
        pass

def defer_phi_wiring(
    dst_vid: int,
    incoming: List[Tuple[int, int]],
    phi_deferrals: List[Tuple[int, List[Tuple[int, int]]]]
) -> None:
    """
    Defer PHI wiring for sealed block approach
    
    Args:
        dst_vid: Destination value ID
        incoming: Incoming edges
        phi_deferrals: List to store deferred PHIs
    """
    phi_deferrals.append((dst_vid, incoming))

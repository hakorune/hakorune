from __future__ import annotations
from typing import Dict, List, Any

import llvmlite.ir as ir

from .common import trace
from .analysis import analyze_incomings, collect_produced_stringish
from .fact_propagation import mark_arrayish_handle, should_mark_phi_arrayish
from .wiring import ensure_phi
from .debug_helper import is_phi_debug_enabled


def _trivial_phi_alias(dst_vid: int, incoming: list) -> int | None:
    """Return source vid only when PHI is strict copy-like (all incoming src vids identical).

    Note:
      Self-carry shapes like {dst, init} are intentionally excluded here.
      They are semantically invariant but not dominance-trivial, so aliasing them
      can mis-resolve block-entry values when snapshots are merged.
    """
    if not isinstance(incoming, list) or len(incoming) == 0:
        return None
    src_set = set()
    for pair in incoming:
        try:
            v_src, _b = pair
            src_set.add(int(v_src))
        except Exception:
            return None
    if len(src_set) == 1:
        src_vid = next(iter(src_set))
        if src_vid == int(dst_vid):
            return None
        return src_vid

    return None


def _propagate_string_tag(builder, dst_vid: int, dst_type0, incoming0, produced_str: Dict[int, bool]) -> None:
    """Propagate string-ish tag for PHI destination (best-effort)."""
    try:
        mark_str = (
            isinstance(dst_type0, dict)
            and (
                dst_type0.get("kind") == "string"
                or (
                    dst_type0.get("kind") == "handle"
                    and dst_type0.get("box_type") == "StringBox"
                )
            )
        )
        if not mark_str:
            for (v_src_i, _b_decl_i) in incoming0:
                try:
                    if produced_str.get(int(v_src_i)):
                        mark_str = True
                        break
                except Exception:
                    pass
        if mark_str and hasattr(builder.resolver, "mark_string"):
            builder.resolver.mark_string(int(dst_vid))
    except Exception:
        pass


def _propagate_array_tag(builder, dst_vid: int, dst_type0, incoming0) -> None:
    """Propagate ArrayBox handle fact for PHI destinations."""
    try:
        resolver = getattr(builder, "resolver", None)
        if should_mark_phi_arrayish(resolver, dst_type0, incoming0):
            mark_arrayish_handle(resolver, int(dst_vid))
    except Exception:
        pass


def _propagate_phi_tags(builder, dst_vid: int, dst_type0, incoming0, produced_str: Dict[int, bool]) -> None:
    _propagate_string_tag(builder, dst_vid, dst_type0, incoming0, produced_str)
    _propagate_array_tag(builder, dst_vid, dst_type0, incoming0)


def _sync_block_phi_incomings(builder, blocks: List[Dict[str, Any]]) -> None:
    analyzed = analyze_incomings(blocks)
    try:
        current = getattr(builder, "block_phi_incomings", None)
        if not isinstance(current, dict):
            builder.block_phi_incomings = {}
    except Exception:
        builder.block_phi_incomings = {}
    builder.block_phi_incomings.clear()
    builder.block_phi_incomings.update(analyzed)
    try:
        builder.resolver.block_phi_incomings = builder.block_phi_incomings
    except Exception:
        pass


def _clear_phi_trivial_aliases(builder) -> None:
    try:
        if hasattr(builder, "phi_trivial_aliases") and isinstance(builder.phi_trivial_aliases, dict):
            builder.phi_trivial_aliases.clear()
    except Exception:
        pass


def _register_trivial_alias(builder, block_id: int, dst_vid: int, alias_src: int) -> None:
    try:
        if hasattr(builder, "phi_trivial_aliases") and isinstance(builder.phi_trivial_aliases, dict):
            builder.phi_trivial_aliases[(int(block_id), int(dst_vid))] = int(alias_src)
        if hasattr(builder, "resolver") and hasattr(builder.resolver, "phi_trivial_aliases"):
            if isinstance(builder.resolver.phi_trivial_aliases, dict):
                builder.resolver.phi_trivial_aliases[(int(block_id), int(dst_vid))] = int(alias_src)
    except Exception:
        pass
    trace({
        "phi": "setup_trivial_alias",
        "block": int(block_id),
        "dst": int(dst_vid),
        "src": int(alias_src),
    })


def _propagate_trivial_alias_string_ptr(builder, dst_vid: int, alias_src: int) -> None:
    try:
        resolver = getattr(builder, "resolver", None)
        ptr_map = getattr(resolver, "string_ptrs", None)
        if isinstance(ptr_map, dict) and int(alias_src) in ptr_map:
            ptr_map[int(dst_vid)] = ptr_map[int(alias_src)]
    except Exception:
        pass


def _register_phi_definition(builder, block_id: int, dst_vid: int) -> None:
    try:
        builder.def_blocks.setdefault(int(dst_vid), set()).add(int(block_id))
        if is_phi_debug_enabled():
            trace({"phi_def_blocks": "registered", "dst": int(dst_vid), "block": int(block_id)})
    except Exception:
        pass


def _phi_instructions(block_data: Dict[str, Any]) -> List[Dict[str, Any]]:
    return [inst for inst in block_data.get("instructions", []) or [] if inst.get("op") == "phi"]


def _debug_phi_block(debug_mode: bool, block_id: int, phi_count: int) -> None:
    if not debug_mode or phi_count <= 0:
        return
    import sys
    print(f"[phi_wiring/setup] Block {block_id}: {phi_count} PHI instructions to create", file=sys.stderr)


def _warn_phi_terminator(debug_mode: bool, block_id: int, dst_vid: int, bb0) -> None:
    if not debug_mode:
        return
    try:
        if bb0.terminator is None:
            return
    except Exception:
        return
    import sys
    print(
        f"[phi_wiring/setup] WARNING: Block {block_id} already has terminator when creating PHI for v{dst_vid}!",
        file=sys.stderr,
    )


def _register_predeclared_phi(builder, block_id: int, dst_vid: int, ph, debug_mode: bool) -> None:
    try:
        if not hasattr(builder, "predeclared_ret_phis") or builder.predeclared_ret_phis is None:
            builder.predeclared_ret_phis = {}
    except Exception:
        builder.predeclared_ret_phis = {}

    try:
        builder.predeclared_ret_phis[(int(block_id), int(dst_vid))] = ph
        if hasattr(builder, "phi_manager"):
            builder.phi_manager.register_phi(int(block_id), int(dst_vid), ph)
        if debug_mode:
            import sys
            print(f"[phi_wiring/setup] Created PHI placeholder for v{dst_vid} in bb{block_id}", file=sys.stderr)
    except Exception:
        pass


def _phi_string_ptr_placeholder_needed(builder, dst_vid: int) -> bool:
    resolver = getattr(builder, "resolver", None)
    if resolver is None:
        return False
    try:
        if hasattr(resolver, "is_stringish") and resolver.is_stringish(int(dst_vid)):
            return True
    except Exception:
        pass
    try:
        marked = getattr(resolver, "marked", None)
        if isinstance(marked, set) and int(dst_vid) in marked:
            return True
    except Exception:
        pass
    return False


def _create_string_ptr_phi_placeholder(builder, block_id: int, dst_vid: int, bb0, debug_mode: bool) -> None:
    resolver = getattr(builder, "resolver", None)
    ptr_map = getattr(resolver, "string_ptrs", None)
    if not isinstance(ptr_map, dict):
        return
    try:
        existing = ptr_map.get(int(dst_vid))
        existing_bb = getattr(getattr(existing, "basic_block", None), "name", None)
        current_bb = getattr(bb0, "name", None)
        if existing is not None and hasattr(existing, "add_incoming") and isinstance(getattr(existing, "type", None), ir.PointerType):
            if existing_bb == current_bb:
                return
    except Exception:
        pass

    b0 = ir.IRBuilder(bb0)
    try:
        instrs = list(bb0.instructions)
        if instrs:
            b0.position_before(instrs[0])
        else:
            b0.position_at_start(bb0)
    except Exception:
        pass
    ptr_map[int(dst_vid)] = b0.phi(ir.IntType(8).as_pointer(), name=f"phi_strptr_{dst_vid}")
    if debug_mode:
        import sys
        print(f"[phi_wiring/setup] Created string-ptr PHI placeholder for v{dst_vid} in bb{block_id}", file=sys.stderr)


def _create_phi_placeholder(builder, block_id: int, dst_vid: int, bb0, inst: Dict[str, Any], debug_mode: bool) -> None:
    _warn_phi_terminator(debug_mode, block_id, dst_vid, bb0)
    try:
        from .type_helper import get_phi_dst_type

        dst_type = get_phi_dst_type(builder, dst_vid, inst=inst)
        ph = ensure_phi(builder, block_id, dst_vid, bb0, dst_type=dst_type)
        _register_predeclared_phi(builder, block_id, dst_vid, ph, debug_mode)
    except Exception as e:
        if debug_mode:
            import sys
            print(f"[phi_wiring/setup] ERROR creating PHI for v{dst_vid} in bb{block_id}: {e}", file=sys.stderr)


def _setup_phi_instruction(
    builder,
    block_id: int,
    bb0,
    inst: Dict[str, Any],
    produced_str: Dict[int, bool],
    debug_mode: bool,
) -> None:
    try:
        dst0 = int(inst.get("dst"))
        incoming0 = inst.get("incoming", []) or []
    except Exception:
        return

    alias_src = _trivial_phi_alias(dst0, incoming0)
    if alias_src is not None:
        _register_trivial_alias(builder, block_id, dst0, alias_src)
        _propagate_trivial_alias_string_ptr(builder, dst0, alias_src)
        _register_phi_definition(builder, block_id, dst0)
        _propagate_phi_tags(builder, int(dst0), inst.get("dst_type"), incoming0, produced_str)
        return

    _create_phi_placeholder(builder, block_id, dst0, bb0, inst, debug_mode)
    _propagate_phi_tags(builder, int(dst0), inst.get("dst_type"), incoming0, produced_str)
    if _phi_string_ptr_placeholder_needed(builder, int(dst0)):
        _create_string_ptr_phi_placeholder(builder, block_id, dst0, bb0, debug_mode)
    _register_phi_definition(builder, block_id, dst0)


def setup_phi_placeholders(builder, blocks: List[Dict[str, Any]]):
    """Predeclare PHIs and collect incoming metadata for finalize_phis.

    Phase 132 Enhancement: Ensure ALL PHI instructions are created at block heads
    BEFORE any other instructions are added. This is critical for LLVM IR validity.

    Function-local: must be invoked after basic blocks are created and before
    lowering individual blocks. Also tags string-ish values to help downstream
    resolvers.
    """
    try:
        produced_str = collect_produced_stringish(blocks)
        # Phase 132-P1: Update existing block_phi_incomings dict (points to context storage)
        # Don't replace it with assignment, as that breaks the connection to context
        _sync_block_phi_incomings(builder, blocks)
        # Phase HOT-02: clear+rebuild trivial alias map per function.
        _clear_phi_trivial_aliases(builder)
        trace({"phi": "setup", "produced_str_keys": list(produced_str.keys())})

        # Phase 132: Create all PHI placeholders FIRST, before any other operations
        # This ensures PHIs are at block heads when blocks are still empty
        debug_mode = is_phi_debug_enabled()

        if debug_mode:
            import sys
            print(f"[phi_wiring/setup] Processing {len(blocks)} blocks for PHI placeholders", file=sys.stderr)

        for block_data in blocks:
            bid0 = block_data.get("id", 0)
            bb0 = builder.bb_map.get(bid0)
            if bb0 is None:
                continue

            phi_insts = _phi_instructions(block_data)
            _debug_phi_block(debug_mode, bid0, len(phi_insts))
            for inst in phi_insts:
                _setup_phi_instruction(builder, bid0, bb0, inst, produced_str, debug_mode)
    except Exception as e:
        if is_phi_debug_enabled():
            import sys
            print(f"[phi_wiring/setup] FATAL ERROR: {e}", file=sys.stderr)
            import traceback
            traceback.print_exc()

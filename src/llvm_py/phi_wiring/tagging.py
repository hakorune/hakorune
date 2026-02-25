from __future__ import annotations
from typing import Dict, List, Any

from .common import trace
from .analysis import analyze_incomings, collect_produced_stringish
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
        analyzed = analyze_incomings(blocks)
        builder.block_phi_incomings.clear()
        builder.block_phi_incomings.update(analyzed)
        # Phase HOT-02: clear+rebuild trivial alias map per function.
        try:
            if hasattr(builder, "phi_trivial_aliases") and isinstance(builder.phi_trivial_aliases, dict):
                builder.phi_trivial_aliases.clear()
        except Exception:
            pass
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

            # Count PHIs in this block for debugging
            phi_count = 0
            for inst in block_data.get("instructions", []) or []:
                if inst.get("op") == "phi":
                    phi_count += 1

            if debug_mode and phi_count > 0:
                import sys
                print(f"[phi_wiring/setup] Block {bid0}: {phi_count} PHI instructions to create", file=sys.stderr)

            for inst in block_data.get("instructions", []) or []:
                if inst.get("op") != "phi":
                    continue
                try:
                    dst0 = int(inst.get("dst"))
                    incoming0 = inst.get("incoming", []) or []
                except Exception:
                    dst0 = None
                    incoming0 = []
                if dst0 is None:
                    continue

                alias_src = _trivial_phi_alias(dst0, incoming0)
                if alias_src is not None:
                    try:
                        if hasattr(builder, "phi_trivial_aliases") and isinstance(builder.phi_trivial_aliases, dict):
                            builder.phi_trivial_aliases[(int(bid0), int(dst0))] = int(alias_src)
                        if hasattr(builder, "resolver") and hasattr(builder.resolver, "phi_trivial_aliases"):
                            if isinstance(builder.resolver.phi_trivial_aliases, dict):
                                builder.resolver.phi_trivial_aliases[(int(bid0), int(dst0))] = int(alias_src)
                    except Exception:
                        pass
                    trace({
                        "phi": "setup_trivial_alias",
                        "block": int(bid0),
                        "dst": int(dst0),
                        "src": int(alias_src),
                    })
                    # Definition hint for resolver: dst is considered defined in this block.
                    try:
                        builder.def_blocks.setdefault(int(dst0), set()).add(int(bid0))
                    except Exception:
                        pass
                    _propagate_string_tag(
                        builder,
                        int(dst0),
                        inst.get("dst_type"),
                        incoming0,
                        produced_str,
                    )
                    # Continue: no placeholder PHI needed for trivial alias.
                    continue

                # Phase 132: Verify block is still empty (no terminator)
                has_terminator = False
                try:
                    if bb0.terminator is not None:
                        has_terminator = True
                except Exception:
                    pass

                if has_terminator and debug_mode:
                    import sys
                    print(f"[phi_wiring/setup] WARNING: Block {bid0} already has terminator "
                          f"when creating PHI for v{dst0}!", file=sys.stderr)

                # Predeclare a placeholder PHI at the block head so that
                # mid-block users (e.g., compare/branch) dominate correctly
                # and refer to the same SSA node that finalize_phis() will wire.
                try:
                    # Phase 275 P0: Get dst_type from instruction JSON (SSOT)
                    from .type_helper import get_phi_dst_type
                    dst_type = get_phi_dst_type(builder, dst0, inst=inst)
                    ph = ensure_phi(builder, bid0, dst0, bb0, dst_type=dst_type)
                    # Keep a strong reference as a predeclared placeholder so
                    # later ensure_phi calls during finalize re-use the same SSA node.
                    # Phase 132-Post: Register PHI with PhiManager Box
                    try:
                        if not hasattr(builder, 'predeclared_ret_phis') or builder.predeclared_ret_phis is None:
                            builder.predeclared_ret_phis = {}
                    except Exception:
                        builder.predeclared_ret_phis = {}
                    try:
                        builder.predeclared_ret_phis[(int(bid0), int(dst0))] = ph
                        # Phase 132-Post: Box-First - register with PhiManager
                        if hasattr(builder, 'phi_manager'):
                            builder.phi_manager.register_phi(int(bid0), int(dst0), ph)
                        if debug_mode:
                            import sys
                            print(f"[phi_wiring/setup] Created PHI placeholder for v{dst0} in bb{bid0}", file=sys.stderr)
                    except Exception:
                        pass
                except Exception as e:
                    if debug_mode:
                        import sys
                        print(f"[phi_wiring/setup] ERROR creating PHI for v{dst0} in bb{bid0}: {e}", file=sys.stderr)

                # Tag propagation
                _propagate_string_tag(
                    builder,
                    int(dst0),
                    inst.get("dst_type"),
                    incoming0,
                    produced_str,
                )
                # Definition hint: PHI defines dst in this block
                try:
                    builder.def_blocks.setdefault(int(dst0), set()).add(int(bid0))
                    if is_phi_debug_enabled():
                        trace({"phi_def_blocks": "registered", "dst": int(dst0), "block": int(bid0)})
                except Exception:
                    pass
        try:
            builder.resolver.block_phi_incomings = builder.block_phi_incomings
        except Exception:
            pass
    except Exception as e:
        if is_phi_debug_enabled():
            import sys
            print(f"[phi_wiring/setup] FATAL ERROR: {e}", file=sys.stderr)
            import traceback
            traceback.print_exc()

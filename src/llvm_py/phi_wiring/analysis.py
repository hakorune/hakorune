from __future__ import annotations
from typing import Dict, List, Any, Tuple

from .common import trace


def collect_produced_stringish(blocks: List[Dict[str, Any]]) -> Dict[int, bool]:
    """Collect value-ids that are known to be string handles (best-effort).

    This is used for early tagging (PHI placeholder setup) before instructions
    are lowered/executed. Keep it monotonic and conservative.

    Phase 102 root-cause:
    - A string accumulator often goes through `copy` then `phi` before it's used
      in `binop '+'`. If we don't propagate stringish across copy/phi here, the
      PHI dst won't be tagged early, and the concat lowerer may incorrectly box
      an i64-handle as an IntegerBox (breaking runtime string length/parity).
    """
    produced_str: Dict[int, bool] = {}

    # Seed: explicit producers with reliable type signals.
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
                elif opx == "newbox":
                    t = inst.get("type") or inst.get("box_type")
                    if t == "StringBox":
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

    # Propagate: copy/phi/binop('+') can carry/produce stringish values even when
    # dst_type metadata is missing. Use a small fixpoint iteration to cover chains.
    changed = True
    while changed:
        changed = False
        for block_data in blocks:
            for inst in block_data.get("instructions", []) or []:
                try:
                    opx = inst.get("op")
                    dstx = inst.get("dst")
                    if dstx is None:
                        continue
                    dst_i = int(dstx)
                    if produced_str.get(dst_i):
                        continue

                    if opx == "copy":
                        src = inst.get("src")
                        if src is not None and produced_str.get(int(src)):
                            produced_str[dst_i] = True
                            changed = True
                            continue

                    if opx == "phi":
                        incoming0 = inst.get("incoming", []) or []
                        # JSON v0 incoming pairs are (value_id, block_id)
                        for (v_src, _b) in incoming0:
                            try:
                                if produced_str.get(int(v_src)):
                                    produced_str[dst_i] = True
                                    changed = True
                                    break
                            except Exception:
                                continue
                        continue

                    if opx == "binop" and inst.get("operation") == "+":
                        lhs = inst.get("lhs")
                        rhs = inst.get("rhs")
                        if lhs is not None and produced_str.get(int(lhs)):
                            produced_str[dst_i] = True
                            changed = True
                            continue
                        if rhs is not None and produced_str.get(int(rhs)):
                            produced_str[dst_i] = True
                            changed = True
                            continue
                except Exception:
                    continue

    return produced_str


def analyze_incomings(blocks: List[Dict[str, Any]]) -> Dict[int, Dict[int, List[Tuple[int, int]]]]:
    """Return block_phi_incomings: block_id -> { dst_vid -> [(decl_b, v_src), ...] }"""
    result: Dict[int, Dict[int, List[Tuple[int, int]]]] = {}
    for block_data in blocks:
        bid0 = block_data.get("id", 0)
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
                try:
                    pairs = [(int(b), int(v)) for (v, b) in incoming0]
                    result.setdefault(int(bid0), {})[dst0] = pairs
                    trace({
                        "phi": "analyze",
                        "block": int(bid0),
                        "dst": dst0,
                        "incoming": pairs,
                    })
                except Exception:
                    pass
    return result

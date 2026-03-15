from __future__ import annotations
from typing import Dict, List, Any, Tuple

from .common import trace


def _string_handle_type(value_type: Any) -> bool:
    if value_type == "string":
        return True
    if not isinstance(value_type, dict):
        return False
    return (
        value_type.get("kind") in ("handle", "ptr")
        and value_type.get("box_type") == "StringBox"
    )


def _seed_stringish_dst(inst: Dict[str, Any]) -> int | None:
    try:
        opx = inst.get("op")
        dstx = inst.get("dst")
        if dstx is None:
            return None

        if opx == "const":
            value = inst.get("value", {}) or {}
            if _string_handle_type(value.get("type")):
                return int(dstx)
            return None

        if opx == "newbox":
            box_type = inst.get("type") or inst.get("box_type")
            if box_type == "StringBox":
                return int(dstx)
            return None

        if opx in ("binop", "boxcall", "externcall"):
            if _string_handle_type(inst.get("dst_type")):
                return int(dstx)
    except Exception:
        return None

    return None


def _seed_produced_stringish(blocks: List[Dict[str, Any]]) -> Dict[int, bool]:
    produced_str: Dict[int, bool] = {}
    for block_data in blocks:
        for inst in block_data.get("instructions", []) or []:
            dst_i = _seed_stringish_dst(inst)
            if dst_i is not None:
                produced_str[dst_i] = True
    return produced_str


def _propagate_stringish_from_inst(
    produced_str: Dict[int, bool],
    inst: Dict[str, Any],
) -> bool:
    try:
        dstx = inst.get("dst")
        if dstx is None:
            return False
        dst_i = int(dstx)
        if produced_str.get(dst_i):
            return False

        opx = inst.get("op")
        if opx == "copy":
            src = inst.get("src")
            if src is not None and produced_str.get(int(src)):
                produced_str[dst_i] = True
                return True
            return False

        if opx == "phi":
            for pair in inst.get("incoming", []) or []:
                try:
                    v_src, _b = pair
                except Exception:
                    continue
                if produced_str.get(int(v_src)):
                    produced_str[dst_i] = True
                    return True
            return False

        if opx == "binop" and inst.get("operation") == "+":
            for side in ("lhs", "rhs"):
                value_id = inst.get(side)
                if value_id is not None and produced_str.get(int(value_id)):
                    produced_str[dst_i] = True
                    return True
    except Exception:
        return False

    return False


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
    produced_str = _seed_produced_stringish(blocks)

    # Propagate: copy/phi/binop('+') can carry/produce stringish values even when
    # dst_type metadata is missing. Use a small fixpoint iteration to cover chains.
    changed = True
    while changed:
        changed = False
        for block_data in blocks:
            for inst in block_data.get("instructions", []) or []:
                if _propagate_stringish_from_inst(produced_str, inst):
                    changed = True

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

"""
CFG utilities
Build predecessor/successor maps and dominance helpers.
"""

from typing import Dict, List, Any, Tuple, Set

def build_preds_succs(block_by_id: Dict[int, Dict[str, Any]]) -> Tuple[Dict[int, List[int]], Dict[int, List[int]]]:
    """Construct predecessor and successor maps from MIR(JSON) blocks."""
    succs: Dict[int, List[int]] = {}
    preds: Dict[int, List[int]] = {}
    for b in block_by_id.values():
        bid = int(b.get('id', 0))
        preds.setdefault(bid, [])
    for b in block_by_id.values():
        src = int(b.get('id', 0))
        for inst in b.get('instructions', []) or []:
            op = inst.get('op')
            if op == 'jump':
                t = inst.get('target')
                if t is not None:
                    t = int(t)
                    succs.setdefault(src, []).append(t)
                    preds.setdefault(t, []).append(src)
            elif op == 'branch':
                th = inst.get('then'); el = inst.get('else')
                if th is not None:
                    th = int(th)
                    succs.setdefault(src, []).append(th)
                    preds.setdefault(th, []).append(src)
                if el is not None:
                    el = int(el)
                    succs.setdefault(src, []).append(el)
                    preds.setdefault(el, []).append(src)
    return preds, succs


def compute_reachable(entry_bid: int, succs: Dict[int, List[int]]) -> Set[int]:
    """Return blocks reachable from entry (including entry)."""
    reachable: Set[int] = set()
    stack: List[int] = [int(entry_bid)]
    while stack:
        bid = stack.pop()
        if bid in reachable:
            continue
        reachable.add(bid)
        for succ in succs.get(bid, []) or []:
            sb = int(succ)
            if sb not in reachable:
                stack.append(sb)
    return reachable


def compute_dominators(
    entry_bid: int,
    preds: Dict[int, List[int]],
    succs: Dict[int, List[int]],
) -> Dict[int, Set[int]]:
    """Compute classical dominator sets over reachable blocks."""
    entry = int(entry_bid)
    reachable = compute_reachable(entry, succs)
    if not reachable:
        return {}

    all_reachable = set(reachable)
    dom: Dict[int, Set[int]] = {}
    for b in reachable:
        dom[b] = {entry} if b == entry else set(all_reachable)

    changed = True
    while changed:
        changed = False
        for b in sorted(reachable):
            if b == entry:
                continue
            pred_list = [int(p) for p in (preds.get(b, []) or []) if int(p) in reachable]
            if not pred_list:
                new_dom = {b}
            else:
                intersect = set(dom[pred_list[0]])
                for p in pred_list[1:]:
                    intersect &= dom[p]
                new_dom = intersect | {b}
            if new_dom != dom[b]:
                dom[b] = new_dom
                changed = True

    return dom


def _read_const_i64(inst: Dict[str, Any]):
    """Read integer constant payload from MIR const instruction."""
    value = inst.get("value")
    if isinstance(value, bool):
        return int(value)
    if isinstance(value, int):
        return int(value)
    if not isinstance(value, dict):
        return None
    raw = value.get("value")
    if isinstance(raw, bool):
        raw = int(raw)
    if not isinstance(raw, int):
        return None
    ty = value.get("type") or value.get("ty")
    if ty is None:
        return int(raw)
    ty_s = str(ty).lower()
    if ty_s in ("i64", "int", "integer", "i1", "bool", "boolean"):
        return int(raw)
    return None


def _incoming_value_ids(incoming_raw: Any) -> List[int]:
    vals: List[int] = []
    for inc in (incoming_raw or []):
        if isinstance(inc, (list, tuple)) and len(inc) >= 1 and isinstance(inc[0], int):
            vals.append(int(inc[0]))
            continue
        if isinstance(inc, dict):
            candidate = inc.get("value")
            if not isinstance(candidate, int):
                candidate = inc.get("val")
            if isinstance(candidate, int):
                vals.append(int(candidate))
    return vals

from .value_analysis import (
    collect_arrayish_value_ids,
    collect_integerish_value_ids,
    collect_non_negative_value_ids,
    collect_stringish_value_ids,
    propagate_arrayish_value_ids,
)

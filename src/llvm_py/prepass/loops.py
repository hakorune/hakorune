"""
Loop prepass utilities
Detect simple while-shaped loops in MIR(JSON) and return a lowering plan.
"""

from typing import Dict, List, Any, Optional, Set
from cfg.utils import build_preds_succs

_NUMERIC_LOOP_ALLOWED_OPS = {
    "const",
    "copy",
    "phi",
    "binop",
    "compare",
    "select",
}


def annotate_numeric_loop_plan(
    block_by_id: Dict[int, Dict[str, Any]],
    loop_plan: Optional[Dict[str, Any]],
    *,
    integerish_ids: Optional[Set[int]] = None,
    non_negative_ids: Optional[Set[int]] = None,
) -> Optional[Dict[str, Any]]:
    """Attach a conservative numeric-loop proof to a simple while plan.

    The current cut only recognizes arithmetic-only bodies with an integerish
    loop condition and at least one integerish arithmetic carrier in the body.
    This is intentionally narrow: it is a proof seam for later induction
    normalization, not a general loop classifier.
    """

    if not isinstance(loop_plan, dict):
        return None

    body_insts = loop_plan.get("body_insts") or []
    if not isinstance(body_insts, list) or not body_insts:
        return None

    integerish = {int(v) for v in (integerish_ids or set()) if isinstance(v, int)}
    if not integerish:
        return None

    cond_vid = loop_plan.get("cond")
    if isinstance(cond_vid, int) and int(cond_vid) not in integerish:
        return None

    induction_value_ids: Set[int] = set()
    saw_binop = False
    saw_numeric_body = False

    for inst in body_insts:
        if not isinstance(inst, dict):
            return None

        op = str(inst.get("op") or "")
        if op not in _NUMERIC_LOOP_ALLOWED_OPS:
            return None
        if op == "binop":
            saw_binop = True

        dst = inst.get("dst")
        if isinstance(dst, int) and int(dst) in integerish:
            saw_numeric_body = True
            if op in ("copy", "phi", "binop", "compare", "select"):
                induction_value_ids.add(int(dst))

    if not saw_binop or not saw_numeric_body or not induction_value_ids:
        return None

    # Optional non-negative hint: if the loop already carries a non-negative
    # arithmetic value, keep it as an additional proof breadcrumb.
    non_negative = {int(v) for v in (non_negative_ids or set()) if isinstance(v, int)}

    annotated = dict(loop_plan)
    annotated["numeric_kind"] = "induction"
    annotated["numeric_induction_value_ids"] = sorted(induction_value_ids)
    annotated["numeric_non_negative_value_ids"] = sorted(induction_value_ids & non_negative) if non_negative else []
    annotated["numeric_proof_source"] = "simple_while_arithmetic_only"
    return annotated

def detect_simple_while(block_by_id: Dict[int, Dict[str, Any]]) -> Optional[Dict[str, Any]]:
    """Detect a simple loop pattern: header(branch cond → then/else),
    a latch that jumps back to header reachable from then, and exit on else.
    Returns a plan dict or None.
    """
    # Build succ and pred maps from JSON quickly
    preds, succs = build_preds_succs(block_by_id)
    # Find a header with a branch terminator and else leading to a ret (direct)
    for b in block_by_id.values():
        bid = int(b.get('id', 0))
        term = None
        if b.get('instructions'):
            last = b.get('instructions')[-1]
            if last.get('op') in ('jump','branch','ret'):
                term = last
        if term is None and 'terminator' in b:
            t = b['terminator']
            if t and t.get('op') in ('jump','branch','ret'):
                term = t
        if not term or term.get('op') != 'branch':
            continue
        then_bid = int(term.get('then'))
        else_bid = int(term.get('else'))
        cond_vid = int(term.get('cond')) if term.get('cond') is not None else None
        if cond_vid is None:
            continue
        # Quick check: else block ends with ret
        else_blk = block_by_id.get(else_bid)
        has_ret = False
        if else_blk is not None:
            insts = else_blk.get('instructions', [])
            if insts and insts[-1].get('op') == 'ret':
                has_ret = True
            elif else_blk.get('terminator', {}).get('op') == 'ret':
                has_ret = True
        if not has_ret:
            continue
        # Find a latch that jumps back to header reachable from then
        latch = None
        visited = set()
        stack = [then_bid]
        while stack:
            cur = stack.pop()
            if cur in visited:
                continue
            visited.add(cur)
            cur_blk = block_by_id.get(cur)
            if cur_blk is None:
                continue
            for inst in cur_blk.get('instructions', []) or []:
                if inst.get('op') == 'jump' and int(inst.get('target')) == bid:
                    latch = cur
                    break
            if latch is not None:
                break
            for nx in succs.get(cur, []) or []:
                if nx not in visited and nx != else_bid:
                    stack.append(nx)
        if latch is None:
            continue
        # Compose body_insts: collect insts along then-branch region up to latch (inclusive),
        # excluding any final jump back to header to prevent double edges.
        collect_order: List[int] = []
        visited2 = set()
        stack2 = [then_bid]
        while stack2:
            cur = stack2.pop()
            if cur in visited2 or cur == bid or cur == else_bid:
                continue
            visited2.add(cur)
            collect_order.append(cur)
            if cur == latch:
                continue
            for nx in succs.get(cur, []) or []:
                if nx not in visited2 and nx != else_bid:
                    stack2.append(nx)
        body_insts: List[Dict[str, Any]] = []
        for bbid in collect_order:
            blk = block_by_id.get(bbid)
            if blk is None:
                continue
            for inst in blk.get('instructions', []) or []:
                if inst.get('op') == 'jump' and int(inst.get('target', -1)) == bid:
                    continue
                body_insts.append(inst)
        skip_blocks = set(collect_order)
        skip_blocks.add(bid)
        return {
            'header': bid,
            'then': then_bid,
            'else': else_bid,
            'latch': latch,
            'exit': else_bid,
            'cond': cond_vid,
            'body_insts': body_insts,
            'skip_blocks': list(skip_blocks),
        }
    return None

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


def collect_non_negative_value_ids(blocks: List[Dict[str, Any]]) -> Set[int]:
    """Conservative non-negative VID analysis for power-of-two modulo lowering."""
    nonneg: Set[int] = set()
    const_i64: Dict[int, int] = {}
    mask_limit = (1 << 63)

    # Small fixed-point loop to propagate through copy/phi/loop recurrences.
    for _ in range(12):
        changed = False
        for blk in (blocks or []):
            for ins in (blk.get("instructions") or []):
                op = ins.get("op")
                dst = ins.get("dst")
                if not isinstance(dst, int):
                    continue
                dst = int(dst)

                if op == "const":
                    cval = _read_const_i64(ins)
                    if cval is not None:
                        const_i64[dst] = cval
                        if cval >= 0 and dst not in nonneg:
                            nonneg.add(dst)
                            changed = True
                    continue

                if op == "copy":
                    src = ins.get("src")
                    if isinstance(src, int):
                        src = int(src)
                        if src in const_i64 and dst not in const_i64:
                            const_i64[dst] = const_i64[src]
                        if src in nonneg and dst not in nonneg:
                            nonneg.add(dst)
                            changed = True
                    continue

                if op == "compare":
                    # compare result is boolean-ish (0/1 path)
                    if dst not in nonneg:
                        nonneg.add(dst)
                        changed = True
                    continue

                if op == "select":
                    tv = ins.get("then_val")
                    ev = ins.get("else_val")
                    if isinstance(tv, int) and isinstance(ev, int):
                        if int(tv) in nonneg and int(ev) in nonneg and dst not in nonneg:
                            nonneg.add(dst)
                            changed = True
                    continue

                if op == "phi":
                    vals = _incoming_value_ids(ins.get("incoming"))
                    if vals and all(v in nonneg for v in vals) and dst not in nonneg:
                        nonneg.add(dst)
                        changed = True
                    continue

                if op == "binop":
                    bop = str(ins.get("operation") or "").lower()
                    lhs = ins.get("lhs")
                    rhs = ins.get("rhs")
                    lhs_id = int(lhs) if isinstance(lhs, int) else None
                    rhs_id = int(rhs) if isinstance(rhs, int) else None
                    lhs_c = const_i64.get(lhs_id) if lhs_id is not None else None
                    rhs_c = const_i64.get(rhs_id) if rhs_id is not None else None

                    if bop in ("+", "add", "plus"):
                        if lhs_id in nonneg and rhs_id in nonneg and dst not in nonneg:
                            nonneg.add(dst)
                            changed = True
                    elif bop in ("*", "mul", "times"):
                        if lhs_id in nonneg and rhs_id in nonneg and dst not in nonneg:
                            nonneg.add(dst)
                            changed = True
                    elif bop in ("%", "mod", "rem"):
                        if lhs_id in nonneg and isinstance(rhs_c, int) and rhs_c > 0 and dst not in nonneg:
                            nonneg.add(dst)
                            changed = True
                    elif bop in ("&", "band", "bitand"):
                        # If one side is a non-negative mask constant (<2^63), result is non-negative.
                        mask_ok = (
                            isinstance(lhs_c, int) and 0 <= lhs_c < mask_limit
                        ) or (
                            isinstance(rhs_c, int) and 0 <= rhs_c < mask_limit
                        )
                        if mask_ok and dst not in nonneg:
                            nonneg.add(dst)
                            changed = True
                    continue

        if not changed:
            break

    return nonneg


def collect_integerish_value_ids(blocks: List[Dict[str, Any]]) -> Set[int]:
    """Conservative integer-like VID analysis for RuntimeData integer-key routes."""
    integerish: Set[int] = set()

    # Carrier graph (copy/phi/binop/select) for SCC-aware loop induction closure.
    carrier_edges: Dict[int, List[int]] = {}
    for blk in (blocks or []):
        for ins in (blk.get("instructions") or []):
            dst = ins.get("dst")
            if not isinstance(dst, int):
                continue
            dst = int(dst)
            op = ins.get("op")
            if op == "copy":
                src = ins.get("src")
                if isinstance(src, int):
                    carrier_edges.setdefault(dst, []).append(int(src))
            elif op == "phi":
                vals = _incoming_value_ids(ins.get("incoming"))
                if vals:
                    carrier_edges.setdefault(dst, []).extend(int(v) for v in vals)
            elif op == "binop":
                lhs = ins.get("lhs")
                rhs = ins.get("rhs")
                if isinstance(lhs, int):
                    carrier_edges.setdefault(dst, []).append(int(lhs))
                if isinstance(rhs, int):
                    carrier_edges.setdefault(dst, []).append(int(rhs))
            elif op == "select":
                tv = ins.get("then_val")
                ev = ins.get("else_val")
                if isinstance(tv, int):
                    carrier_edges.setdefault(dst, []).append(int(tv))
                if isinstance(ev, int):
                    carrier_edges.setdefault(dst, []).append(int(ev))

    # Kosaraju SCC
    visited: Set[int] = set()
    order: List[int] = []

    def dfs1(node: int) -> None:
        if node in visited:
            return
        visited.add(node)
        for nxt in carrier_edges.get(node, []):
            if nxt in carrier_edges:
                dfs1(nxt)
        order.append(node)

    for n in list(carrier_edges.keys()):
        dfs1(n)

    rev: Dict[int, List[int]] = {}
    for src, dsts in carrier_edges.items():
        for dst in dsts:
            if dst in carrier_edges:
                rev.setdefault(dst, []).append(src)

    comp_id: Dict[int, int] = {}
    comp_seq = 0

    def dfs2(node: int, cid: int) -> None:
        if node in comp_id:
            return
        comp_id[node] = cid
        for nxt in rev.get(node, []):
            dfs2(nxt, cid)

    for n in reversed(order):
        if n not in comp_id:
            comp_seq += 1
            dfs2(n, comp_seq)

    comp_nodes: Dict[int, Set[int]] = {}
    for node, cid in comp_id.items():
        comp_nodes.setdefault(cid, set()).add(node)
    comp_external: Dict[int, Set[int]] = {}
    for dst, srcs in carrier_edges.items():
        dst_c = comp_id.get(dst, -1)
        for src in srcs:
            src_c = comp_id.get(src, -2)
            if src_c != dst_c:
                comp_external.setdefault(dst_c, set()).add(int(src))

    for _ in range(20):
        changed = False
        for blk in (blocks or []):
            for ins in (blk.get("instructions") or []):
                dst = ins.get("dst")
                if not isinstance(dst, int):
                    continue
                dst = int(dst)
                op = ins.get("op")

                if op == "const":
                    cval = _read_const_i64(ins)
                    if cval is not None and dst not in integerish:
                        integerish.add(dst)
                        changed = True
                    continue

                if op == "compare":
                    if dst not in integerish:
                        integerish.add(dst)
                        changed = True
                    continue

                if op == "copy":
                    src = ins.get("src")
                    if isinstance(src, int) and int(src) in integerish and dst not in integerish:
                        integerish.add(dst)
                        changed = True
                    continue

                if op == "binop":
                    lhs = ins.get("lhs")
                    rhs = ins.get("rhs")
                    if isinstance(lhs, int) and isinstance(rhs, int):
                        if int(lhs) in integerish and int(rhs) in integerish and dst not in integerish:
                            integerish.add(dst)
                            changed = True
                    continue

                if op == "select":
                    tv = ins.get("then_val")
                    ev = ins.get("else_val")
                    if isinstance(tv, int) and isinstance(ev, int):
                        if int(tv) in integerish and int(ev) in integerish and dst not in integerish:
                            integerish.add(dst)
                            changed = True
                    continue

                if op == "mir_call":
                    mc = ins.get("mir_call")
                    if isinstance(mc, dict):
                        callee = mc.get("callee")
                        if isinstance(callee, dict):
                            ctype = str(callee.get("type") or "")
                            name = str(callee.get("name") or "")
                            # length-like methods always produce integer-like values.
                            if ctype == "Method" and name in ("length", "len", "size"):
                                if dst not in integerish:
                                    integerish.add(dst)
                                    changed = True
                    continue

                if op == "phi":
                    vals = _incoming_value_ids(ins.get("incoming"))
                    if not vals:
                        continue
                    dst_comp = comp_id.get(dst, -1)
                    ext_vals = []
                    for v in vals:
                        vv = int(v)
                        if vv == dst:
                            continue
                        if comp_id.get(vv, -2) == dst_comp:
                            continue
                        ext_vals.append(vv)
                    if ext_vals and all(v in integerish for v in ext_vals):
                        if dst not in integerish:
                            integerish.add(dst)
                            changed = True
                    continue

        # Component closure for loop-carried arithmetic SCCs.
        for cid, nodes in comp_nodes.items():
            if not nodes:
                continue
            if not any(n in integerish for n in nodes):
                continue
            ext_vals = comp_external.get(cid, set())
            if ext_vals and not all(v in integerish for v in ext_vals):
                continue
            for n in nodes:
                if n not in integerish:
                    integerish.add(n)
                    changed = True

        if not changed:
            break

    return integerish


def collect_arrayish_value_ids(blocks: List[Dict[str, Any]]) -> Set[int]:
    """Conservative ArrayBox-handle VID analysis from newbox/copy/phi chains."""
    arrayish: Set[int] = set()

    # Build carrier graph (copy/phi only) for SCC-aware self-carry filtering.
    carrier_edges: Dict[int, List[int]] = {}
    for blk in (blocks or []):
        for ins in (blk.get("instructions") or []):
            dst = ins.get("dst")
            if not isinstance(dst, int):
                continue
            dst = int(dst)
            op = ins.get("op")
            if op == "copy":
                src = ins.get("src")
                if isinstance(src, int):
                    carrier_edges.setdefault(dst, []).append(int(src))
            elif op == "phi":
                vals = _incoming_value_ids(ins.get("incoming"))
                if vals:
                    carrier_edges.setdefault(dst, []).extend(int(v) for v in vals)

    # Kosaraju SCC
    visited: Set[int] = set()
    order: List[int] = []

    def dfs1(node: int) -> None:
        if node in visited:
            return
        visited.add(node)
        for nxt in carrier_edges.get(node, []):
            if nxt in carrier_edges:
                dfs1(nxt)
        order.append(node)

    for n in list(carrier_edges.keys()):
        dfs1(n)

    rev: Dict[int, List[int]] = {}
    for src, dsts in carrier_edges.items():
        for dst in dsts:
            if dst in carrier_edges:
                rev.setdefault(dst, []).append(src)

    comp_id: Dict[int, int] = {}
    comp_seq = 0

    def dfs2(node: int, cid: int) -> None:
        if node in comp_id:
            return
        comp_id[node] = cid
        for nxt in rev.get(node, []):
            dfs2(nxt, cid)

    for n in reversed(order):
        if n not in comp_id:
            comp_seq += 1
            dfs2(n, comp_seq)

    comp_nodes: Dict[int, Set[int]] = {}
    for node, cid in comp_id.items():
        comp_nodes.setdefault(cid, set()).add(node)
    comp_external: Dict[int, Set[int]] = {}
    for dst, srcs in carrier_edges.items():
        dst_c = comp_id.get(dst, -1)
        for src in srcs:
            src_c = comp_id.get(src, -2)
            if src_c != dst_c:
                comp_external.setdefault(dst_c, set()).add(int(src))

    for _ in range(16):
        changed = False
        for blk in (blocks or []):
            for ins in (blk.get("instructions") or []):
                dst = ins.get("dst")
                if not isinstance(dst, int):
                    continue
                dst = int(dst)
                op = ins.get("op")

                if op == "newbox":
                    box_type = ins.get("type")
                    if box_type is None:
                        box_type = ins.get("box_type")
                    if str(box_type or "") == "ArrayBox" and dst not in arrayish:
                        arrayish.add(dst)
                        changed = True
                    continue

                if op == "copy":
                    src = ins.get("src")
                    if isinstance(src, int) and int(src) in arrayish and dst not in arrayish:
                        arrayish.add(dst)
                        changed = True
                    continue

                if op == "phi":
                    vals = _incoming_value_ids(ins.get("incoming"))
                    if not vals:
                        continue
                    dst_comp = comp_id.get(dst, -1)
                    ext_vals = []
                    for v in vals:
                        vv = int(v)
                        if vv == dst:
                            continue
                        if comp_id.get(vv, -2) == dst_comp:
                            continue
                        ext_vals.append(vv)
                    if ext_vals and all(v in arrayish for v in ext_vals):
                        if dst not in arrayish:
                            arrayish.add(dst)
                            changed = True
                    continue

        # Component closure: if one node in a carrier SCC is arrayish and all
        # external incoming values to that SCC are arrayish, mark whole SCC.
        for cid, nodes in comp_nodes.items():
            if not nodes:
                continue
            if not any(n in arrayish for n in nodes):
                continue
            ext_vals = comp_external.get(cid, set())
            if ext_vals and not all(v in arrayish for v in ext_vals):
                continue
            for n in nodes:
                if n not in arrayish:
                    arrayish.add(n)
                    changed = True

        if not changed:
            break

    return arrayish


def collect_stringish_value_ids(blocks: List[Dict[str, Any]]) -> Set[int]:
    """Conservative StringBox-handle VID analysis.

    Goal (cleanup-11):
    - Mark receivers that are *proven* string-like by string-only method usage
      (`substring/indexOf/lastIndexOf`) before lowering order reaches `length`.
    - Propagate string-ish element evidence from RuntimeData `set/push` to `get`
      results across copy/phi carrier chains.
    """
    stringish: Set[int] = set()
    receiver_string_elements: Set[int] = set()

    # Carrier graph (copy/phi) for SCC-aware self-carry filtering.
    carrier_edges: Dict[int, List[int]] = {}
    for blk in (blocks or []):
        for ins in (blk.get("instructions") or []):
            dst = ins.get("dst")
            if not isinstance(dst, int):
                continue
            dst = int(dst)
            op = ins.get("op")
            if op == "copy":
                src = ins.get("src")
                if isinstance(src, int):
                    carrier_edges.setdefault(dst, []).append(int(src))
            elif op == "phi":
                vals = _incoming_value_ids(ins.get("incoming"))
                if vals:
                    carrier_edges.setdefault(dst, []).extend(int(v) for v in vals)

    # Kosaraju SCC
    visited: Set[int] = set()
    order: List[int] = []

    def dfs1(node: int) -> None:
        if node in visited:
            return
        visited.add(node)
        for nxt in carrier_edges.get(node, []):
            if nxt in carrier_edges:
                dfs1(nxt)
        order.append(node)

    for n in list(carrier_edges.keys()):
        dfs1(n)

    rev: Dict[int, List[int]] = {}
    for src, dsts in carrier_edges.items():
        for dst in dsts:
            if dst in carrier_edges:
                rev.setdefault(dst, []).append(src)

    comp_id: Dict[int, int] = {}
    comp_seq = 0

    def dfs2(node: int, cid: int) -> None:
        if node in comp_id:
            return
        comp_id[node] = cid
        for nxt in rev.get(node, []):
            dfs2(nxt, cid)

    for n in reversed(order):
        if n not in comp_id:
            comp_seq += 1
            dfs2(n, comp_seq)

    comp_nodes: Dict[int, Set[int]] = {}
    for node, cid in comp_id.items():
        comp_nodes.setdefault(cid, set()).add(node)
    comp_external: Dict[int, Set[int]] = {}
    for dst, srcs in carrier_edges.items():
        dst_c = comp_id.get(dst, -1)
        for src in srcs:
            src_c = comp_id.get(src, -2)
            if src_c != dst_c:
                comp_external.setdefault(dst_c, set()).add(int(src))

    string_receiver_methods = {"substring", "indexOf", "lastIndexOf"}

    for _ in range(20):
        changed = False
        for blk in (blocks or []):
            for ins in (blk.get("instructions") or []):
                op = ins.get("op")
                dst = ins.get("dst")
                dst_i = int(dst) if isinstance(dst, int) else None

                # Seed: explicit string producers.
                if op == "const" and dst_i is not None:
                    value = ins.get("value")
                    if isinstance(value, dict):
                        ty = value.get("type")
                        if ty == "string":
                            if dst_i not in stringish:
                                stringish.add(dst_i)
                                changed = True
                        elif isinstance(ty, dict):
                            if (
                                ty.get("kind") in ("handle", "ptr")
                                and ty.get("box_type") == "StringBox"
                            ):
                                if dst_i not in stringish:
                                    stringish.add(dst_i)
                                    changed = True

                if op == "newbox" and dst_i is not None:
                    box_type = ins.get("type")
                    if box_type is None:
                        box_type = ins.get("box_type")
                    if str(box_type or "") == "StringBox" and dst_i not in stringish:
                        stringish.add(dst_i)
                        changed = True

                if op in ("binop", "boxcall", "externcall") and dst_i is not None:
                    dst_type = ins.get("dst_type")
                    if (
                        isinstance(dst_type, dict)
                        and dst_type.get("kind") == "handle"
                        and dst_type.get("box_type") == "StringBox"
                    ):
                        if dst_i not in stringish:
                            stringish.add(dst_i)
                            changed = True

                # Propagate through copy.
                if op == "copy" and dst_i is not None:
                    src = ins.get("src")
                    if isinstance(src, int):
                        src_i = int(src)
                        if src_i in stringish and dst_i not in stringish:
                            stringish.add(dst_i)
                            changed = True
                        if src_i in receiver_string_elements and dst_i not in receiver_string_elements:
                            receiver_string_elements.add(dst_i)
                            changed = True

                # `+` over string-ish operand yields string-ish handle.
                if op == "binop" and dst_i is not None and str(ins.get("operation") or "") == "+":
                    lhs = ins.get("lhs")
                    rhs = ins.get("rhs")
                    lhs_i = int(lhs) if isinstance(lhs, int) else None
                    rhs_i = int(rhs) if isinstance(rhs, int) else None
                    if (
                        (lhs_i is not None and lhs_i in stringish)
                        or (rhs_i is not None and rhs_i in stringish)
                    ):
                        if dst_i not in stringish:
                            stringish.add(dst_i)
                            changed = True

                # PHI propagation with SCC-aware external incoming filtering.
                if op == "phi" and dst_i is not None:
                    vals = _incoming_value_ids(ins.get("incoming"))
                    if vals:
                        dst_comp = comp_id.get(dst_i, -1)
                        ext_vals = []
                        for v in vals:
                            vv = int(v)
                            if vv == dst_i:
                                continue
                            if comp_id.get(vv, -2) == dst_comp:
                                continue
                            ext_vals.append(vv)
                        if ext_vals and all(v in stringish for v in ext_vals):
                            if dst_i not in stringish:
                                stringish.add(dst_i)
                                changed = True
                        if ext_vals and all(v in receiver_string_elements for v in ext_vals):
                            if dst_i not in receiver_string_elements:
                                receiver_string_elements.add(dst_i)
                                changed = True

                if op != "mir_call":
                    continue
                mc = ins.get("mir_call")
                if not isinstance(mc, dict):
                    continue
                callee = mc.get("callee")
                if not isinstance(callee, dict):
                    continue
                method = str(callee.get("name") or "")
                recv = callee.get("receiver")
                recv_i = int(recv) if isinstance(recv, int) else None
                args = mc.get("args") or []

                # Use-based inference: methods with string-only receiver contract.
                if method in string_receiver_methods and recv_i is not None:
                    if recv_i not in stringish:
                        stringish.add(recv_i)
                        changed = True

                # Container element type inference for RuntimeData routes.
                if str(callee.get("box_name") or "") == "RuntimeDataBox" and recv_i is not None:
                    if method == "push" and len(args) >= 1 and isinstance(args[0], int):
                        if int(args[0]) in stringish and recv_i not in receiver_string_elements:
                            receiver_string_elements.add(recv_i)
                            changed = True
                    elif method == "set" and len(args) >= 2 and isinstance(args[1], int):
                        if int(args[1]) in stringish and recv_i not in receiver_string_elements:
                            receiver_string_elements.add(recv_i)
                            changed = True
                    elif method == "get" and dst_i is not None:
                        if recv_i in receiver_string_elements and dst_i not in stringish:
                            stringish.add(dst_i)
                            changed = True

        # Component closure for both facts.
        for cid, nodes in comp_nodes.items():
            if not nodes:
                continue
            ext_vals = comp_external.get(cid, set())

            if any(n in stringish for n in nodes):
                if not ext_vals or all(v in stringish for v in ext_vals):
                    for n in nodes:
                        if n not in stringish:
                            stringish.add(n)
                            changed = True

            if any(n in receiver_string_elements for n in nodes):
                if not ext_vals or all(v in receiver_string_elements for v in ext_vals):
                    for n in nodes:
                        if n not in receiver_string_elements:
                            receiver_string_elements.add(n)
                            changed = True

        if not changed:
            break

    return stringish

"""Copy-origin analysis helpers for expression-materialization probes."""

from __future__ import annotations

from collections import Counter, defaultdict
from typing import Any


def value_uses(inst: dict[str, Any]) -> list[Any]:
    op = inst.get("op")
    values: list[Any] = []
    if op == "copy":
        values.append(inst.get("src"))
    elif op == "field_get":
        values.append(inst.get("box"))
    elif op == "field_set":
        values.extend([inst.get("box"), inst.get("value")])
    elif op in {"binop", "compare"}:
        values.extend([inst.get("lhs"), inst.get("rhs")])
    elif op == "branch":
        values.append(inst.get("cond"))
    elif op == "ret":
        values.append(inst.get("value"))
    elif op == "mir_call":
        mir_call = inst.get("mir_call")
        if isinstance(mir_call, dict):
            callee = mir_call.get("callee")
            if isinstance(callee, dict):
                values.append(callee.get("receiver"))
            args = mir_call.get("args", [])
            if isinstance(args, list):
                values.extend(args)
    return [value for value in values if value is not None]


def copy_descendants(seed: Any, src_to_dsts: dict[Any, set[Any]]) -> set[Any]:
    seen: set[Any] = set()
    stack = list(src_to_dsts.get(seed, ()))
    while stack:
        current = stack.pop()
        if current in seen:
            continue
        seen.add(current)
        stack.extend(src_to_dsts.get(current, ()))
    return seen


def copy_ancestors(seed: Any, dst_to_src: dict[Any, Any]) -> set[Any]:
    seen: set[Any] = set()
    current = seed
    while current in dst_to_src and current not in seen:
        seen.add(current)
        current = dst_to_src[current]
    return seen


def collect_call_attributed_copy_dsts(insts: list[dict[str, Any]]) -> set[Any]:
    copies = [inst for inst in insts if inst.get("op") == "copy"]
    dst_to_src = {inst.get("dst"): inst.get("src") for inst in copies}
    src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
    for inst in copies:
        src_to_dsts[inst.get("src")].add(inst.get("dst"))

    attributed: set[Any] = set()
    for inst in insts:
        if inst.get("op") != "mir_call":
            continue
        mir_call = inst.get("mir_call")
        if not isinstance(mir_call, dict):
            continue
        callee = mir_call.get("callee")
        if isinstance(callee, dict):
            attributed.update(copy_ancestors(callee.get("receiver"), dst_to_src))
        args = mir_call.get("args", [])
        if isinstance(args, list):
            for arg in args:
                attributed.update(copy_ancestors(arg, dst_to_src))
        if inst.get("dst") is not None:
            attributed.update(copy_descendants(inst.get("dst"), src_to_dsts))
    return attributed


def classify_copy(
    dst: Any,
    src: Any,
    inst_index: int,
    insts: list[dict[str, Any]],
    phi_dsts: set[Any],
    call_attributed: set[Any],
) -> str:
    if dst in call_attributed:
        return "call_adjacent"
    if src in phi_dsts:
        return "phi_edge"

    next_ops = insts[inst_index + 1 : inst_index + 4]
    prev_ops = insts[max(0, inst_index - 3) : inst_index]
    if any(inst.get("op") == "ret" for inst in next_ops):
        return "return_block"
    if any(inst.get("op") == "branch" for inst in next_ops):
        return "branch_condition"
    if any(inst.get("op") == "field_set" and inst.get("value") == dst for inst in next_ops):
        return "field_set_value"
    if inst_index <= 2:
        return "block_entry"
    if any(inst.get("op") == "jump" for inst in next_ops):
        return "block_exit"
    if any(inst.get("op") in {"field_get", "binop", "compare"} for inst in prev_ops + next_ops):
        return "expression_materialization"
    return "local_ssa"


def origin_label(seed: Any, producers: dict[Any, dict[str, Any]]) -> tuple[str, str, int]:
    current = seed
    seen: set[Any] = set()
    chain_len = 0
    while current in producers and current not in seen:
        seen.add(current)
        inst = producers[current]
        op = str(inst.get("op", "unknown"))
        if op == "copy":
            current = inst.get("src")
            chain_len += 1
            continue
        if op == "field_get":
            return "field_get", str(inst.get("field", "unknown")), chain_len
        if op == "phi":
            return "phi", "phi", chain_len
        if op == "mir_call":
            mir_call = inst.get("mir_call")
            name = "unknown"
            if isinstance(mir_call, dict):
                callee = mir_call.get("callee")
                if isinstance(callee, dict):
                    name = str(callee.get("name", "unknown"))
            return "mir_call", name, chain_len
        return op, op, chain_len
    return "param", "param", chain_len


def sink_label(inst: dict[str, Any]) -> str:
    op = str(inst.get("op", "unknown"))
    if op in {"binop", "compare"}:
        operation = {
            "==": "eq",
            "!=": "ne",
            "<": "lt",
            ">": "gt",
            "<=": "le",
            ">=": "ge",
            "+": "add",
            "-": "sub",
            "*": "mul",
            "/": "div",
        }.get(str(inst.get("operation", "unknown")), str(inst.get("operation", "unknown")))
        return f"{op}_{operation}"
    if op == "field_set":
        return f"field_set:{inst.get('field', 'unknown')}"
    return op


def sink_labels(seed: Any, consumers: dict[Any, list[dict[str, Any]]]) -> list[str]:
    labels: list[str] = []
    queue = [seed]
    seen: set[Any] = set()
    while queue:
        current = queue.pop(0)
        if current in seen:
            continue
        seen.add(current)
        for inst in consumers.get(current, []):
            if inst.get("op") == "copy":
                queue.append(inst.get("dst"))
            else:
                labels.append(sink_label(inst))
    return labels or ["unused_or_phi_only"]


def dominant(counts: Counter[str]) -> str:
    if not counts:
        return "none"
    return max(sorted(counts), key=lambda key: counts[key])


def safe_key(text: str) -> str:
    out: list[str] = []
    for ch in text:
        if ch.isalnum() or ch == "_":
            out.append(ch)
        else:
            out.append("_")
    return "".join(out).strip("_") or "unknown"

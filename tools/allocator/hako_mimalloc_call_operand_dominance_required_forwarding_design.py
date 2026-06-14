#!/usr/bin/env python3
"""Design dominance-required call-operand forwarding for one MIR method."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def find_function(data: dict[str, Any], selected_method: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    for function in functions:
        if isinstance(function, dict) and function.get("name") == selected_method:
            return function
    raise SystemExit(f"selected method not found: {selected_method}")


def block_instructions(function: dict[str, Any]) -> dict[Any, list[dict[str, Any]]]:
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    out: dict[Any, list[dict[str, Any]]] = {}
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if not isinstance(insts, list):
            continue
        out[block.get("id")] = [inst for inst in insts if isinstance(inst, dict)]
    return out


def successors(blocks: dict[Any, list[dict[str, Any]]]) -> dict[Any, list[Any]]:
    out: dict[Any, list[Any]] = {block_id: [] for block_id in blocks}
    for block_id, insts in blocks.items():
        term = insts[-1] if insts else {}
        if not isinstance(term, dict):
            continue
        if term.get("op") == "branch":
            out[block_id].extend([term.get("then"), term.get("else")])
        elif term.get("op") == "jump":
            out[block_id].append(term.get("target"))
    return {block_id: [succ for succ in succs if succ in blocks] for block_id, succs in out.items()}


def dominators(function: dict[str, Any], blocks: dict[Any, list[dict[str, Any]]]) -> dict[Any, set[Any]]:
    entry = function.get("entry_block")
    succ = successors(blocks)
    pred: dict[Any, set[Any]] = {block_id: set() for block_id in blocks}
    for block_id, succs in succ.items():
        for target in succs:
            pred[target].add(block_id)
    dom: dict[Any, set[Any]] = {block_id: set(blocks) for block_id in blocks}
    if entry in dom:
        dom[entry] = {entry}
    changed = True
    while changed:
        changed = False
        for block_id in blocks:
            if block_id == entry:
                continue
            new = {block_id}
            if pred[block_id]:
                new |= set.intersection(*(dom[p] for p in pred[block_id]))
            if new != dom[block_id]:
                dom[block_id] = new
                changed = True
    return dom


def call_operands(inst: dict[str, Any]) -> list[tuple[str, Any]]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return []
    callee = mir_call.get("callee")
    operands: list[tuple[str, Any]] = []
    if isinstance(callee, dict):
        operands.append(("receiver", callee.get("receiver")))
    args = mir_call.get("args", [])
    if isinstance(args, list):
        operands.extend(("arg", arg) for arg in args)
    return operands


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return "unknown"
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return "unknown"
    return str(callee.get("name", "unknown"))


def trace_copy_chain(seed: Any, copy_dst_to_src: dict[Any, Any]) -> tuple[list[Any], Any]:
    chain: list[Any] = []
    current = seed
    seen: set[Any] = set()
    while current in copy_dst_to_src and current not in seen:
        seen.add(current)
        chain.append(current)
        current = copy_dst_to_src[current]
    return chain, current


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--policy-selection", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    policy = read_kv(args.policy_selection)
    require(policy, "output_contract", "hako-mimalloc-call-operand-residual-policy-selection-v0", "policy-selection")
    require(policy, "selected_policy_family", "dominance_required_call_operand_forwarding", "policy-selection")
    require(policy, "summary", "ok", "policy-selection")

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)
    dom = dominators(function, blocks)

    defs: dict[Any, tuple[Any, int, dict[str, Any]]] = {}
    copy_dst_to_src: dict[Any, Any] = {}
    for block_id, insts in blocks.items():
        for inst_index, inst in enumerate(insts):
            dst = inst.get("dst")
            if dst is not None:
                defs[dst] = (block_id, inst_index, inst)
            if inst.get("op") == "copy":
                copy_dst_to_src[dst] = inst.get("src")

    role_counts: Counter[str] = Counter()
    safe_role_counts: Counter[str] = Counter()
    unsafe_count = 0
    candidate_count = 0
    callee_counts: Counter[str] = Counter()

    for block_id, insts in blocks.items():
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "mir_call":
                continue
            for role, operand in call_operands(inst):
                chain, root = trace_copy_chain(operand, copy_dst_to_src)
                if not chain:
                    continue
                root_block = defs.get(root, (None,))[0]
                if root_block is None or root_block == block_id:
                    continue
                candidate_count += 1
                role_counts[role] += 1
                callee_counts[callee_name(inst)] += 1
                if root_block in dom.get(block_id, set()):
                    safe_role_counts[role] += 1
                else:
                    unsafe_count += 1

    selected_keeper_shape = "dominance_guarded_receiver_operand_forwarding"
    selected_keeper_candidate_count = safe_role_counts["receiver"]
    rejected_arg_forwarding_count = safe_role_counts["arg"]

    lines = [
        "output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-design-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-690",
        "selected_policy_family=dominance_required_call_operand_forwarding",
        f"pre_candidate_count={candidate_count}",
        f"safe_dominance_candidate_count={sum(safe_role_counts.values())}",
        f"unsafe_candidate_count={unsafe_count}",
        f"safe_receiver_candidate_count={safe_role_counts['receiver']}",
        f"safe_arg_candidate_count={safe_role_counts['arg']}",
        f"selected_keeper_shape={selected_keeper_shape}",
        f"selected_keeper_candidate_count={selected_keeper_candidate_count}",
        f"rejected_arg_forwarding_count={rejected_arg_forwarding_count}",
        "arg_forwarding_enabled=0",
        "requires_dominance_guard=1",
        "helper_name_special_case=0",
        "variable_map_semantics_changed=0",
        "phi_lifecycle_changed=0",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for idx, (callee, count) in enumerate(callee_counts.most_common(8)):
        lines.append(f"top_callee_{idx}_name={callee}")
        lines.append(f"top_callee_{idx}_dominance_candidate_count={count}")
    lines.append("next_task=call_operand_dominance_required_forwarding_guard_surface")
    lines.append("summary=ok")

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

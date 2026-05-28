#!/usr/bin/env python3
"""Refresh objectLifecycleSmallAlloc MIR copy ownership after field_get cleanup."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def find_function(data: dict[str, Any], selected_method: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    for function in functions:
        if isinstance(function, dict) and function.get("name") == selected_method:
            return function
    raise SystemExit(f"selected method not found: {selected_method}")


def block_instructions(function: dict[str, Any]) -> list[tuple[Any, list[dict[str, Any]]]]:
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    out: list[tuple[Any, list[dict[str, Any]]]] = []
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if isinstance(insts, list):
            out.append((block.get("id"), [inst for inst in insts if isinstance(inst, dict)]))
    return out


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("name", ""))


def callee_receiver(inst: dict[str, Any]) -> Any:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return None
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return None
    return callee.get("receiver")


def call_args(inst: dict[str, Any]) -> list[Any]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return []
    args = mir_call.get("args", [])
    return args if isinstance(args, list) else []


def copy_ancestors(seed: Any, dst_to_src: dict[Any, Any]) -> set[Any]:
    seen: set[Any] = set()
    current = seed
    while current in dst_to_src and current not in seen:
        seen.add(current)
        current = dst_to_src[current]
    return seen


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
        receiver = callee_receiver(inst)
        attributed.update(copy_ancestors(receiver, dst_to_src))
        for arg in call_args(inst):
            attributed.update(copy_ancestors(arg, dst_to_src))
        if inst.get("dst") is not None:
            attributed.update(copy_descendants(inst.get("dst"), src_to_dsts))
    return attributed


def classify_position(
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


def nearest_producer_owner(src: Any, insts: list[dict[str, Any]], inst_index: int) -> str:
    cursor = src
    for prev in reversed(insts[max(0, inst_index - 6) : inst_index]):
        if prev.get("dst") != cursor:
            continue
        op = prev.get("op")
        if op == "field_get":
            return "field_get_result_chain"
        if op == "binop":
            return "binop_result_chain"
        if op == "compare":
            return "compare_result_chain"
        if op == "copy":
            cursor = prev.get("src")
            continue
        return f"{op}_result_chain"
    return "unknown_producer_chain"


def dominant(counts: Counter[str]) -> str:
    if not counts:
        return "none"
    return max(sorted(counts), key=lambda key: counts[key])


def select_next_owner(
    position_counts: Counter[str],
    expression_owner_counts: Counter[str],
    receiver_copy_count: int,
) -> tuple[str, str, str]:
    field_get_count = expression_owner_counts["field_get_result_chain"]
    call_adjacent_count = position_counts["call_adjacent"]
    expression_count = position_counts["expression_materialization"]

    if field_get_count >= 20 and field_get_count >= receiver_copy_count - 4:
        return (
            "field_get_result_chain_follow_on_probe",
            "medium",
            "field_get_result_chain_remains_dominant_expression_owner",
        )
    if call_adjacent_count >= expression_count and receiver_copy_count >= 20:
        return (
            "call_adjacent_receiver_materialization_probe",
            "medium",
            "call_adjacent_and_receiver_copy_pressure_remain_high",
        )
    return (
        "receiver_materialization_probe",
        "low",
        "remaining_copy_surface_is_mixed_after_field_get_cleanup",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    op_counts: Counter[str] = Counter()
    phi_dsts: set[Any] = set()
    receiver_copy_count = 0
    arg_copy_count = 0
    result_copy_count = 0
    local_ssa_copy_count = 0
    position_counts: Counter[str] = Counter()
    expression_owner_counts: Counter[str] = Counter()
    top_block_counts: Counter[Any] = Counter()

    for _, insts in blocks:
        op_counts.update(str(inst.get("op", "")) for inst in insts)
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    for block_id, insts in blocks:
        copies = [inst for inst in insts if inst.get("op") == "copy"]
        dst_to_src = {inst.get("dst"): inst.get("src") for inst in copies}
        src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
        for inst in copies:
            src_to_dsts[inst.get("src")].add(inst.get("dst"))
        call_attributed = collect_call_attributed_copy_dsts(insts)

        call_accounted: set[Any] = set()
        for inst in insts:
            if inst.get("op") != "mir_call":
                continue
            receiver_dsts = copy_ancestors(callee_receiver(inst), dst_to_src)
            arg_dsts: set[Any] = set()
            for arg in call_args(inst):
                arg_dsts.update(copy_ancestors(arg, dst_to_src))
            result_dsts = copy_descendants(inst.get("dst"), src_to_dsts) if inst.get("dst") is not None else set()

            receiver_copy_count += len(receiver_dsts)
            arg_copy_count += len(arg_dsts)
            result_copy_count += len(result_dsts)
            call_accounted.update(receiver_dsts | arg_dsts | result_dsts)

        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            dst = inst.get("dst")
            src = inst.get("src")
            top_block_counts[block_id] += 1
            category = classify_position(dst, src, inst_index, insts, phi_dsts, call_attributed)
            position_counts[category] += 1
            if dst not in call_accounted and category != "phi_edge":
                local_ssa_copy_count += 1
            if category == "expression_materialization":
                expression_owner_counts[nearest_producer_owner(src, insts, inst_index)] += 1

    selected_owner, confidence, reason = select_next_owner(
        position_counts,
        expression_owner_counts,
        receiver_copy_count,
    )

    lines = [
        "output_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0",
        "input_contract=hako-mimalloc-post-field-get-result-chain-cleanup-measurement-v0",
        f"target_method={function.get('name', args.method)}",
        f"block_count={len(blocks)}",
        f"instruction_count={sum(op_counts.values())}",
        f"call_count={op_counts['mir_call']}",
        f"copy_count={op_counts['copy']}",
        f"phi_count={op_counts['phi']}",
        f"receiver_copy_count={receiver_copy_count}",
        f"arg_copy_count={arg_copy_count}",
        f"result_copy_count={result_copy_count}",
        f"local_ssa_copy_count={local_ssa_copy_count}",
        f"call_adjacent_copy_count={position_counts['call_adjacent']}",
        f"phi_edge_copy_count={position_counts['phi_edge']}",
        f"expression_materialization_copy_count={position_counts['expression_materialization']}",
        f"field_get_result_chain_copy_count={expression_owner_counts['field_get_result_chain']}",
        f"dominant_position={dominant(position_counts)}",
        f"dominant_expression_owner={dominant(expression_owner_counts)}",
        f"selected_owner={selected_owner}",
        f"owner_confidence={confidence}",
        f"owner_reason={reason}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key in (
        "call_adjacent",
        "expression_materialization",
        "branch_condition",
        "block_entry",
        "field_set_value",
        "local_ssa",
        "phi_edge",
    ):
        lines.append(f"position_{key}_copy_count={position_counts[key]}")
    for owner, count in expression_owner_counts.most_common():
        lines.append(f"expression_owner_{owner}_copy_count={count}")
    for idx, (block_id, count) in enumerate(top_block_counts.most_common(6)):
        lines.append(f"top_block_{idx}_id=block_{block_id}")
        lines.append(f"top_block_{idx}_copy_count={count}")
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

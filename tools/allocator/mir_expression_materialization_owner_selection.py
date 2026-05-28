#!/usr/bin/env python3
"""Classify expression-materialization copy owners in one selected MIR method."""

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
    src_to_dsts: dict[Any, set[Any]] = {}
    for inst in copies:
        src_to_dsts.setdefault(inst.get("src"), set()).add(inst.get("dst"))

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


def is_expression_materialization(
    dst: Any,
    src: Any,
    inst_index: int,
    insts: list[dict[str, Any]],
    phi_dsts: set[Any],
    call_attributed: set[Any],
) -> bool:
    if dst in call_attributed:
        return False
    if src in phi_dsts:
        return False

    next_ops = insts[inst_index + 1 : inst_index + 4]
    if any(inst.get("op") == "ret" for inst in next_ops):
        return False
    if any(inst.get("op") == "branch" for inst in next_ops):
        return False
    if any(inst.get("op") == "field_set" and inst.get("value") == dst for inst in next_ops):
        return False
    if inst_index <= 2:
        return False
    if any(inst.get("op") == "jump" for inst in next_ops):
        return False

    prev_ops = insts[max(0, inst_index - 3) : inst_index]
    return any(inst.get("op") in {"field_get", "binop", "compare"} for inst in prev_ops + next_ops)


def nearest_producer_owner(src: Any, insts: list[dict[str, Any]], inst_index: int) -> str:
    for prev in reversed(insts[max(0, inst_index - 6) : inst_index]):
        if prev.get("dst") != src:
            continue
        op = prev.get("op")
        if op == "field_get":
            return "field_get_result_chain"
        if op == "binop":
            return "binop_result_chain"
        if op == "compare":
            return "compare_result_chain"
        if op == "const":
            return "const_value_chain"
        if op == "copy":
            src = prev.get("src")
            continue
        return f"{op}_result_chain"
    return "unknown_producer_chain"


def nearest_consumer_owner(dst: Any, insts: list[dict[str, Any]], inst_index: int) -> str:
    for nxt in insts[inst_index + 1 : inst_index + 7]:
        op = nxt.get("op")
        if op == "field_set":
            if nxt.get("value") == dst:
                return "field_set_value_chain"
            if nxt.get("box") == dst:
                return "field_set_receiver_chain"
        if op == "field_get" and nxt.get("box") == dst:
            return "field_get_receiver_chain"
        if op == "binop" and (nxt.get("lhs") == dst or nxt.get("rhs") == dst):
            return "binop_operand_chain"
        if op == "compare" and (nxt.get("lhs") == dst or nxt.get("rhs") == dst):
            return "compare_operand_chain"
        if op == "branch" and nxt.get("cond") == dst:
            return "branch_condition_chain"
        if op == "mir_call":
            return "call_adjacent_chain"
    return "unknown_consumer_chain"


def expression_owner(src: Any, dst: Any, insts: list[dict[str, Any]], inst_index: int) -> str:
    producer = nearest_producer_owner(src, insts, inst_index)
    consumer = nearest_consumer_owner(dst, insts, inst_index)
    if producer != "unknown_producer_chain":
        return producer
    return consumer


def dominant(counts: Counter[str]) -> str:
    if not counts:
        return "none"
    return max(sorted(counts), key=lambda key: counts[key])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=12)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    owner_counts: Counter[str] = Counter()
    block_owner_counts: Counter[tuple[Any, str]] = Counter()
    expression_copy_count = 0
    samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        call_attributed = collect_call_attributed_copy_dsts(insts)
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            if not is_expression_materialization(
                inst.get("dst"),
                inst.get("src"),
                inst_index,
                insts,
                phi_dsts,
                call_attributed,
            ):
                continue
            owner = expression_owner(inst.get("src"), inst.get("dst"), insts, inst_index)
            expression_copy_count += 1
            owner_counts[owner] += 1
            block_owner_counts[(block_id, owner)] += 1
            samples.append(
                {
                    "owner": owner,
                    "block_id": block_id,
                    "inst_index": inst_index,
                    "dst": inst.get("dst"),
                    "src": inst.get("src"),
                }
            )

    selected_owner = dominant(owner_counts)
    samples.sort(
        key=lambda sample: (
            owner_counts[sample["owner"]],
            block_owner_counts[(sample["block_id"], sample["owner"])],
            -int(sample["inst_index"]),
        ),
        reverse=True,
    )

    lines = [
        "output_contract=hako-mimalloc-expression-materialization-owner-selection-v0",
        "input_contract=hako-mimalloc-local-ssa-copy-position-probe-v0",
        f"target_method={function.get('name', args.method)}",
        f"expression_materialization_copy_count={expression_copy_count}",
        f"selected_owner={selected_owner}",
        "owner_confidence=medium",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key, count in owner_counts.most_common():
        lines.append(f"{key}_copy_count={count}")
    for idx, ((block_id, owner), count) in enumerate(block_owner_counts.most_common(8)):
        lines.append(f"top_block_owner_{idx}=block_{block_id}:{owner}")
        lines.append(f"top_block_owner_{idx}_copy_count={count}")
    for idx, sample in enumerate(samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_owner={sample['owner']}",
                f"{prefix}_block=block_{sample['block_id']}",
                f"{prefix}_inst_index={sample['inst_index']}",
                f"{prefix}_dst={sample['dst']}",
                f"{prefix}_src={sample['src']}",
            ]
        )
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

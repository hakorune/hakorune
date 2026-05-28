#!/usr/bin/env python3
"""Classify local-SSA MIR copy positions inside one selected method."""

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
        if not isinstance(insts, list):
            continue
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

    position_counts: Counter[str] = Counter()
    local_like_position_counts: Counter[str] = Counter()
    block_counts: Counter[Any] = Counter()
    total_copy_count = 0
    local_like_count = 0
    samples: list[dict[str, Any]] = []
    local_like_samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        call_attributed = collect_call_attributed_copy_dsts(insts)
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            total_copy_count += 1
            dst = inst.get("dst")
            category = classify_copy(
                dst,
                inst.get("src"),
                inst_index,
                insts,
                phi_dsts,
                call_attributed,
            )
            position_counts[category] += 1
            block_counts[block_id] += 1
            if category not in {"call_adjacent", "phi_edge"}:
                local_like_count += 1
                local_like_position_counts[category] += 1
            samples.append(
                {
                    "category": category,
                    "block_id": block_id,
                    "inst_index": inst_index,
                    "dst": dst,
                    "src": inst.get("src"),
                }
            )
            if category not in {"call_adjacent", "phi_edge"}:
                local_like_samples.append(samples[-1])

    local_like_samples.sort(
        key=lambda sample: (
            local_like_position_counts[sample["category"]],
            block_counts[sample["block_id"]],
            -int(sample["inst_index"]),
        ),
        reverse=True,
    )

    lines = [
        "output_contract=hako-mimalloc-local-ssa-copy-position-probe-v0",
        "input_contract=hako-mimalloc-callsite-copy-owner-selection-v0",
        f"target_method={function.get('name', args.method)}",
        f"block_count={len(blocks)}",
        f"copy_count={total_copy_count}",
        f"local_like_copy_count={local_like_count}",
        f"dominant_position={dominant(position_counts)}",
        f"dominant_local_like_position={dominant(local_like_position_counts)}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key in (
        "local_ssa",
        "expression_materialization",
        "field_set_value",
        "branch_condition",
        "return_block",
        "block_entry",
        "block_exit",
        "call_adjacent",
        "phi_edge",
    ):
        lines.append(f"{key}_copy_count={position_counts[key]}")
    for idx, (block_id, count) in enumerate(block_counts.most_common(8)):
        lines.append(f"top_block_{idx}_id=block_{block_id}")
        lines.append(f"top_block_{idx}_copy_count={count}")
    for idx, sample in enumerate(local_like_samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_category={sample['category']}",
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

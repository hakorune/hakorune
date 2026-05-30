#!/usr/bin/env python3
"""Classify helper-call copy families inside objectLifecycleSmallAlloc MIR."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"

FACADE_RESULT_HELPERS = {
    "resetSmallAllocResult",
    "recordAttempt",
    "recordSelectedPage",
    "recordBlock",
    "recordSmallAllocFailure",
    "recordSmallAllocSuccess",
}
FACADE_STATE_HELPERS = {
    "recordLastAllocPage",
}
PAGE_HOTPATH_HELPERS = {
    "beginSelection",
    "selectSinglePageFastPath",
    "selectPage",
    "acquire",
    "acquireFreshSmall",
    "reuse",
}


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
    matches = [fn for fn in functions if isinstance(fn, dict) and fn.get("name") == selected_method]
    if matches:
        return matches[0]
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


def helper_family(name: str) -> str:
    if name in FACADE_RESULT_HELPERS:
        return "facade_result_helpers"
    if name in FACADE_STATE_HELPERS:
        return "facade_state_helpers"
    if name in PAGE_HOTPATH_HELPERS:
        return "page_hotpath_helpers"
    return "other"


def collect_ancestor_copy_dsts(seed: Any, copy_dst_to_src: dict[Any, Any]) -> set[Any]:
    seen: set[Any] = set()
    current = seed
    while current in copy_dst_to_src and current not in seen:
        seen.add(current)
        current = copy_dst_to_src[current]
    return seen


def collect_descendant_copy_dsts(seed: Any, copy_src_to_dsts: dict[Any, set[Any]]) -> set[Any]:
    seen: set[Any] = set()
    stack = list(copy_src_to_dsts.get(seed, ()))
    while stack:
        current = stack.pop()
        if current in seen:
            continue
        seen.add(current)
        stack.extend(copy_src_to_dsts.get(current, ()))
    return seen


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    receiver_copy_count = 0
    arg_copy_count = 0
    result_copy_count = 0
    local_ssa_copy_count = 0
    helper_call_count = 0
    helper_copy_count = 0

    family_call_counts: Counter[str] = Counter()
    family_receiver_copies: Counter[str] = Counter()
    family_arg_copies: Counter[str] = Counter()
    family_result_copies: Counter[str] = Counter()
    family_local_ssa_copies: Counter[str] = Counter()
    callee_counts: Counter[str] = Counter()

    hottest_blocks: list[tuple[int, Any, int, int, int, int]] = []

    for block_id, insts in blocks:
        copies = [inst for inst in insts if inst.get("op") == "copy"]
        copy_dst_to_src = {inst.get("dst"): inst.get("src") for inst in copies}
        copy_src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
        for inst in copies:
            copy_src_to_dsts[inst.get("src")].add(inst.get("dst"))

        block_accounted_copy_dsts: set[Any] = set()

        for inst in insts:
            if inst.get("op") != "mir_call":
                continue
            name = callee_name(inst)
            family = helper_family(name)
            if family == "other":
                continue

            helper_call_count += 1
            family_call_counts[family] += 1
            callee_counts[name] += 1

            call_receiver = callee_receiver(inst)
            receiver_dsts = collect_ancestor_copy_dsts(call_receiver, copy_dst_to_src)
            receiver_copy_count += len(receiver_dsts)
            family_receiver_copies[family] += len(receiver_dsts)

            arg_dsts: set[Any] = set()
            for arg in call_args(inst):
                arg_dsts.update(collect_ancestor_copy_dsts(arg, copy_dst_to_src))
            arg_copy_count += len(arg_dsts)
            family_arg_copies[family] += len(arg_dsts)

            call_dst = inst.get("dst")
            result_dsts = set()
            if call_dst is not None:
                result_dsts = collect_descendant_copy_dsts(call_dst, copy_src_to_dsts)
            result_copy_count += len(result_dsts)
            family_result_copies[family] += len(result_dsts)

            accounted = receiver_dsts | arg_dsts | result_dsts
            block_accounted_copy_dsts.update(accounted)
            helper_copy_count += len(accounted)
            hottest_blocks.append(
                (
                    len(accounted),
                    block_id,
                    len(receiver_dsts),
                    len(arg_dsts),
                    len(result_dsts),
                    name,
                )
            )

        block_copy_dsts = {inst.get("dst") for inst in copies}
        block_local_ssa = {dst for dst in block_copy_dsts - block_accounted_copy_dsts if dst is not None}
        if block_local_ssa:
            local_ssa_copy_count += len(block_local_ssa)

            hot_families = {
                helper_family(callee_name(inst))
                for inst in insts
                if inst.get("op") == "mir_call" and helper_family(callee_name(inst)) != "other"
            }
            if len(hot_families) == 1:
                only = next(iter(hot_families))
                family_local_ssa_copies[only] += len(block_local_ssa)

    family_total_copies: Counter[str] = Counter()
    for family in set(family_call_counts):
        family_total_copies[family] = (
            family_receiver_copies[family]
            + family_arg_copies[family]
            + family_result_copies[family]
            + family_local_ssa_copies[family]
        )

    dominant_copy_family = "helper_result_local_ssa"
    if receiver_copy_count > max(arg_copy_count, result_copy_count + local_ssa_copy_count):
        dominant_copy_family = "receiver_materialization"
    elif arg_copy_count > max(receiver_copy_count, result_copy_count + local_ssa_copy_count):
        dominant_copy_family = "arg_materialization"

    dominant_callee_family = "none"
    if family_total_copies:
        dominant_callee_family = family_total_copies.most_common(1)[0][0]

    lines = [
        "output_contract=hako-mimalloc-small-alloc-helper-copy-family-probe-v0",
        "input_contract=small-alloc-call-copy-shape-deep-dive-v0",
        f"selected_owner={function.get('name', args.method)}",
        f"helper_call_count={helper_call_count}",
        f"helper_copy_count={helper_copy_count}",
        f"receiver_copy_count={receiver_copy_count}",
        f"arg_copy_count={arg_copy_count}",
        f"result_copy_count={result_copy_count}",
        f"local_ssa_copy_count={local_ssa_copy_count}",
        f"dominant_copy_family={dominant_copy_family}",
        f"dominant_callee_family={dominant_callee_family}",
        "selected_next=same_module_helper_call_lowering_seam",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for family in ("facade_result_helpers", "facade_state_helpers", "page_hotpath_helpers"):
        lines.extend(
            [
                f"{family}_call_count={family_call_counts[family]}",
                f"{family}_receiver_copy_count={family_receiver_copies[family]}",
                f"{family}_arg_copy_count={family_arg_copies[family]}",
                f"{family}_result_copy_count={family_result_copies[family]}",
                f"{family}_local_ssa_copy_count={family_local_ssa_copies[family]}",
                f"{family}_total_copy_count={family_total_copies[family]}",
            ]
        )
    for idx, (callee, count) in enumerate(callee_counts.most_common(8)):
        lines.append(f"top_helper_{idx}={callee}")
        lines.append(f"top_helper_{idx}_call_count={count}")
    hottest_blocks.sort(reverse=True)
    for idx, (count, block_id, receiver_count, arg_count, result_count, callee) in enumerate(hottest_blocks[:8]):
        lines.append(f"hot_block_{idx}=block_{block_id}:{callee}")
        lines.append(f"hot_block_{idx}_accounted_copy_count={count}")
        lines.append(f"hot_block_{idx}_receiver_copy_count={receiver_count}")
        lines.append(f"hot_block_{idx}_arg_copy_count={arg_count}")
        lines.append(f"hot_block_{idx}_result_copy_count={result_count}")
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

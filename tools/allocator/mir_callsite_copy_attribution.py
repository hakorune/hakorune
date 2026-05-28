#!/usr/bin/env python3
"""Attribute MIR copy instructions around callsites in one selected method."""

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
    "acquire_usize",
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


def value_text(value: Any) -> str:
    if value is None:
        return "none"
    return str(value)


def dominant_owner(counts: dict[str, int]) -> str:
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

    op_counts: Counter[str] = Counter()
    for _, insts in blocks:
        op_counts.update(str(inst.get("op", "")) for inst in insts)

    total_instruction_count = sum(op_counts.values())
    total_copy_count = op_counts["copy"]
    total_call_count = op_counts["mir_call"]
    total_phi_count = op_counts["phi"]

    receiver_copy_count = 0
    arg_copy_count = 0
    result_copy_count = 0
    local_ssa_copy_count = 0
    phi_edge_copy_count = 0
    unknown_copy_count = 0
    helper_call_count = 0
    helper_copy_count = 0

    family_counts: Counter[str] = Counter()
    family_copy_counts: Counter[str] = Counter()
    owner_counts: Counter[str] = Counter()
    callsites: list[dict[str, Any]] = []

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    accounted_global: set[tuple[Any, Any]] = set()
    for block_id, insts in blocks:
        copies = [inst for inst in insts if inst.get("op") == "copy"]
        copy_dst_to_src = {inst.get("dst"): inst.get("src") for inst in copies}
        copy_src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
        copy_index_by_dst = {inst.get("dst"): idx for idx, inst in enumerate(insts)}
        for inst in copies:
            copy_src_to_dsts[inst.get("src")].add(inst.get("dst"))

        block_accounted: set[Any] = set()
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "mir_call":
                continue

            name = callee_name(inst)
            family = helper_family(name)
            family_counts[family] += 1
            if family != "other":
                helper_call_count += 1

            receiver = callee_receiver(inst)
            receiver_dsts = collect_ancestor_copy_dsts(receiver, copy_dst_to_src)
            arg_dsts: set[Any] = set()
            for arg in call_args(inst):
                arg_dsts.update(collect_ancestor_copy_dsts(arg, copy_dst_to_src))

            call_dst = inst.get("dst")
            result_dsts: set[Any] = set()
            if call_dst is not None:
                result_dsts = collect_descendant_copy_dsts(call_dst, copy_src_to_dsts)

            accounted = receiver_dsts | arg_dsts | result_dsts
            block_accounted.update(accounted)
            accounted_global.update((block_id, dst) for dst in accounted)

            receiver_count = len(receiver_dsts)
            arg_count = len(arg_dsts)
            result_count = len(result_dsts)
            attributed_count = len(accounted)
            pre_call_count = sum(1 for dst in accounted if copy_index_by_dst.get(dst, 10**9) < inst_index)
            post_call_count = attributed_count - pre_call_count

            receiver_copy_count += receiver_count
            arg_copy_count += arg_count
            result_copy_count += result_count
            if family != "other":
                helper_copy_count += attributed_count
                family_copy_counts[family] += attributed_count

            owner_counts["receiver_materialization"] += receiver_count
            owner_counts["arg_materialization"] += arg_count
            owner_counts["result_materialization"] += result_count

            callsites.append(
                {
                    "callee": name or "unknown",
                    "family": family,
                    "block_id": block_id,
                    "inst_index": inst_index,
                    "receiver": receiver,
                    "receiver_copy_count": receiver_count,
                    "arg_copy_count": arg_count,
                    "result_copy_count": result_count,
                    "pre_call_copy_count": pre_call_count,
                    "post_call_copy_count": post_call_count,
                    "attributed_copy_count": attributed_count,
                }
            )

        block_copy_dsts = {inst.get("dst") for inst in copies if inst.get("dst") is not None}
        unaccounted = block_copy_dsts - block_accounted
        block_phi_edge = {
            dst
            for dst in unaccounted
            if copy_dst_to_src.get(dst) in phi_dsts
        }
        block_unknown = {
            dst
            for dst in unaccounted
            if dst is None
        }
        block_local_ssa = unaccounted - block_phi_edge - block_unknown
        phi_edge_copy_count += len(block_phi_edge)
        local_ssa_copy_count += len(block_local_ssa)
        unknown_copy_count += len(block_unknown)

    owner_counts["local_ssa_copy_materialization"] += local_ssa_copy_count
    owner_counts["phi_edge_copy_materialization"] += phi_edge_copy_count
    owner_counts["unknown_copy_materialization"] += unknown_copy_count

    callsites.sort(
        key=lambda item: (
            item["attributed_copy_count"],
            item["receiver_copy_count"],
            item["result_copy_count"],
            str(item["callee"]),
        ),
        reverse=True,
    )

    dominant_callee_family = "none"
    known_family_counts = Counter(
        {family: count for family, count in family_copy_counts.items() if family != "other"}
    )
    if known_family_counts:
        dominant_callee_family = known_family_counts.most_common(1)[0][0]

    lines = [
        "output_contract=hako-mimalloc-callsite-copy-attribution-v0",
        "input_contract=same-module-helper-call-lowering-seam-v0",
        f"target_method={function.get('name', args.method)}",
        f"block_count={len(blocks)}",
        f"instruction_count={total_instruction_count}",
        f"call_count={total_call_count}",
        f"copy_count={total_copy_count}",
        f"phi_count={total_phi_count}",
        f"helper_call_count={helper_call_count}",
        f"helper_copy_count={helper_copy_count}",
        f"receiver_copy_count={receiver_copy_count}",
        f"arg_copy_count={arg_copy_count}",
        f"result_copy_count={result_copy_count}",
        f"local_ssa_copy_count={local_ssa_copy_count}",
        f"phi_edge_copy_count={phi_edge_copy_count}",
        f"unknown_copy_count={unknown_copy_count}",
        f"dominant_callee_family={dominant_callee_family}",
        f"dominant_copy_owner={dominant_owner(dict(owner_counts))}",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]

    for owner, count in owner_counts.most_common():
        lines.append(f"owner_{owner}_copy_count={count}")
    for family in ("facade_result_helpers", "facade_state_helpers", "page_hotpath_helpers", "other"):
        lines.append(f"{family}_call_count={family_counts[family]}")
        lines.append(f"{family}_attributed_copy_count={family_copy_counts[family]}")

    for idx, callsite in enumerate(callsites[: max(0, args.topn)]):
        prefix = f"callsite_{idx}"
        lines.extend(
            [
                f"{prefix}_callee={callsite['callee']}",
                f"{prefix}_callee_family={callsite['family']}",
                f"{prefix}_block=block_{callsite['block_id']}",
                f"{prefix}_inst_index={callsite['inst_index']}",
                f"{prefix}_receiver_value={value_text(callsite['receiver'])}",
                f"{prefix}_receiver_copy_chain_len={callsite['receiver_copy_count']}",
                f"{prefix}_arg_copy_count={callsite['arg_copy_count']}",
                f"{prefix}_result_copy_count={callsite['result_copy_count']}",
                f"{prefix}_pre_call_copy_count={callsite['pre_call_copy_count']}",
                f"{prefix}_post_call_copy_count={callsite['post_call_copy_count']}",
                f"{prefix}_attributed_copy_count={callsite['attributed_copy_count']}",
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

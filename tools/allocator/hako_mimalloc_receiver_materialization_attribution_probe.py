#!/usr/bin/env python3
"""Attribute receiver materialization copy chains in one MIR method."""

from __future__ import annotations

import argparse
import json
from collections import Counter
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


def helper_family(name: str) -> str:
    if name in FACADE_RESULT_HELPERS:
        return "facade_result_helpers"
    if name in PAGE_HOTPATH_HELPERS:
        return "page_hotpath_helpers"
    return "other"


def trace_copy_chain(receiver: Any, producers: dict[Any, dict[str, Any]]) -> tuple[list[Any], Any, str]:
    chain: list[Any] = []
    current = receiver
    seen: set[Any] = set()
    while current in producers and current not in seen:
        inst = producers[current]
        if inst.get("op") != "copy":
            break
        seen.add(current)
        chain.append(current)
        current = inst.get("src")
    root_kind = "param"
    if current in producers:
        op = str(producers[current].get("op", "unknown"))
        if op == "field_get":
            root_kind = f"field_get:{producers[current].get('field', 'unknown')}"
        else:
            root_kind = op
    return chain, current, root_kind


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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=12)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    receiver_attributed = 0
    unique_receiver_copies: set[tuple[Any, Any]] = set()
    duplicate_attribution = 0
    family_counts: Counter[str] = Counter()
    chain_len_counts: Counter[str] = Counter()
    root_kind_counts: Counter[str] = Counter()
    root_value_counts: Counter[str] = Counter()
    callsites: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        producers = {inst.get("dst"): inst for inst in insts if inst.get("dst") is not None}
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "mir_call":
                continue
            receiver = callee_receiver(inst)
            chain, root_value, root_kind = trace_copy_chain(receiver, producers)
            if not chain:
                continue
            name = callee_name(inst) or "unknown"
            family = helper_family(name)
            chain_len = len(chain)
            receiver_attributed += chain_len
            family_counts[family] += chain_len
            chain_len_counts[str(chain_len)] += 1
            root_kind_counts[root_kind] += chain_len
            root_value_counts[str(root_value)] += chain_len
            for dst in chain:
                key = (block_id, dst)
                if key in unique_receiver_copies:
                    duplicate_attribution += 1
                unique_receiver_copies.add(key)
            callsites.append(
                {
                    "callee": name,
                    "family": family,
                    "block": block_id,
                    "inst_index": inst_index,
                    "receiver": receiver,
                    "chain_len": chain_len,
                    "root_value": root_value,
                    "root_kind": root_kind,
                }
            )

    selected = "receiver_pin_chain_policy_selection"
    next_diagnostic = "receiver_pin_chain_policy_selection"

    callsites.sort(
        key=lambda item: (
            item["chain_len"],
            family_counts[item["family"]],
            str(item["callee"]),
        ),
        reverse=True,
    )

    lines = [
        "output_contract=hako-mimalloc-receiver-materialization-attribution-probe-v0",
        "input_contract=hako-mimalloc-field-get-direct-consumer-forwarding-keeper-v0",
        f"target_method={function.get('name', args.method)}",
        f"receiver_attributed_copy_count={receiver_attributed}",
        f"unique_receiver_copy_count={len(unique_receiver_copies)}",
        f"duplicate_receiver_attribution_count={duplicate_attribution}",
        f"page_hotpath_receiver_copy_count={family_counts['page_hotpath_helpers']}",
        f"other_receiver_copy_count={family_counts['other']}",
        f"facade_result_receiver_copy_count={family_counts['facade_result_helpers']}",
        f"dominant_receiver_family={dominant(family_counts)}",
        f"dominant_receiver_chain_len={dominant(chain_len_counts)}",
        f"selected_receiver_policy={selected}",
        f"next_diagnostic={next_diagnostic}",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key, count in family_counts.most_common():
        lines.append(f"family_{safe_key(key)}_receiver_copy_count={count}")
    for key, count in chain_len_counts.most_common():
        lines.append(f"receiver_chain_len_{safe_key(key)}_callsite_count={count}")
    for key, count in root_kind_counts.most_common(8):
        lines.append(f"root_kind_{safe_key(key)}_receiver_copy_count={count}")
    for key, count in root_value_counts.most_common(8):
        lines.append(f"root_value_{safe_key(key)}_receiver_copy_count={count}")
    for idx, callsite in enumerate(callsites[: max(0, args.topn)]):
        prefix = f"callsite_{idx}"
        lines.extend(
            [
                f"{prefix}_callee={callsite['callee']}",
                f"{prefix}_family={callsite['family']}",
                f"{prefix}_block=block_{callsite['block']}",
                f"{prefix}_inst_index={callsite['inst_index']}",
                f"{prefix}_receiver_value={callsite['receiver']}",
                f"{prefix}_receiver_chain_len={callsite['chain_len']}",
                f"{prefix}_root_value={callsite['root_value']}",
                f"{prefix}_root_kind={callsite['root_kind']}",
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

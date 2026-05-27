#!/usr/bin/env python3
"""Classify return-block local-SSA copy pressure in objectLifecycleSmallAlloc."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
REASON_PREFIX = "HakoAllocObjectLifecycleFacadeReason."


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


def callee(inst: dict[str, Any]) -> dict[str, Any]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return {}
    value = mir_call.get("callee")
    return value if isinstance(value, dict) else {}


def call_args(inst: dict[str, Any]) -> list[Any]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return []
    args = mir_call.get("args", [])
    return args if isinstance(args, list) else []


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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    return_blocks = [
        (bid, insts)
        for bid, insts in block_instructions(function)
        if any(inst.get("op") == "ret" for inst in insts)
    ]

    return_block_copy_count = 0
    receiver_copy_count = 0
    arg_copy_count = 0
    duplicate_reason_call_count = 0
    reason_call_count = 0

    for _, insts in return_blocks:
        copy_dsts = {inst.get("dst") for inst in insts if inst.get("op") == "copy"}
        return_block_copy_count += len(copy_dsts)
        reason_calls: Counter[str] = Counter()
        for inst in insts:
            if inst.get("op") != "mir_call":
                continue
            callee_info = callee(inst)
            name = str(callee_info.get("name", ""))
            if name.startswith(REASON_PREFIX):
                reason_calls[name] += 1
                reason_call_count += 1
            if callee_info.get("receiver") in copy_dsts:
                receiver_copy_count += 1
            arg_copy_count += sum(1 for arg in call_args(inst) if arg in copy_dsts)
        duplicate_reason_call_count += sum(max(0, count - 1) for count in reason_calls.values())

    if duplicate_reason_call_count:
        next_action = "reason_call_probe"
        selected_reason = "failure_return_blocks_duplicate_reason_global_calls"
    elif arg_copy_count >= receiver_copy_count:
        next_action = "arg_materialization_probe"
        selected_reason = "return_block_arg_copy_uses_dominate"
    elif receiver_copy_count:
        next_action = "receiver_materialization_probe"
        selected_reason = "return_block_receiver_copy_uses_dominate"
    else:
        next_action = "stop_line"
        selected_reason = "return_block_copy_owner_unclear"

    lines = [
        "output_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0",
        "input_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0",
        f"selected_owner={function.get('name', args.method)}",
        f"return_block_count={len(return_blocks)}",
        f"return_block_copy_count={return_block_copy_count}",
        f"receiver_copy_count={receiver_copy_count}",
        f"arg_copy_count={arg_copy_count}",
        f"reason_call_count={reason_call_count}",
        f"duplicate_reason_call_count={duplicate_reason_call_count}",
        f"selected_reason={selected_reason}",
        f"next_action={next_action}",
        "next_diagnostic=small_alloc_duplicate_reason_call_probe",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

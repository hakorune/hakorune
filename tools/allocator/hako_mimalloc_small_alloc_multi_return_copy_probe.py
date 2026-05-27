#!/usr/bin/env python3
"""Classify remaining return-block copy shape in objectLifecycleSmallAlloc MIR."""

from __future__ import annotations

import argparse
import json
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
    matches = [fn for fn in functions if isinstance(fn, dict) and fn.get("name") == selected_method]
    if matches:
        return matches[0]
    raise SystemExit(f"selected method not found: {selected_method}")


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("name", ""))


def block_instructions(function: dict[str, Any]) -> list[tuple[Any, list[dict[str, Any]]]]:
    out: list[tuple[Any, list[dict[str, Any]]]] = []
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if not isinstance(insts, list):
            continue
        out.append((block.get("id"), [inst for inst in insts if isinstance(inst, dict)]))
    return out


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    data = load_json(args.mir_json)
    function = find_function(data, args.method)
    blocks = block_instructions(function)
    all_insts = [inst for _, insts in blocks for inst in insts]
    phis = [inst for inst in all_insts if inst.get("op") == "phi"]
    copies = [inst for inst in all_insts if inst.get("op") == "copy"]
    phi_defs = {inst.get("dst") for inst in phis}

    return_blocks = [(bid, insts) for bid, insts in blocks if any(inst.get("op") == "ret" for inst in insts)]
    return_block_copy_count = sum(1 for _, insts in return_blocks for inst in insts if inst.get("op") == "copy")
    failure_return_blocks = [
        (bid, insts)
        for bid, insts in return_blocks
        if any(callee_name(inst) == "recordSmallAllocFailure" for inst in insts)
    ]
    success_return_blocks = [
        (bid, insts)
        for bid, insts in return_blocks
        if any(callee_name(inst) == "recordSmallAllocSuccess" for inst in insts)
    ]
    failure_return_copy_count = sum(1 for _, insts in failure_return_blocks for inst in insts if inst.get("op") == "copy")
    success_return_copy_count = sum(1 for _, insts in success_return_blocks for inst in insts if inst.get("op") == "copy")
    ret_values = {
        inst.get("value")
        for _, insts in return_blocks
        for inst in insts
        if inst.get("op") == "ret"
    }
    copy_to_return_value_count = sum(1 for inst in copies if inst.get("dst") in ret_values)
    copy_from_phi_count = sum(1 for inst in copies if inst.get("src") in phi_defs)

    if return_block_copy_count > 0 and copy_to_return_value_count == 0:
        next_action = "local_ssa_copy_probe"
        selected_reason = "return_blocks_copy_call_receivers_and_args_not_return_values"
    elif len(return_blocks) > 1:
        next_action = "return_lowering_probe"
        selected_reason = "multiple_return_values_still_need_return_lowering_probe"
    elif success_return_copy_count > 0:
        next_action = "hako_shape_probe"
        selected_reason = "success_return_source_shape_still_has_copy_pressure"
    else:
        next_action = "stop_line"
        selected_reason = "copy_owner_unclear"

    lines = [
        "output_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0",
        "input_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0",
        f"selected_owner={function.get('name', args.method)}",
        f"return_count={len(return_blocks)}",
        f"copy_count={len(copies)}",
        f"copy_from_phi_count={copy_from_phi_count}",
        "candidate_source=multi_return_join",
        f"return_block_copy_count={return_block_copy_count}",
        f"failure_return_block_count={len(failure_return_blocks)}",
        f"failure_return_copy_count={failure_return_copy_count}",
        f"success_return_block_count={len(success_return_blocks)}",
        f"success_return_copy_count={success_return_copy_count}",
        f"copy_to_return_value_count={copy_to_return_value_count}",
        f"selected_reason={selected_reason}",
        f"next_action={next_action}",
        "next_diagnostic=small_alloc_return_block_local_ssa_copy_probe",
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

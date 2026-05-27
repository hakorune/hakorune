#!/usr/bin/env python3
"""Classify the phi/copy source in objectLifecycleSmallAlloc MIR."""

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


def instructions(function: dict[str, Any]) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if not isinstance(insts, list):
            continue
        out.extend(inst for inst in insts if isinstance(inst, dict))
    return out


def classify(
    single_incoming_phi_count: int,
    multi_incoming_phi_count: int,
    copy_count: int,
    copy_from_phi_count: int,
    return_count: int,
) -> tuple[str, str]:
    if single_incoming_phi_count > multi_incoming_phi_count * 2 and copy_count > 0:
        return ("local_copy_churn", "mirbuilder_owner_probe")
    if return_count >= 4 and multi_incoming_phi_count > 0:
        return ("multi_return_join", "mirbuilder_owner_probe")
    if multi_incoming_phi_count > 0 and copy_from_phi_count > 0:
        return ("branch_result_merge", "mirbuilder_owner_probe")
    return ("unknown", "stop_line")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    data = load_json(args.mir_json)
    function = find_function(data, args.method)
    insts = instructions(function)
    phis = [inst for inst in insts if inst.get("op") == "phi"]
    copies = [inst for inst in insts if inst.get("op") == "copy"]
    phi_defs = {inst.get("dst") for inst in phis}

    single_incoming_phi_count = sum(1 for inst in phis if len(inst.get("incoming", [])) == 1)
    multi_incoming_phi_count = sum(1 for inst in phis if len(inst.get("incoming", [])) > 1)
    copy_from_phi_count = sum(1 for inst in copies if inst.get("src") in phi_defs)
    return_count = sum(1 for inst in insts if inst.get("op") == "ret")
    branch_count = sum(1 for inst in insts if inst.get("op") == "branch")
    jump_count = sum(1 for inst in insts if inst.get("op") == "jump")
    candidate_source, next_action = classify(
        single_incoming_phi_count,
        multi_incoming_phi_count,
        len(copies),
        copy_from_phi_count,
        return_count,
    )

    lines = [
        "output_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0",
        "input_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0",
        f"selected_owner={function.get('name', args.method)}",
        f"phi_count={len(phis)}",
        f"copy_count={len(copies)}",
        f"single_incoming_phi_count={single_incoming_phi_count}",
        f"multi_incoming_phi_count={multi_incoming_phi_count}",
        f"copy_from_phi_count={copy_from_phi_count}",
        f"return_count={return_count}",
        f"branch_count={branch_count}",
        f"jump_count={jump_count}",
        f"candidate_source={candidate_source}",
        f"next_action={next_action}",
        "next_diagnostic=single_incoming_phi_copy_elision_owner_selection",
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

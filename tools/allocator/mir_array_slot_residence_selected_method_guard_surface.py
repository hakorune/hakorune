#!/usr/bin/env python3
"""Guard surface for the selected-method ArraySlotResidence keeper."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
METHOD = "HakoAllocPageModel.acquire_usize/1"


def emit_mir_json(path: Path) -> None:
    subprocess.run(
        [
            str(ROOT / "target/release/hakorune"),
            "--backend",
            "mir",
            "--emit-mir-json",
            str(path),
            str(APP),
        ],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        check=True,
    )


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be object")
    return data


def find_function(data: dict[str, Any], method: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    matches = [fn for fn in functions if isinstance(fn, dict) and fn.get("name") == method]
    if len(matches) != 1:
        raise SystemExit(f"selected method not found exactly once: {method}")
    return matches[0]


def callee(inst: dict[str, Any]) -> tuple[str, str]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return "", ""
    callee_obj = mir_call.get("callee")
    if not isinstance(callee_obj, dict):
        return "", ""
    return str(callee_obj.get("box_name", "")), str(callee_obj.get("name", ""))


def block_instructions(function: dict[str, Any]) -> list[tuple[int, list[dict[str, Any]]]]:
    out: list[tuple[int, list[dict[str, Any]]]] = []
    for block in function.get("blocks") or []:
        if not isinstance(block, dict):
            continue
        block_id = block.get("id")
        insts = block.get("instructions")
        if isinstance(block_id, int) and isinstance(insts, list):
            out.append((block_id, [inst for inst in insts if isinstance(inst, dict)]))
    return out


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    subprocess.run(["cargo", "build", "--release", "--bin", "hakorune"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory(prefix="hakorune_array_slot_guard_surface.") as tmp:
        mir_json = args.mir_json or Path(tmp) / "app.mir.json"
        if args.mir_json is None:
            emit_mir_json(mir_json)
        function = find_function(load_json(mir_json), METHOD)

    get_blocks: list[int] = []
    set_blocks: list[int] = []
    get_dst = None
    set_uses_get_dst = 0
    get_result_carriers: set[int] = set()
    selected_block = None
    for block_id, insts in block_instructions(function):
        get_result_carriers.clear()
        for inst in insts:
            if inst.get("op") == "copy" and inst.get("src") in get_result_carriers:
                dst = inst.get("dst")
                if isinstance(dst, int):
                    get_result_carriers.add(dst)
                continue
            if inst.get("op") != "mir_call":
                continue
            box_name, method_name = callee(inst)
            if box_name == "ArrayBox" and method_name == "get":
                get_blocks.append(block_id)
                get_dst = inst.get("dst")
                if isinstance(get_dst, int):
                    get_result_carriers.add(get_dst)
                selected_block = block_id
            if box_name == "ArrayBox" and method_name == "set":
                set_blocks.append(block_id)
                args_list = inst.get("mir_call", {}).get("args", [])
                if isinstance(args_list, list) and any(arg in get_result_carriers for arg in args_list):
                    set_uses_get_dst += 1

    same_block_pair = int(len(get_blocks) == 1 and len(set_blocks) == 1 and get_blocks == set_blocks)
    supported = int(same_block_pair == 1 and set_uses_get_dst == 1)

    lines = [
        "output_contract=mir-array-slot-residence-selected-method-guard-surface-v0",
        "input_contract=mir-array-slot-residence-inventory-v0",
        f"selected_method={METHOD}",
        "selected_reason=explicit_hot_context",
        f"selected_block={selected_block if selected_block is not None else 0}",
        f"array_get_call_count={len(get_blocks)}",
        f"array_set_call_count={len(set_blocks)}",
        f"same_block_get_set_pair={same_block_pair}",
        f"set_uses_get_result={set_uses_get_dst}",
        "planned_transform_kind=selected_method_same_block_array_get_set_direct_slot_op",
        "planned_erased_get_set_helper_calls=2",
        "planned_added_guard_helper_calls=1",
        "planned_added_writeback_helper_calls=0",
        "planned_net_helper_call_delta=1",
        f"implementation_surface_supported={supported}",
        "generic_array_residence_open=0",
        "by_name_hako_alloc_special_case=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

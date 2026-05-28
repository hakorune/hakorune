#!/usr/bin/env python3
"""Freeze guard surface for selected-method receiver block-entry copy forwarding."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


METHOD = "HakoAllocPageModel.acquire_usize/1"


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def load_json(path: Path) -> dict[str, Any]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def find_function(data: dict[str, Any]) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    for function in functions:
        if isinstance(function, dict) and function.get("name") == METHOD:
            return function
    raise SystemExit(f"selected method not found: {METHOD}")


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
    src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
    for inst in copies:
        src_to_dsts[inst.get("src")].add(inst.get("dst"))

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


def sink_kind(dst: Any, inst_index: int, insts: list[dict[str, Any]]) -> str:
    for inst in insts[inst_index + 1 :]:
        op = inst.get("op")
        if op == "field_get" and inst.get("box") == dst:
            return "field_get_receiver"
        if op == "field_set" and inst.get("box") == dst:
            return "field_set_receiver"
        if op == "mir_call":
            mir_call = inst.get("mir_call")
            if isinstance(mir_call, dict):
                callee = mir_call.get("callee")
                if isinstance(callee, dict) and callee.get("receiver") == dst:
                    return "mir_call_receiver"
    return "unknown"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--policy-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    policy = read_kv(args.policy_report)
    require(
        policy,
        "output_contract",
        "page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0",
    )
    require(policy, "selected_policy", "selected_method_receiver_block_entry_copy_forwarding_guard_surface")
    require(policy, "summary", "ok")

    function = find_function(load_json(args.mir_json))
    blocks = function.get("blocks", [])
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")

    candidates: list[dict[str, Any]] = []
    sink_counts: Counter[str] = Counter()
    for block in blocks:
        if not isinstance(block, dict):
            continue
        block_id = block.get("id")
        insts = [inst for inst in block.get("instructions", []) if isinstance(inst, dict)]
        call_attributed = collect_call_attributed_copy_dsts(insts)
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            if inst.get("src") != 0:
                continue
            if inst.get("dst") in call_attributed:
                continue
            if inst_index > 2:
                continue
            sink = sink_kind(inst.get("dst"), inst_index, insts)
            if sink not in {"field_get_receiver", "field_set_receiver"}:
                continue
            sink_counts[sink] += 1
            candidates.append(
                {
                    "block": block_id,
                    "inst_index": inst_index,
                    "dst": inst.get("dst"),
                    "sink": sink,
                }
            )

    lines = [
        "output_contract=selected-method-receiver-block-entry-copy-forwarding-guard-surface-v0",
        "input_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0",
        f"target_method={METHOD}",
        f"candidate_count={len(candidates)}",
        f"field_get_receiver_candidate_count={sink_counts['field_get_receiver']}",
        f"field_set_receiver_candidate_count={sink_counts['field_set_receiver']}",
        "receiver_source_value=0",
        "candidate_position=block_entry",
        "candidate_scope=selected_method_only",
        "exclude_call_adjacent_receiver_copy=1",
        "exclude_non_receiver_param_copy=1",
        "exclude_cross_block_rewrite=1",
        "implementation_open=0",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, candidate in enumerate(candidates):
        lines.append(f"candidate_{idx}_block=block_{candidate['block']}")
        lines.append(f"candidate_{idx}_inst_index={candidate['inst_index']}")
        lines.append(f"candidate_{idx}_dst={candidate['dst']}")
        lines.append(f"candidate_{idx}_sink={candidate['sink']}")
    lines.extend(
        [
            "selected_next=selected_method_receiver_block_entry_copy_forwarding_implementation",
            "summary=ok",
        ]
    )

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

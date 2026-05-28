#!/usr/bin/env python3
"""Classify copy materialization inside HakoAllocPageModel.acquire_usize/1."""

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


def source_kind(src: Any) -> str:
    if src == 0:
        return "receiver_param"
    if src == 1:
        return "requested_size_param"
    return "derived_value"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-selection-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    selection = read_kv(args.owner_selection_report)
    require(selection, "output_contract", "page-model-hotpath-shape-owner-selection-v0")
    require(selection, "selected_owner", "page_model_acquire_usize_copy_materialization_probe")
    require(selection, "summary", "ok")

    function = find_function(load_json(args.mir_json), METHOD)
    blocks = block_instructions(function)

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    position_counts: Counter[str] = Counter()
    position_source_counts: dict[str, Counter[str]] = defaultdict(Counter)
    block_counts: Counter[Any] = Counter()
    samples: list[dict[str, Any]] = []
    total_copy_count = 0

    for block_id, insts in blocks:
        call_attributed = collect_call_attributed_copy_dsts(insts)
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            total_copy_count += 1
            category = classify_copy(
                inst.get("dst"),
                inst.get("src"),
                inst_index,
                insts,
                phi_dsts,
                call_attributed,
            )
            src_kind = source_kind(inst.get("src"))
            position_counts[category] += 1
            position_source_counts[category][src_kind] += 1
            block_counts[block_id] += 1
            samples.append(
                {
                    "category": category,
                    "source_kind": src_kind,
                    "block_id": block_id,
                    "inst_index": inst_index,
                    "dst": inst.get("dst"),
                    "src": inst.get("src"),
                }
            )

    dominant_position = dominant(position_counts)
    block_entry_receiver_count = position_source_counts["block_entry"]["receiver_param"]
    expression_param_count = (
        position_source_counts["expression_materialization"]["receiver_param"]
        + position_source_counts["expression_materialization"]["requested_size_param"]
    )

    if dominant_position == "block_entry" and block_entry_receiver_count > 0:
        selected_next = "page_model_acquire_usize_block_entry_receiver_copy_policy_selection"
    elif expression_param_count > 0:
        selected_next = "page_model_acquire_usize_param_expression_copy_policy_selection"
    else:
        selected_next = "page_model_acquire_usize_copy_owner_refresh"

    lines = [
        "output_contract=page-model-acquire-usize-copy-materialization-probe-v0",
        "input_contract=page-model-hotpath-shape-owner-selection-v0",
        f"target_method={function.get('name', METHOD)}",
        f"block_count={len(blocks)}",
        f"copy_count={total_copy_count}",
        f"dominant_copy_position={dominant_position}",
        f"block_entry_copy_count={position_counts['block_entry']}",
        f"block_entry_receiver_param_copy_count={block_entry_receiver_count}",
        f"block_entry_requested_size_param_copy_count={position_source_counts['block_entry']['requested_size_param']}",
        f"block_entry_derived_value_copy_count={position_source_counts['block_entry']['derived_value']}",
        f"call_adjacent_copy_count={position_counts['call_adjacent']}",
        f"expression_materialization_copy_count={position_counts['expression_materialization']}",
        f"expression_param_copy_count={expression_param_count}",
        f"branch_condition_copy_count={position_counts['branch_condition']}",
        f"field_set_value_copy_count={position_counts['field_set_value']}",
        f"local_ssa_copy_count={position_counts['local_ssa']}",
        f"phi_edge_copy_count={position_counts['phi_edge']}",
        "recent_broad_local_ssa_nonkeeper_guard=1",
        "implementation_open=0",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (block_id, count) in enumerate(block_counts.most_common(8)):
        lines.append(f"top_block_{idx}_id=block_{block_id}")
        lines.append(f"top_block_{idx}_copy_count={count}")
    for idx, sample in enumerate(samples[:12]):
        prefix = f"sample_{idx}"
        lines.append(f"{prefix}_category={sample['category']}")
        lines.append(f"{prefix}_source_kind={sample['source_kind']}")
        lines.append(f"{prefix}_block=block_{sample['block_id']}")
        lines.append(f"{prefix}_inst_index={sample['inst_index']}")
        lines.append(f"{prefix}_dst={sample['dst']}")
        lines.append(f"{prefix}_src={sample['src']}")
    lines.extend(
        [
            f"selected_next={selected_next}",
            "summary=ok",
        ]
    )

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

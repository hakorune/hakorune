#!/usr/bin/env python3
"""Inventory page-hotpath helper result materialization copy chains."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
PAGE_HOTPATH_HELPERS = {
    "beginSelection",
    "selectSinglePageFastPath",
    "selectPage",
    "acquire",
    "acquire_usize",
    "acquireFreshSmall",
    "reuse",
}


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


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


def value_uses(inst: dict[str, Any]) -> list[Any]:
    op = inst.get("op")
    values: list[Any] = []
    if op == "copy":
        values.append(inst.get("src"))
    elif op == "field_get":
        values.append(inst.get("box"))
    elif op == "field_set":
        values.extend([inst.get("box"), inst.get("value")])
    elif op in {"binop", "compare"}:
        values.extend([inst.get("lhs"), inst.get("rhs")])
    elif op == "branch":
        values.append(inst.get("cond"))
    elif op == "ret":
        values.append(inst.get("value"))
    elif op == "mir_call":
        mir_call = inst.get("mir_call")
        if isinstance(mir_call, dict):
            callee = mir_call.get("callee")
            if isinstance(callee, dict):
                values.append(callee.get("receiver"))
            args = mir_call.get("args", [])
            if isinstance(args, list):
                values.extend(args)
    return [value for value in values if value is not None]


def sink_label(inst: dict[str, Any]) -> str:
    op = str(inst.get("op", "unknown"))
    if op in {"binop", "compare"}:
        operation = str(inst.get("operation", "unknown"))
        operation_name = {
            "==": "eq",
            "!=": "ne",
            "<": "lt",
            ">": "gt",
            "<=": "le",
            ">=": "ge",
            "+": "add",
            "-": "sub",
            "*": "mul",
            "/": "div",
            "%": "mod",
        }.get(operation, safe_key(operation))
        return f"{op}_{operation_name}"
    if op == "field_set":
        return f"field_set:{inst.get('field', 'unknown')}"
    if op == "mir_call":
        return f"mir_call:{callee_name(inst) or 'unknown'}"
    return op


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


def copy_chain_len(seed: Any, dst_to_src: dict[Any, Any]) -> int:
    length = 0
    current = seed
    seen: set[Any] = set()
    while current in dst_to_src and current not in seen:
        seen.add(current)
        length += 1
        current = dst_to_src[current]
    return length


def safe_key(text: str) -> str:
    out: list[str] = []
    for ch in text:
        if ch.isalnum() or ch == "_":
            out.append(ch)
        else:
            out.append("_")
    return "".join(out).strip("_") or "unknown"


def dominant(counts: Counter[str]) -> str:
    if not counts:
        return "none"
    return max(sorted(counts), key=lambda key: counts[key])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-refresh", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=12)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    owner = read_kv(args.owner_refresh)
    require(
        owner,
        "output_contract",
        "hako-mimalloc-post-field-get-alias-keeper-owner-refresh-v0",
        "owner-refresh",
    )
    require(owner, "selected_next_owner", "page_hotpath_helper_result_materialization_copy_chain", "owner-refresh")
    require(owner, "optimization_open", "0", "owner-refresh")

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    helper_call_count = 0
    helper_attributed_copy_count = 0
    result_materialization_copy_count = 0
    helper_counts: Counter[str] = Counter()
    shape_counts: Counter[str] = Counter()
    sink_counts: Counter[str] = Counter()
    samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        copies = [inst for inst in insts if inst.get("op") == "copy"]
        dst_to_src = {inst.get("dst"): inst.get("src") for inst in copies}
        src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        for inst in copies:
            src_to_dsts[inst.get("src")].add(inst.get("dst"))
        for inst in insts:
            for value in value_uses(inst):
                consumers[value].append(inst)

        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "mir_call":
                continue
            name = callee_name(inst)
            if name not in PAGE_HOTPATH_HELPERS:
                continue
            helper_call_count += 1
            call_dst = inst.get("dst")
            descendants = sorted(copy_descendants(call_dst, src_to_dsts), key=lambda value: str(value))
            helper_counts[name] += len(descendants)
            helper_attributed_copy_count += len(descendants)
            result_materialization_copy_count += len(descendants)
            for dst in descendants:
                chain_len = copy_chain_len(dst, dst_to_src)
                sinks = sorted(sink_label(sink) for sink in consumers.get(dst, []) if sink.get("op") != "copy")
                if not sinks:
                    sinks = ["copy_only"]
                shape = f"call_result_copy_chain_len_{chain_len}"
                shape_counts[shape] += 1
                for sink in sinks:
                    sink_counts[sink] += 1
                if len(samples) < max(0, args.topn):
                    samples.append(
                        {
                            "helper": name,
                            "block": block_id,
                            "call_index": inst_index,
                            "call_dst": call_dst,
                            "copy_dst": dst,
                            "copy_src": dst_to_src.get(dst),
                            "chain_len": chain_len,
                            "sink": "+".join(sinks),
                        }
                    )

    selected_owner = "helper_result_chain_shape_unclear"
    confidence = "low"
    next_task = "page_hotpath_helper_result_materialization_repeat_inventory"
    if dominant(helper_counts) != "none" and dominant(shape_counts) != "none":
        selected_owner = "page_hotpath_helper_result_copy_chain_narrowing"
        confidence = "medium"
        next_task = "page_hotpath_helper_result_copy_chain_narrowing_design"

    lines = [
        "output_contract=hako-mimalloc-page-hotpath-helper-result-materialization-inventory-v0",
        "input_contract=hako-mimalloc-post-field-get-alias-keeper-owner-refresh-v0",
        f"target_method={function.get('name', args.method)}",
        f"page_hotpath_helpers_call_count={helper_call_count}",
        f"page_hotpath_helpers_attributed_copy_count={owner.get('page_hotpath_helpers_attributed_copy_count', '0')}",
        f"page_hotpath_helper_result_copy_count={helper_attributed_copy_count}",
        f"owner_refresh_result_materialization_copy_count={owner.get('result_materialization_copy_count', '0')}",
        f"result_materialization_copy_count={result_materialization_copy_count}",
        f"dominant_helper={dominant(helper_counts)}",
        f"dominant_result_chain_shape={dominant(shape_counts)}",
        f"dominant_result_sink={dominant(sink_counts)}",
        f"selected_owner={selected_owner}",
        f"selected_owner_confidence={confidence}",
        f"next_task={next_task}",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for key, count in helper_counts.most_common(8):
        lines.append(f"helper_{safe_key(key)}_result_copy_count={count}")
    for key, count in shape_counts.most_common(8):
        lines.append(f"shape_{safe_key(key)}_count={count}")
    for key, count in sink_counts.most_common(8):
        lines.append(f"sink_{safe_key(key)}_count={count}")
    for idx, sample in enumerate(samples):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_helper={sample['helper']}",
                f"{prefix}_block=block_{sample['block']}",
                f"{prefix}_call_index={sample['call_index']}",
                f"{prefix}_call_dst={sample['call_dst']}",
                f"{prefix}_copy_dst={sample['copy_dst']}",
                f"{prefix}_copy_src={sample['copy_src']}",
                f"{prefix}_chain_len={sample['chain_len']}",
                f"{prefix}_sink={sample['sink']}",
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

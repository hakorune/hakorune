#!/usr/bin/env python3
"""Classify expression-materialization copy origins in one MIR method."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from hako_mimalloc_expression_materialization_copy_origin_analysis import (
    classify_copy,
    collect_call_attributed_copy_dsts,
    dominant,
    origin_label,
    safe_key,
    sink_labels,
    value_uses,
)


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None or text == "":
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--selection", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=10)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    selection = read_kv(args.selection)
    require(
        selection,
        "output_contract",
        "hako-mimalloc-local-ssa-copy-kind-policy-selection-v0",
        "selection",
    )
    require(selection, "selected_copy_kind_policy", "expression_materialization_copy_policy", "selection")
    require(selection, "optimization_open", "0", "selection")
    selected_expression_count = require_int(selection, "expression_materialization_copy_count", "selection")

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    origin_counts: Counter[str] = Counter()
    origin_detail_counts: Counter[str] = Counter()
    sink_counts: Counter[str] = Counter()
    pair_counts: Counter[str] = Counter()
    chain_len_counts: Counter[str] = Counter()
    samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        call_attributed = collect_call_attributed_copy_dsts(insts)
        producers = {inst.get("dst"): inst for inst in insts if inst.get("dst") is not None}
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        for inst in insts:
            for value in value_uses(inst):
                consumers[value].append(inst)

        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            category = classify_copy(
                inst.get("dst"),
                inst.get("src"),
                inst_index,
                insts,
                phi_dsts,
                call_attributed,
            )
            if category != "expression_materialization":
                continue
            origin, detail, chain_len = origin_label(inst.get("src"), producers)
            sinks = sorted(set(sink_labels(inst.get("dst"), consumers)))
            origin_counts[origin] += 1
            origin_detail_counts[detail] += 1
            chain_len_counts[str(chain_len)] += 1
            for sink in sinks:
                sink_counts[sink] += 1
                pair_counts[f"{origin}->{sink}"] += 1
            samples.append(
                {
                    "block": block_id,
                    "inst_index": inst_index,
                    "dst": inst.get("dst"),
                    "src": inst.get("src"),
                    "origin": origin,
                    "origin_detail": detail,
                    "sink": "+".join(sinks),
                    "copy_chain_len": chain_len,
                }
            )

    expression_count = sum(origin_counts.values())
    if expression_count != selected_expression_count:
        raise SystemExit(
            "selection/probe mismatch: "
            f"selection expression count {selected_expression_count}, probe {expression_count}"
        )

    dominant_origin = dominant(origin_counts)
    selected_policy = "field_get_expression_value_copy_chain"
    next_diagnostic = "field_get_expression_copy_chain_policy_selection"
    if dominant_origin != "field_get":
        selected_policy = f"{dominant_origin}_expression_value_copy_chain"
        next_diagnostic = f"{dominant_origin}_expression_copy_chain_policy_selection"

    lines = [
        "output_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0",
        "input_contract=hako-mimalloc-local-ssa-copy-kind-policy-selection-v0",
        f"target_method={function.get('name', args.method)}",
        f"expression_materialization_copy_count={expression_count}",
        f"dominant_expression_origin={dominant_origin}",
        f"field_get_origin_copy_count={origin_counts['field_get']}",
        f"phi_origin_copy_count={origin_counts['phi']}",
        f"mir_call_origin_copy_count={origin_counts['mir_call']}",
        f"param_origin_copy_count={origin_counts['param']}",
        f"dominant_expression_sink={dominant(sink_counts)}",
        f"selected_origin_policy={selected_policy}",
        f"next_diagnostic={next_diagnostic}",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key, count in origin_counts.most_common():
        lines.append(f"origin_{key}_copy_count={count}")
    for key, count in origin_detail_counts.most_common(8):
        lines.append(f"origin_detail_{key}_copy_count={count}")
    for key, count in sink_counts.most_common(8):
        lines.append(f"sink_{safe_key(key)}_copy_count={count}")
    for key, count in pair_counts.most_common(8):
        lines.append(f"pair_{safe_key(key)}_copy_count={count}")
    for key, count in sorted(chain_len_counts.items(), key=lambda item: int(item[0])):
        lines.append(f"origin_copy_chain_len_{key}_count={count}")
    for idx, sample in enumerate(samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_block=block_{sample['block']}",
                f"{prefix}_inst_index={sample['inst_index']}",
                f"{prefix}_dst={sample['dst']}",
                f"{prefix}_src={sample['src']}",
                f"{prefix}_origin={sample['origin']}",
                f"{prefix}_origin_detail={sample['origin_detail']}",
                f"{prefix}_sink={sample['sink']}",
                f"{prefix}_copy_chain_len={sample['copy_chain_len']}",
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

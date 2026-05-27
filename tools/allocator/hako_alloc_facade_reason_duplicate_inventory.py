#!/usr/bin/env python3
"""Inventory source/MIR duplicate facade reason calls for selected methods."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


REASON_PREFIX = "HakoAllocObjectLifecycleFacadeReason."

METHODS = [
    "objectLifecycleSmallAlloc",
    "objectLifecycleRecordAlignmentRequest",
    "objectLifecycleSmallAllocAligned",
    "objectLifecycleReleaseDirectCachedPage",
    "objectLifecycleReleaseBlock",
    "objectLifecycleReallocGrowFromPage",
    "objectLifecycleReallocShrink",
    "objectLifecycleReallocGrow",
]


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def method_body(source: str, method: str) -> str:
    marker = f"{method}("
    start = source.find(marker)
    if start < 0:
        raise SystemExit(f"source method not found: {method}")
    brace = source.find("{", start)
    if brace < 0:
        raise SystemExit(f"source method body not found: {method}")
    depth = 0
    for idx in range(brace, len(source)):
        char = source[idx]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace:idx]
    raise SystemExit(f"unterminated method body: {method}")


def source_reason_count(source: str, method: str) -> int:
    return method_body(source, method).count(REASON_PREFIX)


def find_function(data: dict[str, Any], source_method: str) -> dict[str, Any]:
    symbol = f"HakoAllocObjectLifecycleFacade.{source_method}"
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    matches = [
        fn
        for fn in functions
        if isinstance(fn, dict) and str(fn.get("name", "")).startswith(symbol + "/")
    ]
    if len(matches) != 1:
        raise SystemExit(f"expected one MIR function for {source_method}, found {len(matches)}")
    return matches[0]


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("name", ""))


def used_values(insts: list[dict[str, Any]]) -> set[Any]:
    values: set[Any] = set()
    for inst in insts:
        if "src" in inst:
            values.add(inst["src"])
        mir_call = inst.get("mir_call")
        if isinstance(mir_call, dict):
            args = mir_call.get("args", [])
            if isinstance(args, list):
                values.update(args)
    return values


def mir_reason_counts(function: dict[str, Any]) -> tuple[int, int, int]:
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit(f"selected function missing blocks[]: {function.get('name')}")
    reason_count = 0
    duplicate_count = 0
    unused_duplicate_count = 0
    for block in blocks:
        if not isinstance(block, dict):
            continue
        raw_insts = block.get("instructions", [])
        if not isinstance(raw_insts, list):
            continue
        insts = [inst for inst in raw_insts if isinstance(inst, dict)]
        reason_calls = [
            inst
            for inst in insts
            if inst.get("op") == "mir_call" and callee_name(inst).startswith(REASON_PREFIX)
        ]
        reason_count += len(reason_calls)
        by_name = Counter(callee_name(inst) for inst in reason_calls)
        used = used_values(insts)
        for name, count in by_name.items():
            if count <= 1:
                continue
            duplicate_count += count - 1
            calls_for_name = [inst for inst in reason_calls if callee_name(inst) == name]
            unused_duplicate_count += sum(
                1 for inst in calls_for_name[:-1] if inst.get("dst") not in used
            )
    return reason_count, duplicate_count, unused_duplicate_count


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    data = load_json(args.mir_json)
    source = args.source.read_text(encoding="utf-8")

    lines = [
        "output_contract=hako-alloc-facade-reason-duplicate-inventory-v0",
        "input_contract=hako-mimalloc-post-hako-reason-bind-measurement-v0",
    ]
    total_source = 0
    total_mir = 0
    total_duplicate = 0
    total_unused_duplicate = 0
    failing_methods: list[str] = []

    for idx, method in enumerate(METHODS):
        src_count = source_reason_count(source, method)
        mir_count, duplicate_count, unused_duplicate_count = mir_reason_counts(
            find_function(data, method)
        )
        total_source += src_count
        total_mir += mir_count
        total_duplicate += duplicate_count
        total_unused_duplicate += unused_duplicate_count
        if src_count != mir_count or unused_duplicate_count:
            failing_methods.append(method)
        lines.extend(
            [
                f"method_{idx}={method}",
                f"method_{idx}_source_reason_call_count={src_count}",
                f"method_{idx}_mir_reason_call_count={mir_count}",
                f"method_{idx}_duplicate_reason_call_count={duplicate_count}",
                f"method_{idx}_unused_duplicate_reason_call_count={unused_duplicate_count}",
            ]
        )

    selected_next = (
        "hako_alloc_facade_reason_duplicate_eval_guard"
        if failing_methods
        else "generic_nested_argument_single_eval_guard"
    )
    lines.extend(
        [
            f"method_count={len(METHODS)}",
            f"total_source_reason_call_count={total_source}",
            f"total_mir_reason_call_count={total_mir}",
            f"total_duplicate_reason_call_count={total_duplicate}",
            f"total_unused_duplicate_reason_call_count={total_unused_duplicate}",
            f"failing_method_count={len(failing_methods)}",
            f"failing_methods={','.join(failing_methods)}",
            "selected_owner=mir_nested_argument_single_evaluation",
            f"selected_next={selected_next}",
            "selected_next_kind=mir_diagnostic",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

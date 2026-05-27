#!/usr/bin/env python3
"""Classify duplicate reason calls in objectLifecycleSmallAlloc failure returns."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
DEFAULT_SOURCE_METHOD = "objectLifecycleSmallAlloc"
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
    for fn in functions:
        if isinstance(fn, dict) and fn.get("name") == selected_method:
            return fn
    raise SystemExit(f"selected method not found: {selected_method}")


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("name", ""))


def call_effects(inst: dict[str, Any]) -> list[Any]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return []
    effects = mir_call.get("effects", [])
    return effects if isinstance(effects, list) else []


def block_instructions(function: dict[str, Any]) -> list[list[dict[str, Any]]]:
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    out: list[list[dict[str, Any]]] = []
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if isinstance(insts, list):
            out.append([inst for inst in insts if isinstance(inst, dict)])
    return out


def count_source_reason_calls(path: Path, method_name: str) -> int:
    text = path.read_text(encoding="utf-8")
    marker = f"{method_name}("
    start = text.find(marker)
    if start < 0:
        raise SystemExit(f"source method not found: {method_name}")
    brace = text.find("{", start)
    if brace < 0:
        raise SystemExit(f"source method body not found: {method_name}")
    depth = 0
    end = brace
    for idx in range(brace, len(text)):
        char = text[idx]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                end = idx
                break
    body = text[brace:end]
    return body.count(REASON_PREFIX)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--source-method", default=DEFAULT_SOURCE_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    return_blocks = [
        insts
        for insts in block_instructions(function)
        if any(inst.get("op") == "ret" for inst in insts)
    ]

    reason_call_count = 0
    reason_effect_io_count = 0
    duplicate_reason_call_count = 0
    duplicate_unused_reason_call_count = 0
    failure_return_block_count = 0

    for insts in return_blocks:
        reason_calls = [
            inst
            for inst in insts
            if inst.get("op") == "mir_call" and callee_name(inst).startswith(REASON_PREFIX)
        ]
        if not reason_calls:
            continue
        failure_return_block_count += 1
        reason_call_count += len(reason_calls)
        reason_effect_io_count += sum(1 for inst in reason_calls if "IO" in call_effects(inst))
        name_counts = Counter(callee_name(inst) for inst in reason_calls)
        duplicate_reason_call_count += sum(max(0, count - 1) for count in name_counts.values())

        used_values = set()
        for inst in insts:
            if "src" in inst:
                used_values.add(inst["src"])
            mir_call = inst.get("mir_call")
            if isinstance(mir_call, dict):
                call_arg_values = mir_call.get("args", [])
                if isinstance(call_arg_values, list):
                    used_values.update(call_arg_values)
        for name, count in name_counts.items():
            if count <= 1:
                continue
            calls_for_name = [inst for inst in reason_calls if callee_name(inst) == name]
            duplicate_unused_reason_call_count += sum(
                1 for inst in calls_for_name[:-1] if inst.get("dst") not in used_values
            )

    source_reason_call_count = count_source_reason_calls(args.source, args.source_method)

    if duplicate_unused_reason_call_count == duplicate_reason_call_count and reason_effect_io_count:
        next_action = "hako_reason_bind_probe"
        selected_reason = "nested_reason_call_duplicated_with_unused_first_result_and_io_effect"
    elif duplicate_reason_call_count and reason_effect_io_count == 0:
        next_action = "mir_call_cse_probe"
        selected_reason = "duplicate_pure_reason_calls_can_be_cse_probed"
    elif duplicate_reason_call_count:
        next_action = "reason_singleton_lowering_probe"
        selected_reason = "duplicate_reason_calls_need_lowering_policy_probe"
    else:
        next_action = "stop_line"
        selected_reason = "duplicate_reason_calls_not_observed"

    lines = [
        "output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0",
        "input_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0",
        f"selected_owner={function.get('name', args.method)}",
        f"source_reason_call_count={source_reason_call_count}",
        f"reason_call_count={reason_call_count}",
        f"reason_effect_io_count={reason_effect_io_count}",
        f"duplicate_reason_call_count={duplicate_reason_call_count}",
        f"duplicate_unused_reason_call_count={duplicate_unused_reason_call_count}",
        f"failure_return_block_count={failure_return_block_count}",
        f"selected_reason={selected_reason}",
        f"next_action={next_action}",
        "next_diagnostic=small_alloc_hako_reason_bind_probe",
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

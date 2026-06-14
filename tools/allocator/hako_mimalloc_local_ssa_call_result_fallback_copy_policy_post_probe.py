#!/usr/bin/env python3
"""Post-probe for the LocalSSA call-result fallback Copy keeper."""

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


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        if value.startswith("<") and key in values:
            continue
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
        raise SystemExit(f"{label}: {key} must be integer, got {text!r}") from exc


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
    for function in functions:
        if isinstance(function, dict) and function.get("name") == selected_method:
            return function
    raise SystemExit(f"selected method not found: {selected_method}")


def block_instructions(function: dict[str, Any]) -> list[list[dict[str, Any]]]:
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    out: list[list[dict[str, Any]]] = []
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if not isinstance(insts, list):
            continue
        out.append([inst for inst in insts if isinstance(inst, dict)])
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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--guard-surface", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    guard = read_kv(args.guard_surface)
    require(
        guard,
        "output_contract",
        "hako-mimalloc-local-ssa-call-result-fallback-copy-policy-guard-surface-v0",
        "guard-surface",
    )
    require(guard, "allowed_use_kind", "CompareOperand", "guard-surface")
    require(guard, "arg_forwarding_enabled", "0", "guard-surface")
    require(guard, "helper_name_special_case", "0", "guard-surface")
    require(guard, "optimization_open", "0", "guard-surface")
    pre_candidates = require_int(guard, "pre_candidate_result_copy_count", "guard-surface")
    pre_terminal = require_int(guard, "pre_terminal_compare_operand_count", "guard-surface")
    terminal_target = require_int(guard, "post_terminal_compare_operand_target", "guard-surface")
    candidate_upper = require_int(guard, "post_candidate_result_copy_count_upper_bound", "guard-surface")

    function = find_function(load_json(args.mir_json), args.method)

    post_candidates = 0
    post_terminal = 0
    helper_counts: Counter[str] = Counter()
    for insts in block_instructions(function):
        src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        for inst in insts:
            if inst.get("op") == "copy":
                src_to_dsts[inst.get("src")].add(inst.get("dst"))
            for value in value_uses(inst):
                consumers[value].append(inst)
        for inst in insts:
            if inst.get("op") != "mir_call":
                continue
            helper = callee_name(inst)
            if helper not in PAGE_HOTPATH_HELPERS:
                continue
            descendants = copy_descendants(inst.get("dst"), src_to_dsts)
            post_candidates += len(descendants)
            helper_counts[helper] += len(descendants)
            for dst in descendants:
                non_copy_sinks = [sink for sink in consumers.get(dst, []) if sink.get("op") != "copy"]
                if any(sink.get("op") == "compare" for sink in non_copy_sinks):
                    post_terminal += 1

    if post_terminal != terminal_target:
        raise SystemExit(
            f"post-probe: terminal compare operand count expected {terminal_target}, got {post_terminal}"
        )
    if post_candidates > candidate_upper:
        raise SystemExit(
            f"post-probe: candidate count expected <= {candidate_upper}, got {post_candidates}"
        )

    lines = [
        "output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-implementation-v0",
        f"target_method={function.get('name', args.method)}",
        "source_evidence=296x-680",
        f"pre_candidate_result_copy_count={pre_candidates}",
        f"pre_terminal_compare_operand_count={pre_terminal}",
        f"post_terminal_compare_operand_count={post_terminal}",
        f"post_candidate_result_copy_count={post_candidates}",
        f"post_candidate_result_copy_count_upper_bound={candidate_upper}",
        "allowed_use_kind=CompareOperand",
        "arg_forwarding_enabled=0",
        "helper_name_special_case=0",
        "variable_map_semantics_changed=0",
        "phi_lifecycle_changed=0",
        "implementation_started=1",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for key, count in helper_counts.most_common(8):
        lines.append(f"post_helper_{key}_candidate_count={count}")
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

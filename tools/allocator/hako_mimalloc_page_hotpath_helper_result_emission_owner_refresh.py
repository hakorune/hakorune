#!/usr/bin/env python3
"""Refresh the emission owner for page-hotpath helper result copy chains."""

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


def safe_key(text: str) -> str:
    out: list[str] = []
    for ch in text:
        if ch.isalnum() or ch == "_":
            out.append(ch)
        else:
            out.append("_")
    return "".join(out).strip("_") or "unknown"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--design", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    design = read_kv(args.design)
    require(
        design,
        "output_contract",
        "hako-mimalloc-page-hotpath-helper-result-copy-chain-narrowing-design-v0",
        "design",
    )
    require(
        design,
        "selected_keeper_shape",
        "same_block_call_result_terminal_consumer_rewrite",
        "design",
    )
    require(design, "selected_keeper_owner", "LocalSSA::ensure_call_result_alias_to_consumer", "design")
    require(design, "optimization_open", "0", "design")
    expected_candidates = require_int(design, "candidate_result_copy_count", "design")
    expected_terminal = require_int(design, "terminal_consumer_rewrite_candidate_count", "design")

    function = find_function(load_json(args.mir_json), args.method)

    candidate_count = 0
    first_hop_call_result_copy_count = 0
    chain_internal_copy_count = 0
    terminal_compare_operand_count = 0
    terminal_compare_first_hop_root_count = 0
    helper_counts: Counter[str] = Counter()
    terminal_helper_counts: Counter[str] = Counter()

    for _block_id, insts in block_instructions(function):
        copies = [inst for inst in insts if inst.get("op") == "copy"]
        dst_to_src = {inst.get("dst"): inst.get("src") for inst in copies}
        src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        for inst in copies:
            src_to_dsts[inst.get("src")].add(inst.get("dst"))
        for inst in insts:
            for value in value_uses(inst):
                consumers[value].append(inst)

        for inst in insts:
            if inst.get("op") != "mir_call":
                continue
            helper = callee_name(inst)
            if helper not in PAGE_HOTPATH_HELPERS:
                continue
            call_dst = inst.get("dst")
            descendants = copy_descendants(call_dst, src_to_dsts)
            if not descendants:
                continue
            helper_counts[helper] += len(descendants)
            helper_has_terminal_compare = False
            for dst in descendants:
                candidate_count += 1
                if dst_to_src.get(dst) == call_dst:
                    first_hop_call_result_copy_count += 1
                else:
                    chain_internal_copy_count += 1

                non_copy_sinks = [sink for sink in consumers.get(dst, []) if sink.get("op") != "copy"]
                if any(sink.get("op") == "compare" for sink in non_copy_sinks):
                    terminal_compare_operand_count += 1
                    helper_has_terminal_compare = True
            if helper_has_terminal_compare:
                terminal_compare_first_hop_root_count += 1
                terminal_helper_counts[helper] += 1

    if candidate_count != expected_candidates:
        raise SystemExit(
            f"owner-refresh: candidate count drift expected {expected_candidates}, got {candidate_count}"
        )
    if terminal_compare_operand_count != expected_terminal:
        raise SystemExit(
            "owner-refresh: terminal compare count drift expected "
            f"{expected_terminal}, got {terminal_compare_operand_count}"
        )

    dominant_owner = "emission_owner_unclear"
    selected_owner = "page_hotpath_helper_result_emission_repeat_probe"
    confidence = "low"
    next_task = "page_hotpath_helper_result_emission_repeat_probe"
    if (
        first_hop_call_result_copy_count == terminal_compare_first_hop_root_count
        and chain_internal_copy_count > first_hop_call_result_copy_count
    ):
        dominant_owner = "LocalSSA::ensure_fallback_copy"
        selected_owner = "local_ssa_call_result_fallback_copy_policy"
        confidence = "medium"
        next_task = "local_ssa_call_result_fallback_copy_policy_design"

    lines = [
        "output_contract=hako-mimalloc-page-hotpath-helper-result-emission-owner-refresh-v0",
        f"target_method={function.get('name', args.method)}",
        "source_evidence=296x-677",
        f"candidate_result_copy_count={candidate_count}",
        f"terminal_consumer_rewrite_candidate_count={terminal_compare_operand_count}",
        "local_ssa_terminal_rewrite_owner_rejected=1",
        f"first_hop_call_result_copy_count={first_hop_call_result_copy_count}",
        f"chain_internal_copy_count={chain_internal_copy_count}",
        f"terminal_compare_operand_count={terminal_compare_operand_count}",
        f"terminal_compare_first_hop_root_count={terminal_compare_first_hop_root_count}",
        f"dominant_emission_owner={dominant_owner}",
        f"selected_next_owner={selected_owner}",
        f"selected_owner_confidence={confidence}",
        f"next_task={next_task}",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for key, count in helper_counts.most_common(8):
        lines.append(f"helper_{safe_key(key)}_candidate_count={count}")
    for key, count in terminal_helper_counts.most_common(8):
        lines.append(f"terminal_helper_{safe_key(key)}_count={count}")
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

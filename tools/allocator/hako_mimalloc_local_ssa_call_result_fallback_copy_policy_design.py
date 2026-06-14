#!/usr/bin/env python3
"""Design the LocalSSA fallback Copy policy for helper call-result chains."""

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


def call_root(seed: Any, dst_to_inst: dict[Any, dict[str, Any]], dst_to_src: dict[Any, Any]) -> Any | None:
    current = seed
    seen: set[Any] = set()
    for _ in range(8):
        if current in seen:
            return None
        seen.add(current)
        inst = dst_to_inst.get(current)
        if not isinstance(inst, dict):
            return None
        if inst.get("op") == "mir_call":
            return current
        if inst.get("op") != "copy":
            return None
        current = dst_to_src.get(current)
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-refresh", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    owner = read_kv(args.owner_refresh)
    require(
        owner,
        "output_contract",
        "hako-mimalloc-page-hotpath-helper-result-emission-owner-refresh-v0",
        "owner-refresh",
    )
    require(owner, "dominant_emission_owner", "LocalSSA::ensure_fallback_copy", "owner-refresh")
    require(owner, "selected_next_owner", "local_ssa_call_result_fallback_copy_policy", "owner-refresh")
    require(owner, "optimization_open", "0", "owner-refresh")
    expected_candidates = require_int(owner, "candidate_result_copy_count", "owner-refresh")
    expected_first_hop = require_int(owner, "first_hop_call_result_copy_count", "owner-refresh")
    expected_internal = require_int(owner, "chain_internal_copy_count", "owner-refresh")
    expected_terminal = require_int(owner, "terminal_compare_operand_count", "owner-refresh")

    function = find_function(load_json(args.mir_json), args.method)

    candidate_count = 0
    first_hop_count = 0
    internal_count = 0
    terminal_compare_count = 0
    terminal_compare_covered_count = 0
    uncovered_terminal_count = 0
    residual_first_hop_after_policy = 0
    helper_counts: Counter[str] = Counter()
    covered_helper_counts: Counter[str] = Counter()

    for block_id, insts in block_instructions(function):
        copies = [inst for inst in insts if inst.get("op") == "copy"]
        dst_to_src = {inst.get("dst"): inst.get("src") for inst in copies}
        src_to_dsts: dict[Any, set[Any]] = defaultdict(set)
        dst_to_inst: dict[Any, dict[str, Any]] = {}
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        call_helpers: dict[Any, str] = {}
        for inst in insts:
            if "dst" in inst:
                dst_to_inst[inst.get("dst")] = inst
            if inst.get("op") == "copy":
                src_to_dsts[inst.get("src")].add(inst.get("dst"))
            if inst.get("op") == "mir_call":
                helper = callee_name(inst)
                if helper in PAGE_HOTPATH_HELPERS:
                    call_helpers[inst.get("dst")] = helper
            for value in value_uses(inst):
                consumers[value].append(inst)

        for call_dst, helper in call_helpers.items():
            descendants = copy_descendants(call_dst, src_to_dsts)
            helper_counts[helper] += len(descendants)
            for dst in descendants:
                candidate_count += 1
                if dst_to_src.get(dst) == call_dst:
                    first_hop_count += 1
                    non_copy_sinks = [sink for sink in consumers.get(dst, []) if sink.get("op") != "copy"]
                    if not non_copy_sinks:
                        residual_first_hop_after_policy += 1
                else:
                    internal_count += 1

                non_copy_sinks = [sink for sink in consumers.get(dst, []) if sink.get("op") != "copy"]
                if any(sink.get("op") == "compare" for sink in non_copy_sinks):
                    terminal_compare_count += 1
                    root = call_root(dst, dst_to_inst, dst_to_src)
                    if root == call_dst:
                        terminal_compare_covered_count += 1
                        covered_helper_counts[helper] += 1
                    else:
                        uncovered_terminal_count += 1

    if candidate_count != expected_candidates:
        raise SystemExit(
            f"policy-design: candidate count drift expected {expected_candidates}, got {candidate_count}"
        )
    if first_hop_count != expected_first_hop:
        raise SystemExit(
            f"policy-design: first-hop count drift expected {expected_first_hop}, got {first_hop_count}"
        )
    if internal_count != expected_internal:
        raise SystemExit(
            f"policy-design: internal count drift expected {expected_internal}, got {internal_count}"
        )
    if terminal_compare_count != expected_terminal:
        raise SystemExit(
            f"policy-design: terminal count drift expected {expected_terminal}, got {terminal_compare_count}"
        )

    selected_shape = "repeat_probe"
    confidence = "low"
    next_task = "local_ssa_call_result_fallback_copy_policy_repeat_design"
    if terminal_compare_covered_count == terminal_compare_count and uncovered_terminal_count == 0:
        selected_shape = "same_block_call_result_root_for_compare_operand"
        confidence = "medium"
        next_task = "local_ssa_call_result_fallback_copy_policy_guard_surface"

    post_upper_bound = residual_first_hop_after_policy

    lines = [
        "output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-design-v0",
        f"target_method={function.get('name', args.method)}",
        "source_evidence=296x-678",
        f"candidate_result_copy_count={candidate_count}",
        f"first_hop_call_result_copy_count={first_hop_count}",
        f"chain_internal_copy_count={internal_count}",
        f"terminal_compare_operand_count={terminal_compare_count}",
        f"terminal_compare_covered_by_same_block_call_root_count={terminal_compare_covered_count}",
        f"uncovered_terminal_compare_operand_count={uncovered_terminal_count}",
        f"residual_first_hop_copy_after_policy_count={residual_first_hop_after_policy}",
        f"post_candidate_result_copy_count_upper_bound={post_upper_bound}",
        f"selected_policy_shape={selected_shape}",
        "selected_policy_owner=LocalSSA::ensure_fallback_copy",
        f"selected_owner_confidence={confidence}",
        f"next_task={next_task}",
        "allowed_use_kind=CompareOperand",
        "arg_forwarding_enabled=0",
        "helper_name_special_case=0",
        "variable_map_semantics_changed=0",
        "phi_lifecycle_changed=0",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for key, count in helper_counts.most_common(8):
        lines.append(f"helper_{key}_candidate_count={count}")
    for key, count in covered_helper_counts.most_common(8):
        lines.append(f"covered_helper_{key}_count={count}")
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

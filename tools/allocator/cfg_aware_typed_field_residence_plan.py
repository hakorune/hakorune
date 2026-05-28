#!/usr/bin/env python3
"""Build a conservative CFG-aware typed-field residence plan for one MIR method."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocPageModel.acquire_usize/1"
SCALAR_DECLARED_TYPES = {"i64", "usize", "u64", "isize", "i8", "i16", "i32", "u8", "u16", "u32"}


def load_module(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def functions_by_name(module: dict[str, Any]) -> dict[str, dict[str, Any]]:
    funcs = module.get("functions") or []
    if isinstance(funcs, dict):
        return funcs
    return {fn.get("name", ""): fn for fn in funcs if fn.get("name")}


def is_scalar_declared(declared: Any) -> bool:
    return isinstance(declared, str) and declared in SCALAR_DECLARED_TYPES


def is_handle_declared(declared: Any) -> bool:
    return isinstance(declared, dict) and declared.get("kind") == "handle"


def root_value(value: Any, aliases: dict[int, int]) -> str:
    if not isinstance(value, int):
        return "dynamic"
    seen: set[int] = set()
    cur = value
    while cur in aliases and cur not in seen:
        seen.add(cur)
        cur = aliases[cur]
    return str(cur)


def field_key(inst: dict[str, Any], aliases: dict[int, int]) -> tuple[str, str, str]:
    declared = inst.get("declared_type")
    return (
        root_value(inst.get("box"), aliases),
        str(inst.get("field") or "unknown"),
        str(declared) if isinstance(declared, str) else "non_scalar",
    )


def block_successors(block: dict[str, Any]) -> list[int]:
    instructions = block.get("instructions") or []
    if not instructions:
        return []
    last = instructions[-1]
    op = last.get("op")
    if op == "branch":
        return [int(last["then"]), int(last["else"])]
    if op == "jump":
        return [int(last["target"])]
    return []


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    fn = functions_by_name(module).get(args.method)
    if fn is None:
        raise SystemExit(f"method not found: {args.method}")

    aliases: dict[int, int] = {}
    blocks = fn.get("blocks") or []
    for block in blocks:
        for inst in block.get("instructions") or []:
            if inst.get("op") == "copy" and isinstance(inst.get("dst"), int) and isinstance(inst.get("src"), int):
                aliases[inst["dst"]] = int(root_value(inst["src"], aliases))

    counts: Counter[str] = Counter()
    field_blocks: defaultdict[tuple[str, str, str], set[int]] = defaultdict(set)
    block_dirty: defaultdict[int, set[tuple[str, str, str]]] = defaultdict(set)
    dirty_preds: defaultdict[int, Counter[tuple[str, str, str]]] = defaultdict(Counter)

    for block in blocks:
        block_id = int(block.get("id"))
        seen_in_block: set[tuple[str, str, str]] = set()
        dirty_in_block: set[tuple[str, str, str]] = set()
        for inst in block.get("instructions") or []:
            op = inst.get("op")
            if op not in {"field_get", "field_set"}:
                if op in {"mir_call", "call", "boxcall", "externcall"}:
                    seen_in_block.clear()
                    dirty_in_block.clear()
                continue
            declared = inst.get("declared_type")
            if is_handle_declared(declared):
                counts["rejected_handle_field_count"] += 1
                continue
            if not is_scalar_declared(declared):
                counts["fallback_field_count"] += 1
                continue
            key = field_key(inst, aliases)
            field_blocks[key].add(block_id)
            if op == "field_get":
                counts["scalar_field_get_count"] += 1
                counts["inserted_helper_load_count"] += 1
                if key in seen_in_block:
                    counts["same_block_reused_get_count"] += 1
                    counts["inserted_helper_load_count"] -= 1
                seen_in_block.add(key)
            else:
                counts["scalar_field_set_count"] += 1
                if key in dirty_in_block:
                    counts["coalesced_writeback_count"] += 1
                dirty_in_block.add(key)
                seen_in_block.add(key)
        block_dirty[block_id] = dirty_in_block

    predecessor_count: Counter[int] = Counter()
    for block in blocks:
        block_id = int(block.get("id"))
        for succ in block_successors(block):
            predecessor_count[succ] += 1
            for key in block_dirty[block_id]:
                dirty_preds[succ][key] += 1

    phi_dirty_required = 0
    for succ, field_counts in dirty_preds.items():
        if predecessor_count[succ] <= 1:
            continue
        for _, dirty_count in field_counts.items():
            if 0 < dirty_count < predecessor_count[succ]:
                phi_dirty_required += 1

    eligible_resident_field_count = len(field_blocks)
    erased_get = counts["scalar_field_get_count"]
    erased_set = counts["scalar_field_set_count"]
    inserted_load = counts["inserted_helper_load_count"]
    inserted_writeback = counts["scalar_field_set_count"] - counts["coalesced_writeback_count"]
    net_delta = erased_get + erased_set - inserted_load - inserted_writeback
    positive = net_delta > 0

    lines = [
        "output_contract=cfg-aware-typed-field-residence-plan-v0",
        "input_contract=cfg-aware-typed-field-residence-ssot-v0",
        f"selected_method={args.method}",
        f"block_count={len(blocks)}",
        f"eligible_resident_field_count={eligible_resident_field_count}",
        f"scalar_field_get_count={counts['scalar_field_get_count']}",
        f"scalar_field_set_count={counts['scalar_field_set_count']}",
        f"erased_field_get_count={erased_get}",
        f"erased_field_set_count={erased_set}",
        f"inserted_helper_load_count={inserted_load}",
        f"inserted_helper_writeback_count={inserted_writeback}",
        f"same_block_reused_get_count={counts['same_block_reused_get_count']}",
        f"coalesced_writeback_count={counts['coalesced_writeback_count']}",
        f"net_helper_call_delta={net_delta}",
        f"net_helper_call_delta_positive={1 if positive else 0}",
        f"cross_block_field_count={sum(1 for block_set in field_blocks.values() if len(block_set) > 1)}",
        f"phi_dirty_required_count={phi_dirty_required}",
        "phi_value_required_count=0",
        "flush_before_call_count=0",
        f"flush_before_return_count={inserted_writeback}",
        f"fallback_field_count={counts['fallback_field_count']}",
        f"rejected_handle_field_count={counts['rejected_handle_field_count']}",
        "implementation_recommendation="
        + ("implement_cfg_aware_residence" if positive else "do_not_implement_cfg_aware_residence_for_selected_method"),
        "next_diagnostic="
        + ("selected_method_cfg_residence_keeper" if positive else "large_owner_refresh_after_residence_zero_net"),
        "transform_open=0",
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

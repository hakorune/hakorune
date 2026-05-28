#!/usr/bin/env python3
"""Probe field/copy traffic in HakoAllocPageModel.releaseLocalKnownLive/1."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


TARGET_METHOD = "HakoAllocPageModel.releaseLocalKnownLive/1"


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


def load_function(path: Path, name: str) -> dict[str, Any]:
    data = json.loads(path.read_text(encoding="utf-8"))
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    for fn in functions:
        if isinstance(fn, dict) and fn.get("name") == name:
            return fn
    raise SystemExit(f"method not found: {name}")


def resolve_copy(value: int, copies: dict[int, int]) -> int:
    seen: set[int] = set()
    current = value
    while current in copies and current not in seen:
        seen.add(current)
        current = copies[current]
    return current


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-selection-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    owner = read_kv(args.owner_selection_report)
    require(owner, "output_contract", "page-model-hotpath-shape-owner-selection-v0")
    require(owner, "selected_owner", "page_model_release_known_live_field_traffic_probe")
    require(owner, "selected_owner_method", TARGET_METHOD)
    require(owner, "summary", "ok")

    fn = load_function(args.mir_json, TARGET_METHOD)
    ops: Counter[str] = Counter()
    field_gets: Counter[str] = Counter()
    field_sets: Counter[str] = Counter()
    copies: dict[int, int] = {}
    value_fields: dict[int, str] = {}
    binop_sources: dict[int, tuple[int, str, int]] = {}
    field_get_blocks: dict[int, str] = {}
    field_set_blocks: dict[str, list[tuple[str, int]]] = defaultdict(list)
    array_set_call_count = 0
    receiver_copy_count = 0

    for block in fn.get("blocks", []):
        if not isinstance(block, dict):
            continue
        block_id = str(block.get("id", ""))
        for ins in block.get("instructions", []):
            if not isinstance(ins, dict):
                continue
            op = str(ins.get("op", ""))
            ops[op] += 1
            if op == "copy":
                dst = ins.get("dst")
                src = ins.get("src")
                if isinstance(dst, int) and isinstance(src, int):
                    copies[dst] = src
                    if src == 0:
                        receiver_copy_count += 1
            elif op == "field_get":
                field = str(ins.get("field", ""))
                dst = ins.get("dst")
                field_gets[field] += 1
                if isinstance(dst, int):
                    value_fields[dst] = field
                    field_get_blocks[dst] = block_id
            elif op == "field_set":
                field = str(ins.get("field", ""))
                value = ins.get("value")
                field_sets[field] += 1
                if isinstance(value, int):
                    field_set_blocks[field].append((block_id, value))
            elif op == "binop":
                dst = ins.get("dst")
                lhs = ins.get("lhs")
                rhs = ins.get("rhs")
                operation = str(ins.get("operation", ""))
                if isinstance(dst, int) and isinstance(lhs, int) and isinstance(rhs, int):
                    binop_sources[dst] = (lhs, operation, rhs)
            elif op == "mir_call":
                callee = ins.get("mir_call", {}).get("callee", {})
                if isinstance(callee, dict) and callee.get("box_name") == "ArrayBox" and callee.get("name") == "set":
                    array_set_call_count += 1

    same_block_get_set_count = 0
    rmw_candidate_count = 0
    rmw_multi_use_candidate_count = 0
    rmw_single_use_candidate_count = 0
    set_from_get_fields: list[str] = []
    copy_uses: Counter[int] = Counter(copies.values())
    binop_uses: Counter[int] = Counter()
    for lhs, _operation, rhs in binop_sources.values():
        binop_uses[resolve_copy(lhs, copies)] += 1
        binop_uses[resolve_copy(rhs, copies)] += 1

    for field, sets in field_set_blocks.items():
        for set_block, set_value in sets:
            source = resolve_copy(set_value, copies)
            if source in value_fields and value_fields[source] == field and field_get_blocks.get(source) == set_block:
                same_block_get_set_count += 1
                set_from_get_fields.append(field)
                continue
            binop = binop_sources.get(source)
            if binop is None:
                continue
            lhs, _operation, _rhs = binop
            lhs_source = resolve_copy(lhs, copies)
            if value_fields.get(lhs_source) == field and field_get_blocks.get(lhs_source) == set_block:
                same_block_get_set_count += 1
                rmw_candidate_count += 1
                set_from_get_fields.append(field)
                uses = copy_uses[lhs_source] + binop_uses[lhs_source]
                if uses > 1:
                    rmw_multi_use_candidate_count += 1
                else:
                    rmw_single_use_candidate_count += 1

    array_bridge_field_get_count = field_gets["block_used"] + field_gets["local_free"]
    scalar_counter_field_op_count = (
        field_gets["local_free_top"]
        + field_sets["local_free_top"]
        + field_gets["local_free_count"]
        + field_sets["local_free_count"]
        + field_gets["used"]
        + field_sets["used"]
        + field_gets["retired"]
        + field_sets["retired"]
        + field_gets["retire_count"]
        + field_sets["retire_count"]
    )

    lines = [
        "output_contract=page-model-release-known-live-field-traffic-probe-v0",
        "input_contract=page-model-hotpath-shape-owner-selection-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"target_method={TARGET_METHOD}",
        f"target_method_pct={owner.get('selected_owner_method_pct', '0.00')}",
        f"block_count={len(fn.get('blocks', []))}",
        f"field_get_count={ops['field_get']}",
        f"field_set_count={ops['field_set']}",
        f"field_op_count={ops['field_get'] + ops['field_set']}",
        f"copy_count={ops['copy']}",
        f"call_count={ops['mir_call']}",
        f"branch_count={ops['branch']}",
        f"array_set_call_count={array_set_call_count}",
        f"array_bridge_field_get_count={array_bridge_field_get_count}",
        f"scalar_counter_field_op_count={scalar_counter_field_op_count}",
        f"same_block_get_set_count={same_block_get_set_count}",
        f"rmw_candidate_count={rmw_candidate_count}",
        f"rmw_single_use_candidate_count={rmw_single_use_candidate_count}",
        f"rmw_multi_use_candidate_count={rmw_multi_use_candidate_count}",
        f"receiver_copy_count={receiver_copy_count}",
        "recent_acquire_usize_copy_retry_blocked=1",
        "implementation_open=0",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, field in enumerate(sorted(set(field_gets) | set(field_sets))):
        lines.append(f"field_{idx}_name={field}")
        lines.append(f"field_{idx}_get_count={field_gets[field]}")
        lines.append(f"field_{idx}_set_count={field_sets[field]}")
    for idx, field in enumerate(set_from_get_fields):
        lines.append(f"same_block_get_set_field_{idx}={field}")
    lines.extend(
        [
            "selected_next=page_model_release_known_live_owner_selection",
            "summary=ok",
        ]
    )

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

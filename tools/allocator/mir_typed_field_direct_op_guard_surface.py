#!/usr/bin/env python3
"""Freeze the selected-method typed-field direct-op guard surface."""

from __future__ import annotations

import argparse
from collections import Counter
from pathlib import Path
from typing import Any

from mir_typed_field_direct_op_net_inventory import (
    EXACT_HELPER_SYMBOLS,
    analyze_method,
    functions_by_name,
    load_module,
    resolve_box_type,
    storage_bucket,
    typed_plans,
)


DEFAULT_METHOD = "HakoAllocPageModel.acquire_usize/1"


def build_const_values(fn: dict[str, Any]) -> dict[int, int]:
    values: dict[int, int] = {}
    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            if inst.get("op") != "const" or "dst" not in inst:
                continue
            value = inst.get("value")
            if isinstance(value, dict):
                raw_value = value.get("value")
            else:
                raw_value = value
            try:
                values[int(inst["dst"])] = int(raw_value)
            except (TypeError, ValueError):
                continue
    return values


def build_copy_sources(fn: dict[str, Any]) -> dict[int, int]:
    sources: dict[int, int] = {}
    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            if inst.get("op") == "copy" and "dst" in inst and "src" in inst:
                sources[int(inst["dst"])] = int(inst["src"])
    return sources


def build_value_types(fn: dict[str, Any]) -> dict[int, Any]:
    raw = (fn.get("metadata") or {}).get("value_types") or {}
    value_types: dict[int, Any] = {}
    for key, value in raw.items():
        try:
            value_types[int(key)] = value
        except (TypeError, ValueError):
            continue
    return value_types


def resolve_const(value: Any, const_values: dict[int, int], copies: dict[int, int]) -> int | None:
    try:
        current = int(value)
    except (TypeError, ValueError):
        return None

    seen: set[int] = set()
    while current not in seen:
        seen.add(current)
        if current in const_values:
            return const_values[current]
        if current not in copies:
            return None
        current = copies[current]
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    fn = functions_by_name(module).get(args.method)
    if fn is None:
        raise SystemExit(f"method not found: {args.method}")

    plans = typed_plans(module)
    counts = analyze_method(fn, plans)
    copies = build_copy_sources(fn)
    value_types = build_value_types(fn)
    const_values = build_const_values(fn)
    details: list[tuple[int, int, str, str, str, str]] = []
    symbols: Counter[str] = Counter()
    fields: Counter[str] = Counter()
    guards: Counter[str] = Counter()

    for block in fn.get("blocks") or []:
        block_id = int(block.get("id") or 0)
        for index, inst in enumerate(block.get("instructions") or []):
            op = inst.get("op")
            if op not in {"field_get", "field_set"}:
                continue
            box_name = resolve_box_type(inst.get("box"), value_types, copies)
            field = str(inst.get("field") or "")
            field_plan = plans.get(box_name or "", {}).get(field)
            if box_name is None or field_plan is None or bool(field_plan.get("weak")):
                continue
            storage = storage_bucket(str(field_plan.get("storage") or "unknown"))
            if storage == "unsupported":
                continue
            symbol = EXACT_HELPER_SYMBOLS[(op, storage)]
            symbols[symbol] += 1
            fields[f"{box_name}.{field}.{storage}"] += 1
            details.append((block_id, index, op, f"{box_name}.{field}", storage, symbol))
            if op == "field_set":
                guards["set_status_trap_count"] += 1
                if storage in {"usize", "u64"}:
                    value_const = resolve_const(inst.get("value"), const_values, copies)
                    if value_const is None:
                        guards["unsigned_set_nonnegative_guard_count"] += 1
                    elif value_const >= 0:
                        guards["unsigned_set_const_nonnegative_count"] += 1
                    else:
                        guards["unsigned_set_const_negative_reject_count"] += 1

    print("output_contract=mir-typed-field-direct-op-guard-surface-v0")
    print("input_contract=mir-typed-field-direct-op-net-inventory-v0")
    print("workload_id=representative-object-lifecycle-small-block-v0")
    print(f"selected_method={args.method}")
    print(f"candidate_field_get_count={counts['eligible_field_get_count']}")
    print(f"candidate_field_set_count={counts['eligible_field_set_count']}")
    print(f"candidate_total={counts['eligible_total']}")
    print(f"projected_net_helper_call_delta={counts['net_helper_call_delta']}")
    print(f"candidate_i64_count={counts['eligible_i64_count']}")
    print(f"candidate_u64_count={counts['eligible_u64_count']}")
    print(f"candidate_usize_count={counts['eligible_usize_count']}")
    print(f"candidate_handle_count={counts['eligible_handle_count']}")
    print(f"unsigned_set_nonnegative_guard_count={guards['unsigned_set_nonnegative_guard_count']}")
    print(f"unsigned_set_const_nonnegative_count={guards['unsigned_set_const_nonnegative_count']}")
    print(f"unsigned_set_const_negative_reject_count={guards['unsigned_set_const_negative_reject_count']}")
    print(f"set_status_trap_count={guards['set_status_trap_count']}")
    print("helper_free_direct_op_required=1")
    print("slot_constant_required=1")
    print("typed_object_plan_required=1")
    print("weak_field_rejected=1")
    print("unsupported_storage_rejected=1")
    print("fallback_silent_success=0")
    print("residence_transform_open=0")
    print("direct_op_transform_open=0")
    print("implementation_open=0")
    for idx, (symbol, count) in enumerate(sorted(symbols.items())):
        print(f"projected_symbol_{idx}={symbol}")
        print(f"projected_symbol_{idx}_count={count}")
    for idx, (field, count) in enumerate(fields.most_common(10)):
        print(f"candidate_field_{idx}={field}")
        print(f"candidate_field_{idx}_count={count}")
    for idx, (block_id, inst_idx, op, field, storage, symbol) in enumerate(details):
        print(f"candidate_{idx}_block={block_id}")
        print(f"candidate_{idx}_instruction_index={inst_idx}")
        print(f"candidate_{idx}_op={op}")
        print(f"candidate_{idx}_field={field}")
        print(f"candidate_{idx}_storage={storage}")
        print(f"candidate_{idx}_projected_symbol={symbol}")
    print("selected_next=mir_typed_field_direct_op_selected_method_keeper")
    print("by_name_special_case=0")
    print("winner_claim=0")
    print("replacement_active=0")
    print("hook_installed=0")
    print("global_allocator=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

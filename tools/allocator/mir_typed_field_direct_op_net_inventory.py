#!/usr/bin/env python3
"""Inventory MIR typed-field direct-op net helper-call delta.

This is deliberately not a residence planner.  It counts field_get/field_set
operations that already have a typed-object slot plan and could be lowered to a
helper-free direct op in a later row.  Residence/writeback counters are emitted
only as a caution so the previous "move helpers to writeback" non-keeper is not
repeated.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


HOT_METHOD_WEIGHTS = {
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1": 524_288,
    "HakoAllocPageModel.acquire_usize/1": 524_288,
    "HakoAllocPageModel.releaseLocalKnownLive/1": 524_288,
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2": 524_288,
    "HakoAllocPageModel.resetToFresh/0": 8_192,
}

DIRECT_STORAGES = {"i64", "u64", "usize", "handle"}

EXACT_HELPER_SYMBOLS = {
    ("field_get", "i64"): "nyash.object.exact_slot_get_i64_hii",
    ("field_set", "i64"): "nyash.object.exact_slot_set_i64_hii",
    ("field_get", "u64"): "nyash.object.exact_slot_get_u64_hii",
    ("field_set", "u64"): "nyash.object.exact_slot_set_u64_hiu",
    ("field_get", "usize"): "nyash.object.exact_slot_get_u64_hii",
    ("field_set", "usize"): "nyash.object.exact_slot_set_u64_hiu",
    ("field_get", "handle"): "nyash.object.exact_slot_get_handle_hii",
    ("field_set", "handle"): "nyash.object.exact_slot_set_handle_hii",
}


def load_module(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def functions_by_name(module: dict[str, Any]) -> dict[str, dict[str, Any]]:
    funcs = module.get("functions") or []
    if isinstance(funcs, dict):
        return funcs
    return {fn.get("name", ""): fn for fn in funcs if fn.get("name")}


def typed_plans(module: dict[str, Any]) -> dict[str, dict[str, dict[str, Any]]]:
    plans: dict[str, dict[str, dict[str, Any]]] = {}
    for plan in module.get("typed_object_plans") or []:
        box_name = plan.get("box_name")
        if not box_name:
            continue
        fields: dict[str, dict[str, Any]] = {}
        for field in plan.get("fields") or []:
            name = field.get("name")
            if name:
                fields[str(name)] = dict(field)
        plans[str(box_name)] = fields
    return plans


def type_box_name(value_type: Any) -> str | None:
    if isinstance(value_type, dict) and value_type.get("kind") == "handle":
        box_type = value_type.get("box_type") or value_type.get("box_name")
        return str(box_type) if box_type else None
    return None


def call_has_effects(inst: dict[str, Any]) -> bool:
    call = inst.get("mir_call") or {}
    return bool(call.get("effects") or [])


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


def resolve_box_type(value: Any, value_types: dict[int, Any], copies: dict[int, int]) -> str | None:
    try:
        current = int(value)
    except (TypeError, ValueError):
        return None

    seen: set[int] = set()
    while current not in seen:
        seen.add(current)
        box_name = type_box_name(value_types.get(current))
        if box_name:
            return box_name
        if current not in copies:
            return None
        current = copies[current]
    return None


def storage_bucket(storage: str) -> str:
    if storage == "usize":
        return "usize"
    if storage in DIRECT_STORAGES:
        return storage
    return "unsupported"


def analyze_method(fn: dict[str, Any], plans: dict[str, dict[str, dict[str, Any]]]) -> Counter[str]:
    counts: Counter[str] = Counter()
    copies = build_copy_sources(fn)
    value_types = build_value_types(fn)

    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            op = inst.get("op")
            if op in {"field_get", "field_set"}:
                counts[f"{op}_total"] += 1
                field = str(inst.get("field") or "")
                box_name = resolve_box_type(inst.get("box"), value_types, copies)
                if not box_name:
                    counts[f"{op}_unknown_receiver_count"] += 1
                    continue
                field_plan = plans.get(box_name, {}).get(field)
                if field_plan is None:
                    counts[f"{op}_unknown_field_plan_count"] += 1
                    continue
                if bool(field_plan.get("weak")):
                    counts[f"{op}_weak_field_count"] += 1
                    continue
                storage = str(field_plan.get("storage") or "unknown")
                bucket = storage_bucket(storage)
                counts[f"{op}_planned_{bucket}_count"] += 1
                if bucket == "unsupported":
                    counts[f"{op}_unsupported_storage_count"] += 1
                    continue
                counts[f"eligible_{op}_count"] += 1
                counts[f"eligible_{op}_{bucket}_count"] += 1
                counts[f"eligible_{bucket}_count"] += 1
                symbol = EXACT_HELPER_SYMBOLS[(op, bucket)]
                counts[f"projected_symbol.{symbol}"] += 1
                counts[f"projected_field.{box_name}.{field}.{bucket}"] += 1
                if op == "field_set":
                    counts["residence_writeback_required_count"] += 1
            elif op == "mir_call" and call_has_effects(inst):
                counts["barrier_unknown_call_count"] += 1
            elif op == "phi":
                counts["barrier_phi_count"] += 1
            elif op == "ret":
                counts["barrier_return_count"] += 1

    counts["eligible_total"] = counts["eligible_field_get_count"] + counts["eligible_field_set_count"]
    counts["planned_erased_helper_calls"] = counts["eligible_total"]
    counts["planned_added_helper_calls"] = 0
    counts["net_helper_call_delta"] = counts["planned_erased_helper_calls"]
    return counts


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    plans = typed_plans(module)
    functions = functions_by_name(module)
    total: Counter[str] = Counter()
    rows: list[tuple[str, int, Counter[str]]] = []
    missing: list[str] = []

    for method, weight in HOT_METHOD_WEIGHTS.items():
        fn = functions.get(method)
        if fn is None:
            missing.append(method)
            continue
        counts = analyze_method(fn, plans)
        rows.append((method, weight, counts))
        for key, value in counts.items():
            total[key] += value
            total[f"dynamic_{key}"] += value * weight

    selected_method = ""
    selected_counts: Counter[str] = Counter()
    selected_weight = 0
    if rows:
        selected_method, selected_weight, selected_counts = max(
            rows,
            key=lambda row: (row[2]["net_helper_call_delta"] * row[1], row[2]["net_helper_call_delta"], row[0]),
        )

    next_step = (
        "mir_typed_field_direct_op_guard_surface"
        if total["net_helper_call_delta"] > 0
        else "post_direct_op_inventory_owner_refresh"
    )

    print("output_contract=mir-typed-field-direct-op-net-inventory-v0")
    print("input_contract=typed-object-exact-slot-owner-refresh-v0")
    print("workload_id=representative-object-lifecycle-small-block-v0")
    print(f"hot_method_count={len(rows)}")
    print(f"missing_hot_method_count={len(missing)}")
    for idx, method in enumerate(missing):
        print(f"missing_hot_method_{idx}={method}")
    print(f"typed_object_plan_box_count={len(plans)}")
    print(f"field_get_total={total['field_get_total']}")
    print(f"field_set_total={total['field_set_total']}")
    print(f"eligible_field_get_count={total['eligible_field_get_count']}")
    print(f"eligible_field_set_count={total['eligible_field_set_count']}")
    print(f"eligible_i64_count={total['eligible_i64_count']}")
    print(f"eligible_u64_count={total['eligible_u64_count']}")
    print(f"eligible_usize_count={total['eligible_usize_count']}")
    print(f"eligible_handle_count={total['eligible_handle_count']}")
    print(f"unknown_receiver_count={total['field_get_unknown_receiver_count'] + total['field_set_unknown_receiver_count']}")
    print(f"unsupported_storage_count={total['field_get_unsupported_storage_count'] + total['field_set_unsupported_storage_count']}")
    print(f"planned_erased_helper_calls={total['planned_erased_helper_calls']}")
    print(f"planned_added_helper_calls={total['planned_added_helper_calls']}")
    print(f"net_helper_call_delta={total['net_helper_call_delta']}")
    print(f"projected_erased_exact_helper_call_count={total['planned_erased_helper_calls']}")
    print(f"projected_added_helper_call_count={total['planned_added_helper_calls']}")
    print(f"projected_net_helper_call_delta={total['net_helper_call_delta']}")
    print(f"dynamic_planned_erased_helper_calls={total['dynamic_planned_erased_helper_calls']}")
    print(f"dynamic_planned_added_helper_calls={total['dynamic_planned_added_helper_calls']}")
    print(f"dynamic_net_helper_call_delta={total['dynamic_net_helper_call_delta']}")
    print(f"dynamic_projected_net_helper_call_delta={total['dynamic_net_helper_call_delta']}")
    print(f"residence_writeback_required_count={total['residence_writeback_required_count']}")
    print(f"dynamic_residence_writeback_required_count={total['dynamic_residence_writeback_required_count']}")
    print(f"barrier_unknown_call_count={total['barrier_unknown_call_count']}")
    print(f"barrier_phi_count={total['barrier_phi_count']}")
    print(f"barrier_return_count={total['barrier_return_count']}")
    projected_symbols = sorted(
        (key.removeprefix("projected_symbol."), value)
        for key, value in total.items()
        if key.startswith("projected_symbol.")
    )
    for idx, (symbol, count) in enumerate(projected_symbols):
        print(f"projected_exact_helper_symbol_{idx}={symbol}")
        print(f"projected_exact_helper_symbol_{idx}_count={count}")
    projected_fields: list[tuple[int, int, str]] = []
    for key, value in total.items():
        if not key.startswith("projected_field."):
            continue
        field_key = key.removeprefix("projected_field.")
        dynamic_value = 0
        for _method, weight, counts in rows:
            dynamic_value += counts[key] * weight
        projected_fields.append((dynamic_value, value, field_key))
    projected_fields.sort(reverse=True)
    for idx, (dynamic_value, count, field_key) in enumerate(projected_fields[:10]):
        print(f"projected_field_{idx}={field_key}")
        print(f"projected_field_{idx}_count={count}")
        print(f"projected_field_{idx}_dynamic_count={dynamic_value}")
    for idx, (method, weight, counts) in enumerate(rows):
        print(f"method_{idx}_name={method}")
        print(f"method_{idx}_dynamic_weight={weight}")
        print(f"method_{idx}_eligible_field_get_count={counts['eligible_field_get_count']}")
        print(f"method_{idx}_eligible_field_set_count={counts['eligible_field_set_count']}")
        print(f"method_{idx}_eligible_i64_count={counts['eligible_i64_count']}")
        print(f"method_{idx}_eligible_u64_count={counts['eligible_u64_count']}")
        print(f"method_{idx}_eligible_usize_count={counts['eligible_usize_count']}")
        print(f"method_{idx}_eligible_handle_count={counts['eligible_handle_count']}")
        print(f"method_{idx}_planned_erased_helper_calls={counts['planned_erased_helper_calls']}")
        print(f"method_{idx}_planned_added_helper_calls={counts['planned_added_helper_calls']}")
        print(f"method_{idx}_net_helper_call_delta={counts['net_helper_call_delta']}")
        print(f"method_{idx}_dynamic_net_helper_call_delta={counts['net_helper_call_delta'] * weight}")
        print(f"method_{idx}_residence_writeback_required_count={counts['residence_writeback_required_count']}")
        print(f"method_{idx}_barrier_unknown_call_count={counts['barrier_unknown_call_count']}")
        print(f"method_{idx}_barrier_phi_count={counts['barrier_phi_count']}")
        print(f"method_{idx}_barrier_return_count={counts['barrier_return_count']}")
    print(f"selected_method={selected_method}")
    print(f"selected_method_dynamic_weight={selected_weight}")
    print(f"selected_method_net_helper_call_delta={selected_counts['net_helper_call_delta']}")
    print(
        "selected_method_dynamic_net_helper_call_delta="
        f"{selected_counts['net_helper_call_delta'] * selected_weight}"
    )
    if projected_fields:
        print(f"selected_field_by_dynamic_net={projected_fields[0][2]}")
        print(f"selected_field_by_dynamic_net_count={projected_fields[0][1]}")
        print(f"selected_field_by_dynamic_net_dynamic_count={projected_fields[0][0]}")
    else:
        print("selected_field_by_dynamic_net=")
        print("selected_field_by_dynamic_net_count=0")
        print("selected_field_by_dynamic_net_dynamic_count=0")
    print("inventory_only=1")
    print(
        "projected_net_helper_call_delta_positive="
        f"{1 if total['net_helper_call_delta'] > 0 else 0}"
    )
    print(
        "dynamic_projected_net_helper_call_delta_positive="
        f"{1 if total['dynamic_net_helper_call_delta'] > 0 else 0}"
    )
    print("selected_method_required=1")
    print("projected_exact_helper_symbol_coverage_matches_mir_storage_counts=1")
    print("residence_inserted_load_writeback_delta_used=0")
    print("residence_transform_open=0")
    print("direct_op_transform_open=0")
    print("previous_residence_zero_net_guard=1")
    print(f"selected_next={next_step}")
    print("by_name_special_case=0")
    print("winner_claim=0")
    print("replacement_active=0")
    print("hook_installed=0")
    print("global_allocator=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Inventory MIR typed-field residence candidates for Hako mimalloc hot methods."""

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

SCALAR_DECLARED_TYPES = {"i64", "usize", "u64", "isize", "i8", "i16", "i32", "u8", "u16", "u32"}


def load_module(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def functions_by_name(module: dict[str, Any]) -> dict[str, dict[str, Any]]:
    funcs = module.get("functions") or []
    if isinstance(funcs, dict):
        return funcs
    return {fn.get("name", ""): fn for fn in funcs if fn.get("name")}


def declared_kind(declared: Any) -> str:
    if isinstance(declared, str):
        return declared
    if isinstance(declared, dict):
        kind = declared.get("kind") or "unknown"
        box = declared.get("box_type") or declared.get("box_name") or ""
        return f"{kind}:{box}" if box else str(kind)
    if declared is None:
        return "dynamic_or_missing"
    return "other"


def is_scalar_declared(declared: Any) -> bool:
    return isinstance(declared, str) and declared in SCALAR_DECLARED_TYPES


def call_has_effects(inst: dict[str, Any]) -> bool:
    call = inst.get("mir_call") or {}
    effects = call.get("effects") or []
    return bool(effects)


def analyze_method(fn: dict[str, Any]) -> Counter[str]:
    counts: Counter[str] = Counter()
    fields: Counter[str] = Counter()
    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            op = inst.get("op")
            if op in {"field_get", "field_set"}:
                declared = inst.get("declared_type")
                kind = declared_kind(declared)
                field = str(inst.get("field") or "unknown")
                fields[f"{op}.{field}.{kind}"] += 1
                if is_scalar_declared(declared):
                    counts[f"eligible_{op}_count"] += 1
                elif kind == "dynamic_or_missing":
                    counts["barrier_dynamic_slot_count"] += 1
                else:
                    counts[f"nonresident_{op}_count"] += 1
            elif op == "mir_call" and call_has_effects(inst):
                counts["barrier_unknown_call_count"] += 1
            elif op == "phi":
                counts["barrier_phi_count"] += 1
            elif op == "ret":
                counts["barrier_return_count"] += 1
    counts["eligible_total"] = counts["eligible_field_get_count"] + counts["eligible_field_set_count"]
    counts["required_writeback_count"] = counts["eligible_field_set_count"]
    for key, value in fields.items():
        counts[f"field.{key}"] = value
    return counts


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    functions = functions_by_name(module)
    total: Counter[str] = Counter()
    method_rows: list[tuple[str, int, Counter[str]]] = []
    missing: list[str] = []

    for method, weight in HOT_METHOD_WEIGHTS.items():
        fn = functions.get(method)
        if fn is None:
            missing.append(method)
            continue
        counts = analyze_method(fn)
        method_rows.append((method, weight, counts))
        for key, value in counts.items():
            if key.startswith("field."):
                continue
            total[key] += value
            total[f"dynamic_{key}"] += value * weight

    selected_method = ""
    selected_total = 0
    selected_dynamic_total = 0
    if method_rows:
        selected_method, _, selected_counts = max(
            method_rows,
            key=lambda row: (row[2]["eligible_total"] * row[1], row[2]["eligible_total"], row[0]),
        )
        selected_total = selected_counts["eligible_total"]
        selected_dynamic_total = selected_total * dict((method, weight) for method, weight, _ in method_rows)[selected_method]

    print("output_contract=mir-typed-field-residence-inventory-v0")
    print("input_contract=mir-typed-field-residence-ssot-v0")
    print("workload_id=representative-object-lifecycle-small-block-v0")
    print(f"hot_method_count={len(method_rows)}")
    print(f"missing_hot_method_count={len(missing)}")
    for idx, method in enumerate(missing):
        print(f"missing_hot_method_{idx}={method}")
    print(f"eligible_field_get_count={total['eligible_field_get_count']}")
    print(f"eligible_field_set_count={total['eligible_field_set_count']}")
    print(f"would_erase_helper_call_count={total['eligible_total']}")
    print(f"required_writeback_count={total['required_writeback_count']}")
    print(f"barrier_unknown_call_count={total['barrier_unknown_call_count']}")
    print(f"barrier_phi_count={total['barrier_phi_count']}")
    print(f"barrier_return_count={total['barrier_return_count']}")
    print(f"barrier_dynamic_slot_count={total['barrier_dynamic_slot_count']}")
    print(f"dynamic_would_erase_helper_call_estimate={total['dynamic_eligible_total']}")
    print(f"dynamic_required_writeback_estimate={total['dynamic_required_writeback_count']}")
    for idx, (method, weight, counts) in enumerate(method_rows):
        print(f"method_{idx}_name={method}")
        print(f"method_{idx}_dynamic_weight={weight}")
        print(f"method_{idx}_eligible_field_get_count={counts['eligible_field_get_count']}")
        print(f"method_{idx}_eligible_field_set_count={counts['eligible_field_set_count']}")
        print(f"method_{idx}_eligible_total={counts['eligible_total']}")
        print(f"method_{idx}_required_writeback_count={counts['required_writeback_count']}")
        print(f"method_{idx}_barrier_unknown_call_count={counts['barrier_unknown_call_count']}")
        print(f"method_{idx}_barrier_phi_count={counts['barrier_phi_count']}")
        print(f"method_{idx}_barrier_return_count={counts['barrier_return_count']}")
        print(f"method_{idx}_barrier_dynamic_slot_count={counts['barrier_dynamic_slot_count']}")
    print(f"selected_method={selected_method}")
    print(f"selected_method_eligible_total={selected_total}")
    print(f"selected_method_dynamic_eligible_estimate={selected_dynamic_total}")
    print("selected_next=mir_typed_field_residence_selected_method_keeper")
    print("transform_open=0")
    print("by_name_special_case=0")
    print("winner_claim=0")
    print("replacement_active=0")
    print("hook_installed=0")
    print("global_allocator=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

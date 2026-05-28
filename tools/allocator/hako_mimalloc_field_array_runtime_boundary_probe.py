#!/usr/bin/env python3
"""Classify field/Array runtime boundaries for the Hako mimalloc hot workload."""

from __future__ import annotations

import argparse
import json
import re
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

PERF_SYMBOL_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+\S+\s+\[.\]\s+(.+?)\s*$")


def load_module(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def functions_by_name(module: dict[str, Any]) -> dict[str, dict[str, Any]]:
    funcs = module.get("functions") or []
    if isinstance(funcs, dict):
        return funcs
    return {fn.get("name", ""): fn for fn in funcs if fn.get("name")}


def declared_kind(declared: Any) -> str:
    if declared == "i64":
        return "i64_scalar"
    if declared == "usize":
        return "usize_scalar"
    if declared == "u64":
        return "u64_scalar"
    if isinstance(declared, dict):
        box = declared.get("box_type") or declared.get("box_name")
        kind = declared.get("kind")
        if box == "ArrayBox" and kind == "handle":
            return "arraybox_handle"
        if kind == "handle":
            return "typed_object_handle"
    if declared is None:
        return "unknown_declared_type"
    return "other_declared_type"


def callee_name(inst: dict[str, Any]) -> str:
    call = inst.get("mir_call") or {}
    callee = call.get("callee") or {}
    ctype = callee.get("type") or ""
    if ctype == "Method":
        return (
            f"{callee.get('box_name') or callee.get('box_type')}."
            f"{callee.get('method') or callee.get('name')}"
        )
    if ctype == "Global":
        return str(callee.get("name") or "")
    if ctype == "Extern":
        return str(callee.get("name") or "")
    return ctype


def parse_perf_report(path: Path | None) -> tuple[float, float, list[tuple[float, str]]]:
    if path is None or not path.exists():
        return 0.0, 0.0, []
    field_pct = 0.0
    array_pct = 0.0
    rows: list[tuple[float, str]] = []
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        m = PERF_SYMBOL_RE.match(raw)
        if not m:
            continue
        pct = float(m.group(1))
        symbol = m.group(2)
        rows.append((pct, symbol))
        if "nyash.object.field_" in symbol:
            field_pct += pct
        if "array_runtime" in symbol or "array_slot" in symbol or "array_handle_cache" in symbol:
            array_pct += pct
    rows.sort(key=lambda row: (-row[0], row[1]))
    return field_pct, array_pct, rows[:8]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mir-json", required=True)
    parser.add_argument("--perf-report")
    args = parser.parse_args()

    module = load_module(Path(args.mir_json))
    functions = functions_by_name(module)
    field_counts: Counter[str] = Counter()
    field_static_total = 0
    field_dynamic_total = 0
    array_method_static_total = 0
    array_method_dynamic_total = 0
    missing: list[str] = []

    for method, weight in HOT_METHOD_WEIGHTS.items():
        fn = functions.get(method)
        if fn is None:
            missing.append(method)
            continue
        method_field_count = 0
        method_array_count = 0
        for block in fn.get("blocks") or []:
            for inst in block.get("instructions") or []:
                op = inst.get("op")
                if op in {"field_get", "field_set"}:
                    kind = declared_kind(inst.get("declared_type"))
                    field_counts[f"{op}.{kind}"] += 1
                    field_counts[f"{op}.total"] += 1
                    method_field_count += 1
                if op == "mir_call" and callee_name(inst) in {"ArrayBox.get", "ArrayBox.set", "RuntimeDataBox.set"}:
                    field_counts[f"call.{callee_name(inst)}"] += 1
                    method_array_count += 1
        field_static_total += method_field_count
        field_dynamic_total += method_field_count * weight
        array_method_static_total += method_array_count
        array_method_dynamic_total += method_array_count * weight
        field_counts[f"method.{method}.field_static"] = method_field_count
        field_counts[f"method.{method}.field_dynamic"] = method_field_count * weight
        field_counts[f"method.{method}.array_method_static"] = method_array_count
        field_counts[f"method.{method}.array_method_dynamic"] = method_array_count * weight

    perf_report = Path(args.perf_report) if args.perf_report else None
    perf_field_pct, perf_array_pct, top_rows = parse_perf_report(perf_report)

    print("output_contract=hako-mimalloc-field-array-runtime-boundary-probe-v0")
    print("input_contract=object-lifecycle-large-owner-reality-check-v0")
    print("workload_id=representative-object-lifecycle-small-block-v0")
    print("operation_count_alloc=524288")
    print("operation_count_free=524288")
    print(f"hot_method_count={len(HOT_METHOD_WEIGHTS) - len(missing)}")
    print(f"missing_hot_method_count={len(missing)}")
    for idx, method in enumerate(missing):
        print(f"missing_hot_method_{idx}={method}")
    print(f"field_static_total={field_static_total}")
    print(f"field_dynamic_estimate={field_dynamic_total}")
    print(f"array_method_static_total={array_method_static_total}")
    print(f"array_method_dynamic_estimate={array_method_dynamic_total}")
    for key in sorted(field_counts):
        print(f"{key}={field_counts[key]}")
    if perf_report is not None:
        print("perf_report_available=1")
        print(f"perf_field_helper_pct={perf_field_pct:.2f}")
        print(f"perf_array_helper_pct={perf_array_pct:.2f}")
        for idx, (pct, symbol) in enumerate(top_rows):
            print(f"perf_top_{idx}_pct={pct:.2f}")
            print(f"perf_top_{idx}_symbol={symbol}")
    else:
        print("perf_report_available=0")
        print("perf_field_helper_pct=0.00")
        print("perf_array_helper_pct=0.00")

    selected = "typed_object_field_helper_lowering"
    secondary = "array_runtime_slot_helper_lowering"
    if perf_report is not None and perf_array_pct > perf_field_pct:
        selected, secondary = secondary, selected
    print(f"selected_boundary={selected}")
    print(f"secondary_boundary={secondary}")
    print("next_diagnostic=typed_object_field_helper_fast_lane_selection")
    print("optimization_open=0")
    print("winner_claim=0")
    print("replacement_active=0")
    print("hook_installed=0")
    print("global_allocator=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Classify the owner after supported-storage DirectSlot NativeDirect lowering."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


TOP_RE = re.compile(r"^\s*(?P<pct>\d+(?:\.\d+)?)%\s+\[\.\]\s+(?P<symbol>.+?)\s{2,}\S+")


def parse_symbols(path: Path) -> list[tuple[str, float]]:
    rows: list[tuple[str, float]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        match = TOP_RE.match(line)
        if not match:
            continue
        rows.append((match.group("symbol").strip(), float(match.group("pct"))))
    return rows


def fmt_pct(value: float) -> str:
    return f"{value:.2f}"


def classify(rows: list[tuple[str, float]]) -> dict[str, str]:
    field_helper = 0.0
    array_store = 0.0
    array_load = 0.0
    array_hash = 0.0
    array_direct = 0.0
    array_slot_helper = 0.0
    hako_method = 0.0

    for symbol, pct in rows:
        if "nyash.object.field_" in symbol:
            field_helper += pct
        if "array_slot_backend::single_thread_store_i64" in symbol:
            array_store += pct
        if "array_slot_backend::single_thread_load_encoded_i64" in symbol:
            array_load += pct
        if "core::hash::BuildHasher::hash_one" in symbol or "core::hash::sip::Hasher" in symbol:
            array_hash += pct
        if "nyash.array.slot_load_store_i64_hihi" in symbol:
            array_direct += pct
        if (
            ("nyash.array.slot_store_" in symbol or "nyash.array.slot_load_" in symbol)
            and "nyash.array.slot_load_store_i64_hihi" not in symbol
        ):
            array_slot_helper += pct
        if symbol.startswith("HakoAlloc"):
            hako_method += pct

    array_total = array_store + array_load + array_hash + array_direct + array_slot_helper
    if array_hash >= 20.0 and array_store >= 20.0:
        selected = "array_single_thread_exact_handle_cache"
        reason = "array_single_thread_hash_lookup_dominates_after_direct_slot_supported_storage"
    else:
        selected = "post_supported_storage_owner_selection"
        reason = "array_owner_not_dominant_enough_for_handle_cache"

    return {
        "output_contract": "direct-slot-post-supported-storage-owner-refresh-v0",
        "input_contract": "direct-slot-supported-storage-nativedirect-implementation-v0",
        "workload_id": "representative-object-lifecycle-small-block-v0",
        "attribution_source": "perf_callgraph",
        "field_helper_pct": fmt_pct(field_helper),
        "array_store_pct": fmt_pct(array_store),
        "array_load_pct": fmt_pct(array_load),
        "array_hash_pct": fmt_pct(array_hash),
        "array_direct_op_pct": fmt_pct(array_direct),
        "array_slot_helper_pct": fmt_pct(array_slot_helper),
        "array_total_pct": fmt_pct(array_total),
        "hako_method_pct": fmt_pct(hako_method),
        "selected_boundary": selected,
        "next_diagnostic": selected,
        "selected_reason": reason,
        "optimization_open": "0",
        "winner_claim": "0",
        "replacement_active": "0",
        "hook_installed": "0",
        "global_allocator": "0",
        "summary": "ok",
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--perf-report", required=True, type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    rows = parse_symbols(args.perf_report)
    report = classify(rows)
    text = "\n".join(f"{key}={value}" for key, value in report.items()) + "\n"
    if args.out:
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

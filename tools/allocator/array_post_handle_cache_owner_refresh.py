#!/usr/bin/env python3
"""Classify the owner after the Array single-thread handle cache keeper."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


TOP_RE = re.compile(r"^\s*(?P<pct>\d+(?:\.\d+)?)%\s+\[\.\]\s+(?P<symbol>.+?)\s{2,}\S+")


def parse_symbols(path: Path) -> list[tuple[str, float]]:
    rows: list[tuple[str, float]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        match = TOP_RE.match(line)
        if match:
            rows.append((match.group("symbol").strip(), float(match.group("pct"))))
    return rows


def fmt_pct(value: float) -> str:
    return f"{value:.2f}"


def classify(rows: list[tuple[str, float]]) -> dict[str, str]:
    array_store = 0.0
    array_load = 0.0
    array_hash = 0.0
    array_slot_helper = 0.0
    hako_method = 0.0

    for symbol, pct in rows:
        if "array_slot_backend::single_thread_store_i64" in symbol:
            array_store += pct
        if "array_slot_backend::single_thread_load_encoded_i64" in symbol:
            array_load += pct
        if "core::hash::BuildHasher::hash_one" in symbol or "core::hash::sip::Hasher" in symbol:
            array_hash += pct
        if "nyash.array.slot_store_" in symbol or "nyash.array.slot_load_" in symbol:
            array_slot_helper += pct
        if symbol.startswith("HakoAlloc"):
            hako_method += pct

    array_total = array_store + array_load + array_hash + array_slot_helper
    if array_store + array_load >= 50.0 and array_hash < 5.0:
        selected = "array_slot_nativedirect_guard_surface"
        reason = "array_helper_call_boundary_dominates_after_hash_removed"
    else:
        selected = "post_array_cache_owner_selection"
        reason = "array_helper_boundary_not_dominant_enough"

    return {
        "output_contract": "array-post-handle-cache-owner-refresh-v0",
        "input_contract": "array-single-thread-exact-handle-cache-v0",
        "workload_id": "representative-object-lifecycle-small-block-v0",
        "attribution_source": "perf_callgraph",
        "array_store_pct": fmt_pct(array_store),
        "array_load_pct": fmt_pct(array_load),
        "array_hash_pct": fmt_pct(array_hash),
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

    text = "\n".join(
        f"{key}={value}" for key, value in classify(parse_symbols(args.perf_report)).items()
    )
    text += "\n"
    if args.out:
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

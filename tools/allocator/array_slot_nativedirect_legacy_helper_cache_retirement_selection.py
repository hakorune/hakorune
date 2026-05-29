#!/usr/bin/env python3
"""Select the first legacy helper/cache retirement target after DirectArray dominates."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


ROW_RE = re.compile(r"^\s*(?P<pct>\d+(?:\.\d+)?)%\s+\S+\s+\S+\s+\[\.\]\s+(?P<symbol>.+?)\s*$")


def parse_perf(path: Path) -> list[tuple[str, float]]:
    rows: list[tuple[str, float]] = []
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = ROW_RE.match(line)
        if match:
            rows.append((match.group("symbol").strip(), float(match.group("pct"))))
    if not rows:
        raise SystemExit(f"{path}: no perf rows found")
    return rows


def fmt_pct(value: float) -> str:
    return f"{value:.2f}"


def classify(rows: list[tuple[str, float]]) -> dict[str, str]:
    direct_store = 0.0
    direct_load = 0.0
    direct_fused = 0.0
    legacy_field = 0.0
    legacy_array = 0.0
    legacy_hash = 0.0
    hako_method = 0.0

    for symbol, pct in rows:
        if "nyash_kernel::plugin::array_slot_backend::single_thread_store_i64" in symbol:
            direct_store += pct
        if "nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64" in symbol:
            direct_load += pct
        if "nyash.array.slot_load_store_i64_hihi" in symbol:
            direct_fused += pct
        if "nyash.object.field_" in symbol:
            legacy_field += pct
        if ("nyash.array.slot_store_" in symbol or "nyash.array.slot_load_" in symbol) and "nyash.array.slot_load_store_i64_hihi" not in symbol:
            legacy_array += pct
        if "core::hash::BuildHasher::hash_one" in symbol or "core::hash::sip::Hasher" in symbol:
            legacy_hash += pct
        if symbol.startswith("HakoAlloc"):
            hako_method += pct

    direct_total = direct_store + direct_load + direct_fused
    legacy_total = legacy_field + legacy_array + legacy_hash
    dominates = direct_total >= legacy_total and direct_total >= hako_method

    if dominates:
        selected_boundary = "arraybox_public_semantics_and_directarray_split_ssot"
        next_diagnostic = "arraybox_public_semantics_and_directarray_split_ssot"
        selected_next = "arraybox_public_semantics_and_directarray_split_ssot"
        selected_candidate = "single_thread_exact_array_helper_backend"
        selected_reason = "direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke"
        retirement_open = "1"
    else:
        selected_boundary = "array_slot_nativedirect_post_semantic_perf_owner_refresh"
        next_diagnostic = "array_slot_nativedirect_post_semantic_perf_owner_refresh"
        selected_next = "array_slot_nativedirect_post_semantic_perf_owner_refresh"
        selected_candidate = "array_slot_public_helper_fast_lane"
        selected_reason = "legacy_helper_cache_still_dominant_after_semantic_smoke"
        retirement_open = "0"

    return {
        "output_contract": "array-slot-nativedirect-legacy-helper-cache-retirement-selection-v0",
        "input_contract": "array-slot-nativedirect-post-semantic-perf-owner-refresh-v0",
        "workload_id": "representative-object-lifecycle-small-block-v0",
        "attribution_source": "perf_callgraph",
        "selected_method": "HakoAllocPageModel.acquire_usize/1",
        "direct_array_backend_store_pct": fmt_pct(direct_store),
        "direct_array_backend_load_pct": fmt_pct(direct_load),
        "direct_array_backend_direct_op_pct": fmt_pct(direct_fused),
        "direct_array_backend_total_pct": fmt_pct(direct_total),
        "legacy_field_helper_pct": fmt_pct(legacy_field),
        "legacy_array_helper_pct": fmt_pct(legacy_array),
        "legacy_hash_pct": fmt_pct(legacy_hash),
        "legacy_helper_cache_total_pct": fmt_pct(legacy_total),
        "hako_method_pct": fmt_pct(hako_method),
        "direct_array_dominates_legacy_helper_cache": "1" if dominates else "0",
        "legacy_helper_cache_retirement_open": retirement_open,
        "selected_retirement_candidate": selected_candidate,
        "selected_retirement_reason": selected_reason,
        "selected_boundary": selected_boundary,
        "next_diagnostic": next_diagnostic,
        "selected_next": selected_next,
        "legacy_retirement_candidate_0": "single_thread_exact_array_helper_backend",
        "legacy_retirement_candidate_1": "array_slot_handle_entry_cache",
        "legacy_retirement_candidate_2": "array_slot_public_helper_fast_lane",
        "legacy_retirement_now": "0",
        "optimization_open": "0",
        "winner_claim": "0",
        "replacement_active": "0",
        "hook_installed": "0",
        "global_allocator": "0",
        "summary": "ok",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", required=True, type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    report = classify(parse_perf(args.perf_report))
    text = "\n".join(f"{key}={value}" for key, value in report.items()) + "\n"
    if args.out:
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Refresh the post-fusion owner after the selected Array slot direct op."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


PERF_SYMBOL_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+\S+\s+\[.\]\s+(.+?)\s*$")


def parse_perf(path: Path) -> tuple[float, float, float, float, list[tuple[float, str]]]:
    field_pct = 0.0
    array_slot_pct = 0.0
    fused_pct = 0.0
    array_hash_pct = 0.0
    rows: list[tuple[float, str]] = []
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = PERF_SYMBOL_RE.match(raw)
        if not match:
            continue
        pct = float(match.group(1))
        symbol = match.group(2)
        rows.append((pct, symbol))
        if "nyash.object.field_" in symbol:
            field_pct += pct
        if "array_slot_backend::single_thread_" in symbol:
            array_slot_pct += pct
        if "nyash.array.slot_load_store_i64_hihi" in symbol:
            fused_pct += pct
        if "core::hash::" in symbol or "core::hash::sip::" in symbol:
            array_hash_pct += pct
    rows.sort(key=lambda row: (-row[0], row[1]))
    return field_pct, array_slot_pct, fused_pct, array_hash_pct, rows[:8]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    field_pct, array_slot_pct, fused_pct, array_hash_pct, rows = parse_perf(args.perf_report)
    array_total_pct = array_slot_pct + fused_pct + array_hash_pct
    if field_pct >= array_total_pct:
        selected = "typed_object_field_helper_lowering"
        secondary = "array_slot_backend_handle_map_hash"
        next_diagnostic = "typed_object_field_helper_subowner_refresh"
    else:
        selected = "array_slot_backend_handle_map_hash"
        secondary = "typed_object_field_helper_lowering"
        next_diagnostic = "array_slot_backend_handle_map_refresh"

    lines = [
        "output_contract=selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0",
        "input_contract=selected-method-array-slot-direct-op-keeper-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"perf_field_helper_pct={field_pct:.2f}",
        f"perf_array_slot_backend_pct={array_slot_pct:.2f}",
        f"perf_fused_direct_op_pct={fused_pct:.2f}",
        f"perf_array_backend_hash_pct={array_hash_pct:.2f}",
        f"perf_array_total_pct={array_total_pct:.2f}",
    ]
    for index, (pct, symbol) in enumerate(rows):
        lines.append(f"perf_top_{index}_pct={pct:.2f}")
        lines.append(f"perf_top_{index}_symbol={symbol}")
    lines.extend(
        [
            f"selected_boundary={selected}",
            f"secondary_boundary={secondary}",
            f"next_diagnostic={next_diagnostic}",
            "optimization_open=0",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

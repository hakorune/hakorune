#!/usr/bin/env python3
"""Refresh hot owner after rolling back the page queue get/set non-keeper."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


ROW_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+\S+\s+\[\.\]\s+(.+?)\s*$")


def parse_perf(path: Path) -> list[tuple[float, str]]:
    rows: list[tuple[float, str]] = []
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = ROW_RE.match(line)
        if match:
            rows.append((float(match.group(1)), match.group(2).strip()))
    if not rows:
        raise SystemExit(f"{path}: no perf rows found")
    return rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    rows = parse_perf(args.perf_report)
    exact_slot_pct = sum(pct for pct, sym in rows if "nyash.object.exact_slot_" in sym)
    rmw_pct = sum(pct for pct, sym in rows if "nyash.object.exact_slot_rmw_" in sym)
    exact_get_set_pct = exact_slot_pct - rmw_pct
    legacy_field_pct = sum(pct for pct, sym in rows if "nyash.object.field_" in sym)
    array_backend_pct = sum(
        pct for pct, sym in rows if "nyash_kernel::plugin::array_slot_backend::" in sym
    )
    array_hash_pct = sum(
        pct
        for pct, sym in rows
        if "core::hash::BuildHasher::hash_one" in sym
        or "<core::hash::sip::Hasher" in sym
    )
    hako_method_pct = sum(pct for pct, sym in rows if sym.startswith("HakoAlloc"))
    array_total = array_backend_pct + array_hash_pct

    if exact_get_set_pct >= array_total and exact_get_set_pct >= hako_method_pct:
        selected_boundary = "weighted_exact_slot_callsite_attribution_refresh"
        next_diagnostic = "weighted_exact_slot_callsite_attribution_refresh"
    elif array_total >= hako_method_pct:
        selected_boundary = "array_slot_backend_owner_refresh"
        next_diagnostic = "array_slot_backend_hash_or_direct_map_refresh"
    else:
        selected_boundary = "hako_method_body_owner_refresh"
        next_diagnostic = "hako_method_body_shape_refresh"

    lines = [
        "output_contract=post-page-queue-rollback-owner-refresh-v0",
        "input_contract=rollback-selected-page-queue-same-block-get-set-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"perf_exact_slot_helper_pct={exact_slot_pct:.2f}",
        f"perf_exact_slot_get_set_pct={exact_get_set_pct:.2f}",
        f"perf_exact_slot_rmw_helper_pct={rmw_pct:.2f}",
        f"perf_legacy_field_helper_pct={legacy_field_pct:.2f}",
        f"perf_array_slot_backend_pct={array_backend_pct:.2f}",
        f"perf_array_backend_hash_pct={array_hash_pct:.2f}",
        f"perf_array_total_pct={array_total:.2f}",
        f"perf_hako_method_pct={hako_method_pct:.2f}",
        f"selected_boundary={selected_boundary}",
        f"next_diagnostic={next_diagnostic}",
        "static_candidate_count_only_rejected=1",
        "weighted_hot_candidate_score_required=1",
        "ir_shape_diff_required_before_next_keeper=1",
        "sample_count_3_required_for_keeper_decision=1",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (pct, symbol) in enumerate(rows[:10]):
        lines.append(f"perf_top_{idx}_pct={pct:.2f}")
        lines.append(f"perf_top_{idx}_symbol={symbol}")
    lines.append("summary=ok")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

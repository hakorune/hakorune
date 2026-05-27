#!/usr/bin/env python3
"""Emit the row 75 selfhost handoff decision for hako mimalloc parity."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-mimalloc-perf-parity-selfhost-handoff-gate-v0",
        "input_contract=hako-mimalloc-hakmem-ldpreload-bench-pilot-v0",
        "selfhost_handoff_decision=parked",
        "park_reason=hako_mimalloc_small_block_gap_still_large",
        "remaining_allocator_gap_classified=1",
        "next_diagnostic=hako_check_perf_surface_inventory",
        "next_row=HAKO-CHECK-PERF-SURFACE-CONTRACT-296X-001",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

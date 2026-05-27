#!/usr/bin/env python3
"""Select the next hako mimalloc keeper after row 79 measurement."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-mimalloc-perf-next-keeper-selection-v0",
        "input_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0",
        "previous_keeper=release_known_page_fast_path",
        "next_keeper=select_page_single_page_fast_path",
        "selection_reason=hako_check_perf_surface_found_objectLifecycleSmallAlloc_selectPage_hot_path",
        "selected_target_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako",
        "selected_target_method=objectLifecycleSmallAlloc",
        "implementation_row=HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001",
        "winner_claim=0",
        "replacement_active=0",
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

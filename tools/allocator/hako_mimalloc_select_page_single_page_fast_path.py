#!/usr/bin/env python3
"""Validate the row 81 selectPage single-page fast path keeper surface."""

from __future__ import annotations

import argparse
from pathlib import Path


QUEUE = Path("lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako")
FACADE = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")
APP = Path("apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako")


def require_text(path: Path, needle: str, label: str) -> None:
    text = path.read_text(encoding="utf-8", errors="replace")
    if needle not in text:
        raise SystemExit(f"{label}: missing {needle!r} in {path}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    require_text(QUEUE, "single_page_fast_path_count", "queue")
    require_text(QUEUE, "selectSinglePageFastPath()", "queue")
    require_text(QUEUE, "if me.page_count == 1", "queue")
    require_text(QUEUE, "return me.selectSinglePageFastPath()", "queue")
    require_text(FACADE, "objectLifecycleSinglePageFastPathCount()", "facade")
    require_text(FACADE, "objectLifecycleSinglePageFallbackCount()", "facade")
    require_text(APP, "select_page_single_fast_path_count=", "proof app")
    require_text(APP, "select_page_single_fallback_count=", "proof app")

    lines = [
        "output_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0",
        "input_contract=hako-mimalloc-perf-next-keeper-selection-v0",
        "keeper=select_page_single_page_fast_path",
        "target_method=objectLifecycleSmallAlloc",
        "queue_fast_path_method=selectSinglePageFastPath",
        "proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako",
        "proof_expected_select_page_single=524288,0",
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

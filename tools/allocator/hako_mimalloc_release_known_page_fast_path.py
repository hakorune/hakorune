#!/usr/bin/env python3
"""Validate the row 78 release known-page fast path keeper surface."""

from __future__ import annotations

import argparse
from pathlib import Path


DEFAULT_TARGET = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", type=Path, default=DEFAULT_TARGET)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = args.target.read_text(encoding="utf-8", errors="replace")
    required = {
        "recordLastAllocPage": "recordLastAllocPage(" in source,
        "release_lookup": "objectLifecycleReleaseKnownPageIndex(page_id)" in source,
        "release_call": "local known_index = me.objectLifecycleReleaseKnownPageIndex(page_id)" in source,
        "fast_observer": "objectLifecycleReleaseKnownPageFastPathCount()" in source,
        "fallback_observer": "objectLifecycleReleaseKnownPageFallbackCount()" in source,
        "fallback_route": "return me.objectLifecycleKnownPageIndexById(page_id)" in source,
    }
    missing = [name for name, ok in required.items() if not ok]
    if missing:
        raise SystemExit(f"missing release fast-path surface: {', '.join(missing)}")

    lines = [
        "output_contract=hako-mimalloc-perf-release-known-page-fast-path-v0",
        "input_contract=hako-check-perf-surface-inventory-v0",
        f"target_file={args.target.as_posix()}",
        "keeper=release_known_page_fast_path",
        "fast_path_observer=objectLifecycleReleaseKnownPageFastPathCount",
        "fallback_observer=objectLifecycleReleaseKnownPageFallbackCount",
        "release_uses_known_page_fast_path=1",
        "normal_release_route_intact=1",
        "proof_app=apps/mimalloc-facade-release-one-block-proof/main.hako",
        "proof_expected_release_known_page=1,0",
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

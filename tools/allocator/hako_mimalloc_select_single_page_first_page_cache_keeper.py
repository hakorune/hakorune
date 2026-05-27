#!/usr/bin/env python3
"""Validate the row98 select single-page first-page cache keeper."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


QUEUE = Path("lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako")
APP = Path("apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako")


def find_method_body(source: str, method_name: str) -> str:
    match = re.search(rf"^\s*{re.escape(method_name)}\s*\([^)]*\)\s*\{{", source, re.M)
    if match is None:
        raise SystemExit(f"method not found: {method_name}")
    brace_start = source.find("{", match.start())
    depth = 0
    for idx in range(brace_start, len(source)):
        ch = source[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return source[brace_start + 1 : idx]
    raise SystemExit(f"method body not closed: {method_name}")


def require_text(text: str, needle: str, label: str) -> None:
    if needle not in text:
        raise SystemExit(f"{label}: missing {needle!r}")


def forbid_text(text: str, needle: str, label: str) -> None:
    if needle in text:
        raise SystemExit(f"{label}: forbidden {needle!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = QUEUE.read_text(encoding="utf-8", errors="replace")
    require_text(source, "first_page: HakoAllocPageModel = null", "queue")
    add_body = find_method_body(source, "addPage")
    require_text(add_body, "if index == 0", "addPage")
    require_text(add_body, "me.first_page = page", "addPage")

    fast_body = find_method_body(source, "selectSinglePageFastPath")
    require_text(fast_body, "local page = me.first_page", "fast path")
    forbid_text(fast_body, "me.pages.get(0)", "fast path")

    app_source = APP.read_text(encoding="utf-8", errors="replace")
    require_text(app_source, "select_page_single_fast_path_count=", "proof app")

    lines = [
        "output_contract=hako-mimalloc-select-single-page-first-page-cache-keeper-v0",
        "input_contract=hako-mimalloc-post-release-direct-cached-page-source-mir-refresh-v0",
        "keeper=select_single_page_first_page_cache",
        "keeper_kind=box_count",
        "target_method=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0",
        "first_page_cache_used=1",
        "removed_single_page_pages_get=1",
        "proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako",
        "winner_claim=0",
        "replacement_active=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

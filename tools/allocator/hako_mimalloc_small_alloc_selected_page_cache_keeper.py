#!/usr/bin/env python3
"""Validate the row89 small-alloc selected-page cache keeper."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


FACADE = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")
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

    facade_source = FACADE.read_text(encoding="utf-8", errors="replace")
    body = find_method_body(facade_source, "objectLifecycleSmallAlloc")
    require_text(body, "me.object_lifecycle_queue.selectPage()", "facade")
    require_text(body, "local page = me.object_lifecycle_queue.last_selected_page", "facade")
    require_text(body, "if page == null", "facade")
    require_text(body, "local selected_index = me.object_lifecycle_queue.last_selected_index", "facade")
    forbid_text(body, "local page = me.object_lifecycle_queue.selectPage()", "facade")
    forbid_text(body, "local pages = me.object_lifecycle_queue.pages", "facade")
    forbid_text(body, "pages.get(selected_index)", "facade")

    queue_source = QUEUE.read_text(encoding="utf-8", errors="replace")
    require_text(queue_source, "last_selected_page: HakoAllocPageModel = null", "queue")
    require_text(queue_source, "me.last_selected_page = null", "queue")
    require_text(queue_source, "me.last_selected_page = page", "queue")

    app_source = APP.read_text(encoding="utf-8", errors="replace")
    require_text(app_source, "select_page_single_fast_path_count=", "proof app")
    require_text(app_source, "release_known_page_fast_path_count=", "proof app")

    lines = [
        "output_contract=hako-mimalloc-small-alloc-selected-page-cache-keeper-v0",
        "input_contract=hako-mimalloc-multi-method-source-mir-observation-v0",
        "keeper=small_alloc_selected_page_cache_reuse",
        "keeper_kind=box_count",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "selected_page_cache_reused=1",
        "removed_repeated_pages_get=1",
        "proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako",
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

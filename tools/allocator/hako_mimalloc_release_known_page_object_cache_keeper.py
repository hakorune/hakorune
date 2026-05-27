#!/usr/bin/env python3
"""Validate the row92 release known-page object cache keeper."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


FACADE = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")
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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = FACADE.read_text(encoding="utf-8", errors="replace")
    require_text(source, "last_alloc_page: HakoAllocPageModel = null", "facade")
    require_text(source, "release_known_page: HakoAllocPageModel = null", "facade")
    require_text(source, "me.last_alloc_page = page", "facade")

    known_body = find_method_body(source, "objectLifecycleReleaseKnownPageIndex")
    require_text(known_body, "me.release_known_page = null", "known index")
    require_text(known_body, "local page = me.last_alloc_page", "known index")
    require_text(known_body, "me.release_known_page = page", "known index")

    release_body = find_method_body(source, "objectLifecycleReleaseBlock")
    require_text(release_body, "local page = me.release_known_page", "release")
    require_text(release_body, "if page == null", "release fallback")
    require_text(release_body, "page = pages.get(known_index)", "release fallback")

    app_source = APP.read_text(encoding="utf-8", errors="replace")
    require_text(app_source, "release_known_page_fast_path_count=", "proof app")

    lines = [
        "output_contract=hako-mimalloc-release-known-page-object-cache-keeper-v0",
        "input_contract=hako-mimalloc-post-small-alloc-cache-source-mir-refresh-v0",
        "keeper=release_known_page_object_cache",
        "keeper_kind=box_shape",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
        "release_known_page_object_cache_reused=1",
        "fallback_pages_get_preserved=1",
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

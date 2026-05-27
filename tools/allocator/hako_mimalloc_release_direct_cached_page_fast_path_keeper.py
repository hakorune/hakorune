#!/usr/bin/env python3
"""Validate the row95 release direct cached-page fast path keeper."""

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
    direct_body = find_method_body(source, "objectLifecycleReleaseDirectCachedPage")
    require_text(direct_body, "local page = me.last_alloc_page", "direct")
    require_text(direct_body, "me.release_known_page_fast_path_count", "direct")
    require_text(direct_body, "page.releaseLocal(block_id)", "direct")
    require_text(direct_body, "return -1", "direct")

    release_body = find_method_body(source, "objectLifecycleReleaseBlock")
    require_text(
        release_body,
        "local direct_release = me.objectLifecycleReleaseDirectCachedPage(page_id, block_id)",
        "release",
    )
    require_text(release_body, "if direct_release >= 0", "release")
    require_text(release_body, "me.objectLifecycleReleaseKnownPageIndex(page_id)", "release fallback")

    app_source = APP.read_text(encoding="utf-8", errors="replace")
    require_text(app_source, "release_known_page_fast_path_count=", "proof app")

    lines = [
        "output_contract=hako-mimalloc-release-direct-cached-page-fast-path-keeper-v0",
        "input_contract=hako-mimalloc-post-release-object-cache-source-mir-refresh-v0",
        "keeper=release_direct_cached_page_fast_path",
        "keeper_kind=box_count",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
        "direct_cached_page_release_fast_path=1",
        "fallback_lookup_preserved=1",
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

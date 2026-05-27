#!/usr/bin/env python3
"""Validate rollback of the active field fast path keeper."""

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
    require_text(source, "local page = me.first_page", "queue")
    fast_body = find_method_body(source, "selectSinglePageFastPath")
    forbid_text(fast_body, "if page.decommitted == 0", "rollback")
    forbid_text(fast_body, "if page.retired == 0", "rollback")
    forbid_text(fast_body, "if page.free_top > 0", "rollback")
    require_text(fast_body, "if page.isDecommitted() != 0", "generic fallback")
    require_text(fast_body, "if page.isRetired() != 0", "generic fallback")
    require_text(fast_body, "if page.freeCount() > 0", "generic fallback")

    app_source = APP.read_text(encoding="utf-8", errors="replace")
    require_text(app_source, "select_page_single_fast_path_count=", "proof app")

    lines = [
        "output_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0",
        "input_contract=hako-mimalloc-post-active-field-fast-path-keeper-measurement-v0",
        "rolled_back_keeper=select_single_page_active_field_fast_path",
        "active_field_fast_path_present=0",
        "first_page_cache_preserved=1",
        "generic_lifecycle_fallback_preserved=1",
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

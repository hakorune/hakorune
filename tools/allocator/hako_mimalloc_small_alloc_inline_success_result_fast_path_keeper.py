#!/usr/bin/env python3
"""Validate the row109 small-alloc inline success result fast path keeper."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


FACADE = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")
APP = Path("apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako")


def find_method_body(source: str, method_name: str) -> str:
    match = re.search(
        rf"^\s*{re.escape(method_name)}\s*\([^)]*\)\s*(?::[^\{{]+)?\s*\{{",
        source,
        re.M,
    )
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

    source = FACADE.read_text(encoding="utf-8", errors="replace")
    body = find_method_body(source, "objectLifecycleSmallAlloc")
    require_text(body, "return alloc_result.recordFailureAfterSelectedPage", "failure helpers")
    require_text(body, "return alloc_result.recordFailureNoSelection", "failure helpers")
    require_text(body, "me.recordLastAllocPage(0, selected_page_id, page)", "success inline")
    require_text(body, "me.recordLastAllocPage(selected_index, selected_page_id, page)", "success inline")
    require_text(body, "return alloc_result.recordSuccess(2)", "success inline")
    require_text(body, "return alloc_result.recordSuccess(selected_kind)", "success inline")
    forbid_text(body, "return me.recordSmallAllocSuccess(selected_kind)", "success wrapper")

    app_source = APP.read_text(encoding="utf-8", errors="replace")
    require_text(app_source, "select_page_single_fast_path_count=", "proof app")

    lines = [
        "output_contract=hako-mimalloc-small-alloc-inline-success-result-fast-path-keeper-v0",
        "input_contract=hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh-v0",
        "keeper=small_alloc_inline_success_result_fast_path",
        "keeper_kind=box_count",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "inline_success_result_used=1",
        "failure_helpers_preserved=1",
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

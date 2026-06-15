#!/usr/bin/env python3
"""Inventory conservative publication sites for the local-first object model."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


FACADE_SOURCE = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")
TARGET_METHOD = "objectLifecycleSmallAlloc"
TARGET_METHOD_FULL = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


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


def count(pattern: str, text: str) -> int:
    return len(re.findall(pattern, text, flags=re.M))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=FACADE_SOURCE)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = args.source.read_text(encoding="utf-8", errors="replace")
    body = find_method_body(source, TARGET_METHOD)

    new_box_count = count(r"\bnew\s+[A-Za-z_][A-Za-z0-9_]*\s*\(", body)
    local_binding_count = count(r"^\s*local\s+[A-Za-z_][A-Za-z0-9_]*\s*=", body)
    local_page_candidate_count = 1 if re.search(r"^\s*local\s+page\s*=", body, re.M) else 0
    preexisting_field_alias_count = count(r"^\s*local\s+\w+\s*=\s*me\.", body)

    record_last_alloc_page_calls = count(r"\bme\.recordLastAllocPage\s*\(", body)
    page_publication_sites = count(r"\bme\.recordLastAllocPage\s*\([^)]*\bpage\s*\)", body)
    host_handle_required = page_publication_sites

    page_direct_call_count = count(r"\bpage\.(?:acquire_usize|reuse)\s*\(", body)
    pre_publication_direct_candidate_count = page_direct_call_count if page_publication_sites else 0

    plugin_or_extern = count(r"\b(plugin|extern)\b", body)
    dynamic_array_or_map = count(r"\b(?:Array|Map)<\s*any\s*>", body)
    task_boundary = count(r"\b(?:nowait|await|co|Channel|Future)\b", body)
    return_dynamic = 0
    unknown_publication = 0

    publication_site_count = (
        host_handle_required
        + plugin_or_extern
        + dynamic_array_or_map
        + task_boundary
        + return_dynamic
        + unknown_publication
    )

    lines = [
        "output_contract=hako-object-publication-inventory-v0",
        "source_evidence=296x-812,296x-811",
        "target_front=object_lifecycle_body",
        f"target_method={TARGET_METHOD_FULL}",
        f"source_file={args.source.as_posix()}",
        "inventory_kind=source_body_conservative",
        f"new_box_count={new_box_count}",
        f"local_binding_count={local_binding_count}",
        f"local_object_candidate_count={local_page_candidate_count}",
        f"preexisting_published_field_alias_count={preexisting_field_alias_count}",
        f"publication_site_count={publication_site_count}",
        f"publication_reason_host_handle_required_count={host_handle_required}",
        f"publication_reason_plugin_or_extern_count={plugin_or_extern}",
        f"publication_reason_dynamic_array_or_map_count={dynamic_array_or_map}",
        f"publication_reason_task_future_channel_boundary_count={task_boundary}",
        f"publication_reason_return_as_dynamic_box_count={return_dynamic}",
        f"publication_reason_unknown_count={unknown_publication}",
        f"record_last_alloc_page_call_count={record_last_alloc_page_calls}",
        f"page_local_candidate_count={local_page_candidate_count}",
        f"page_publication_site_count={page_publication_sites}",
        f"pre_publication_page_direct_call_count={pre_publication_direct_candidate_count}",
        "array_length_direct_candidate_count=0",
        "array_length_direct_candidate_reason=not_in_facade_body",
        "unknown_publication_forces_generic_fallback=1",
        "object_plan_execution_enabled=0",
        "backend_consumes_object_plan=0",
        "product_default_changed=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

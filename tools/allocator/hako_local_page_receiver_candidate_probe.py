#!/usr/bin/env python3
"""Probe the page receiver candidate selected by the local-first pilot row."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


FACADE_SOURCE = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")
QUEUE_SOURCE = Path("lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako")
PAGE_SOURCE = Path("lang/src/hako_alloc/memory/page_box.hako")
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


def has(pattern: str, text: str) -> bool:
    return re.search(pattern, text, flags=re.M) is not None


def method_decl_exists(source: str, method_name: str) -> bool:
    return has(rf"^\s*{re.escape(method_name)}\s*\([^)]*\)\s*(?::[^\{{]+)?\s*\{{", source)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--facade-source", type=Path, default=FACADE_SOURCE)
    parser.add_argument("--queue-source", type=Path, default=QUEUE_SOURCE)
    parser.add_argument("--page-source", type=Path, default=PAGE_SOURCE)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    facade_source = args.facade_source.read_text(encoding="utf-8", errors="replace")
    queue_source = args.queue_source.read_text(encoding="utf-8", errors="replace")
    page_source = args.page_source.read_text(encoding="utf-8", errors="replace")
    body = find_method_body(facade_source, TARGET_METHOD)

    page_local_binding_count = count(r"^\s*local\s+page\s*=", body)
    page_birth_in_body = 1 if has(r"\bpage\s*=\s*new\s+", body) else 0

    fast_path_assignments = count(r"\bpage\s*=\s*queue\.selectSinglePageFastPath\s*\(", body)
    normal_select_assignments = count(r"\bpage\s*=\s*queue\.selectPage\s*\(", body)
    page_from_queue_selection_count = fast_path_assignments + normal_select_assignments
    page_from_queue_selection = 1 if page_from_queue_selection_count > 0 else 0

    selector_return_type_known_count = 0
    selector_return_type_known_count += 1 if method_decl_exists(queue_source, "selectSinglePageFastPath") and has(
        r"selectSinglePageFastPath\s*\(\)\s*:\s*HakoAllocPageModel", queue_source
    ) else 0
    selector_return_type_known_count += 1 if method_decl_exists(queue_source, "selectPage") and has(
        r"selectPage\s*\(\)\s*:\s*HakoAllocPageModel", queue_source
    ) else 0
    page_type_known = 1 if selector_return_type_known_count == 2 else 0

    acquire_call_count = count(r"\bpage\.acquire_usize\s*\(", body)
    reuse_call_count = count(r"\bpage\.reuse\s*\(", body)
    page_pre_publication_call_count = acquire_call_count + reuse_call_count
    page_publication_site_count = count(r"\bme\.recordLastAllocPage\s*\([^)]*\bpage\s*\)", body)

    last_publication = body.rfind("recordLastAllocPage")
    after_last_publication = body[last_publication:] if last_publication >= 0 else ""
    page_call_after_publication_count = count(r"\bpage\.(?:acquire_usize|reuse)\s*\(", after_last_publication)

    page_method_surface_known_count = 0
    page_method_surface_known_count += 1 if method_decl_exists(page_source, "acquire_usize") else 0
    page_method_surface_known_count += 1 if method_decl_exists(page_source, "reuse") else 0

    dynamic_api_count = count(r"\bpage\.(?:share_box|clone_box|as_any|type_name)\s*\(", body)
    plugin_or_extern_count = count(r"\b(?:plugin|extern)\b", body)
    task_boundary_count = count(r"\b(?:nowait|await|co|Channel|Future)\b", body)

    lines = [
        "output_contract=hako-local-page-receiver-candidate-probe-v0",
        "source_evidence=296x-816,296x-814,296x-813",
        "target_front=object_lifecycle_body",
        f"target_method={TARGET_METHOD_FULL}",
        f"facade_source_file={args.facade_source.as_posix()}",
        f"queue_source_file={args.queue_source.as_posix()}",
        f"page_source_file={args.page_source.as_posix()}",
        "probe_kind=source_body_conservative",
        f"page_local_binding_count={page_local_binding_count}",
        f"page_birth_in_body={page_birth_in_body}",
        f"page_from_queue_selection={page_from_queue_selection}",
        f"page_from_queue_selection_count={page_from_queue_selection_count}",
        f"page_select_single_fast_path_assignment_count={fast_path_assignments}",
        f"page_select_page_assignment_count={normal_select_assignments}",
        f"page_selector_return_type_known_count={selector_return_type_known_count}",
        f"page_type_known={page_type_known}",
        f"page_method_surface_known_count={page_method_surface_known_count}",
        f"page_acquire_usize_call_count={acquire_call_count}",
        f"page_reuse_call_count={reuse_call_count}",
        f"page_pre_publication_call_count={page_pre_publication_call_count}",
        f"page_publication_site_count={page_publication_site_count}",
        f"page_call_after_publication_count={page_call_after_publication_count}",
        f"page_dynamic_api_required_count={dynamic_api_count}",
        f"page_plugin_or_extern_escape_count={plugin_or_extern_count}",
        f"page_task_boundary_escape_count={task_boundary_count}",
        "page_storage_direct_required=0",
        "page_hosthandle_bypass_required=0",
        "closed_world_direct_call_proof_count=0",
        "routeplan_backend_consumable_proof_count=0",
        "candidate_probe_open=1",
        "guard_surface_required=1",
        "implementation_allowed=0",
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

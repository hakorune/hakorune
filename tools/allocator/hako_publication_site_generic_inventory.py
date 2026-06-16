#!/usr/bin/env python3
"""Inventory generic ObjectPublicationReason vocabulary from code."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


DEFAULT_SOURCE = Path("src/object_storage_plan.rs")
DEFAULT_SOURCE_DIR = Path("src/object_storage_plan")

EXPECTED_REASONS = [
    "PluginOrExternBoundary",
    "HostHandleRequired",
    "DynamicArrayOrMapStorage",
    "DynamicNyashBoxApi",
    "ReturnAsDynamicBox",
    "TaskFutureChannelBoundary",
    "UnknownFiniOrDrop",
    "Unknown",
]


def extract_enum_variants(source: str, enum_name: str) -> list[str]:
    match = re.search(rf"pub\s+enum\s+{re.escape(enum_name)}\s*\{{", source)
    if match is None:
        raise SystemExit(f"enum not found: {enum_name}")
    brace_start = source.find("{", match.start())
    depth = 0
    for idx in range(brace_start, len(source)):
        ch = source[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                body = source[brace_start + 1 : idx]
                variants = []
                for line in body.splitlines():
                    stripped = line.strip().rstrip(",")
                    if not stripped or stripped.startswith("//"):
                        continue
                    variants.append(stripped.split()[0])
                return variants
    raise SystemExit(f"enum body not closed: {enum_name}")


def read_source_surface(source: Path) -> str:
    parts = [source.read_text(encoding="utf-8", errors="replace")]
    source_dir = source.with_suffix("")
    if source_dir.is_dir():
        for path in sorted(source_dir.glob("*.rs")):
            parts.append(path.read_text(encoding="utf-8", errors="replace"))
    elif DEFAULT_SOURCE_DIR.is_dir() and source == DEFAULT_SOURCE:
        for path in sorted(DEFAULT_SOURCE_DIR.glob("*.rs")):
            parts.append(path.read_text(encoding="utf-8", errors="replace"))
    return "\n".join(parts)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = read_source_surface(args.source)
    variants = extract_enum_variants(source, "ObjectPublicationReason")
    missing = [reason for reason in EXPECTED_REASONS if reason not in variants]
    extra = [reason for reason in variants if reason not in EXPECTED_REASONS]

    lines = [
        "output_contract=hako-publication-site-generic-inventory-v0",
        "source_evidence=296x-828,296x-829",
        f"source_file={args.source.as_posix()}",
        "inventory_kind=code_vocabulary",
        f"publication_reason_vocabulary_count={len(variants)}",
        f"publication_reason_expected_count={len(EXPECTED_REASONS)}",
        f"publication_reason_missing_count={len(missing)}",
        f"publication_reason_extra_count={len(extra)}",
        "publication_reason_plugin_or_extern=1"
        if "PluginOrExternBoundary" in variants
        else "publication_reason_plugin_or_extern=0",
        "publication_reason_host_handle_required=1"
        if "HostHandleRequired" in variants
        else "publication_reason_host_handle_required=0",
        "publication_reason_dynamic_array_or_map=1"
        if "DynamicArrayOrMapStorage" in variants
        else "publication_reason_dynamic_array_or_map=0",
        "publication_reason_dynamic_nyashbox_api=1"
        if "DynamicNyashBoxApi" in variants
        else "publication_reason_dynamic_nyashbox_api=0",
        "publication_reason_return_as_dynamic_box=1"
        if "ReturnAsDynamicBox" in variants
        else "publication_reason_return_as_dynamic_box=0",
        "publication_reason_task_future_channel_boundary=1"
        if "TaskFutureChannelBoundary" in variants
        else "publication_reason_task_future_channel_boundary=0",
        "publication_reason_unknown_fini_or_drop=1"
        if "UnknownFiniOrDrop" in variants
        else "publication_reason_unknown_fini_or_drop=0",
        "publication_reason_unknown=1" if "Unknown" in variants else "publication_reason_unknown=0",
        "unknown_publication_forces_generic_fallback=1",
        "standalone_publication_plan_enabled=0",
        "objectplan_execution_enabled=0",
        "backend_consumes_objectplan=0",
        "product_default_changed=0",
        "summary=ok" if not missing and not extra else "summary=drift",
    ]
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0 if not missing and not extra else 1


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Build a report-only local object shadow plan from publication inventory."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    out: dict[str, str] = {}
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        out[key] = value
    return out


def as_int(values: dict[str, str], key: str) -> int:
    try:
        return int(values.get(key, "0"))
    except ValueError:
        return 0


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--publication-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    inv = read_kv(args.publication_report)
    require(inv, "output_contract", "hako-object-publication-inventory-v0")
    require(inv, "summary", "ok")

    local_candidates = as_int(inv, "local_object_candidate_count")
    publication_sites = as_int(inv, "publication_site_count")
    pre_publication_calls = as_int(inv, "pre_publication_page_direct_call_count")
    array_len_candidates = as_int(inv, "array_length_direct_candidate_count")

    local_identity_candidates = local_candidates
    published_fallback_candidates = 1 if publication_sites > 0 else 0
    shadow_candidate_count = local_identity_candidates + published_fallback_candidates
    pilot_open = 1 if array_len_candidates > 0 else 0

    lines = [
        "output_contract=hako-local-object-shadow-v0",
        "source_evidence=296x-813,296x-812",
        "target_front=object_lifecycle_body",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        f"local_object_candidate_count={local_candidates}",
        f"local_identity_object_candidate_count={local_identity_candidates}",
        "local_scalar_candidate_count=0",
        "local_struct_candidate_count=0",
        f"published_fallback_candidate_count={published_fallback_candidates}",
        f"publication_site_count={publication_sites}",
        f"pre_publication_direct_call_count={pre_publication_calls}",
        f"array_length_direct_candidate_count={array_len_candidates}",
        f"local_direct_array_len_pilot_open={pilot_open}",
        "shadow_plan_behavior_changed=0",
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

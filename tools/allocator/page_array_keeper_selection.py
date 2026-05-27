#!/usr/bin/env python3
"""Select one page-array keeper from dynamic weight evidence."""

from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DYNAMIC_TOOL = ROOT / "tools/allocator/page_array_dynamic_weight_probe.py"


def read_kv_text(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
        if line and "=" in line:
            key, value = line.split("=", 1)
            values[key] = value
    return values


def run_text(cmd: list[str]) -> str:
    return subprocess.run(
        cmd,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        check=True,
    ).stdout


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    dynamic = read_kv_text(run_text([str(DYNAMIC_TOOL)]))
    release_get_weight = int(dynamic["page_release_array_get_weight"])
    total_weight = int(dynamic["total_array_weight"])
    reduction_percent = (release_get_weight * 100) // total_weight

    lines = [
        "output_contract=page-array-keeper-selection-v0",
        "input_contract=page-array-dynamic-weight-probe-v0",
        "selected_keeper=release_direct_cached_page_known_live_release",
        "keeper_owner=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako",
        "keeper_uses_existing_method=HakoAllocPageModel.releaseLocalKnownLive/1",
        f"expected_dynamic_weight_reduction={release_get_weight}",
        f"expected_dynamic_weight_reduction_percent={reduction_percent}",
        "fallback_preservation=generic_releaseLocal_unchanged",
        "safety_precondition=direct_cached_page_same_page_id_and_cached_page_non_null",
        "compiler_helper_copy_secondary=1",
        "winner_claim=0",
        "replacement_active=0",
        "selected_next=release_direct_cached_page_known_live_release_implementation",
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

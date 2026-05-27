#!/usr/bin/env python3
"""Validate the landed .hako reason-local bind keeper."""

from __future__ import annotations

import argparse
from pathlib import Path


def parse_report(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--shape-report", type=Path, required=True)
    parser.add_argument("--proof-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    shape = parse_report(args.shape_report)
    proof = parse_report(args.proof_report)

    require(shape, "summary", "ok", "shape")
    require(shape, "reason_call_count", "5", "shape")
    require(shape, "duplicate_reason_call_count", "0", "shape")
    require(proof, "summary", "ok", "proof")
    require(proof, "output_summary_ok", "1", "proof")
    require(proof, "allocation_count", "524288", "proof")
    require(proof, "free_count", "524288", "proof")
    require(proof, "select_page_single_fast_path_count", "524288", "proof")
    require(proof, "release_known_page_fast_path_count", "524288", "proof")
    require(proof, "host_replacement", "0", "proof")
    require(proof, "hook_installed", "0", "proof")
    require(proof, "global_allocator_installed", "0", "proof")

    selected_owner = shape.get(
        "selected_owner", "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
    )

    lines = [
        "output_contract=hako-mimalloc-small-alloc-hako-reason-bind-keeper-v0",
        "input_contract=hako-mimalloc-small-alloc-hako-reason-bind-probe-v0",
        f"selected_owner={selected_owner}",
        "keeper=small_alloc_hako_reason_bind",
        "keeper_kind=box_count",
        f"reason_call_count={shape['reason_call_count']}",
        f"duplicate_reason_call_count={shape['duplicate_reason_call_count']}",
        f"allocation_count={proof['allocation_count']}",
        f"free_count={proof['free_count']}",
        f"select_page_single_fast_path_count={proof['select_page_single_fast_path_count']}",
        f"release_known_page_fast_path_count={proof['release_known_page_fast_path_count']}",
        "semantic_summary=ok",
        "next_action=post_hako_reason_bind_measurement",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
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

#!/usr/bin/env python3
"""Estimate dynamic page-local ArrayBox operation weight for the proof workload."""

from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"


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

    proof = read_kv_text(run_text([str(ROOT / "target/release/hakorune"), str(APP)]))
    operation_repeat = int(proof["in_process_operation_repeat"])
    alloc_count = int(proof["allocation_count"])
    release_count = int(proof["free_count"])
    capacity = 64
    reset_count = operation_repeat

    # Current proof path:
    # acquire: free.get + block_used.set per allocation
    # releaseLocal: block_used.get + block_used.set + local_free.set per release
    # resetToFresh: free.set + local_free.set + block_used.set per block per repeat
    page_acquire_array_get_weight = alloc_count
    page_acquire_array_set_weight = alloc_count
    page_release_array_get_weight = release_count
    page_release_array_set_weight = release_count * 2
    reset_array_set_weight = reset_count * capacity * 3
    reset_array_get_weight = 0

    total_array_get_weight = page_acquire_array_get_weight + page_release_array_get_weight
    total_array_set_weight = (
        page_acquire_array_set_weight
        + page_release_array_set_weight
        + reset_array_set_weight
    )
    total_array_weight = total_array_get_weight + total_array_set_weight
    reset_array_weight = reset_array_get_weight + reset_array_set_weight
    alloc_release_array_weight = total_array_weight - reset_array_weight
    reset_array_weight_percent = (reset_array_weight * 100) // total_array_weight
    alloc_release_array_weight_percent = (alloc_release_array_weight * 100) // total_array_weight

    if reset_array_weight_percent >= 50:
        dynamic_owner = "benchmark_harness"
        selected_next = "reset_setup_measurement_split"
    else:
        dynamic_owner = "allocator_page_array_surface"
        selected_next = "page_array_keeper_selection"

    lines = [
        "output_contract=page-array-dynamic-weight-probe-v0",
        "input_contract=mir-builder-post-boxshape-correctness-closeout-v0",
        f"operation_repeat={operation_repeat}",
        f"alloc_count={alloc_count}",
        f"release_count={release_count}",
        f"reset_count={reset_count}",
        f"page_capacity={capacity}",
        f"page_acquire_array_get_weight={page_acquire_array_get_weight}",
        f"page_acquire_array_set_weight={page_acquire_array_set_weight}",
        f"page_release_array_get_weight={page_release_array_get_weight}",
        f"page_release_array_set_weight={page_release_array_set_weight}",
        f"reset_array_get_weight={reset_array_get_weight}",
        f"reset_array_set_weight={reset_array_set_weight}",
        f"total_array_get_weight={total_array_get_weight}",
        f"total_array_set_weight={total_array_set_weight}",
        f"total_array_weight={total_array_weight}",
        f"reset_array_weight={reset_array_weight}",
        f"alloc_release_array_weight={alloc_release_array_weight}",
        f"reset_array_weight_percent={reset_array_weight_percent}",
        f"alloc_release_array_weight_percent={alloc_release_array_weight_percent}",
        f"dynamic_owner={dynamic_owner}",
        "compiler_helper_copy_secondary=1",
        "winner_claim=0",
        "replacement_active=0",
        f"selected_next={selected_next}",
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

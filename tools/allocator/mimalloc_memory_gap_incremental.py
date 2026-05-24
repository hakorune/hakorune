#!/usr/bin/env python3
"""Compute baseline-subtracted RSS evidence without winner claims."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", required=True, type=Path)
    parser.add_argument("--pack", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    baseline = read_kv(args.baseline)
    pack = read_kv(args.pack)

    require(baseline, "output_contract", "mimalloc-comparison-repeated-measurement-v0", "baseline")
    require(pack, "output_contract", "mimalloc-comparison-repeated-measurement-v0", "pack")
    require(baseline, "summary", "ok", "baseline")
    require(pack, "summary", "ok", "pack")
    require(baseline, "workloads", "representative-empty-v0", "baseline")
    require(baseline, "workload_0_operation_family", "empty-baseline", "baseline")
    require(baseline, "winner_claim", "0", "baseline")
    require(pack, "winner_claim", "0", "pack")

    baseline_hako = as_int(baseline, "workload_0_hako_external_rss_median_bytes")
    baseline_c = as_int(baseline, "workload_0_c_external_rss_median_bytes")
    if baseline_hako <= 0 or baseline_c <= 0:
        raise SystemExit("baseline medians must be positive")

    workload_count = as_int(pack, "workload_count")
    if workload_count <= 0:
        raise SystemExit("pack workload_count must be positive")

    lines = [
        "mimalloc_memory_gap_incremental=1",
        "output_contract=mimalloc-comparison-memory-gap-incremental-v0",
        "baseline_workload=representative-empty-v0",
        f"baseline_hako_external_rss_median_bytes={baseline_hako}",
        f"baseline_c_external_rss_median_bytes={baseline_c}",
        f"fixed_process_runtime_baseline_delta_bytes={baseline_hako - baseline_c}",
        f"workload_count={workload_count}",
        "winner_claim=0",
    ]

    for idx in range(workload_count):
        workload = pack.get(f"workload_{idx}_id", "")
        operation_family = pack.get(f"workload_{idx}_operation_family", "")
        hako_total = as_int(pack, f"workload_{idx}_hako_external_rss_median_bytes")
        c_total = as_int(pack, f"workload_{idx}_c_external_rss_median_bytes")
        hako_incremental = hako_total - baseline_hako
        c_incremental = c_total - baseline_c
        total_delta = hako_total - c_total
        incremental_delta = hako_incremental - c_incremental
        residual_delta = total_delta - (baseline_hako - baseline_c) - incremental_delta
        prefix = f"workload_{idx}"
        lines.extend(
            [
                f"{prefix}_id={workload}",
                f"{prefix}_operation_family={operation_family}",
                f"{prefix}_hako_total_rss_median_bytes={hako_total}",
                f"{prefix}_c_total_rss_median_bytes={c_total}",
                f"{prefix}_total_delta_bytes={total_delta}",
                f"{prefix}_hako_incremental_rss_bytes={hako_incremental}",
                f"{prefix}_c_incremental_rss_bytes={c_incremental}",
                f"{prefix}_incremental_delta_bytes={incremental_delta}",
                f"{prefix}_unattributed_residual_bytes={residual_delta}",
                f"{prefix}_winner_claim=0",
            ]
        )

    lines.extend(
        [
            "provider_activation=0",
            "host_replacement=0",
            "hook_installed=0",
            "global_allocator_installed=0",
            "summary=ok",
        ]
    )
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

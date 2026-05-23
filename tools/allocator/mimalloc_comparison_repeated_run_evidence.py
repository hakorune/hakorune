#!/usr/bin/env python3
"""Aggregate repeated same-workload RSS presentation samples without winner claims."""

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
    parser = argparse.ArgumentParser(
        description="Build repeated-run RSS evidence from presentation samples."
    )
    parser.add_argument("--sample", action="append", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    if len(args.sample) < 2:
        raise SystemExit("at least two --sample files are required")

    samples = [read_kv(path) for path in args.sample]
    hako_workload = samples[0].get("hako_workload", "")
    c_workload = samples[0].get("c_workload", "")

    hako_peaks: list[int] = []
    c_peaks: list[int] = []
    deltas: list[int] = []
    abs_deltas: list[int] = []

    for index, sample in enumerate(samples):
        label = f"sample[{index}]"
        require(sample, "output_contract", "mimalloc-comparison-rss-presentation-v0", label)
        require(sample, "summary", "ok", label)
        require(sample, "measurement_scope", "single-run", label)
        require(sample, "rss_unit", "bytes", label)
        require(sample, "workload_match", "1", label)
        require(sample, "requested_bytes_delta", "0", label)
        require(sample, "repeated_runs", "0", label)
        require(sample, "winner_claim", "0", label)
        require(sample, "provider_activation", "0", label)
        require(sample, "host_replacement", "0", label)
        require(sample, "hook_installed", "0", label)
        require(sample, "global_allocator_installed", "0", label)
        if sample.get("hako_workload", "") != hako_workload:
            raise SystemExit(f"{label}: hako_workload changed")
        if sample.get("c_workload", "") != c_workload:
            raise SystemExit(f"{label}: c_workload changed")

        hako_peak = as_int(sample, "hako_peak_rss_bytes")
        c_peak = as_int(sample, "c_peak_rss_bytes")
        delta = as_int(sample, "peak_rss_bytes_delta")
        abs_delta = as_int(sample, "peak_rss_abs_delta_bytes")
        if hako_peak <= 0 or c_peak <= 0:
            raise SystemExit(f"{label}: RSS peaks must be positive")
        if abs(delta) != abs_delta:
            raise SystemExit(f"{label}: absolute delta mismatch")
        hako_peaks.append(hako_peak)
        c_peaks.append(c_peak)
        deltas.append(delta)
        abs_deltas.append(abs_delta)

    lines = [
        "mimalloc_comparison_repeated_run_evidence=1",
        "output_contract=mimalloc-comparison-repeated-run-evidence-v0",
        "measurement_scope=repeated-rss-samples",
        "rss_unit=bytes",
        f"sample_count={len(samples)}",
        f"hako_workload={hako_workload}",
        f"c_workload={c_workload}",
        "workload_match=1",
        "requested_bytes_delta=0",
        f"hako_peak_rss_min_bytes={min(hako_peaks)}",
        f"hako_peak_rss_max_bytes={max(hako_peaks)}",
        f"c_peak_rss_min_bytes={min(c_peaks)}",
        f"c_peak_rss_max_bytes={max(c_peaks)}",
        f"peak_rss_delta_min_bytes={min(deltas)}",
        f"peak_rss_delta_max_bytes={max(deltas)}",
        f"peak_rss_abs_delta_min_bytes={min(abs_deltas)}",
        f"peak_rss_abs_delta_max_bytes={max(abs_deltas)}",
        "winner_claim=0",
        "provider_activation=0",
        "host_replacement=0",
        "hook_installed=0",
        "global_allocator_installed=0",
        "summary=ok",
    ]
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

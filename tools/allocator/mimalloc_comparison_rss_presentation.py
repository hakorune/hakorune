#!/usr/bin/env python3
"""Format single-run RSS evidence without making a winner claim."""

from __future__ import annotations

import argparse
from pathlib import Path


MIB = 1024 * 1024


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


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def mib_x100(value: int) -> int:
    return (value * 100) // MIB


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Build a presentation-only RSS report from memory evidence."
    )
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    report = read_kv(args.report)
    require(report, "output_contract", "mimalloc-comparison-memory-report-v0")
    require(report, "summary", "ok")
    require(report, "workload_match", "1")
    require(report, "requested_bytes_delta", "0")
    require(report, "winner_claim", "0")

    hako_peak = as_int(report, "hako_peak_rss_bytes")
    c_peak = as_int(report, "c_peak_rss_bytes")
    delta = hako_peak - c_peak

    lines = [
        "mimalloc_comparison_rss_presentation=1",
        "output_contract=mimalloc-comparison-rss-presentation-v0",
        "measurement_scope=single-run",
        "rss_unit=bytes",
        f"hako_workload={report.get('hako_workload', '')}",
        f"c_workload={report.get('c_workload', '')}",
        "workload_match=1",
        "requested_bytes_delta=0",
        f"hako_peak_rss_bytes={hako_peak}",
        f"c_peak_rss_bytes={c_peak}",
        f"peak_rss_bytes_delta={delta}",
        f"peak_rss_abs_delta_bytes={abs(delta)}",
        f"hako_peak_rss_mib_x100={mib_x100(hako_peak)}",
        f"c_peak_rss_mib_x100={mib_x100(c_peak)}",
        f"peak_rss_abs_delta_mib_x100={mib_x100(abs(delta))}",
        "repeated_runs=0",
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

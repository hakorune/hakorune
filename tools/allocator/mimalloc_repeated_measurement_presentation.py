#!/usr/bin/env python3
"""Format repeated measurement evidence without winner claims."""

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


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def mib_x100(value: int) -> int:
    return (value * 100) // (1024 * 1024)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    report = read_kv(args.report)
    require(report, "output_contract", "mimalloc-comparison-repeated-measurement-v0")
    require(report, "measurement_profile", "phase295x-repeated-v0")
    require(report, "canonical_rss_collector", "external-time")
    require(report, "winner_claim", "0")
    require(report, "summary", "ok")
    require(report, "provider_activation", "0")
    require(report, "host_replacement", "0")
    require(report, "hook_installed", "0")
    require(report, "global_allocator_installed", "0")

    workload_count = as_int(report, "workload_count")
    if workload_count <= 0:
        raise SystemExit("workload_count must be positive")

    lines = [
        "mimalloc_repeated_measurement_presentation=1",
        "output_contract=mimalloc-comparison-repeated-measurement-presentation-v0",
        "input_contract=mimalloc-comparison-repeated-measurement-v0",
        "measurement_profile=phase295x-repeated-v0",
        f"workload_count={workload_count}",
        "rss_unit=bytes",
        "rss_display_unit=mib_x100",
        "canonical_rss_collector=external-time",
    ]

    for idx in range(workload_count):
        workload = report.get(f"workload_{idx}_id", "")
        family = report.get(f"workload_{idx}_operation_family", "")
        sample_count = as_int(report, f"workload_{idx}_sample_count")
        hako_median = as_int(report, f"workload_{idx}_hako_external_rss_median_bytes")
        c_median = as_int(report, f"workload_{idx}_c_external_rss_median_bytes")
        hako_min = as_int(report, f"workload_{idx}_hako_external_rss_min_bytes")
        c_min = as_int(report, f"workload_{idx}_c_external_rss_min_bytes")
        hako_max = as_int(report, f"workload_{idx}_hako_external_rss_max_bytes")
        c_max = as_int(report, f"workload_{idx}_c_external_rss_max_bytes")
        if not workload or not family:
            raise SystemExit(f"workload {idx} identity missing")
        if hako_median <= 0 or c_median <= 0:
            raise SystemExit(f"workload {idx} median RSS must be positive")
        delta = hako_median - c_median
        lines.extend(
            [
                f"workload_{idx}_id={workload}",
                f"workload_{idx}_operation_family={family}",
                f"workload_{idx}_sample_count={sample_count}",
                f"workload_{idx}_hako_external_rss_min_bytes={hako_min}",
                f"workload_{idx}_hako_external_rss_median_bytes={hako_median}",
                f"workload_{idx}_hako_external_rss_max_bytes={hako_max}",
                f"workload_{idx}_c_external_rss_min_bytes={c_min}",
                f"workload_{idx}_c_external_rss_median_bytes={c_median}",
                f"workload_{idx}_c_external_rss_max_bytes={c_max}",
                f"workload_{idx}_external_rss_median_delta_bytes={delta}",
                f"workload_{idx}_hako_external_rss_median_mib_x100={mib_x100(hako_median)}",
                f"workload_{idx}_c_external_rss_median_mib_x100={mib_x100(c_median)}",
                f"workload_{idx}_external_rss_median_delta_mib_x100={mib_x100(delta)}",
                f"workload_{idx}_presentation_only=1",
                f"workload_{idx}_winner_claim=0",
            ]
        )

    lines.extend(
        [
            "presentation_only=1",
            "provider_activation=0",
            "host_replacement=0",
            "hook_installed=0",
            "global_allocator_installed=0",
            "winner_claim=0",
            "summary=ok",
        ]
    )
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

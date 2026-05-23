#!/usr/bin/env python3
"""Format repeated-run mimalloc comparison evidence without claiming a winner."""

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


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Build a no-winner summary from repeated-run RSS evidence."
    )
    parser.add_argument("--evidence", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    evidence = read_kv(args.evidence)
    require(evidence, "output_contract", "mimalloc-comparison-repeated-run-evidence-v0")
    require(evidence, "summary", "ok")
    require(evidence, "measurement_scope", "repeated-rss-samples")
    require(evidence, "rss_unit", "bytes")
    require(evidence, "workload_match", "1")
    require(evidence, "requested_bytes_delta", "0")
    require(evidence, "winner_claim", "0")
    require(evidence, "provider_activation", "0")
    require(evidence, "host_replacement", "0")
    require(evidence, "hook_installed", "0")
    require(evidence, "global_allocator_installed", "0")

    sample_count = as_int(evidence, "sample_count")
    if sample_count < 2:
        raise SystemExit(f"sample_count must be at least 2, got {sample_count}")

    lines = [
        "mimalloc_comparison_summary_no_winner=1",
        "output_contract=mimalloc-comparison-summary-no-winner-v0",
        "measurement_scope=repeated-rss-samples",
        "rss_unit=bytes",
        f"sample_count={sample_count}",
        f"hako_workload={evidence.get('hako_workload', '')}",
        f"c_workload={evidence.get('c_workload', '')}",
        "workload_match=1",
        "requested_bytes_delta=0",
        f"hako_peak_rss_range_bytes={evidence['hako_peak_rss_min_bytes']}..{evidence['hako_peak_rss_max_bytes']}",
        f"c_peak_rss_range_bytes={evidence['c_peak_rss_min_bytes']}..{evidence['c_peak_rss_max_bytes']}",
        f"peak_rss_delta_range_bytes={evidence['peak_rss_delta_min_bytes']}..{evidence['peak_rss_delta_max_bytes']}",
        f"peak_rss_abs_delta_range_bytes={evidence['peak_rss_abs_delta_min_bytes']}..{evidence['peak_rss_abs_delta_max_bytes']}",
        "comparison_claim=range-only",
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

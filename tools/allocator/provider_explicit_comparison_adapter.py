#!/usr/bin/env python3
"""Adapt landed hako/C/provider evidence into the 3-way comparison contract."""

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


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"{label}: missing {key}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hako-c-report", type=Path, required=True)
    parser.add_argument("--provider-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--workload-index", type=int, default=0)
    args = parser.parse_args()

    hako_c = read_kv(args.hako_c_report)
    provider = read_kv(args.provider_report)
    require(hako_c, "output_contract", "mimalloc-comparison-repeated-measurement-v0", "hako_c")
    require(provider, "output_contract", "hakorune-provider-explicit-repeated-measurement-v0", "provider")
    require(hako_c, "winner_claim", "0", "hako_c")
    require(provider, "winner_claim", "0", "provider")
    require(provider, "replacement_active", "0", "provider")
    require(provider, "hook_installed", "0", "provider")
    require(provider, "global_allocator", "0", "provider")

    prefix = f"workload_{args.workload_index}"
    workload = require_key(hako_c, f"{prefix}_id", "hako_c")
    sample_count = require_key(hako_c, f"{prefix}_sample_count", "hako_c")
    operation_repeat = require_key(hako_c, f"{prefix}_operation_repeat", "hako_c")
    operation_family = require_key(hako_c, f"{prefix}_operation_family", "hako_c")
    warmup_count = require_key(hako_c, "warmup_count", "hako_c")
    require(provider, "sample_count", sample_count, "provider")
    require(provider, "warmup_count", warmup_count, "provider")
    require(provider, "operation_repeat", operation_repeat, "provider")

    lines = [
        "output_contract=mimalloc-provider-explicit-comparison-adapter-v0",
        "input_contract=mimalloc-provider-explicit-comparison-contract-v0",
        "measurement_profile=phase296x-provider-explicit-comparison-v0",
        "comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free",
        f"workload_id={workload}",
        f"operation_family={operation_family}",
        f"sample_count={sample_count}",
        f"warmup_count={warmup_count}",
        f"operation_repeat={operation_repeat}",
        "summary_statistic=min,median,max",
        "subject_count=3",
        "subject_0_id=hako_exact_exe",
        "subject_0_elapsed_median_unit=ms",
        f"subject_0_elapsed_median_ms={require_key(hako_c, f'{prefix}_hako_external_elapsed_median_ms', 'hako_c')}",
        f"subject_0_rss_median_bytes={require_key(hako_c, f'{prefix}_hako_external_rss_median_bytes', 'hako_c')}",
        "subject_0_winner_claim=0",
        "subject_1_id=c_mimalloc_explicit_runner",
        "subject_1_elapsed_median_unit=ms",
        f"subject_1_elapsed_median_ms={require_key(hako_c, f'{prefix}_c_external_elapsed_median_ms', 'hako_c')}",
        f"subject_1_rss_median_bytes={require_key(hako_c, f'{prefix}_c_external_rss_median_bytes', 'hako_c')}",
        "subject_1_winner_claim=0",
        "subject_2_id=provider_package_explicit_alloc_free",
        "subject_2_elapsed_median_unit=ns",
        f"subject_2_elapsed_median_ns={require_key(provider, 'sample_elapsed_median_ns', 'provider')}",
        f"subject_2_rss_median_bytes={require_key(provider, 'sample_rss_median_bytes', 'provider')}",
        "subject_2_winner_claim=0",
        "provider_activation_lane=parked",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

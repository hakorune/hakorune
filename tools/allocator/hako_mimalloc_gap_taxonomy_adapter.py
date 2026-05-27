#!/usr/bin/env python3
"""Classify hako/C mimalloc parity gaps without making winner claims."""

from __future__ import annotations

import argparse
from pathlib import Path


ALLOWED_OWNERS = {
    "allocator_algorithm",
    "compiler_lowering",
    "hako_runtime_baseline",
    "c_abi_memory_bridge",
    "osvm_page_source",
    "provider_wrapper",
    "benchmark_harness",
}


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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = require_key(values, key, label)
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def ratio(num: int, den: int) -> str:
    if den <= 0:
        raise SystemExit("ratio denominator must be positive")
    return f"{num / den:.3f}"


def classify(
    hako_min: int,
    hako_median: int,
    hako_max: int,
    c_min: int,
    c_median: int,
    c_max: int,
) -> tuple[str, str, str, str]:
    hako_ratio = hako_max / hako_median
    c_ratio = c_max / c_median
    outlier_observed = hako_ratio >= 3.0 or c_ratio >= 3.0

    if outlier_observed:
        return ("1", "noisy", "benchmark_harness", "medium")

    median_gap = hako_median - c_median
    if abs(median_gap) <= max(10, int(max(c_median, 1) * 0.25)):
        return ("0", "stable", "hako_runtime_baseline", "low")

    if median_gap > 0:
        return ("0", "stable", "hako_runtime_baseline", "medium")

    return ("0", "stable", "benchmark_harness", "low")


def next_diagnostic(owner: str, evidence_quality: str, confidence: str) -> str:
    if owner == "benchmark_harness" or evidence_quality == "noisy":
        return "measurement_hygiene_refresh"
    if confidence == "low":
        return "owner_confidence_refresh"
    if owner == "hako_runtime_baseline":
        return "empty_workload_or_repeat_scaling_runtime_diagnostic"
    if owner == "compiler_lowering":
        return "mir_or_body_shape_diagnostic"
    if owner == "allocator_algorithm":
        return "operation_repeat_scaling_or_allocator_counter_diagnostic"
    if owner == "c_abi_memory_bridge":
        return "c_runner_api_or_load_boundary_diagnostic"
    if owner == "provider_wrapper":
        return "provider_explicit_call_overhead_diagnostic"
    raise SystemExit(f"unknown owner: {owner}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--workload-index", type=int, default=0)
    args = parser.parse_args()

    values = read_kv(args.input)
    require(values, "output_contract", "mimalloc-comparison-repeated-measurement-v0", "input")
    require(values, "winner_claim", "0", "input")
    require(values, "provider_activation", "0", "input")
    require(values, "host_replacement", "0", "input")
    require(values, "hook_installed", "0", "input")
    require(values, "global_allocator_installed", "0", "input")
    require(values, "summary", "ok", "input")

    prefix = f"workload_{args.workload_index}"
    workload_id = require_key(values, f"{prefix}_id", "input")
    hako_min = require_int(values, f"{prefix}_hako_external_elapsed_min_ms", "input")
    hako_median = require_int(values, f"{prefix}_hako_external_elapsed_median_ms", "input")
    hako_max = require_int(values, f"{prefix}_hako_external_elapsed_max_ms", "input")
    c_min = require_int(values, f"{prefix}_c_external_elapsed_min_ms", "input")
    c_median = require_int(values, f"{prefix}_c_external_elapsed_median_ms", "input")
    c_max = require_int(values, f"{prefix}_c_external_elapsed_max_ms", "input")
    hako_rss = require_int(values, f"{prefix}_hako_external_rss_median_bytes", "input")
    c_rss = require_int(values, f"{prefix}_c_external_rss_median_bytes", "input")

    if hako_min <= 0 or hako_median <= 0 or hako_max <= 0:
        raise SystemExit("input: hako elapsed values must be positive")
    if c_min <= 0 or c_median <= 0 or c_max <= 0:
        raise SystemExit("input: c elapsed values must be positive")
    if hako_min > hako_median or hako_median > hako_max:
        raise SystemExit("input: hako elapsed min/median/max order invalid")
    if c_min > c_median or c_median > c_max:
        raise SystemExit("input: c elapsed min/median/max order invalid")

    outlier, quality, owner, confidence = classify(hako_min, hako_median, hako_max, c_min, c_median, c_max)
    if owner not in ALLOWED_OWNERS:
        raise SystemExit(f"classifier produced invalid owner: {owner}")

    median_gap = hako_median - c_median
    rss_gap = hako_rss - c_rss
    diagnostic = next_diagnostic(owner, quality, confidence)
    optimization_allowed = "1" if quality == "stable" and confidence != "low" and owner in {
        "compiler_lowering",
        "allocator_algorithm",
    } else "0"

    lines = [
        "output_contract=hako-mimalloc-gap-taxonomy-v0",
        "input_contract=mimalloc-comparison-repeated-measurement-v0",
        f"workload_id={workload_id}",
        f"measurement_profile={require_key(values, 'measurement_profile', 'input')}",
        "hako_subject=hako_mimalloc_exact_exe",
        "c_subject=c_mimalloc_explicit_runner",
        f"sample_count={require_key(values, f'{prefix}_sample_count', 'input')}",
        f"warmup_count={require_key(values, 'warmup_count', 'input')}",
        f"operation_repeat={require_key(values, f'{prefix}_operation_repeat', 'input')}",
        f"hako_elapsed_min_ms={hako_min}",
        f"hako_elapsed_median_ms={hako_median}",
        f"hako_elapsed_max_ms={hako_max}",
        f"c_elapsed_min_ms={c_min}",
        f"c_elapsed_median_ms={c_median}",
        f"c_elapsed_max_ms={c_max}",
        f"elapsed_median_gap_ms={median_gap}",
        f"elapsed_median_ratio={ratio(hako_median, c_median)}",
        f"hako_rss_median_bytes={hako_rss}",
        f"c_rss_median_bytes={c_rss}",
        f"rss_median_gap_bytes={rss_gap}",
        f"hako_max_to_median_ratio={ratio(hako_max, hako_median)}",
        f"c_max_to_median_ratio={ratio(c_max, c_median)}",
        f"outlier_observed={outlier}",
        f"evidence_quality={quality}",
        f"gap_owner={owner}",
        f"gap_confidence={confidence}",
        f"next_diagnostic={diagnostic}",
        f"next_optimization_allowed={optimization_allowed}",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
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

#!/usr/bin/env python3
"""Refresh confidence for stable low-confidence hako mimalloc owner evidence."""

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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = require_key(values, key, label)
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--taxonomy", type=Path, required=True)
    parser.add_argument("--empty-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    taxonomy = read_kv(args.taxonomy)
    require(taxonomy, "output_contract", "hako-mimalloc-gap-taxonomy-v0", "taxonomy")
    require(taxonomy, "winner_claim", "0", "taxonomy")
    require(taxonomy, "provider_active", "0", "taxonomy")
    require(taxonomy, "replacement_active", "0", "taxonomy")
    require(taxonomy, "hook_installed", "0", "taxonomy")
    require(taxonomy, "global_allocator", "0", "taxonomy")
    require(taxonomy, "summary", "ok", "taxonomy")

    empty = read_kv(args.empty_report)
    require(empty, "output_contract", "mimalloc-comparison-repeated-measurement-v0", "empty")
    require(empty, "winner_claim", "0", "empty")
    require(empty, "provider_activation", "0", "empty")
    require(empty, "host_replacement", "0", "empty")
    require(empty, "hook_installed", "0", "empty")
    require(empty, "global_allocator_installed", "0", "empty")
    require(empty, "summary", "ok", "empty")

    small_gap = require_int(taxonomy, "elapsed_median_gap_ms", "taxonomy")
    empty_hako = require_int(empty, "workload_0_hako_external_elapsed_median_ms", "empty")
    empty_c = require_int(empty, "workload_0_c_external_elapsed_median_ms", "empty")
    empty_gap = empty_hako - empty_c

    if abs(empty_gap) >= max(10, abs(small_gap) // 2):
        refreshed_owner = "hako_runtime_baseline"
        refreshed_confidence = "medium"
        next_diagnostic = "repeat_scaling_runtime_diagnostic"
        next_optimization_allowed = "0"
    else:
        refreshed_owner = require_key(taxonomy, "gap_owner", "taxonomy")
        refreshed_confidence = "low"
        next_diagnostic = "operation_repeat_scaling_or_allocator_counter_diagnostic"
        next_optimization_allowed = "0"

    lines = [
        "output_contract=hako-mimalloc-owner-confidence-refresh-v0",
        "input_contract=hako-mimalloc-gap-taxonomy-v0",
        f"workload_id={require_key(taxonomy, 'workload_id', 'taxonomy')}",
        "confidence_refresh_kind=empty_workload_runtime_baseline",
        f"original_gap_owner={require_key(taxonomy, 'gap_owner', 'taxonomy')}",
        f"original_gap_confidence={require_key(taxonomy, 'gap_confidence', 'taxonomy')}",
        f"small_elapsed_median_gap_ms={small_gap}",
        "empty_workload_id=representative-empty-v0",
        f"empty_hako_elapsed_median_ms={empty_hako}",
        f"empty_c_elapsed_median_ms={empty_c}",
        f"empty_elapsed_median_gap_ms={empty_gap}",
        f"refreshed_gap_owner={refreshed_owner}",
        f"refreshed_gap_confidence={refreshed_confidence}",
        f"next_diagnostic={next_diagnostic}",
        f"next_optimization_allowed={next_optimization_allowed}",
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

#!/usr/bin/env python3
"""Classify hako mimalloc elapsed gap scaling across process invocation repeats."""

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


def parse_report_arg(text: str) -> tuple[int, Path]:
    if ":" not in text:
        raise SystemExit("--report must be formatted as REPEAT:PATH")
    repeat_text, path_text = text.split(":", 1)
    try:
        repeat = int(repeat_text)
    except ValueError as exc:
        raise SystemExit(f"invalid repeat in --report: {repeat_text!r}") from exc
    if repeat <= 0:
        raise SystemExit(f"repeat must be positive: {repeat}")
    return repeat, Path(path_text)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", action="append", required=True, help="REPEAT:PATH")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    reports = sorted(parse_report_arg(item) for item in args.report)
    if len(reports) < 2:
        raise SystemExit("at least two --report entries are required")

    rows: list[dict[str, int | str]] = []
    workload_id: str | None = None
    sample_count: str | None = None
    for repeat, path in reports:
        values = read_kv(path)
        label = f"report({repeat})"
        require(values, "output_contract", "mimalloc-comparison-repeated-measurement-v0", label)
        require(values, "winner_claim", "0", label)
        require(values, "provider_activation", "0", label)
        require(values, "host_replacement", "0", label)
        require(values, "hook_installed", "0", label)
        require(values, "global_allocator_installed", "0", label)
        require(values, "summary", "ok", label)
        observed_repeat = require_int(values, "workload_0_operation_repeat", label)
        if observed_repeat != repeat:
            raise SystemExit(f"{label}: repeat mismatch, expected {repeat}, got {observed_repeat}")
        current_workload = require_key(values, "workload_0_id", label)
        if workload_id is None:
            workload_id = current_workload
        elif workload_id != current_workload:
            raise SystemExit(f"{label}: workload mismatch, expected {workload_id}, got {current_workload}")
        current_sample_count = require_key(values, "workload_0_sample_count", label)
        if sample_count is None:
            sample_count = current_sample_count
        elif sample_count != current_sample_count:
            raise SystemExit(f"{label}: sample_count mismatch, expected {sample_count}, got {current_sample_count}")

        hako_median = require_int(values, "workload_0_hako_external_elapsed_median_ms", label)
        c_median = require_int(values, "workload_0_c_external_elapsed_median_ms", label)
        gap = hako_median - c_median
        rows.append(
            {
                "repeat": repeat,
                "hako_median": hako_median,
                "c_median": c_median,
                "gap": gap,
            }
        )

    baseline = rows[0]
    highest = rows[-1]
    baseline_gap = int(baseline["gap"])
    highest_gap = int(highest["gap"])
    gap_growth = highest_gap - baseline_gap
    repeat_growth = int(highest["repeat"]) - int(baseline["repeat"])
    per_invocation_gap_growth_us = 0
    if repeat_growth > 0:
        per_invocation_gap_growth_us = (gap_growth * 1000) // repeat_growth

    growth_threshold_ms = max(50, abs(baseline_gap) * 4)
    per_invocation_growth_observed = "1" if gap_growth >= growth_threshold_ms else "0"
    fixed_runtime_gap_observed = "0" if per_invocation_growth_observed == "1" else "1"
    if per_invocation_growth_observed == "1":
        refreshed_owner = "process_invocation_scaling_gap"
        refreshed_confidence = "medium"
        next_diagnostic = "runtime_vs_workload_repeat_split_diagnostic"
    else:
        refreshed_owner = "hako_runtime_baseline"
        refreshed_confidence = "medium"
        next_diagnostic = "runtime_baseline_closeout_or_process_shell_diagnostic"

    lines = [
        "output_contract=hako-mimalloc-runtime-baseline-scaling-diagnostic-v0",
        "input_contract=mimalloc-comparison-repeated-measurement-v0",
        f"workload_id={workload_id}",
        f"sample_count={sample_count}",
        "warmup_count=1",
        f"repeat_count={len(rows)}",
    ]
    for idx, row in enumerate(rows):
        lines.extend(
            [
                f"repeat_{idx}_operation_repeat={row['repeat']}",
                f"repeat_{idx}_hako_elapsed_median_ms={row['hako_median']}",
                f"repeat_{idx}_c_elapsed_median_ms={row['c_median']}",
                f"repeat_{idx}_elapsed_gap_ms={row['gap']}",
            ]
        )
    lines.extend(
        [
            f"baseline_operation_repeat={baseline['repeat']}",
            f"baseline_elapsed_gap_ms={baseline_gap}",
            f"max_operation_repeat={highest['repeat']}",
            f"max_elapsed_gap_ms={highest_gap}",
            f"gap_growth_ms={gap_growth}",
            f"per_invocation_gap_growth_us={per_invocation_gap_growth_us}",
            f"per_invocation_growth_observed={per_invocation_growth_observed}",
            f"runtime_baseline_fixed_gap_observed={fixed_runtime_gap_observed}",
            f"refreshed_gap_owner={refreshed_owner}",
            f"refreshed_gap_confidence={refreshed_confidence}",
            f"next_diagnostic={next_diagnostic}",
            "next_optimization_allowed=0",
            "winner_claim=0",
            "provider_active=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

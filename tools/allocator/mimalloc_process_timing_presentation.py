#!/usr/bin/env python3
"""Format process-repeat timing evidence without allocator-body timing claims."""

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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    report = read_kv(args.report)
    require(report, "output_contract", "mimalloc-comparison-repeated-measurement-v0")
    require(report, "measurement_profile", "phase295x-repeated-v0")
    require(report, "canonical_rss_collector", "external-time")
    require(report, "timing_repeat_kind", "process-invocation-v0")
    require(report, "winner_claim", "0")
    require(report, "summary", "ok")
    require(report, "provider_activation", "0")
    require(report, "host_replacement", "0")
    require(report, "hook_installed", "0")
    require(report, "global_allocator_installed", "0")

    operation_repeat = as_int(report, "operation_repeat")
    if operation_repeat <= 1:
        raise SystemExit("operation_repeat must be greater than 1 for process timing presentation")
    workload_count = as_int(report, "workload_count")
    if workload_count <= 0:
        raise SystemExit("workload_count must be positive")

    lines = [
        "mimalloc_process_timing_presentation=1",
        "output_contract=mimalloc-comparison-process-timing-presentation-v0",
        "input_contract=mimalloc-comparison-repeated-measurement-v0",
        "measurement_profile=phase295x-repeated-v0",
        f"workload_count={workload_count}",
        f"operation_repeat={operation_repeat}",
        "timing_repeat_kind=process-invocation-v0",
        "timing_claim_kind=process-repeat-presentation-only",
        "allocator_body_timing=0",
        "process_runtime_cost_included=1",
        "evidence_output_cost_included=1",
        "rss_evidence_present=1",
        "canonical_rss_collector=external-time",
    ]

    for idx in range(workload_count):
        prefix = f"workload_{idx}"
        workload = report.get(f"{prefix}_id", "")
        family = report.get(f"{prefix}_operation_family", "")
        sample_count = as_int(report, f"{prefix}_sample_count")
        hako_elapsed = as_int(report, f"{prefix}_hako_external_elapsed_median_ms")
        c_elapsed = as_int(report, f"{prefix}_c_external_elapsed_median_ms")
        hako_rss = as_int(report, f"{prefix}_hako_external_rss_median_bytes")
        c_rss = as_int(report, f"{prefix}_c_external_rss_median_bytes")
        repeat = as_int(report, f"{prefix}_operation_repeat")
        kind = report.get(f"{prefix}_timing_repeat_kind", "")
        if not workload or not family:
            raise SystemExit(f"{prefix}: workload identity missing")
        if repeat != operation_repeat:
            raise SystemExit(f"{prefix}: operation_repeat mismatch")
        if kind != "process-invocation-v0":
            raise SystemExit(f"{prefix}: timing_repeat_kind mismatch")
        if hako_elapsed <= 1 or c_elapsed <= 1:
            raise SystemExit(f"{prefix}: elapsed medians must escape the 1ms floor")
        if hako_rss <= 0 or c_rss <= 0:
            raise SystemExit(f"{prefix}: RSS medians must be positive")

        lines.extend(
            [
                f"{prefix}_id={workload}",
                f"{prefix}_operation_family={family}",
                f"{prefix}_sample_count={sample_count}",
                f"{prefix}_operation_repeat={repeat}",
                f"{prefix}_timing_repeat_kind=process-invocation-v0",
                f"{prefix}_hako_process_elapsed_median_ms={hako_elapsed}",
                f"{prefix}_c_process_elapsed_median_ms={c_elapsed}",
                f"{prefix}_process_elapsed_median_delta_ms={hako_elapsed - c_elapsed}",
                f"{prefix}_hako_external_rss_median_bytes={hako_rss}",
                f"{prefix}_c_external_rss_median_bytes={c_rss}",
                f"{prefix}_external_rss_median_delta_bytes={hako_rss - c_rss}",
                f"{prefix}_allocator_body_timing=0",
                f"{prefix}_presentation_only=1",
                f"{prefix}_winner_claim=0",
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

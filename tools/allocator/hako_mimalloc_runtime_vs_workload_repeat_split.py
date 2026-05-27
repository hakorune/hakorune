#!/usr/bin/env python3
"""Split process-repeat scaling between empty runtime baseline and workload cost."""

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
        raise SystemExit("--empty-report/--small-report must be formatted as REPEAT:PATH")
    repeat_text, path_text = text.split(":", 1)
    try:
        repeat = int(repeat_text)
    except ValueError as exc:
        raise SystemExit(f"invalid repeat: {repeat_text!r}") from exc
    if repeat <= 0:
        raise SystemExit(f"repeat must be positive: {repeat}")
    return repeat, Path(path_text)


def load_reports(items: list[str], expected_workload: str, label: str) -> list[tuple[int, int]]:
    reports = sorted(parse_report_arg(item) for item in items)
    if len(reports) < 2:
        raise SystemExit(f"{label}: at least two reports are required")
    rows: list[tuple[int, int]] = []
    for repeat, path in reports:
        values = read_kv(path)
        entry = f"{label}({repeat})"
        require(values, "output_contract", "mimalloc-comparison-repeated-measurement-v0", entry)
        require(values, "winner_claim", "0", entry)
        require(values, "provider_activation", "0", entry)
        require(values, "host_replacement", "0", entry)
        require(values, "hook_installed", "0", entry)
        require(values, "global_allocator_installed", "0", entry)
        require(values, "summary", "ok", entry)
        require(values, "workload_0_id", expected_workload, entry)
        observed_repeat = require_int(values, "workload_0_operation_repeat", entry)
        if observed_repeat != repeat:
            raise SystemExit(f"{entry}: repeat mismatch, expected {repeat}, got {observed_repeat}")
        hako = require_int(values, "workload_0_hako_external_elapsed_median_ms", entry)
        c = require_int(values, "workload_0_c_external_elapsed_median_ms", entry)
        rows.append((repeat, hako - c))
    return rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--empty-report", action="append", required=True, help="REPEAT:PATH")
    parser.add_argument("--small-report", action="append", required=True, help="REPEAT:PATH")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    empty_rows = load_reports(args.empty_report, "representative-empty-v0", "empty")
    small_rows = load_reports(args.small_report, "representative-small-block-v0", "small")
    empty_repeats = [repeat for repeat, _ in empty_rows]
    small_repeats = [repeat for repeat, _ in small_rows]
    if empty_repeats != small_repeats:
        raise SystemExit(f"repeat ladder mismatch: empty={empty_repeats}, small={small_repeats}")

    empty_base_gap = empty_rows[0][1]
    empty_max_gap = empty_rows[-1][1]
    small_base_gap = small_rows[0][1]
    small_max_gap = small_rows[-1][1]
    empty_growth = empty_max_gap - empty_base_gap
    small_growth = small_max_gap - small_base_gap
    if small_growth <= 0:
        runtime_explains_ratio_pct = 100
    else:
        runtime_explains_ratio_pct = max(0, min(200, (empty_growth * 100) // small_growth))

    if runtime_explains_ratio_pct >= 80:
        selected_owner = "benchmark_harness"
        selected_confidence = "high"
        next_diagnostic = "in_process_operation_repeat_contract"
        next_optimization_allowed = "0"
    else:
        selected_owner = "allocator_algorithm"
        selected_confidence = "low"
        next_diagnostic = "compiler_allocator_owner_split_diagnostic"
        next_optimization_allowed = "0"

    lines = [
        "output_contract=hako-mimalloc-runtime-vs-workload-repeat-split-v0",
        "input_contract=mimalloc-comparison-repeated-measurement-v0",
        "empty_workload_id=representative-empty-v0",
        "small_workload_id=representative-small-block-v0",
        f"repeat_count={len(empty_rows)}",
    ]
    for idx, ((repeat, empty_gap), (_, small_gap)) in enumerate(zip(empty_rows, small_rows)):
        lines.extend(
            [
                f"repeat_{idx}_operation_repeat={repeat}",
                f"repeat_{idx}_empty_elapsed_gap_ms={empty_gap}",
                f"repeat_{idx}_small_elapsed_gap_ms={small_gap}",
            ]
        )
    lines.extend(
        [
            f"empty_gap_growth_ms={empty_growth}",
            f"small_gap_growth_ms={small_growth}",
            f"runtime_explains_ratio_pct={runtime_explains_ratio_pct}",
            f"selected_gap_owner={selected_owner}",
            f"selected_gap_confidence={selected_confidence}",
            f"next_diagnostic={next_diagnostic}",
            f"next_optimization_allowed={next_optimization_allowed}",
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

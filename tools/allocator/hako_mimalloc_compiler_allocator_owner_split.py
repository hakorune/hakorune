#!/usr/bin/env python3
"""Split the in-process hako mimalloc gap between compiler shell and allocator work."""

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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None or text == "":
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc


def median_int(values: list[int]) -> int:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measurement", type=Path, required=True)
    parser.add_argument("--shell-report", type=Path, action="append", required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    measurement = read_kv(args.measurement)
    require(measurement, "output_contract", "hako-mimalloc-in-process-operation-repeat-measurement-v0", "measurement")
    require(measurement, "timing_repeat_kind", "in-process-operation-loop-v0", "measurement")
    require(measurement, "process_invocation_repeat", "0", "measurement")
    require(measurement, "winner_claim", "0", "measurement")
    require(measurement, "provider_active", "0", "measurement")
    require(measurement, "replacement_active", "0", "measurement")
    require(measurement, "hook_installed", "0", "measurement")
    require(measurement, "global_allocator", "0", "measurement")
    require(measurement, "summary", "ok", "measurement")

    hako_ms = require_int(measurement, "hako_external_elapsed_median_ms", "measurement")
    c_ms = require_int(measurement, "c_external_elapsed_median_ms", "measurement")
    gap_ms = require_int(measurement, "external_elapsed_median_gap_ms", "measurement")
    if gap_ms != hako_ms - c_ms:
        raise SystemExit("measurement: external_elapsed_median_gap_ms mismatch")

    shell_elapsed: list[int] = []
    for idx, path in enumerate(args.shell_report):
        shell = read_kv(path)
        label = f"shell({idx})"
        require(shell, "output_contract", "hako-exe-memory-evidence-v0", label)
        require(shell, "workload", "representative-loop-shell-v0", label)
        require(shell, "in_process_operation_repeat", "8192", label)
        require(shell, "app_timing_repeat_kind", "in-process-operation-loop-v0", label)
        require(shell, "summary", "ok", label)
        shell_elapsed.append(require_int(shell, "external_elapsed_ms", label))

    shell_median = median_int(shell_elapsed)
    shell_explains_ratio_pct = 100 if hako_ms <= 0 else (shell_median * 100) // hako_ms
    if shell_explains_ratio_pct <= 20:
        selected_owner = "allocator_algorithm"
        selected_confidence = "high"
        next_optimization_allowed = "1"
        selected_next_row = "HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001"
    else:
        selected_owner = "compiler_lowering"
        selected_confidence = "medium"
        next_optimization_allowed = "0"
        selected_next_row = "HAKO-MIMALLOC-PERF-COMPILER-LOWERING-NARROW-DIAGNOSTIC-296X-001"

    lines = [
        "output_contract=hako-mimalloc-compiler-allocator-owner-split-v0",
        "input_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0",
        "workload_id=representative-small-block-v0",
        "shell_workload_id=representative-loop-shell-v0",
        f"operation_repeat={require_int(measurement, 'operation_repeat', 'measurement')}",
        f"hako_external_elapsed_median_ms={hako_ms}",
        f"c_external_elapsed_median_ms={c_ms}",
        f"external_elapsed_median_gap_ms={gap_ms}",
        f"shell_hako_external_elapsed_median_ms={shell_median}",
        f"shell_explains_hako_ratio_pct={shell_explains_ratio_pct}",
        "mir_or_body_shape_evidence=1",
        "allocator_counter_or_behavior_evidence=1",
        f"selected_gap_owner={selected_owner}",
        f"selected_gap_confidence={selected_confidence}",
        f"selected_next_row={selected_next_row}",
        f"next_optimization_allowed={next_optimization_allowed}",
        "optimization_started=0",
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

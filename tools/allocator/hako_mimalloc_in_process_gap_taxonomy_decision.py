#!/usr/bin/env python3
"""Classify the first in-process hako/C mimalloc measurement gap."""

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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    values = read_kv(args.input)
    require(values, "output_contract", "hako-mimalloc-in-process-operation-repeat-measurement-v0", "input")
    require(values, "measurement_profile", "hako-mimalloc-in-process-operation-repeat-v0", "input")
    require(values, "timing_repeat_kind", "in-process-operation-loop-v0", "input")
    require(values, "workload_id", "representative-small-block-v0", "input")
    require(values, "process_invocation_repeat", "0", "input")
    require(values, "same_workload", "1", "input")
    require(values, "same_operation_count", "1", "input")
    require(values, "winner_claim", "0", "input")
    require(values, "provider_active", "0", "input")
    require(values, "replacement_active", "0", "input")
    require(values, "hook_installed", "0", "input")
    require(values, "global_allocator", "0", "input")
    require(values, "summary", "ok", "input")

    hako_ms = require_int(values, "hako_external_elapsed_median_ms", "input")
    c_ms = require_int(values, "c_external_elapsed_median_ms", "input")
    c_body_ns = require_int(values, "c_body_elapsed_median_ns", "input")
    gap_ms = require_int(values, "external_elapsed_median_gap_ms", "input")
    if gap_ms != hako_ms - c_ms:
        raise SystemExit("input: external_elapsed_median_gap_ms mismatch")

    if gap_ms <= 0:
        gap_owner = "hako_runtime_baseline"
        gap_confidence = "low"
        next_diagnostic = "repeat_measurement_refresh"
        next_optimization_allowed = "0"
    else:
        # The process-repeat harness has already been excluded. The remaining
        # gap is in the hako workload path, but this row still cannot separate
        # compiler lowering from allocator model/algorithm overhead.
        gap_owner = "allocator_algorithm"
        gap_confidence = "low"
        next_diagnostic = "compiler_allocator_owner_split_diagnostic"
        next_optimization_allowed = "0"

    lines = [
        "output_contract=hako-mimalloc-in-process-gap-taxonomy-decision-v0",
        "input_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0",
        "workload_id=representative-small-block-v0",
        f"operation_repeat={require_int(values, 'operation_repeat', 'input')}",
        f"process_repeat={require_int(values, 'process_repeat', 'input')}",
        f"hako_external_elapsed_median_ms={hako_ms}",
        f"c_external_elapsed_median_ms={c_ms}",
        f"c_body_elapsed_median_ns={c_body_ns}",
        f"external_elapsed_median_gap_ms={gap_ms}",
        f"gap_owner={gap_owner}",
        f"gap_confidence={gap_confidence}",
        f"next_diagnostic={next_diagnostic}",
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

#!/usr/bin/env python3
"""Compare two provider replacement decision reports without winner claims."""

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
    value = require_key(values, key, label)
    try:
        return int(value)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be integer, got {value!r}") from exc


def validate(values: dict[str, str], label: str) -> None:
    require(values, "output_contract", "hako-mimalloc-provider-replacement-decision-adapter-v0", label)
    require(values, "summary", "ok", label)
    require(values, "decision", "external_process_repeated_ready_no_product_default_change", label)
    require(values, "winner_claim", "0", label)
    require(values, "provider_activation", "0", label)
    require(values, "production_replacement_active", "0", label)
    require(values, "hook_installed", "0", label)
    require(values, "global_allocator_product_claim", "0", label)
    require(values, "subject_3_timing_repeat_kind", "external-process-ldpreload-v0", label)
    require(values, "subject_3_replacement_active", "1", label)
    require(values, "subject_3_replacement_product_claim", "0", label)
    require(values, "subject_3_winner_claim", "0", label)
    require_int(values, "subject_3_throughput_median_ops_per_sec", label)


def ratio_ppm(numerator: int, denominator: int) -> int:
    if denominator <= 0:
        raise SystemExit("denominator must be positive")
    return (numerator * 1_000_000) // denominator


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--left", type=Path, required=True)
    parser.add_argument("--right", type=Path, required=True)
    parser.add_argument("--left-label", default="repo_local_fixture")
    parser.add_argument("--right-label", default="external_corpus")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    left = read_kv(args.left)
    right = read_kv(args.right)
    validate(left, "left")
    validate(right, "right")

    for key in ("workload_id", "operation_family", "operation_repeat", "sample_count"):
        if require_key(left, key, "left") != require_key(right, key, "right"):
            raise SystemExit(f"{key} mismatch")

    left_tput = require_int(left, "subject_3_throughput_median_ops_per_sec", "left")
    right_tput = require_int(right, "subject_3_throughput_median_ops_per_sec", "right")
    delta = right_tput - left_tput
    delta_abs = abs(delta)
    ratio = ratio_ppm(right_tput, left_tput)

    lines = [
        "output_contract=hako-mimalloc-provider-replacement-decision-pair-compare-v0",
        "input_contract=hako-mimalloc-provider-replacement-decision-adapter-v0",
        f"left_label={args.left_label}",
        f"right_label={args.right_label}",
        f"left_report={args.left.resolve()}",
        f"right_report={args.right.resolve()}",
        f"workload_id={require_key(left, 'workload_id', 'left')}",
        f"operation_family={require_key(left, 'operation_family', 'left')}",
        f"operation_repeat={require_key(left, 'operation_repeat', 'left')}",
        f"sample_count={require_key(left, 'sample_count', 'left')}",
        f"left_throughput_median_ops_per_sec={left_tput}",
        f"right_throughput_median_ops_per_sec={right_tput}",
        f"throughput_delta_ops_per_sec={delta}",
        f"throughput_delta_abs_ops_per_sec={delta_abs}",
        f"right_over_left_ratio_ppm={ratio}",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
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

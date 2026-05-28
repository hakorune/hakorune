#!/usr/bin/env python3
"""Classify the object-lifecycle Hako/C body timing gap before optimization."""

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
        value = int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc
    if value <= 0:
        raise SystemExit(f"{label}: {key} must be positive, got {value}")
    return value


def ratio(num: int, den: int) -> str:
    if den <= 0:
        raise SystemExit("ratio denominator must be positive")
    return f"{num / den:.3f}"


def classify(body_ratio: float, external_ratio: float) -> tuple[str, str, str, str, str]:
    if body_ratio >= 10.0:
        return (
            "compiler_lowering",
            "medium",
            "single_sample_large_gap",
            "body_gap_large_hako_exact_exe_hot_loop",
            "object_lifecycle_mir_body_owner_selection",
        )
    if external_ratio >= 10.0:
        return (
            "hako_runtime_baseline",
            "low",
            "single_sample_external_gap",
            "body_gap_small_external_gap_large",
            "empty_runtime_or_process_baseline_refresh",
        )
    return (
        "measurement_harness",
        "low",
        "single_sample_small_gap",
        "body_gap_not_large_enough_for_owner",
        "measurement_hygiene_refresh",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    values = read_kv(args.input)
    require(values, "output_contract", "hako-mimalloc-object-lifecycle-body-timing-pair-v0", "input")
    require(values, "body_elapsed_comparable", "1", "input")
    require(values, "winner_claim", "0", "input")
    require(values, "provider_active", "0", "input")
    require(values, "replacement_active", "0", "input")
    require(values, "hook_installed", "0", "input")
    require(values, "global_allocator", "0", "input")
    require(values, "summary", "ok", "input")

    hako_body_ns = require_int(values, "hako_body_elapsed_ns", "input")
    c_body_ns = require_int(values, "c_body_elapsed_ns", "input")
    hako_external_ms = require_int(values, "hako_external_elapsed_ms", "input")
    c_external_ms = require_int(values, "c_external_elapsed_ms", "input")
    body_ratio_text = ratio(hako_body_ns, c_body_ns)
    external_ratio_text = ratio(hako_external_ms, c_external_ms)
    owner, confidence, quality, reason, diagnostic = classify(
        float(body_ratio_text),
        float(external_ratio_text),
    )

    lines = [
        "output_contract=hako-mimalloc-object-lifecycle-body-timing-gap-taxonomy-v0",
        "input_contract=hako-mimalloc-object-lifecycle-body-timing-pair-v0",
        f"workload_id={require_key(values, 'workload_id', 'input')}",
        f"operation_sequence_id={require_key(values, 'operation_sequence_id', 'input')}",
        f"free_order_id={require_key(values, 'free_order_id', 'input')}",
        f"in_process_operation_repeat={require_key(values, 'in_process_operation_repeat', 'input')}",
        f"allocation_count={require_key(values, 'allocation_count', 'input')}",
        f"free_count={require_key(values, 'free_count', 'input')}",
        f"requested_bytes={require_key(values, 'requested_bytes', 'input')}",
        "body_elapsed_role=primary_hot_loop_diagnostic",
        "external_elapsed_role=secondary_process_runtime_evidence",
        f"hako_body_elapsed_ns={hako_body_ns}",
        f"c_body_elapsed_ns={c_body_ns}",
        f"body_elapsed_gap_ns={hako_body_ns - c_body_ns}",
        f"body_elapsed_ratio={body_ratio_text}",
        f"hako_external_elapsed_ms={hako_external_ms}",
        f"c_external_elapsed_ms={c_external_ms}",
        f"external_elapsed_ratio={external_ratio_text}",
        f"gap_owner={owner}",
        f"gap_confidence={confidence}",
        f"evidence_quality={quality}",
        f"gap_reason={reason}",
        f"next_diagnostic={diagnostic}",
        "next_optimization_allowed=0",
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

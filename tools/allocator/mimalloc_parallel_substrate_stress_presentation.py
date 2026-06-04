#!/usr/bin/env python3
"""Format native multi-worker substrate stress evidence without winner claims."""

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
    require(report, "output_contract", "mimalloc-comparison-par-stress-evidence-v0")
    require(report, "cargo_test_target", "nyash_kernel")
    require(report, "cargo_test_filter", "mimalloc_parallel_stress")
    require(report, "cargo_test_passed", "1")
    require(report, "summary", "ok")
    require(report, "winner_claim", "0")

    worker_count = as_int(report, "worker_count")
    iterations_per_worker = as_int(report, "iterations_per_worker")
    expected_remote_free_count = as_int(report, "expected_remote_free_count")
    observed_remote_free_count = as_int(report, "observed_remote_free_count")
    drained_nodes = as_int(report, "drained_nodes")
    payload_sum_nonzero = as_int(report, "payload_sum_nonzero")

    if worker_count <= 0:
        raise SystemExit("worker_count must be positive")
    if iterations_per_worker <= 0:
        raise SystemExit("iterations_per_worker must be positive")
    if expected_remote_free_count <= 0:
        raise SystemExit("expected_remote_free_count must be positive")
    if observed_remote_free_count != expected_remote_free_count:
        raise SystemExit("observed_remote_free_count mismatch")
    if drained_nodes != expected_remote_free_count:
        raise SystemExit("drained_nodes mismatch")
    if payload_sum_nonzero != 1:
        raise SystemExit("payload_sum_nonzero must be 1")

    lines = [
        "mimalloc_parallel_substrate_stress_presentation=1",
        "output_contract=mimalloc-comparison-par-stress-presentation-v0",
        "input_contract=mimalloc-comparison-par-stress-evidence-v0",
        f"worker_count={worker_count}",
        f"iterations_per_worker={iterations_per_worker}",
        f"expected_remote_free_count={expected_remote_free_count}",
        f"observed_remote_free_count={observed_remote_free_count}",
        f"drained_nodes={drained_nodes}",
        f"payload_sum_nonzero={payload_sum_nonzero}",
        "presentation_only=1",
        "provider_activation=0",
        "host_replacement=0",
        "hook_installed=0",
        "global_allocator_installed=0",
        "winner_claim=0",
        "summary=ok",
    ]
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Run and normalize the native multi-worker substrate stress evidence."""

from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TEST_FILTER = "mimalloc_parallel_substrate_stress"
PACKAGE = "nyash_kernel"


def read_kv_lines(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    completed = subprocess.run(
        [
            "cargo",
            "test",
            "-q",
            "-p",
            PACKAGE,
            TEST_FILTER,
            "--",
            "--nocapture",
        ],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=True,
    )

    values = read_kv_lines(completed.stdout)
    for key in (
        "mimalloc_parallel_substrate_stress",
        "worker_count",
        "iterations_per_worker",
        "expected_remote_free_count",
        "observed_remote_free_count",
        "drained_nodes",
        "payload_sum_nonzero",
        "summary",
    ):
        if key not in values:
            raise SystemExit(f"missing stress report field: {key}")

    if values.get("mimalloc_parallel_substrate_stress") != "1":
        raise SystemExit("stress report marker missing")
    if values.get("summary") != "ok":
        raise SystemExit("stress report did not finish cleanly")

    worker_count = as_int(values, "worker_count")
    iterations_per_worker = as_int(values, "iterations_per_worker")
    expected_remote_free_count = as_int(values, "expected_remote_free_count")
    observed_remote_free_count = as_int(values, "observed_remote_free_count")
    drained_nodes = as_int(values, "drained_nodes")
    payload_sum_nonzero = as_int(values, "payload_sum_nonzero")

    if expected_remote_free_count != worker_count * iterations_per_worker:
        raise SystemExit("expected_remote_free_count mismatch")
    if observed_remote_free_count != expected_remote_free_count:
        raise SystemExit("observed_remote_free_count mismatch")
    if drained_nodes != expected_remote_free_count:
        raise SystemExit("drained_nodes mismatch")
    if payload_sum_nonzero != 1:
        raise SystemExit("payload_sum_nonzero must be 1")

    lines = [
        "mimalloc_parallel_substrate_stress_runner=1",
        "output_contract=mimalloc-comparison-par-stress-evidence-v0",
        "cargo_test_target=nyash_kernel",
        "cargo_test_filter=mimalloc_parallel_substrate_stress",
        "cargo_test_passed=1",
        f"worker_count={worker_count}",
        f"iterations_per_worker={iterations_per_worker}",
        f"expected_remote_free_count={expected_remote_free_count}",
        f"observed_remote_free_count={observed_remote_free_count}",
        f"drained_nodes={drained_nodes}",
        f"payload_sum_nonzero={payload_sum_nonzero}",
        "winner_claim=0",
        "summary=ok",
    ]
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

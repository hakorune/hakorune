#!/usr/bin/env python3
"""Measure SafeMutexStore versus SingleThreadExactStore on object lifecycle."""

from __future__ import annotations

import argparse
import os
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
RUNNER = ROOT / "tools/allocator/hako_exe_memory_runner.sh"
WORKLOAD = "representative-object-lifecycle-small-block-v0"


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def require_positive_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None:
        raise SystemExit(f"{label}: missing {key}")
    try:
        value = int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc
    if value <= 0:
        raise SystemExit(f"{label}: {key} must be positive, got {value}")
    return value


def median(values: list[int]) -> int:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def run_one(tmp_dir: Path, backend: str, sample_idx: int) -> dict[str, str]:
    report = tmp_dir / f"{backend}-{sample_idx}.out"
    env = os.environ.copy()
    env["HAKO_TYPED_OBJECT_STORE"] = backend
    subprocess.run(
        [
            "bash",
            str(RUNNER),
            "--app",
            str(APP),
            "--workload",
            WORKLOAD,
            "--runtime-config",
            "empty",
            "--operation-repeat",
            "1",
            "--out",
            str(report),
        ],
        cwd=ROOT,
        env=env,
        stdout=subprocess.DEVNULL,
        check=True,
    )
    values = read_kv(report)
    label = f"{backend} sample {sample_idx}"
    require(values, "output_contract", "hako-exe-memory-evidence-v0", label)
    require(values, "summary", "ok", label)
    require(values, "workload", WORKLOAD, label)
    require(values, "provider_activation", "0", label)
    require(values, "host_replacement", "0", label)
    require(values, "hook_installed", "0", label)
    require(values, "global_allocator_installed", "0", label)
    return values


def ratio_pct(numerator: int, denominator: int) -> int:
    return int(round((numerator * 100) / denominator)) if denominator else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--sample-count", type=int, default=1)
    args = parser.parse_args()

    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")

    subprocess.run(["cargo", "build", "--release", "-p", "nyash_kernel"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory(prefix="hakorune_typed_object_fast_lane_keeper.") as tmp:
        tmp_dir = Path(tmp)
        safe_samples = [run_one(tmp_dir, "safe_mutex", idx) for idx in range(args.sample_count)]
        exact_samples = [
            run_one(tmp_dir, "single_thread_exact", idx) for idx in range(args.sample_count)
        ]

    safe_body = [
        require_positive_int(sample, "body_elapsed_ns", f"safe_mutex sample {idx}")
        for idx, sample in enumerate(safe_samples)
    ]
    exact_body = [
        require_positive_int(sample, "body_elapsed_ns", f"single_thread_exact sample {idx}")
        for idx, sample in enumerate(exact_samples)
    ]
    safe_external = [
        require_positive_int(sample, "external_elapsed_ms", f"safe_mutex sample {idx}")
        for idx, sample in enumerate(safe_samples)
    ]
    exact_external = [
        require_positive_int(sample, "external_elapsed_ms", f"single_thread_exact sample {idx}")
        for idx, sample in enumerate(exact_samples)
    ]

    safe_body_median = median(safe_body)
    exact_body_median = median(exact_body)
    delta = safe_body_median - exact_body_median
    accepted = exact_body_median < safe_body_median

    lines = [
        "output_contract=typed-object-runtime-fast-lane-keeper-measurement-v0",
        "input_contract=typed-object-runtime-single-thread-fast-lane-v0",
        f"workload_id={WORKLOAD}",
        "measurement_scope=object_lifecycle_exact_exe_typed_object_store_backend_pair",
        f"sample_count={args.sample_count}",
        f"safe_mutex_body_elapsed_ns={safe_body_median}",
        f"single_thread_exact_body_elapsed_ns={exact_body_median}",
        f"body_elapsed_delta_ns={delta}",
        f"single_thread_exact_body_ratio_pct={ratio_pct(exact_body_median, safe_body_median)}",
        f"safe_mutex_external_elapsed_ms={median(safe_external)}",
        f"single_thread_exact_external_elapsed_ms={median(exact_external)}",
        f"keeper_effect={'accepted' if accepted else 'no_effect'}",
        f"runtime_fast_lane_keeper={1 if accepted else 0}",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

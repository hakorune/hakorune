#!/usr/bin/env python3
"""Measure exact-EXE after the small-alloc acquire_usize fast path keeper."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
RUNNER = ROOT / "tools/allocator/hako_exe_memory_runner.sh"
WORKLOAD = "representative-object-lifecycle-small-block-v0"
PREVIOUS_CHECKPOINT_MS = 600


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if line and "=" in line:
            key, value = line.split("=", 1)
            values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def as_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None:
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc


def median(values: list[int]) -> int:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def run_sample(tmp_dir: Path, sample: int, timeout_seconds: int) -> dict[str, str]:
    report = tmp_dir / f"hako-{sample}.out"
    subprocess.run(
        [
            "timeout",
            f"{timeout_seconds}s",
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
        stdout=subprocess.DEVNULL,
        check=True,
    )
    values = read_kv(report)
    require(values, "output_contract", "hako-exe-memory-evidence-v0", f"sample {sample}")
    require(values, "summary", "ok", f"sample {sample}")
    require(values, "workload", WORKLOAD, f"sample {sample}")
    require(values, "in_process_operation_repeat", "8192", f"sample {sample}")
    require(values, "allocation_count", "524288", f"sample {sample}")
    require(values, "free_count", "524288", f"sample {sample}")
    require(values, "release_known_page_fast_path_count", "524288", f"sample {sample}")
    require(values, "release_known_page_fallback_count", "0", f"sample {sample}")
    require(values, "provider_activation", "0", f"sample {sample}")
    require(values, "host_replacement", "0", f"sample {sample}")
    require(values, "hook_installed", "0", f"sample {sample}")
    require(values, "global_allocator_installed", "0", f"sample {sample}")
    return values


def classify_effect(after_median: int) -> str:
    if after_median < PREVIOUS_CHECKPOINT_MS:
        return "accepted"
    if after_median == PREVIOUS_CHECKPOINT_MS:
        return "neutral"
    return "regressed"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--sample-count", type=int, default=1)
    parser.add_argument("--timeout-seconds", type=int, default=240)
    args = parser.parse_args()

    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")

    with tempfile.TemporaryDirectory(prefix="hakorune_post_acquire_usize.") as tmp:
        tmp_dir = Path(tmp)
        samples = [
            run_sample(tmp_dir, idx, args.timeout_seconds)
            for idx in range(args.sample_count)
        ]

    elapsed = [
        as_int(sample, "external_elapsed_ms", f"sample {idx}")
        for idx, sample in enumerate(samples)
    ]
    rss = [
        as_int(sample, "external_peak_rss_bytes", f"sample {idx}")
        for idx, sample in enumerate(samples)
    ]
    elapsed_median = median(elapsed)

    lines = [
        "output_contract=post-page-acquire-usize-fast-path-measurement-v0",
        "input_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0",
        "measurement_scope=object_lifecycle_facade_exact_exe_after_acquire_usize_fast_path",
        f"workload_id={WORKLOAD}",
        "operation_repeat=8192",
        f"sample_count={args.sample_count}",
        "release_known_page_fast_path_count=524288",
        "release_known_page_fallback_count=0",
    ]
    for idx, value in enumerate(elapsed):
        lines.append(f"sample_{idx}_hako_external_elapsed_ms={value}")
    lines.extend(
        [
            f"elapsed_median_ms={elapsed_median}",
            f"elapsed_min_ms={min(elapsed)}",
            f"elapsed_max_ms={max(elapsed)}",
            f"external_rss_median_bytes={median(rss)}",
            f"previous_checkpoint_median_ms={PREVIOUS_CHECKPOINT_MS}",
            "previous_checkpoint_source=296x-149-post-known-live-release-measurement",
            f"keeper_effect={classify_effect(elapsed_median)}",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "selected_next=post_page_acquire_usize_source_mir_refresh",
            "summary=ok",
        ]
    )

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

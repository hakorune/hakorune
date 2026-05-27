#!/usr/bin/env python3
"""Run the first hako/C mimalloc in-process operation-repeat pilot."""

from __future__ import annotations

import argparse
import subprocess
import sys
import tempfile
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
HAKO_APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-small-block-proof/main.hako"
HAKO_RUNNER = ROOT / "tools/allocator/hako_exe_memory_runner.sh"
C_SRC = ROOT / "tools/allocator/c_mimalloc_explicit_runner.c"


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


def as_int(values: dict[str, str], key: str, label: str) -> int:
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


def run_hako_sample(tmp_dir: Path, sample_index: int, operation_repeat: int) -> dict[str, str]:
    out = tmp_dir / f"hako-{sample_index}.out"
    stdout = tmp_dir / f"hako-{sample_index}.stdout"
    with stdout.open("w", encoding="utf-8") as stdout_file:
        subprocess.run(
            [
                "bash",
                str(HAKO_RUNNER),
                "--app",
                str(HAKO_APP),
                "--workload",
                "representative-small-block-v0",
                "--runtime-config",
                "empty",
                "--operation-repeat",
                "1",
                "--out",
                str(out),
            ],
            cwd=ROOT,
            stdout=stdout_file,
            check=True,
        )
    values = read_kv(out)
    require(values, "output_contract", "hako-exe-memory-evidence-v0", "hako")
    require(values, "summary", "ok", "hako")
    require(values, "workload", "representative-small-block-v0", "hako")
    require(values, "in_process_operation_repeat", str(operation_repeat), "hako")
    require(values, "app_timing_repeat_kind", "in-process-operation-loop-v0", "hako")
    require(values, "provider_activation", "0", "hako")
    require(values, "host_replacement", "0", "hako")
    require(values, "hook_installed", "0", "hako")
    require(values, "global_allocator_installed", "0", "hako")
    return values


def build_c_runner(tmp_dir: Path) -> Path:
    binary = tmp_dir / "c_mimalloc_explicit_runner"
    subprocess.run(
        ["cc", "-std=c11", "-O2", "-Wall", "-Wextra", str(C_SRC), "-ldl", "-o", str(binary)],
        cwd=ROOT,
        check=True,
    )
    return binary


def run_c_sample(
    binary: Path,
    tmp_dir: Path,
    sample_index: int,
    library: Path,
    operation_repeat: int,
) -> tuple[dict[str, str], int]:
    out = tmp_dir / f"c-{sample_index}.out"
    start = time.perf_counter_ns()
    with out.open("w", encoding="utf-8") as out_file:
        subprocess.run(
            [
                str(binary),
                "--library",
                str(library),
                "--workload",
                "representative-small-block-v0",
                "--in-process-repeat",
                str(operation_repeat),
            ],
            cwd=ROOT,
            stdout=out_file,
            check=True,
        )
    elapsed_ms = max(1, round((time.perf_counter_ns() - start) / 1_000_000))
    values = read_kv(out)
    require(values, "output_contract", "allocator-comparison-c-mimalloc-explicit-runner-v0", "c")
    require(values, "summary", "ok", "c")
    require(values, "workload", "representative-small-block-v0", "c")
    require(values, "in_process_operation_repeat", str(operation_repeat), "c")
    require(values, "timing_repeat_kind", "in-process-operation-loop-v0", "c")
    require(values, "process_replacement_executed", "0", "c")
    require(values, "hook_installed", "0", "c")
    require(values, "global_allocator_installed", "0", "c")
    return values, elapsed_ms


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--c-library", type=Path, required=True)
    parser.add_argument("--operation-repeat", type=int, default=8192)
    parser.add_argument("--process-repeat", type=int, default=3)
    args = parser.parse_args()

    if args.operation_repeat != 8192:
        raise SystemExit("pilot fixture currently supports --operation-repeat 8192")
    if args.process_repeat < 1:
        raise SystemExit("--process-repeat must be positive")
    if not args.c_library.exists():
        raise SystemExit(f"--c-library path does not exist: {args.c_library}")
    if not HAKO_APP.exists():
        raise SystemExit(f"missing hako app: {HAKO_APP}")

    with tempfile.TemporaryDirectory(prefix="hakorune_in_process_pilot.") as tmp:
        tmp_dir = Path(tmp)
        c_binary = build_c_runner(tmp_dir)
        hako_samples: list[dict[str, str]] = []
        c_samples: list[dict[str, str]] = []
        c_external_elapsed: list[int] = []
        for sample in range(args.process_repeat):
            hako = run_hako_sample(tmp_dir, sample, args.operation_repeat)
            c, c_elapsed = run_c_sample(c_binary, tmp_dir, sample, args.c_library, args.operation_repeat)
            for key in ("allocation_count", "free_count", "requested_bytes"):
                if as_int(hako, key, "hako") != as_int(c, key, "c"):
                    raise SystemExit(f"{key} mismatch between hako and C")
            hako_samples.append(hako)
            c_samples.append(c)
            c_external_elapsed.append(c_elapsed)

    hako_elapsed = [as_int(sample, "external_elapsed_ms", "hako") for sample in hako_samples]
    hako_rss = [as_int(sample, "external_peak_rss_bytes", "hako") for sample in hako_samples]
    c_body_ns = [as_int(sample, "body_elapsed_ns", "c") for sample in c_samples]
    c_rss = [as_int(sample, "peak_rss_bytes", "c") for sample in c_samples]

    hako_median = median_int(hako_elapsed)
    c_external_median = median_int(c_external_elapsed)
    c_body_median_ns = median_int(c_body_ns)

    lines = [
        "output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0",
        "measurement_profile=hako-mimalloc-in-process-operation-repeat-v0",
        "timing_repeat_kind=in-process-operation-loop-v0",
        "workload_id=representative-small-block-v0",
        f"operation_repeat={args.operation_repeat}",
        f"process_repeat={args.process_repeat}",
        f"sample_count={args.process_repeat}",
        "same_workload=1",
        "same_operation_count=1",
        "process_invocation_repeat=0",
        f"allocation_count={as_int(hako_samples[0], 'allocation_count', 'hako')}",
        f"free_count={as_int(hako_samples[0], 'free_count', 'hako')}",
        f"requested_bytes={as_int(hako_samples[0], 'requested_bytes', 'hako')}",
    ]
    for idx in range(args.process_repeat):
        lines.extend(
            [
                f"sample_{idx}_hako_external_elapsed_ms={hako_elapsed[idx]}",
                f"sample_{idx}_c_external_elapsed_ms={c_external_elapsed[idx]}",
                f"sample_{idx}_c_body_elapsed_ns={c_body_ns[idx]}",
            ]
        )
    lines.extend(
        [
            f"hako_external_elapsed_median_ms={hako_median}",
            f"c_external_elapsed_median_ms={c_external_median}",
            f"c_body_elapsed_median_ns={c_body_median_ns}",
            f"external_elapsed_median_gap_ms={hako_median - c_external_median}",
            f"hako_external_rss_median_bytes={median_int(hako_rss)}",
            f"c_rss_median_bytes={median_int(c_rss)}",
            "winner_claim=0",
            "provider_active=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

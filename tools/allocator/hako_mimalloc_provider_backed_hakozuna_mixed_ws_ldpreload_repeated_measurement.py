#!/usr/bin/env python3
"""Run repeated provider-backed Hakozuna mixed-ws LD_PRELOAD samples."""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

from hako_mimalloc_provider_backed_hakmem_ldpreload_bench_pilot import read_kv


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_HAKOZUNA_ROOT = ROOT / "benchmarks" / "external" / "hakozuna" / "mixed-ws" / "build"
SMOKE_TOOL = Path(__file__).resolve().with_name("provider_package_ldpreload_replacement_smoke.py")
OPS_RE = re.compile(r"ops/s=([0-9]+(?:\.[0-9]+)?)")


def median_float(values: list[float]) -> float:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def positive_int(value: int, label: str) -> None:
    if value < 1:
        raise SystemExit(f"{label} must be positive")


def run_one(
    bench: Path,
    root: Path,
    shim: Path,
    provider_binary: Path,
    out_dir: Path,
    run_index: int,
    kind: str,
    threads: int,
    iters_per_thread: int,
    working_set: int,
    min_size: int,
    max_size: int,
) -> tuple[float, dict[str, str], int]:
    run_dir = out_dir / f"{kind}_{run_index}"
    run_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = run_dir / "bench.stdout"
    stderr_path = run_dir / "bench.stderr"
    counts_path = run_dir / "bench-shim-counts.out"
    env = os.environ.copy()
    env["LD_PRELOAD"] = str(shim)
    env["HAKORUNE_PROVIDER_LIBRARY"] = str(provider_binary)
    env["HAKORUNE_PROVIDER_LDPRELOAD_REPORT"] = str(counts_path)
    completed = subprocess.run(
        [
            str(bench),
            str(threads),
            str(iters_per_thread),
            str(working_set),
            str(min_size),
            str(max_size),
        ],
        cwd=root,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    stdout_path.write_text(completed.stdout, encoding="utf-8")
    stderr_path.write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise SystemExit(
            f"hakozuna mixed-ws {kind} run {run_index} failed with "
            f"{completed.returncode}: {completed.stderr.strip()}"
        )
    match = OPS_RE.search(completed.stdout)
    if match is None:
        raise SystemExit(
            f"hakozuna mixed-ws {kind} run {run_index} output missing ops/s line"
        )
    counts = read_kv(counts_path)
    return float(match.group(1)), counts, completed.returncode


def require_good_counts(counts: dict[str, str], label: str) -> None:
    provider_alloc_count = int(counts.get("shim_provider_alloc_count", "0"))
    provider_free_count = int(counts.get("shim_provider_free_count", "0"))
    runtime_fallback_count = int(counts.get("shim_runtime_real_fallback_count", "0"))
    pointer_table_overflow = int(counts.get("shim_pointer_table_overflow", "0"))
    if provider_alloc_count <= 0:
        raise SystemExit(f"{label}: shim_provider_alloc_count must be positive")
    if provider_free_count <= 0:
        raise SystemExit(f"{label}: shim_provider_free_count must be positive")
    if runtime_fallback_count != 0:
        raise SystemExit(f"{label}: shim_runtime_real_fallback_count must be 0")
    if pointer_table_overflow != 0:
        raise SystemExit(f"{label}: shim_pointer_table_overflow must be 0")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--hakozuna-root", type=Path, default=DEFAULT_HAKOZUNA_ROOT)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--sample-count", type=int, default=3)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument("--threads", type=int, default=1)
    parser.add_argument("--iters-per-thread", type=int, default=1000)
    parser.add_argument("--working-set", type=int, default=128)
    parser.add_argument("--min-size", type=int, default=16)
    parser.add_argument("--max-size", type=int, default=1024)
    args = parser.parse_args()

    positive_int(args.sample_count, "--sample-count")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    positive_int(args.threads, "--threads")
    positive_int(args.iters_per_thread, "--iters-per-thread")
    positive_int(args.working_set, "--working-set")
    positive_int(args.min_size, "--min-size")
    positive_int(args.max_size, "--max-size")
    if args.max_size < args.min_size:
        raise SystemExit("--max-size must be >= --min-size")

    root = args.hakozuna_root.resolve()
    bench = root / "bench_mixed_ws_crt"
    if not bench.is_file() or not os.access(bench, os.X_OK):
        raise SystemExit(
            "missing executable hakozuna mixed-ws bench: "
            f"{bench}\n"
            "hint: run `make -C benchmarks/external/hakozuna/mixed-ws` "
            "or pass --hakozuna-root for an external hakozuna build"
        )

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    smoke_report = out_dir / "provider-ldpreload-smoke.out"
    subprocess.run(
        [
            sys.executable,
            str(SMOKE_TOOL),
            "--manifest",
            str(args.manifest.resolve()),
            "--out-dir",
            str(out_dir / "provider-ldpreload-smoke"),
            "--out",
            str(smoke_report),
        ],
        check=True,
    )
    smoke = read_kv(smoke_report)
    shim = Path(smoke["shim_artifact_path"])
    provider_binary = Path(smoke["provider_binary_path"])

    total_runs = args.warmup_count + args.sample_count
    sample_throughputs: list[float] = []
    total_provider_alloc = 0
    total_provider_free = 0
    total_provider_calloc = 0
    total_provider_realloc = 0
    total_runtime_real_fallback = 0
    total_init_real_fallback = 0
    total_host_passthrough = 0
    total_provider_bind_success = 0
    total_provider_bind_failure = 0
    total_pointer_table_overflow = 0

    lines = [
        "output_contract=hako-mimalloc-provider-backed-hakozuna-mixed-ws-ldpreload-repeated-measurement-v0",
        "input_contract=hako-mimalloc-provider-backed-ldpreload-shim-smoke-v0",
        "dll_mode=provider-backed-hakozuna-mixed-ws-ldpreload-repeated-measurement",
        f"manifest={args.manifest.resolve()}",
        f"provider_binary_path={provider_binary}",
        "hakozuna_fixture_kind=minimal-repo-mixed-ws-crt",
        f"hakozuna_root={root}",
        "benchmark_id=bench_mixed_ws_crt",
        f"benchmark_path={bench}",
        f"benchmark_threads={args.threads}",
        f"benchmark_iters_per_thread={args.iters_per_thread}",
        f"benchmark_working_set={args.working_set}",
        f"benchmark_min_size={args.min_size}",
        f"benchmark_max_size={args.max_size}",
        f"warmup_count={args.warmup_count}",
        f"sample_count={args.sample_count}",
        "timing_repeat_kind=external-process-ldpreload-v0",
        "summary_statistic=min,median,max",
        "ld_preload_env_applied=1",
        f"ld_preload_artifact={shim}",
        "provider_api_bound=1",
        "provider_call_executed=1",
        "allocator_entrypoint_called=1",
        "replacement_active=1",
        "replacement_scope=external-hakozuna-mixed-ws-bench-process-repeated",
        "replacement_product_claim=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
    ]

    for run_index in range(total_runs):
        kind = "warmup" if run_index < args.warmup_count else "sample"
        throughput, counts, exit_code = run_one(
            bench,
            root,
            shim,
            provider_binary,
            out_dir,
            run_index,
            kind,
            args.threads,
            args.iters_per_thread,
            args.working_set,
            args.min_size,
            args.max_size,
        )
        require_good_counts(counts, f"{kind}_{run_index}")
        total_provider_alloc += int(counts.get("shim_provider_alloc_count", "0"))
        total_provider_free += int(counts.get("shim_provider_free_count", "0"))
        total_provider_calloc += int(counts.get("shim_provider_calloc_count", "0"))
        total_provider_realloc += int(counts.get("shim_provider_realloc_count", "0"))
        total_runtime_real_fallback += int(counts.get("shim_runtime_real_fallback_count", "0"))
        total_init_real_fallback += int(counts.get("shim_init_real_fallback_count", "0"))
        total_host_passthrough += int(counts.get("shim_host_passthrough_count", "0"))
        total_provider_bind_success += int(counts.get("shim_provider_bind_success", "0"))
        total_provider_bind_failure += int(counts.get("shim_provider_bind_failure", "0"))
        total_pointer_table_overflow += int(counts.get("shim_pointer_table_overflow", "0"))
        if kind == "sample":
            sample_index = run_index - args.warmup_count
            sample_throughputs.append(throughput)
            lines.extend(
                [
                    f"sample_{sample_index}_throughput_ops_per_sec={throughput:.3f}",
                    f"sample_{sample_index}_exit_code={exit_code}",
                    f"sample_{sample_index}_shim_provider_alloc_count={counts.get('shim_provider_alloc_count', '0')}",
                    f"sample_{sample_index}_shim_provider_free_count={counts.get('shim_provider_free_count', '0')}",
                    f"sample_{sample_index}_shim_runtime_real_fallback_count={counts.get('shim_runtime_real_fallback_count', '0')}",
                    f"sample_{sample_index}_shim_init_real_fallback_count={counts.get('shim_init_real_fallback_count', '0')}",
                    f"sample_{sample_index}_shim_host_passthrough_count={counts.get('shim_host_passthrough_count', '0')}",
                    f"sample_{sample_index}_shim_pointer_table_overflow={counts.get('shim_pointer_table_overflow', '0')}",
                    f"sample_{sample_index}_winner_claim=0",
                ]
            )

    lines.extend(
        [
            f"throughput_min_ops_per_sec={min(sample_throughputs):.3f}",
            f"throughput_median_ops_per_sec={median_float(sample_throughputs):.3f}",
            f"throughput_max_ops_per_sec={max(sample_throughputs):.3f}",
            f"shim_provider_alloc_count_total={total_provider_alloc}",
            f"shim_provider_free_count_total={total_provider_free}",
            f"shim_provider_calloc_count_total={total_provider_calloc}",
            f"shim_provider_realloc_count_total={total_provider_realloc}",
            f"shim_provider_bind_success_total={total_provider_bind_success}",
            f"shim_provider_bind_failure_total={total_provider_bind_failure}",
            f"shim_runtime_real_fallback_count_total={total_runtime_real_fallback}",
            f"shim_init_real_fallback_count_total={total_init_real_fallback}",
            f"shim_host_passthrough_count_total={total_host_passthrough}",
            f"shim_pointer_table_overflow_total={total_pointer_table_overflow}",
            "shim_init_real_fallback_is_perf_diagnostic=1",
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

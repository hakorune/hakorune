#!/usr/bin/env python3
"""Compare Hakozuna mixed-ws under system, C mimalloc, and optional provider LD_PRELOAD."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

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


def find_mimalloc_library(c_library: Path | None, allow_ldconfig_discovery: bool) -> Path:
    if c_library is not None:
        path = c_library.resolve()
    else:
        if not allow_ldconfig_discovery:
            raise SystemExit("--mimalloc-library PATH or --allow-ldconfig-discovery is required")
        completed = subprocess.run(
            [
                "bash",
                "-lc",
                r"ldconfig -p 2>/dev/null | awk '/libmimalloc\.so\.2[[:space:]]/ { print $NF; exit }'",
            ],
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        path = Path(completed.stdout.strip()).resolve() if completed.stdout.strip() else Path("")
    if not path.is_file():
        raise SystemExit(f"libmimalloc.so.2 not found: {path}")
    return path


def run_one(
    *,
    bench: Path,
    root: Path,
    out_dir: Path,
    subject: str,
    run_index: int,
    kind: str,
    threads: int,
    iters_per_thread: int,
    working_set: int,
    min_size: int,
    max_size: int,
    ld_preload: Path | None,
    provider_binary: Path | None,
) -> tuple[float, dict[str, str], int]:
    run_dir = out_dir / subject / f"{kind}_{run_index}"
    run_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = run_dir / "bench.stdout"
    stderr_path = run_dir / "bench.stderr"
    counts_path = run_dir / "shim-counts.out"
    env = os.environ.copy()
    if ld_preload is not None:
        env["LD_PRELOAD"] = str(ld_preload)
    if provider_binary is not None:
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
            f"{subject} {kind} run {run_index} failed with "
            f"{completed.returncode}: {completed.stderr.strip()}"
        )
    match = OPS_RE.search(completed.stdout)
    if match is None:
        raise SystemExit(f"{subject} {kind} run {run_index} output missing ops/s line")
    counts = read_kv(counts_path) if provider_binary is not None else {}
    return float(match.group(1)), counts, completed.returncode


def run_subject(
    *,
    bench: Path,
    root: Path,
    out_dir: Path,
    subject: str,
    warmup_count: int,
    sample_count: int,
    threads: int,
    iters_per_thread: int,
    working_set: int,
    min_size: int,
    max_size: int,
    ld_preload: Path | None,
    provider_binary: Path | None,
) -> tuple[list[float], dict[str, int]]:
    sample_throughputs: list[float] = []
    counter_totals: dict[str, int] = {}
    total_runs = warmup_count + sample_count
    for run_index in range(total_runs):
        kind = "warmup" if run_index < warmup_count else "sample"
        throughput, counts, _exit_code = run_one(
            bench=bench,
            root=root,
            out_dir=out_dir,
            subject=subject,
            run_index=run_index,
            kind=kind,
            threads=threads,
            iters_per_thread=iters_per_thread,
            working_set=working_set,
            min_size=min_size,
            max_size=max_size,
            ld_preload=ld_preload,
            provider_binary=provider_binary,
        )
        for key, value in counts.items():
            if value.isdigit():
                counter_totals[key] = counter_totals.get(key, 0) + int(value)
        if kind == "sample":
            sample_throughputs.append(throughput)
    return sample_throughputs, counter_totals


def format_ratio(value: float, base: float) -> str:
    if base <= 0:
        return "0.000000"
    return f"{value / base:.6f}"


def load_manifest_metadata(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid provider manifest JSON: {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise SystemExit(f"provider manifest root must be an object: {path}")

    build = data.get("build")
    activation = data.get("activation")
    if not isinstance(build, dict):
        build = {}
    if not isinstance(activation, dict):
        activation = {}

    def manifest_string(source: dict[str, Any], key: str, default: str = "unknown") -> str:
        value = source.get(key)
        if isinstance(value, bool):
            return "1" if value else "0"
        if value is None:
            return default
        return str(value)

    return {
        "provider_manifest_provider_name": manifest_string(data, "provider_name"),
        "provider_manifest_provider_kind": manifest_string(data, "provider_kind"),
        "provider_manifest_profile": manifest_string(data, "profile"),
        "provider_manifest_build_mode": manifest_string(build, "mode"),
        "provider_manifest_hako_semantic_provider_codegen": manifest_string(
            build, "hako_semantic_provider_codegen", "none"
        ),
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_verified": manifest_string(
            build, "hako_provider_object_lifecycle_entrypoint_verified", "0"
        ),
        "provider_manifest_hako_provider_alloc_free_route": manifest_string(
            build, "hako_provider_alloc_free_route", "unknown"
        ),
        "provider_manifest_hako_provider_alloc_free_uses_host_malloc": manifest_string(
            build, "hako_provider_alloc_free_uses_host_malloc", "unknown"
        ),
        "provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle": manifest_string(
            build, "hako_provider_alloc_free_uses_hako_object_lifecycle", "unknown"
        ),
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_usage": manifest_string(
            build, "hako_provider_object_lifecycle_entrypoint_usage", "unknown"
        ),
        "provider_manifest_allocator_replacement_allowed": manifest_string(
            activation, "allocator_replacement_allowed", "0"
        ),
        "provider_manifest_hook_allowed": manifest_string(activation, "hook_allowed", "0"),
        "provider_manifest_global_allocator_allowed": manifest_string(
            activation, "global_allocator_allowed", "0"
        ),
    }


def format_per_operation(numerator: int, denominator: int) -> str:
    if denominator <= 0:
        return "0.000000"
    return f"{numerator / denominator:.6f}"


def init_fallback_dominates_provider_ops(counters: dict[str, int], provider_ops: int) -> bool:
    if provider_ops <= 0:
        return False
    init_fallback = counters.get("shim_init_real_fallback_count", 0)
    return init_fallback * 2 >= provider_ops


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hakozuna-root", type=Path, default=DEFAULT_HAKOZUNA_ROOT)
    parser.add_argument("--mimalloc-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    parser.add_argument("--manifest", type=Path)
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
    mimalloc_library = find_mimalloc_library(args.mimalloc_library, args.allow_ldconfig_discovery)
    provider_manifest_metadata = load_manifest_metadata(
        args.manifest.resolve() if args.manifest else None
    )

    provider_shim: Path | None = None
    provider_binary: Path | None = None
    if args.manifest is not None:
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
        provider_shim = Path(smoke["shim_artifact_path"])
        provider_binary = Path(smoke["provider_binary_path"])

    subject_specs: list[tuple[str, Path | None, Path | None]] = [
        ("system_malloc", None, None),
        ("c_mimalloc_ldpreload", mimalloc_library, None),
    ]
    if provider_shim is not None and provider_binary is not None:
        subject_specs.append(("hakorune_provider_ldpreload", provider_shim, provider_binary))

    reports: dict[str, tuple[list[float], dict[str, int]]] = {}
    for subject, ld_preload, provider in subject_specs:
        reports[subject] = run_subject(
            bench=bench,
            root=root,
            out_dir=out_dir,
            subject=subject,
            warmup_count=args.warmup_count,
            sample_count=args.sample_count,
            threads=args.threads,
            iters_per_thread=args.iters_per_thread,
            working_set=args.working_set,
            min_size=args.min_size,
            max_size=args.max_size,
            ld_preload=ld_preload,
            provider_binary=provider,
        )

    c_mimalloc_median = median_float(reports["c_mimalloc_ldpreload"][0])
    lines = [
        "output_contract=hakozuna-mixed-ws-ldpreload-compare-v0",
        "benchmark_id=bench_mixed_ws_crt",
        f"benchmark_path={bench}",
        f"hakozuna_root={root}",
        f"mimalloc_library={mimalloc_library}",
        f"provider_manifest={args.manifest.resolve() if args.manifest is not None else 'none'}",
        f"sample_count={args.sample_count}",
        f"warmup_count={args.warmup_count}",
        f"benchmark_threads={args.threads}",
        f"benchmark_iters_per_thread={args.iters_per_thread}",
        f"benchmark_working_set={args.working_set}",
        f"benchmark_min_size={args.min_size}",
        f"benchmark_max_size={args.max_size}",
        f"subject_count={len(subject_specs)}",
        "reference_subject=c_mimalloc_ldpreload",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
    ]
    for key in sorted(provider_manifest_metadata):
        lines.append(f"{key}={provider_manifest_metadata[key]}")
    if args.manifest is not None:
        lines.extend(
            [
                "provider_ldpreload_measurement_interpretation=provider_abi_wrapper_and_shim_bridge",
                "provider_ldpreload_is_product_allocator_claim=0",
                "provider_ldpreload_is_hako_core_speed_claim=0",
            ]
        )
    for index, (subject, _ld_preload, _provider) in enumerate(subject_specs):
        samples, counters = reports[subject]
        median = median_float(samples)
        lines.extend(
            [
                f"subject_{index}_id={subject}",
                f"subject_{index}_throughput_min_ops_per_sec={min(samples):.3f}",
                f"subject_{index}_throughput_median_ops_per_sec={median:.3f}",
                f"subject_{index}_throughput_max_ops_per_sec={max(samples):.3f}",
                f"subject_{index}_throughput_vs_c_mimalloc={format_ratio(median, c_mimalloc_median)}",
                f"subject_{index}_winner_claim=0",
            ]
        )
        if counters:
            for key in sorted(counters):
                lines.append(f"subject_{index}_{key}_total={counters[key]}")
            provider_ops = (
                counters.get("shim_provider_alloc_count", 0)
                + counters.get("shim_provider_calloc_count", 0)
                + counters.get("shim_provider_realloc_count", 0)
                + counters.get("shim_provider_free_count", 0)
            )
            init_fallback_dominates = init_fallback_dominates_provider_ops(counters, provider_ops)
            lines.extend(
                [
                    f"subject_{index}_shim_provider_operation_count_total={provider_ops}",
                    "subject_"
                    f"{index}_shim_init_real_fallback_per_provider_operation="
                    f"{format_per_operation(counters.get('shim_init_real_fallback_count', 0), provider_ops)}",
                    "subject_"
                    f"{index}_shim_host_passthrough_per_provider_operation="
                    f"{format_per_operation(counters.get('shim_host_passthrough_count', 0), provider_ops)}",
                    "subject_"
                    f"{index}_shim_init_real_fallback_dominates_provider_ops="
                    f"{1 if init_fallback_dominates else 0}",
                ]
            )
            if init_fallback_dominates:
                lines.extend(
                    [
                        "subject_"
                        f"{index}_next_owner_family=provider_alloc_free_internal_real_malloc_boundary",
                        "subject_"
                        f"{index}_gap_classification=provider_bridge_not_hako_core_speed",
                    ]
                )
            lines.append(f"subject_{index}_shim_init_real_fallback_is_perf_diagnostic=1")
    lines.append("summary=ok")
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

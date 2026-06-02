#!/usr/bin/env python3
"""Compare repo-local Hakmem fixtures under system, C mimalloc, and replacement front."""

from __future__ import annotations

import argparse
import os
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path

from hako_mimalloc_provider_backed_hakmem_ldpreload_bench_pilot import read_kv
from hakozuna_mixed_ws_ldpreload_compare import (
    build_replacement_front_shim,
    find_mimalloc_library,
    format_ratio,
    median_float,
    positive_int,
)


ROOT = Path(__file__).resolve().parents[2]
OPS_RE = re.compile(r"(?:Throughput\s*=\s*|ops/s=)([0-9]+(?:\.[0-9]+)?)")


@dataclass(frozen=True)
class FixtureSpec:
    fixture_id: str
    root: Path
    executable: str
    default_args: tuple[str, ...]
    role: str


FIXTURES: dict[str, FixtureSpec] = {
    "random-mixed-system": FixtureSpec(
        fixture_id="random-mixed-system",
        root=ROOT / "benchmarks" / "external" / "hakmem" / "random-mixed-system" / "build",
        executable="bench_random_mixed_system",
        default_args=("1000", "128", "42"),
        role="random mixed small allocation/free fixture",
    ),
    "tiny-hot-system": FixtureSpec(
        fixture_id="tiny-hot-system",
        root=ROOT / "benchmarks" / "external" / "hakmem" / "tiny-hot-system" / "build",
        executable="bench_tiny_hot_system",
        default_args=("64", "100", "1000"),
        role="small malloc/free hot-path fixture",
    ),
    "mid-large-mt-system": FixtureSpec(
        fixture_id="mid-large-mt-system",
        root=ROOT / "benchmarks" / "external" / "hakmem" / "mid-large-mt-system" / "build",
        executable="bench_mid_large_mt_system",
        default_args=("2", "1000", "128", "42"),
        role="8-32KiB multi-thread allocation/free fixture",
    ),
}


def run_one(
    *,
    bench: Path,
    root: Path,
    args: list[str],
    out_dir: Path,
    subject: str,
    run_index: int,
    kind: str,
    ld_preload: Path | None,
    replacement_front_mode: bool,
) -> tuple[float, dict[str, str]]:
    run_dir = out_dir / subject / f"{kind}_{run_index}"
    run_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = run_dir / "bench.stdout"
    stderr_path = run_dir / "bench.stderr"
    counts_path = run_dir / "replacement-front-counts.out"
    env = os.environ.copy()
    if ld_preload is not None:
        env["LD_PRELOAD"] = str(ld_preload)
    if replacement_front_mode:
        env["HAKORUNE_REPLACEMENT_FRONT_REPORT"] = str(counts_path)
    completed = subprocess.run(
        [str(bench), *args],
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
    counts = read_kv(counts_path) if counts_path.exists() else {}
    return float(match.group(1)), counts


def run_subject(
    *,
    bench: Path,
    root: Path,
    bench_args: list[str],
    out_dir: Path,
    subject: str,
    warmup_count: int,
    sample_count: int,
    ld_preload: Path | None,
    replacement_front_mode: bool,
) -> tuple[list[float], dict[str, int]]:
    samples: list[float] = []
    counter_totals: dict[str, int] = {}
    total_runs = warmup_count + sample_count
    for run_index in range(total_runs):
        kind = "warmup" if run_index < warmup_count else "sample"
        throughput, counts = run_one(
            bench=bench,
            root=root,
            args=bench_args,
            out_dir=out_dir,
            subject=subject,
            run_index=run_index,
            kind=kind,
            ld_preload=ld_preload,
            replacement_front_mode=replacement_front_mode,
        )
        for key, value in counts.items():
            if value.isdigit():
                counter_totals[key] = counter_totals.get(key, 0) + int(value)
        if kind == "sample":
            samples.append(throughput)
    return samples, counter_totals


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixture", choices=sorted(FIXTURES), required=True)
    parser.add_argument("--fixture-root", type=Path)
    parser.add_argument("--bench-arg", action="append", default=[])
    parser.add_argument("--mimalloc-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--sample-count", type=int, default=3)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument(
        "--replacement-front-native-slot-mode",
        action="store_true",
        help="benchmark-only: add a thin native-slot replacement front subject",
    )
    parser.add_argument(
        "--replacement-front-lock-mode",
        action="store_true",
        help="benchmark-only: build replacement front with a global mutex",
    )
    parser.add_argument(
        "--replacement-front-thread-local-mode",
        action="store_true",
        help="benchmark-only: build replacement front with same-thread TLS arenas",
    )
    parser.add_argument(
        "--replacement-front-skip-hot-counters",
        action="store_true",
        help="measurement-only: skip malloc/free hot-path replacement front counters",
    )
    parser.add_argument(
        "--replacement-front-tls-counter-mode",
        action="store_true",
        help="benchmark-only: aggregate replacement front counters through thread-local buffers",
    )
    parser.add_argument(
        "--replacement-front-slot-size",
        type=int,
        help="benchmark-only: override replacement front fixed slot size in bytes",
    )
    args = parser.parse_args()

    positive_int(args.sample_count, "--sample-count")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    if args.replacement_front_lock_mode and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-lock-mode requires --replacement-front-native-slot-mode"
        )
    if args.replacement_front_thread_local_mode and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-thread-local-mode requires --replacement-front-native-slot-mode"
        )
    if args.replacement_front_lock_mode and args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-lock-mode and --replacement-front-thread-local-mode are exclusive"
        )
    if args.replacement_front_skip_hot_counters and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-skip-hot-counters requires "
            "--replacement-front-native-slot-mode"
        )
    if args.replacement_front_tls_counter_mode and not args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-tls-counter-mode requires "
            "--replacement-front-thread-local-mode"
        )
    if args.replacement_front_tls_counter_mode and args.replacement_front_skip_hot_counters:
        raise SystemExit(
            "--replacement-front-tls-counter-mode and "
            "--replacement-front-skip-hot-counters are exclusive"
        )
    if args.replacement_front_slot_size is not None:
        positive_int(args.replacement_front_slot_size, "--replacement-front-slot-size")
        if not args.replacement_front_native_slot_mode:
            raise SystemExit(
                "--replacement-front-slot-size requires "
                "--replacement-front-native-slot-mode"
            )

    spec = FIXTURES[args.fixture]
    root = (args.fixture_root or spec.root).resolve()
    bench = root / spec.executable
    if not bench.is_file() or not os.access(bench, os.X_OK):
        raise SystemExit(
            f"missing executable for fixture {spec.fixture_id}: {bench}\n"
            f"hint: run `make -C {spec.root.parent.relative_to(ROOT)}` "
            "or pass --fixture-root"
        )
    bench_args = args.bench_arg or list(spec.default_args)
    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    mimalloc_library = find_mimalloc_library(args.mimalloc_library, args.allow_ldconfig_discovery)

    replacement_front_shim: Path | None = None
    if args.replacement_front_native_slot_mode:
        replacement_front_shim = build_replacement_front_shim(
            out_dir,
            locked=args.replacement_front_lock_mode,
            thread_local=args.replacement_front_thread_local_mode,
            skip_hot_counters=args.replacement_front_skip_hot_counters,
            tls_counters=args.replacement_front_tls_counter_mode,
            slot_size=args.replacement_front_slot_size,
        )

    subject_specs: list[tuple[str, Path | None, bool]] = [
        ("system_malloc", None, False),
        ("c_mimalloc_ldpreload", mimalloc_library, False),
    ]
    if replacement_front_shim is not None:
        subject_specs.append(
            ("hakorune_replacement_front_ldpreload", replacement_front_shim, True)
        )

    reports: dict[str, tuple[list[float], dict[str, int]]] = {}
    for subject, ld_preload, replacement_front_mode in subject_specs:
        reports[subject] = run_subject(
            bench=bench,
            root=root,
            bench_args=bench_args,
            out_dir=out_dir,
            subject=subject,
            warmup_count=args.warmup_count,
            sample_count=args.sample_count,
            ld_preload=ld_preload,
            replacement_front_mode=replacement_front_mode,
        )

    c_mimalloc_median = median_float(reports["c_mimalloc_ldpreload"][0])
    replacement_slot_size = args.replacement_front_slot_size or 2048
    lines = [
        "output_contract=hakmem-fixture-ldpreload-compare-v0",
        f"fixture_id={spec.fixture_id}",
        f"fixture_role={spec.role}",
        f"fixture_root={root}",
        f"benchmark_id={spec.executable}",
        f"benchmark_path={bench}",
        "benchmark_args=" + ",".join(bench_args),
        f"mimalloc_library={mimalloc_library}",
        f"sample_count={args.sample_count}",
        f"warmup_count={args.warmup_count}",
        f"subject_count={len(subject_specs)}",
        "reference_subject=c_mimalloc_ldpreload",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
        f"replacement_front_native_slot_mode={1 if args.replacement_front_native_slot_mode else 0}",
        f"replacement_front_lock_mode={1 if args.replacement_front_lock_mode else 0}",
        f"replacement_front_thread_local_mode={1 if args.replacement_front_thread_local_mode else 0}",
        f"replacement_front_skip_hot_counters={1 if args.replacement_front_skip_hot_counters else 0}",
        f"replacement_front_tls_counter_mode={1 if args.replacement_front_tls_counter_mode else 0}",
        f"replacement_front_slot_size={replacement_slot_size}",
    ]
    for index, (subject, _ld_preload, replacement_front_mode) in enumerate(subject_specs):
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
        if replacement_front_mode:
            lines.extend(
                [
                    f"subject_{index}_provider_table_dispatch=0",
                    f"subject_{index}_function_pointer_hot_call=0",
                    f"subject_{index}_owns_check_hot_path=0",
                    f"subject_{index}_tracking_hot_path=0",
                    f"subject_{index}_direct_core_call=1",
                    f"subject_{index}_provider_api_hot_path_required=0",
                    f"subject_{index}_activation=0",
                    f"subject_{index}_benchmark_only=1",
                    f"subject_{index}_replacement_front_hotpath_plan_v0=1",
                    f"subject_{index}_replacement_front_slot_size={replacement_slot_size}",
                ]
            )
        for key in sorted(counters):
            lines.append(f"subject_{index}_{key}_total={counters[key]}")
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

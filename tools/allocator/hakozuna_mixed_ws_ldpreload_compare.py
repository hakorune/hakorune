#!/usr/bin/env python3
"""Compare Hakozuna mixed-ws under system, C mimalloc, and optional provider LD_PRELOAD."""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

from hako_mimalloc_provider_backed_hakmem_ldpreload_bench_pilot import read_kv
from hakozuna_mixed_ws_report_render import render_hakozuna_mixed_ws_report
from replacement_front_smokes import run_replacement_front_cross_thread_smokes


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_HAKOZUNA_ROOT = ROOT / "benchmarks" / "external" / "hakozuna" / "mixed-ws" / "build"
SMOKE_TOOL = Path(__file__).resolve().with_name("provider_package_ldpreload_replacement_smoke.py")
OPS_RE = re.compile(r"ops/s=([0-9]+(?:\.[0-9]+)?)")
TIME_RE = re.compile(r"time=([0-9]+(?:\.[0-9]+)?)")


from replacement_front_support import (
    hako_good_size,
    hako_size_to_bin,
    mixed_ws_workload_histogram,
    positive_int,
)
from replacement_front_templates import (
    REPLACEMENT_FRONT_SHIM_C,
    generate_replacement_front_bins_shim_c,
)
def build_replacement_front_shim(
    out_dir: Path,
    *,
    locked: bool,
    thread_local: bool,
    skip_hot_counters: bool,
    tls_counters: bool,
    slot_size: int | None,
) -> Path:
    front_dir = out_dir / (
        "replacement-front-native-slot-locked" if locked else "replacement-front-native-slot"
    )
    if thread_local:
        front_dir = out_dir / "replacement-front-native-slot-thread-local"
    if skip_hot_counters:
        front_dir = out_dir / f"{front_dir.name}-skip-hot-counters"
    if tls_counters:
        front_dir = out_dir / f"{front_dir.name}-tls-counters"
    if slot_size is not None:
        front_dir = out_dir / f"{front_dir.name}-slot-size-{slot_size}"
    front_dir.mkdir(parents=True, exist_ok=True)
    source = front_dir / "hako_alloc_replacement_front_native_slot.c"
    binary = front_dir / "libhako_alloc_replacement_front_native_slot.so"
    source.write_text(REPLACEMENT_FRONT_SHIM_C.lstrip(), encoding="utf-8")
    cmd = [
        "cc",
        "-shared",
        "-fPIC",
        "-O3",
        "-Wall",
        "-Wextra",
    ]
    if locked:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_LOCKED=1")
    if thread_local:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_THREAD_LOCAL=1")
    if skip_hot_counters:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS=1")
    if tls_counters:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_TLS_COUNTERS=1")
    if slot_size is not None:
        cmd.append(f"-DHAKO_REPLACEMENT_SLOT_SIZE={slot_size}u")
    cmd.extend([str(source), "-ldl"])
    if locked or thread_local:
        cmd.append("-pthread")
    cmd.extend(["-o", str(binary)])
    subprocess.run(cmd, check=True)
    return binary


def build_replacement_front_bins_shim(
    out_dir: Path,
    *,
    required_bins: list[int],
    page_shaped: bool = False,
    hotcore_page_model: bool = False,
    size_class_table: bool = False,
    eager_init: bool = False,
    product_pages_nonlinear_lookup: bool = False,
) -> Path:
    front_name = "replacement-front-page-bins" if page_shaped else "replacement-front-native-bins"
    if hotcore_page_model:
        front_name = f"{front_name}-hotcore-page-model"
    if size_class_table:
        front_name = f"{front_name}-size-table"
    if eager_init:
        front_name = f"{front_name}-eager-init"
    if product_pages_nonlinear_lookup:
        front_name = f"{front_name}-product-pages-nonlinear"
    source_name = (
        "hako_alloc_replacement_front_page_bins.c"
        if page_shaped
        else "hako_alloc_replacement_front_native_bins.c"
    )
    binary_name = (
        "libhako_alloc_replacement_front_page_bins.so"
        if page_shaped
        else "libhako_alloc_replacement_front_native_bins.so"
    )
    front_dir = out_dir / front_name
    front_dir.mkdir(parents=True, exist_ok=True)
    source = front_dir / source_name
    binary = front_dir / binary_name
    source.write_text(
        generate_replacement_front_bins_shim_c(
            required_bins,
            page_shaped=page_shaped,
            hotcore_page_model=hotcore_page_model,
            size_class_table=size_class_table,
            eager_init=eager_init,
            product_pages_nonlinear_lookup=product_pages_nonlinear_lookup,
        ).lstrip(),
        encoding="utf-8",
    )
    subprocess.run(
        [
            "cc",
            "-shared",
            "-fPIC",
            "-O3",
            "-Wall",
            "-Wextra",
            str(source),
            "-ldl",
            "-o",
            str(binary),
        ],
        check=True,
    )
    return binary


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
    provider_usable_size_mode: bool,
    provider_assume_owned_mode: bool,
    replacement_front_mode: bool,
) -> tuple[float, float, dict[str, str], int]:
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
        if provider_usable_size_mode:
            env["HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE"] = "1"
        if provider_assume_owned_mode:
            env["HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED"] = "1"
    if replacement_front_mode:
        env["HAKORUNE_REPLACEMENT_FRONT_REPORT"] = str(counts_path)
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
    time_match = TIME_RE.search(completed.stdout)
    if time_match is None:
        raise SystemExit(f"{subject} {kind} run {run_index} output missing time line")
    counts = read_kv(counts_path) if counts_path.exists() else {}
    return float(match.group(1)), float(time_match.group(1)), counts, completed.returncode


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
    provider_usable_size_mode: bool,
    provider_assume_owned_mode: bool,
    replacement_front_mode: bool,
) -> tuple[list[float], list[float], dict[str, int]]:
    sample_throughputs: list[float] = []
    sample_seconds: list[float] = []
    counter_totals: dict[str, int] = {}
    total_runs = warmup_count + sample_count
    for run_index in range(total_runs):
        kind = "warmup" if run_index < warmup_count else "sample"
        throughput, elapsed_seconds, counts, _exit_code = run_one(
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
            provider_usable_size_mode=provider_usable_size_mode,
            provider_assume_owned_mode=provider_assume_owned_mode,
            replacement_front_mode=replacement_front_mode,
        )
        for key, value in counts.items():
            if value.isdigit():
                counter_totals[key] = counter_totals.get(key, 0) + int(value)
        if kind == "sample":
            sample_throughputs.append(throughput)
            sample_seconds.append(elapsed_seconds)
    return sample_throughputs, sample_seconds, counter_totals


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
    parser.add_argument(
        "--min-sample-seconds",
        type=float,
        default=0.0,
        help=(
            "mark measurement_quality=too_short if any sampled bench run is "
            "shorter than this many seconds; default 0 keeps legacy smoke-sized "
            "probes accepted"
        ),
    )
    parser.add_argument("--threads", type=int, default=1)
    parser.add_argument("--iters-per-thread", type=int, default=1000)
    parser.add_argument("--working-set", type=int, default=128)
    parser.add_argument("--min-size", type=int, default=16)
    parser.add_argument("--max-size", type=int, default=1024)
    parser.add_argument(
        "--provider-usable-size-mode",
        action="store_true",
        help="measurement-only: bypass provider shim tracking through private usable_size symbol",
    )
    parser.add_argument(
        "--provider-assume-owned-mode",
        action="store_true",
        help="measurement-only: with usable-size mode, skip provider owns checks before free/realloc",
    )
    parser.add_argument(
        "--replacement-front-native-slot-mode",
        action="store_true",
        help="benchmark-only: add a thin native-slot malloc/free replacement front subject",
    )
    parser.add_argument(
        "--replacement-front-native-bins-mode",
        action="store_true",
        help=(
            "benchmark-only: add a thin native multi-bin malloc/free replacement "
            "front subject using workload .hako size-class bins"
        ),
    )
    parser.add_argument(
        "--replacement-front-page-bins-mode",
        action="store_true",
        help=(
            "benchmark-only: add a page-shaped multi-bin malloc/free replacement "
            "front subject using workload .hako size-class bins"
        ),
    )
    parser.add_argument(
        "--replacement-front-hotcore-page-model-mode",
        action="store_true",
        help=(
            "benchmark-only: with page-bins mode, route alloc/free through "
            "HotCore/PageModel-shaped acquire/release helpers"
        ),
    )
    parser.add_argument(
        "--replacement-front-product-pages-nonlinear-mode",
        action="store_true",
        help=(
            "benchmark-only: requires page-bins mode; use a page-key indexed "
            "ownership lookup instead of the linear generated find_owned scan. "
            "HotCore/eager-init/size-table are the recommended measurement stack."
        ),
    )
    parser.add_argument(
        "--replacement-front-size-class-table-mode",
        action="store_true",
        help=(
            "benchmark-only: with bins mode, lower SizeClassBox size lookup "
            "through an 8-byte bucket table instead of an ordered range scan"
        ),
    )
    parser.add_argument(
        "--replacement-front-eager-init-mode",
        action="store_true",
        help=(
            "benchmark-only: with bins mode, initialize replacement bins in the "
            "constructor and keep hot malloc on the already-initialized path"
        ),
    )
    parser.add_argument(
        "--replacement-front-lock-mode",
        action="store_true",
        help="benchmark-only: build the replacement front with a global arena mutex",
    )
    parser.add_argument(
        "--replacement-front-thread-local-mode",
        action="store_true",
        help="benchmark-only: build the replacement front with same-thread TLS arenas",
    )
    parser.add_argument(
        "--replacement-front-cross-thread-smoke",
        action="store_true",
        help="run focused cross-thread free and abandoned-owner replacement front smokes",
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
    parser.add_argument(
        "--replacement-front-match-workload-realloc-size",
        action="store_true",
        help=(
            "benchmark-only: set replacement front slot size to max-size + 16, "
            "matching the mixed-ws realloc grow request"
        ),
    )
    parser.add_argument(
        "--replacement-front-match-hako-size-class",
        action="store_true",
        help=(
            "benchmark-only: set replacement front slot size to "
            "SizeClassBox.good_size(max-size + 16), matching the mixed-ws "
            "allocation/realloc request ceiling"
        ),
    )
    args = parser.parse_args()

    positive_int(args.sample_count, "--sample-count")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    if args.min_sample_seconds < 0.0:
        raise SystemExit("--min-sample-seconds must be non-negative")
    positive_int(args.threads, "--threads")
    positive_int(args.iters_per_thread, "--iters-per-thread")
    positive_int(args.working_set, "--working-set")
    positive_int(args.min_size, "--min-size")
    positive_int(args.max_size, "--max-size")
    if args.max_size < args.min_size:
        raise SystemExit("--max-size must be >= --min-size")
    replacement_shape_modes = sum(
        1
        for enabled in (
            args.replacement_front_native_slot_mode,
            args.replacement_front_native_bins_mode,
            args.replacement_front_page_bins_mode,
        )
        if enabled
    )
    if replacement_shape_modes > 1:
        raise SystemExit(
            "--replacement-front-native-slot-mode, "
            "--replacement-front-native-bins-mode, and "
            "--replacement-front-page-bins-mode are exclusive"
        )
    match_modes = sum(
        1
        for enabled in (
            args.replacement_front_slot_size is not None,
            args.replacement_front_match_workload_realloc_size,
            args.replacement_front_match_hako_size_class,
        )
        if enabled
    )
    if match_modes > 1:
        raise SystemExit(
            "--replacement-front-slot-size, "
            "--replacement-front-match-workload-realloc-size, and "
            "--replacement-front-match-hako-size-class are mutually exclusive"
        )
    replacement_front_size_class_request_ceiling = args.max_size + 16
    replacement_front_size_class_selected_bin = hako_size_to_bin(
        replacement_front_size_class_request_ceiling
    )
    replacement_front_size_class_selected_good_size = hako_good_size(
        replacement_front_size_class_request_ceiling
    )
    if args.replacement_front_match_workload_realloc_size:
        args.replacement_front_slot_size = args.max_size + 16
    if args.replacement_front_match_hako_size_class:
        if replacement_front_size_class_selected_good_size <= 0:
            raise SystemExit(
                "--replacement-front-match-hako-size-class selected huge bin; "
                "use --replacement-front-slot-size explicitly for this workload"
            )
        args.replacement_front_slot_size = replacement_front_size_class_selected_good_size
    if args.replacement_front_slot_size is not None:
        positive_int(args.replacement_front_slot_size, "--replacement-front-slot-size")
        if args.replacement_front_slot_size < args.max_size:
            raise SystemExit("--replacement-front-slot-size must be >= --max-size")
    if args.provider_assume_owned_mode and not args.provider_usable_size_mode:
        raise SystemExit("--provider-assume-owned-mode requires --provider-usable-size-mode")
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
    if args.replacement_front_cross_thread_smoke and not args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-cross-thread-smoke requires "
            "--replacement-front-thread-local-mode"
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
    if args.replacement_front_slot_size is not None and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-slot-size requires --replacement-front-native-slot-mode"
        )
    if (
        args.replacement_front_match_workload_realloc_size
        and not args.replacement_front_native_slot_mode
    ):
        raise SystemExit(
            "--replacement-front-match-workload-realloc-size requires "
            "--replacement-front-native-slot-mode"
        )
    if (
        args.replacement_front_match_hako_size_class
        and not args.replacement_front_native_slot_mode
    ):
        raise SystemExit(
            "--replacement-front-match-hako-size-class requires "
            "--replacement-front-native-slot-mode"
        )
    if args.replacement_front_cross_thread_smoke and args.replacement_front_skip_hot_counters:
        raise SystemExit(
            "--replacement-front-cross-thread-smoke cannot be combined with "
            "--replacement-front-skip-hot-counters because the smoke validates counters"
        )
    if (
        args.replacement_front_hotcore_page_model_mode
        and not args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-hotcore-page-model-mode requires "
            "--replacement-front-page-bins-mode"
        )
    if (
        args.replacement_front_product_pages_nonlinear_mode
        and not args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-product-pages-nonlinear-mode requires "
            "--replacement-front-page-bins-mode"
        )
    if args.replacement_front_size_class_table_mode and not (
        args.replacement_front_native_bins_mode or args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-size-class-table-mode requires "
            "--replacement-front-native-bins-mode or --replacement-front-page-bins-mode"
        )
    if args.replacement_front_eager_init_mode and not (
        args.replacement_front_native_bins_mode or args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-eager-init-mode requires "
            "--replacement-front-native-bins-mode or --replacement-front-page-bins-mode"
        )
    replacement_front_bins_mode = (
        args.replacement_front_native_bins_mode or args.replacement_front_page_bins_mode
    )
    if replacement_front_bins_mode:
        if args.threads != 1:
            raise SystemExit(
                "--replacement-front-native-bins-mode and "
                "--replacement-front-page-bins-mode are v0 single-thread only"
            )
        if (
            args.replacement_front_lock_mode
            or args.replacement_front_thread_local_mode
            or args.replacement_front_cross_thread_smoke
            or args.replacement_front_skip_hot_counters
            or args.replacement_front_tls_counter_mode
            or args.replacement_front_slot_size is not None
        ):
            raise SystemExit(
                "--replacement-front-native-bins-mode and "
                "--replacement-front-page-bins-mode cannot be combined with "
                "slot/thread/counter replacement-front modifiers in v0"
            )

    replacement_slot_size = args.replacement_front_slot_size or 2048
    workload_histogram = mixed_ws_workload_histogram(
        threads=args.threads,
        iters_per_thread=args.iters_per_thread,
        working_set=args.working_set,
        min_size=args.min_size,
        max_size=args.max_size,
        replacement_slot_size=replacement_slot_size,
    )
    required_regular_bins = [
        int(part)
        for part in str(workload_histogram["size_class_regular_bins"]).split(",")
        if part and part != "none"
    ]

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

    provider_shim: Path | None = None
    provider_binary: Path | None = None
    replacement_front_shim: Path | None = None
    if args.manifest is not None:
        smoke_report = out_dir / "provider-ldpreload-smoke.out"
        smoke_cmd = [
            sys.executable,
            str(SMOKE_TOOL),
            "--manifest",
            str(args.manifest.resolve()),
            "--out-dir",
            str(out_dir / "provider-ldpreload-smoke"),
            "--out",
            str(smoke_report),
        ]
        if args.provider_usable_size_mode:
            smoke_cmd.append("--use-provider-usable-size")
        if args.provider_assume_owned_mode:
            smoke_cmd.append("--assume-provider-owned")
        subprocess.run(smoke_cmd, check=True)
        smoke = read_kv(smoke_report)
        provider_shim = Path(smoke["shim_artifact_path"])
        provider_binary = Path(smoke["provider_binary_path"])
    if args.replacement_front_native_slot_mode:
        replacement_front_shim = build_replacement_front_shim(
            out_dir,
            locked=args.replacement_front_lock_mode,
            thread_local=args.replacement_front_thread_local_mode,
            skip_hot_counters=args.replacement_front_skip_hot_counters,
            tls_counters=args.replacement_front_tls_counter_mode,
            slot_size=args.replacement_front_slot_size,
        )
    if replacement_front_bins_mode:
        if not required_regular_bins:
            raise SystemExit(
                "--replacement-front-native-bins-mode/--replacement-front-page-bins-mode "
                "found no regular bins"
            )
        if int(workload_histogram["size_class_huge_count"]) > 0:
            raise SystemExit(
                "--replacement-front-native-bins-mode/--replacement-front-page-bins-mode "
                "v0 does not support huge bins"
            )
        replacement_front_shim = build_replacement_front_bins_shim(
            out_dir,
            required_bins=required_regular_bins,
            page_shaped=args.replacement_front_page_bins_mode,
            hotcore_page_model=args.replacement_front_hotcore_page_model_mode,
            size_class_table=args.replacement_front_size_class_table_mode,
            eager_init=args.replacement_front_eager_init_mode,
            product_pages_nonlinear_lookup=args.replacement_front_product_pages_nonlinear_mode,
        )
    replacement_front_smokes: dict[str, dict[str, str]] = {}
    if args.replacement_front_cross_thread_smoke:
        if replacement_front_shim is None:
            raise SystemExit("--replacement-front-cross-thread-smoke requires a replacement front")
        replacement_front_smokes = run_replacement_front_cross_thread_smokes(
            out_dir=out_dir,
            replacement_front_shim=replacement_front_shim,
        )

    subject_specs: list[tuple[str, Path | None, Path | None, bool]] = [
        ("system_malloc", None, None, False),
        ("c_mimalloc_ldpreload", mimalloc_library, None, False),
    ]
    if provider_shim is not None and provider_binary is not None:
        subject_specs.append(("hakorune_provider_ldpreload", provider_shim, provider_binary, False))
    if replacement_front_shim is not None:
        subject_specs.append(
            ("hakorune_replacement_front_ldpreload", replacement_front_shim, None, True)
        )

    reports: dict[str, tuple[list[float], list[float], dict[str, int]]] = {}
    for subject, ld_preload, provider, replacement_front_mode in subject_specs:
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
            provider_usable_size_mode=(
                args.provider_usable_size_mode and subject == "hakorune_provider_ldpreload"
            ),
            provider_assume_owned_mode=(
                args.provider_assume_owned_mode and subject == "hakorune_provider_ldpreload"
            ),
            replacement_front_mode=replacement_front_mode,
        )
    report = render_hakozuna_mixed_ws_report(
        args=args,
        bench=bench,
        root=root,
        mimalloc_library=mimalloc_library,
        workload_histogram=workload_histogram,
        replacement_front_smokes=replacement_front_smokes,
        subject_specs=subject_specs,
        reports=reports,
        replacement_front_bins_mode=replacement_front_bins_mode,
        replacement_slot_size=replacement_slot_size,
        replacement_front_size_class_request_ceiling=replacement_front_size_class_request_ceiling,
        replacement_front_size_class_selected_bin=replacement_front_size_class_selected_bin,
        replacement_front_size_class_selected_good_size=(
            replacement_front_size_class_selected_good_size
        ),
    )
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

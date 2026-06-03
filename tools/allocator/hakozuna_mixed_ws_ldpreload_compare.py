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


from replacement_front_templates import (
    REPLACEMENT_FRONT_ABANDONED_OWNER_SMOKE_C,
    REPLACEMENT_FRONT_CROSS_THREAD_FREE_SMOKE_C,
    REPLACEMENT_FRONT_CROSS_THREAD_REALLOC_SMOKE_C,
    REPLACEMENT_FRONT_SHIM_C,
    WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS,
    counter_value,
    generate_replacement_front_bins_shim_c,
    hako_good_size,
    hako_size_class_bin_size,
    hako_size_to_bin,
    median_float,
    mixed_ws_workload_histogram,
    positive_int,
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
) -> Path:
    front_name = "replacement-front-page-bins" if page_shaped else "replacement-front-native-bins"
    if hotcore_page_model:
        front_name = f"{front_name}-hotcore-page-model"
    if size_class_table:
        front_name = f"{front_name}-size-table"
    if eager_init:
        front_name = f"{front_name}-eager-init"
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


def build_c_smoke(out_dir: Path, *, name: str, source_text: str) -> Path:
    smoke_dir = out_dir / "replacement-front-cross-thread-smoke"
    smoke_dir.mkdir(parents=True, exist_ok=True)
    source = smoke_dir / f"{name}.c"
    binary = smoke_dir / f"{name}.bin"
    source.write_text(source_text.lstrip(), encoding="utf-8")
    subprocess.run(
        [
            "cc",
            "-O2",
            "-Wall",
            "-Wextra",
            str(source),
            "-pthread",
            "-o",
            str(binary),
        ],
        check=True,
    )
    return binary


def run_replacement_front_focused_smoke(
    *,
    out_dir: Path,
    replacement_front_shim: Path,
    name: str,
    source_text: str,
) -> dict[str, str]:
    binary = build_c_smoke(out_dir, name=name, source_text=source_text)
    smoke_dir = out_dir / "replacement-front-cross-thread-smoke" / name
    smoke_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = smoke_dir / "stdout.txt"
    stderr_path = smoke_dir / "stderr.txt"
    report_path = smoke_dir / "replacement-front-report.out"
    env = os.environ.copy()
    env["LD_PRELOAD"] = str(replacement_front_shim)
    env["HAKORUNE_REPLACEMENT_FRONT_REPORT"] = str(report_path)
    completed = subprocess.run(
        [str(binary)],
        cwd=out_dir,
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
            f"replacement front focused smoke {name} failed with "
            f"{completed.returncode}: {completed.stderr.strip()}"
        )
    if not report_path.exists():
        raise SystemExit(f"replacement front focused smoke {name} did not write a report")
    return read_kv(report_path)


def run_replacement_front_cross_thread_smokes(
    *,
    out_dir: Path,
    replacement_front_shim: Path,
) -> dict[str, dict[str, str]]:
    cross_thread_free = run_replacement_front_focused_smoke(
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        name="cross_thread_free",
        source_text=REPLACEMENT_FRONT_CROSS_THREAD_FREE_SMOKE_C,
    )
    if counter_value(cross_thread_free, "replacement_front_remote_free_push_count") < 1:
        raise SystemExit("cross_thread_free smoke did not push a remote free")
    if counter_value(cross_thread_free, "replacement_front_remote_free_drain_count") < 1:
        raise SystemExit("cross_thread_free smoke did not drain a remote free")
    if counter_value(cross_thread_free, "replacement_front_arena_registry_overflow_count") != 0:
        raise SystemExit("cross_thread_free smoke overflowed the arena registry")

    abandoned_owner = run_replacement_front_focused_smoke(
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        name="abandoned_owner",
        source_text=REPLACEMENT_FRONT_ABANDONED_OWNER_SMOKE_C,
    )
    if counter_value(abandoned_owner, "replacement_front_abandoned_arena_count") < 1:
        raise SystemExit("abandoned_owner smoke did not mark an abandoned arena")
    if counter_value(abandoned_owner, "replacement_front_abandoned_remote_free_count") < 1:
        raise SystemExit("abandoned_owner smoke did not count abandoned remote free")
    if counter_value(abandoned_owner, "replacement_front_host_passthrough_count") != 0:
        raise SystemExit("abandoned_owner smoke passed a recognized pointer to host free")

    cross_thread_realloc = run_replacement_front_focused_smoke(
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        name="cross_thread_realloc",
        source_text=REPLACEMENT_FRONT_CROSS_THREAD_REALLOC_SMOKE_C,
    )
    if (
        counter_value(
            cross_thread_realloc,
            "replacement_front_cross_thread_realloc_unsupported_count",
        )
        < 1
    ):
        raise SystemExit("cross_thread_realloc smoke did not count unsupported realloc")
    if counter_value(cross_thread_realloc, "replacement_front_host_passthrough_count") != 0:
        raise SystemExit("cross_thread_realloc smoke used host realloc/free fallback")

    return {
        "cross_thread_free": cross_thread_free,
        "abandoned_owner": abandoned_owner,
        "cross_thread_realloc": cross_thread_realloc,
    }


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
    counts = read_kv(counts_path) if counts_path.exists() else {}
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
    provider_usable_size_mode: bool,
    provider_assume_owned_mode: bool,
    replacement_front_mode: bool,
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
            provider_usable_size_mode=provider_usable_size_mode,
            provider_assume_owned_mode=provider_assume_owned_mode,
            replacement_front_mode=replacement_front_mode,
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
    provider_manifest_metadata = load_manifest_metadata(
        args.manifest.resolve() if args.manifest else None
    )

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

    reports: dict[str, tuple[list[float], dict[str, int]]] = {}
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

    c_mimalloc_median = median_float(reports["c_mimalloc_ldpreload"][0])
    replacement_front_size_class_policy_source = (
        "hako_size_class_box_report_mirror"
        if args.replacement_front_match_hako_size_class
        or replacement_front_bins_mode
        else "hako_model_not_consumed"
    )
    replacement_front_algorithm_shape = (
        "page_bin_hotcore_page_model_benchmark_front"
        if args.replacement_front_hotcore_page_model_mode
        else "page_bin_benchmark_front"
        if args.replacement_front_page_bins_mode
        else "multi_bin_native_benchmark_front"
        if args.replacement_front_native_bins_mode
        else "fixed_slot_native_benchmark_front"
    )
    replacement_front_size_class_bridge_enabled = int(
        args.replacement_front_match_hako_size_class
        or replacement_front_bins_mode
    )
    replacement_front_size_class_bridge_mode = (
        "workload_regular_bins_page_shaped_hotcore_page_model_hako_size_class"
        if args.replacement_front_hotcore_page_model_mode
        else "workload_regular_bins_page_shaped_hako_size_class"
        if args.replacement_front_page_bins_mode
        else "workload_regular_bins_hako_size_class"
        if args.replacement_front_native_bins_mode
        else (
            "hako_good_size_request_ceiling"
            if args.replacement_front_match_hako_size_class
            else "none"
        )
    )
    replacement_front_evidence_owner = "none"
    replacement_front_multithread_perf_candidate = 0
    replacement_front_thread_local_perf_candidate = 0
    replacement_front_correctness_smoke = 0
    if args.replacement_front_page_bins_mode:
        replacement_front_evidence_owner = (
            "single_thread_page_bins_hotcore_page_model"
            if args.replacement_front_hotcore_page_model_mode
            else "single_thread_page_bins"
        )
    elif args.replacement_front_native_bins_mode:
        replacement_front_evidence_owner = "single_thread_native_bins"
    elif args.replacement_front_native_slot_mode:
        replacement_front_evidence_owner = "fixed_slot_native_front"
        if args.threads > 1 and args.replacement_front_lock_mode:
            replacement_front_evidence_owner = "locked_global_multithread_front"
            if args.replacement_front_skip_hot_counters:
                replacement_front_multithread_perf_candidate = 1
        elif args.threads > 1 and args.replacement_front_thread_local_mode:
            replacement_front_evidence_owner = "thread_local_multithread_front"
            replacement_front_thread_local_perf_candidate = int(
                args.replacement_front_skip_hot_counters
            )
            replacement_front_correctness_smoke = int(args.replacement_front_cross_thread_smoke)
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
        f"provider_usable_size_mode={1 if args.provider_usable_size_mode else 0}",
        f"provider_assume_owned_mode={1 if args.provider_assume_owned_mode else 0}",
        f"replacement_front_native_slot_mode={1 if args.replacement_front_native_slot_mode else 0}",
        f"replacement_front_native_bins_mode={1 if args.replacement_front_native_bins_mode else 0}",
        f"replacement_front_page_bins_mode={1 if args.replacement_front_page_bins_mode else 0}",
        "replacement_front_hotcore_page_model_mode="
        f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
        "replacement_front_size_class_table_mode="
        f"{1 if args.replacement_front_size_class_table_mode else 0}",
        "replacement_front_eager_init_mode="
        f"{1 if args.replacement_front_eager_init_mode else 0}",
        "replacement_front_is_full_hako_algorithm=0",
        f"replacement_front_algorithm_shape={replacement_front_algorithm_shape}",
        "replacement_front_size_class_bridge_plan_v0=1",
        "replacement_front_size_class_bridge_report_only=1",
        f"replacement_front_size_class_policy_bridge={replacement_front_size_class_bridge_enabled}",
        "replacement_front_size_class_count="
        f"{workload_histogram['size_class_regular_distinct_count'] if replacement_front_bins_mode else 1}",
        f"replacement_front_size_class_policy_source={replacement_front_size_class_policy_source}",
        f"replacement_front_size_class_bridge_mode={replacement_front_size_class_bridge_mode}",
        "replacement_front_size_class_lookup_route="
        f"{'table_8byte_bucket' if args.replacement_front_size_class_table_mode else 'range_scan' if replacement_front_bins_mode else 'not_consumed'}",
        "replacement_front_size_class_request_ceiling="
        f"{replacement_front_size_class_request_ceiling}",
        "replacement_front_size_class_selected_bin="
        f"{replacement_front_size_class_selected_bin}",
        "replacement_front_size_class_selected_good_size="
        f"{replacement_front_size_class_selected_good_size}",
        "replacement_front_product_bins_plan_v0=1",
        "replacement_front_product_bins_report_only=1",
        "replacement_front_product_bins_consumer_enabled="
        f"{1 if replacement_front_bins_mode else 0}",
        "replacement_front_product_bins_connected=0",
        "replacement_front_product_bins_route="
        f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'benchmark_page_bins' if args.replacement_front_page_bins_mode else 'benchmark_native_bins' if args.replacement_front_native_bins_mode else 'not_consumed'}",
        "replacement_front_product_pages_plan_v0=1",
        "replacement_front_product_pages_report_only=1",
        "replacement_front_product_pages_consumer_enabled=0",
        "replacement_front_product_pages_connected=0",
        "replacement_front_page_bins_plan_v0=1",
        "replacement_front_page_bins_report_only=1",
        "replacement_front_page_bins_consumer_enabled="
        f"{1 if args.replacement_front_page_bins_mode else 0}",
        "replacement_front_page_bins_route="
        f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'benchmark_page_bins' if args.replacement_front_page_bins_mode else 'not_consumed'}",
        "replacement_front_page_bins_lookup_route="
        f"{'range_scan' if args.replacement_front_page_bins_mode else 'not_consumed'}",
        "replacement_front_page_bins_owner=benchmark_only",
        "replacement_front_page_bins_product_claim=0",
        "replacement_front_product_bins_required_regular_distinct_count="
        f"{workload_histogram['size_class_regular_distinct_count']}",
        "replacement_front_product_bins_required_regular_bins="
        f"{workload_histogram['size_class_regular_bins']}",
        "replacement_front_product_bins_required_max_bin="
        f"{workload_histogram['size_class_max_bin']}",
        "replacement_front_product_bins_huge_route_required="
        f"{1 if int(workload_histogram['size_class_huge_count']) > 0 else 0}",
        "replacement_front_hotcore_bridge_plan_v0=1",
        "replacement_front_hotcore_bridge_report_only=1",
        "replacement_front_hotcore_consumer_enabled="
        f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
        "replacement_front_hotcore_route="
        f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'not_consumed_by_replacement_front'}",
        "hako_mimalloc_algorithm_claim=0",
        f"replacement_front_lock_mode={1 if args.replacement_front_lock_mode else 0}",
        f"replacement_front_thread_local_mode={1 if args.replacement_front_thread_local_mode else 0}",
        f"replacement_front_evidence_owner={replacement_front_evidence_owner}",
        "replacement_front_multithread_perf_candidate="
        f"{replacement_front_multithread_perf_candidate}",
        "replacement_front_thread_local_perf_candidate="
        f"{replacement_front_thread_local_perf_candidate}",
        f"replacement_front_correctness_smoke={replacement_front_correctness_smoke}",
        f"replacement_front_cross_thread_smoke={1 if args.replacement_front_cross_thread_smoke else 0}",
        f"replacement_front_skip_hot_counters={1 if args.replacement_front_skip_hot_counters else 0}",
        f"replacement_front_tls_counter_mode={1 if args.replacement_front_tls_counter_mode else 0}",
        f"replacement_front_slot_size={replacement_slot_size}",
        "replacement_front_match_workload_realloc_size="
        f"{1 if args.replacement_front_match_workload_realloc_size else 0}",
        "replacement_front_match_hako_size_class="
        f"{1 if args.replacement_front_match_hako_size_class else 0}",
        f"workload_size_histogram_source={workload_histogram['source']}",
        "workload_size_histogram_max_total_iters="
        f"{WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS}",
        "workload_size_histogram_sample_exact="
        f"{workload_histogram['sample_exact']}",
        "workload_size_histogram_sampled_iters_per_thread="
        f"{workload_histogram['sampled_iters_per_thread']}",
        "workload_size_histogram_sampled_total_iterations="
        f"{workload_histogram['sampled_total_iterations']}",
        "workload_size_histogram_full_total_iterations="
        f"{workload_histogram['full_total_iterations']}",
        "workload_alloc_request_count="
        f"{workload_histogram['alloc_request_count']}",
        "workload_free_path_count="
        f"{workload_histogram['free_path_count']}",
        "workload_cleanup_free_count="
        f"{workload_histogram['cleanup_free_count']}",
        "workload_realloc_request_count="
        f"{workload_histogram['realloc_request_count']}",
        "workload_realloc_request_gt_replacement_slot_size="
        f"{workload_histogram['realloc_request_gt_replacement_slot_size']}",
        "workload_realloc_request_gt_max_size="
        f"{workload_histogram['realloc_request_gt_max_size']}",
        "workload_memset_le_64_count="
        f"{workload_histogram['memset_le_64_count']}",
        "workload_memset_gt_64_count="
        f"{workload_histogram['memset_gt_64_count']}",
        "workload_size_class_policy_source="
        f"{workload_histogram['size_class_policy_source']}",
        "workload_size_class_distinct_count="
        f"{workload_histogram['size_class_distinct_count']}",
        "workload_size_class_regular_distinct_count="
        f"{workload_histogram['size_class_regular_distinct_count']}",
        "workload_size_class_regular_bins="
        f"{workload_histogram['size_class_regular_bins']}",
        "workload_size_class_max_bin="
        f"{workload_histogram['size_class_max_bin']}",
        "workload_size_class_max_good_size="
        f"{workload_histogram['size_class_max_good_size']}",
        "workload_size_class_huge_count="
        f"{workload_histogram['size_class_huge_count']}",
        "workload_size_class_regular_request_count="
        f"{workload_histogram['size_class_regular_request_count']}",
        "workload_request_le_64="
        f"{workload_histogram['request_le_64']}",
        "workload_request_le_128="
        f"{workload_histogram['request_le_128']}",
        "workload_request_le_256="
        f"{workload_histogram['request_le_256']}",
        "workload_request_le_512="
        f"{workload_histogram['request_le_512']}",
        "workload_request_le_1024="
        f"{workload_histogram['request_le_1024']}",
        "workload_request_gt_1024="
        f"{workload_histogram['request_gt_1024']}",
    ]
    if replacement_front_smokes:
        cross_thread_free = replacement_front_smokes["cross_thread_free"]
        abandoned_owner = replacement_front_smokes["abandoned_owner"]
        cross_thread_realloc = replacement_front_smokes["cross_thread_realloc"]
        lines.extend(
            [
                "replacement_front_cross_thread_free_smoke_ok=1",
                "replacement_front_abandoned_owner_smoke_ok=1",
                "replacement_front_cross_thread_realloc_smoke_ok=1",
                "replacement_front_cross_thread_free_policy=remote_queue",
                "replacement_front_abandoned_owner_policy=mark_abandoned_no_host_free",
                "replacement_front_cross_thread_realloc_policy=unsupported_counted",
                "replacement_front_cross_thread_free_remote_free_push_count="
                f"{counter_value(cross_thread_free, 'replacement_front_remote_free_push_count')}",
                "replacement_front_cross_thread_free_remote_free_drain_count="
                f"{counter_value(cross_thread_free, 'replacement_front_remote_free_drain_count')}",
                "replacement_front_cross_thread_free_arena_registry_overflow_count="
                f"{counter_value(cross_thread_free, 'replacement_front_arena_registry_overflow_count')}",
                "replacement_front_abandoned_owner_abandoned_arena_count="
                f"{counter_value(abandoned_owner, 'replacement_front_abandoned_arena_count')}",
                "replacement_front_abandoned_owner_abandoned_remote_free_count="
                f"{counter_value(abandoned_owner, 'replacement_front_abandoned_remote_free_count')}",
                "replacement_front_abandoned_owner_host_passthrough_count="
                f"{counter_value(abandoned_owner, 'replacement_front_host_passthrough_count')}",
                "replacement_front_cross_thread_realloc_unsupported_count="
                f"{counter_value(cross_thread_realloc, 'replacement_front_cross_thread_realloc_unsupported_count')}",
                "replacement_front_cross_thread_realloc_host_passthrough_count="
                f"{counter_value(cross_thread_realloc, 'replacement_front_host_passthrough_count')}",
            ]
        )
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
    for index, (subject, _ld_preload, _provider, replacement_front_mode) in enumerate(subject_specs):
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
            single_thread_smoke = args.threads == 1
            thread_local_smoke = args.threads > 1 and args.replacement_front_thread_local_mode
            multithread_smoke = args.threads > 1 and (
                args.replacement_front_lock_mode or args.replacement_front_thread_local_mode
            )
            tls_initial_exec_enabled = (
                counter_value(counters, "replacement_front_tls_initial_exec_model_enabled") > 0
            )
            tls_get_addr_hot_path = (
                args.replacement_front_thread_local_mode and not tls_initial_exec_enabled
            )
            hot_atomic_rmw = not (
                replacement_front_bins_mode
                or args.replacement_front_skip_hot_counters
                or args.replacement_front_tls_counter_mode
            )
            lines.extend(
                [
                    f"subject_{index}_provider_table_dispatch=0",
                    f"subject_{index}_function_pointer_hot_call=0",
                    f"subject_{index}_owns_check_hot_path=0",
                    f"subject_{index}_tracking_hot_path=0",
                    f"subject_{index}_direct_core_call=1",
                    f"subject_{index}_single_thread_replacement_front_smoke={1 if single_thread_smoke else 0}",
                    f"subject_{index}_multithread_replacement_front_smoke={1 if multithread_smoke else 0}",
                    f"subject_{index}_thread_local_replacement_front_smoke={1 if thread_local_smoke else 0}",
                    f"subject_{index}_thread_safety_claim={'measured' if (multithread_smoke or thread_local_smoke) else 'none'}",
                    f"subject_{index}_thread_local_arena={1 if args.replacement_front_thread_local_mode else 0}",
                    "subject_"
                    f"{index}_cross_thread_free_policy="
                    f"{'remote_queue' if args.replacement_front_thread_local_mode else 'global_lock_or_not_applicable'}",
                    f"subject_{index}_provider_api_hot_path_required=0",
                    f"subject_{index}_activation=0",
                    f"subject_{index}_benchmark_only=1",
                    f"subject_{index}_replacement_front_is_full_hako_algorithm=0",
                    f"subject_{index}_replacement_front_algorithm_shape={replacement_front_algorithm_shape}",
                    f"subject_{index}_replacement_front_evidence_owner="
                    f"{replacement_front_evidence_owner}",
                    "subject_"
                    f"{index}_replacement_front_multithread_perf_candidate="
                    f"{replacement_front_multithread_perf_candidate}",
                    "subject_"
                    f"{index}_replacement_front_thread_local_perf_candidate="
                    f"{replacement_front_thread_local_perf_candidate}",
                    "subject_"
                    f"{index}_replacement_front_correctness_smoke="
                    f"{replacement_front_correctness_smoke}",
                    f"subject_{index}_replacement_front_native_bins_mode={1 if args.replacement_front_native_bins_mode else 0}",
                    f"subject_{index}_replacement_front_page_bins_mode={1 if args.replacement_front_page_bins_mode else 0}",
                    "subject_"
                    f"{index}_replacement_front_hotcore_page_model_mode="
                    f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
                    f"subject_{index}_replacement_front_size_class_bridge_plan_v0=1",
                    f"subject_{index}_replacement_front_size_class_bridge_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_size_class_policy_bridge="
                    f"{replacement_front_size_class_bridge_enabled}",
                    "subject_"
                    f"{index}_replacement_front_size_class_count="
                    f"{workload_histogram['size_class_regular_distinct_count'] if replacement_front_bins_mode else 1}",
                    "subject_"
                    f"{index}_replacement_front_size_class_policy_source="
                    f"{replacement_front_size_class_policy_source}",
                    "subject_"
                    f"{index}_replacement_front_size_class_bridge_mode="
                    f"{replacement_front_size_class_bridge_mode}",
                    "subject_"
                    f"{index}_replacement_front_size_class_request_ceiling="
                    f"{replacement_front_size_class_request_ceiling}",
                    "subject_"
                    f"{index}_replacement_front_size_class_selected_bin="
                    f"{replacement_front_size_class_selected_bin}",
                    "subject_"
                    f"{index}_replacement_front_size_class_selected_good_size="
                    f"{replacement_front_size_class_selected_good_size}",
                    f"subject_{index}_replacement_front_product_bins_plan_v0=1",
                    f"subject_{index}_replacement_front_product_bins_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_product_bins_consumer_enabled="
                    f"{1 if replacement_front_bins_mode else 0}",
                    f"subject_{index}_replacement_front_product_bins_connected=0",
                    "subject_"
                    f"{index}_replacement_front_product_bins_route="
                    f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'benchmark_page_bins' if args.replacement_front_page_bins_mode else 'benchmark_native_bins' if args.replacement_front_native_bins_mode else 'not_consumed'}",
                    f"subject_{index}_replacement_front_product_pages_plan_v0=1",
                    f"subject_{index}_replacement_front_product_pages_report_only=1",
                    f"subject_{index}_replacement_front_product_pages_consumer_enabled=0",
                    f"subject_{index}_replacement_front_product_pages_connected=0",
                    f"subject_{index}_replacement_front_page_bins_plan_v0=1",
                    f"subject_{index}_replacement_front_page_bins_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_page_bins_consumer_enabled="
                    f"{1 if args.replacement_front_page_bins_mode else 0}",
                    "subject_"
                    f"{index}_replacement_front_page_bins_route="
                    f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'benchmark_page_bins' if args.replacement_front_page_bins_mode else 'not_consumed'}",
                    "subject_"
                    f"{index}_replacement_front_page_bins_lookup_route="
                    f"{'range_scan' if args.replacement_front_page_bins_mode else 'not_consumed'}",
                    f"subject_{index}_replacement_front_page_bins_owner=benchmark_only",
                    f"subject_{index}_replacement_front_page_bins_product_claim=0",
                    "subject_"
                    f"{index}_replacement_front_product_bins_required_regular_distinct_count="
                    f"{workload_histogram['size_class_regular_distinct_count']}",
                    "subject_"
                    f"{index}_replacement_front_product_bins_required_regular_bins="
                    f"{workload_histogram['size_class_regular_bins']}",
                    "subject_"
                    f"{index}_replacement_front_product_bins_required_max_bin="
                    f"{workload_histogram['size_class_max_bin']}",
                    "subject_"
                    f"{index}_replacement_front_product_bins_huge_route_required="
                    f"{1 if int(workload_histogram['size_class_huge_count']) > 0 else 0}",
                    f"subject_{index}_replacement_front_hotcore_bridge_plan_v0=1",
                    f"subject_{index}_replacement_front_hotcore_bridge_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_hotcore_consumer_enabled="
                    f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
                    "subject_"
                    f"{index}_replacement_front_hotcore_route="
                    f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'not_consumed_by_replacement_front'}",
                    f"subject_{index}_hako_mimalloc_algorithm_claim=0",
                    f"subject_{index}_replacement_front_hotpath_plan_v0=1",
                    f"subject_{index}_replacement_front_hotpath_report_only=1",
                    f"subject_{index}_tls_get_addr_hot_path={1 if tls_get_addr_hot_path else 0}",
                    f"subject_{index}_hot_atomic_rmw={1 if hot_atomic_rmw else 0}",
                    "subject_"
                    f"{index}_remote_free_drain_hot_path="
                    "0",
                    "subject_"
                    f"{index}_remote_owner_publication_after_local_fail="
                    f"{1 if args.replacement_front_thread_local_mode else 0}",
                    f"subject_{index}_cold_init_in_hot_path=0",
                    "subject_"
                    f"{index}_register_thread_arena_hot_path="
                    "0",
                    f"subject_{index}_fast_cold_split_plan=1",
                    f"subject_{index}_tls_arena_fast_alloc_plan=1",
                    f"subject_{index}_tls_arena_local_free_plan=1",
                    f"subject_{index}_free_local_first=1",
                    f"subject_{index}_free_remote_path_after_local_fail="
                    f"{1 if args.replacement_front_thread_local_mode else 0}",
                    f"subject_{index}_free_hot_remote_queue_call=0",
                    f"subject_{index}_replacement_entry_inline_plan=1",
                    f"subject_{index}_malloc_to_direct_alloc_boundary=always_inline",
                    f"subject_{index}_free_to_direct_free_boundary=always_inline",
                    f"subject_{index}_replacement_front_inplace_realloc_within_slot_plan=1",
                    f"subject_{index}_replacement_front_slot_size="
                    f"{replacement_slot_size}",
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

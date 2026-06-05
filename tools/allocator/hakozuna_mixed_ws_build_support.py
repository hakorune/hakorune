"""Build helpers for Hakozuna mixed-ws compare probes."""

from __future__ import annotations

import subprocess
import os
import re
from pathlib import Path

from hako_mimalloc_provider_backed_hakmem_ldpreload_bench_pilot import read_kv
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
    locked: bool = False,
    page_shaped: bool = False,
    hotcore_page_model: bool = False,
    thread_local_page_arena: bool = False,
    size_class_table: bool = False,
    eager_init: bool = False,
    product_pages_nonlinear_lookup: bool = False,
    skip_hot_counters: bool = False,
) -> Path:
    front_name = "replacement-front-page-bins" if page_shaped else "replacement-front-native-bins"
    if hotcore_page_model:
        front_name = f"{front_name}-hotcore-page-model"
    if thread_local_page_arena:
        front_name = f"{front_name}-tls-page-arena"
    if locked:
        front_name = f"{front_name}-locked"
    if size_class_table:
        front_name = f"{front_name}-size-table"
    if eager_init:
        front_name = f"{front_name}-eager-init"
    if product_pages_nonlinear_lookup:
        front_name = f"{front_name}-product-pages-nonlinear"
    if skip_hot_counters:
        front_name = f"{front_name}-skip-hot-counters"
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
            locked=locked,
            page_shaped=page_shaped,
            hotcore_page_model=hotcore_page_model,
            thread_local_page_arena=thread_local_page_arena,
            size_class_table=size_class_table,
            eager_init=eager_init,
            product_pages_nonlinear_lookup=product_pages_nonlinear_lookup,
            skip_hot_counters=skip_hot_counters,
        ).lstrip(),
        encoding="utf-8",
    )
    cmd = ["cc", "-shared", "-fPIC", "-O3", "-Wall", "-Wextra"]
    if locked:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_LOCKED=1")
    if thread_local_page_arena:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_TLS_PAGE_ARENA=1")
    if skip_hot_counters:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS=1")
    cmd.extend([str(source), "-ldl"])
    if locked:
        cmd.append("-pthread")
    cmd.extend(["-o", str(binary)])
    subprocess.run(cmd, check=True)
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
    match = re.search(r"ops/s=([0-9]+(?:\.[0-9]+)?)", completed.stdout)
    if match is None:
        raise SystemExit(f"{subject} {kind} run {run_index} output missing ops/s line")
    time_match = re.search(r"time=([0-9]+(?:\.[0-9]+)?)", completed.stdout)
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

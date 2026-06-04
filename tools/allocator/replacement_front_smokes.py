#!/usr/bin/env python3
"""Focused non-activating smokes for the benchmark replacement front.

This module compiles and runs small C smoke programs under the benchmark
replacement-front LD_PRELOAD shim. It does not emit product activation report
fields and does not change allocator defaults.
"""

from __future__ import annotations

import os
import subprocess
from pathlib import Path

from hako_mimalloc_provider_backed_hakmem_ldpreload_bench_pilot import read_kv
from replacement_front_templates import (
    REPLACEMENT_FRONT_ABANDONED_OWNER_SMOKE_C,
    REPLACEMENT_FRONT_CROSS_THREAD_FREE_SMOKE_C,
    REPLACEMENT_FRONT_CROSS_THREAD_REALLOC_SMOKE_C,
    REPLACEMENT_FRONT_MALLOC_FAMILY_SMOKE_C,
    counter_value,
)


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
    malloc_family = run_replacement_front_focused_smoke(
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        name="malloc_family",
        source_text=REPLACEMENT_FRONT_MALLOC_FAMILY_SMOKE_C,
    )
    if counter_value(malloc_family, "replacement_front_alloc_count") < 1:
        raise SystemExit("malloc_family smoke did not count malloc")
    if counter_value(malloc_family, "replacement_front_calloc_count") < 1:
        raise SystemExit("malloc_family smoke did not count calloc")
    if counter_value(malloc_family, "replacement_front_realloc_count") < 1:
        raise SystemExit("malloc_family smoke did not count realloc")
    if counter_value(malloc_family, "replacement_front_free_count") < 2:
        raise SystemExit("malloc_family smoke did not count frees")
    if counter_value(malloc_family, "replacement_front_realloc_inplace_count") < 1:
        raise SystemExit("malloc_family smoke did not count in-place realloc")
    if counter_value(malloc_family, "replacement_front_calloc_zero_bytes") < 64:
        raise SystemExit("malloc_family smoke did not count calloc zero bytes")
    if counter_value(malloc_family, "replacement_front_host_passthrough_count") != 0:
        raise SystemExit("malloc_family smoke used host passthrough")

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
        "malloc_family": malloc_family,
        "cross_thread_free": cross_thread_free,
        "abandoned_owner": abandoned_owner,
        "cross_thread_realloc": cross_thread_realloc,
    }

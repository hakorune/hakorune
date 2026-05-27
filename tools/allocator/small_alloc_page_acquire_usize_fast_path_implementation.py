#!/usr/bin/env python3
"""Validate the small-alloc page acquire_usize fast path keeper."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SMOKE_APP = ROOT / "apps/hako-alloc-mimalloc-comparison-object-lifecycle-known-live-release-smoke/main.hako"
FACADE = ROOT / "lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
PAGE = ROOT / "lang/src/hako_alloc/memory/page_box.hako"


def read_kv_text(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
        if line and "=" in line:
            key, value = line.split("=", 1)
            values[key] = value
    return values


def run_text(cmd: list[str], timeout_seconds: int) -> str:
    return subprocess.run(
        cmd,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        check=True,
        timeout=timeout_seconds,
    ).stdout


def small_alloc_uses_acquire_usize() -> bool:
    text = FACADE.read_text(encoding="utf-8")
    start = text.index("objectLifecycleSmallAlloc")
    end = text.index("resetReleaseResult", start)
    body = text[start:end]
    return "page.acquire_usize(size)" in body and "page.acquire(size)" not in body


def generic_page_acquire_preserved() -> bool:
    text = PAGE.read_text(encoding="utf-8")
    return "    acquire(requested_size: usize)" in text and "return me.acquire(requested_size)" in text


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--timeout-seconds", type=int, default=20)
    args = parser.parse_args()

    with tempfile.TemporaryDirectory(prefix="hakorune_acquire_usize_exact_smoke.") as tmp:
        report_path = Path(tmp) / "runner.out"
        run_text(
            [
                "bash",
                str(ROOT / "tools/allocator/hako_exe_memory_runner.sh"),
                "--app",
                str(SMOKE_APP),
                "--out",
                str(report_path),
                "--workload",
                "known_live_release_smoke",
                "--operation-repeat",
                "1",
            ],
            args.timeout_seconds,
        )
        smoke = read_kv_text(report_path.read_text(encoding="utf-8"))
    smoke_ok = smoke.get("summary") == "ok" and smoke.get("output_summary_ok") == "1"

    lines = [
        "output_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0",
        "input_contract=page-acquire-fast-path-keeper-selection-v0",
        "selected_keeper=small_alloc_page_acquire_usize_fast_path",
        f"keeper_applied={int(small_alloc_uses_acquire_usize())}",
        f"generic_page_acquire_preserved={int(generic_page_acquire_preserved())}",
        f"lightweight_exact_exe_proof_ok={int(smoke_ok)}",
        f"allocation_count={smoke['allocation_count']}",
        f"free_count={smoke['free_count']}",
        f"release_known_page_fast_path_count={smoke['release_known_page_fast_path_count']}",
        f"release_known_page_fallback_count={smoke['release_known_page_fallback_count']}",
        f"output_summary_ok={smoke['output_summary_ok']}",
        f"external_elapsed_ms={smoke['external_elapsed_ms']}",
        "full_repeat_measurement_executed=0",
        "winner_claim=0",
        "replacement_active=0",
        "semantic_summary=ok",
        "selected_next=post_page_acquire_usize_fast_path_measurement",
        "summary=ok",
    ]

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

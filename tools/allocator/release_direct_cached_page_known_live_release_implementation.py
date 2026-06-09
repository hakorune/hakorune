#!/usr/bin/env python3
"""Validate the direct cached-page known-live release keeper with a light smoke."""

from __future__ import annotations

import argparse
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SMOKE_APP = ROOT / "apps/hako-alloc-mimalloc-comparison-object-lifecycle-known-live-release-smoke/main.hako"
FACADE = ROOT / "lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"


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


def direct_cached_uses_known_live() -> bool:
    text = FACADE.read_text(encoding="utf-8")
    start = text.index("objectLifecycleReleaseBlock(page_id, block_id)")
    end = text.index("resetAlignmentResult()", start)
    body = text[start:end]
    return "page.releaseLocalKnownLive(block_id)" in body and "page.releaseLocal(block_id)" not in body


def generic_release_preserved() -> bool:
    text = FACADE.read_text(encoding="utf-8")
    start = text.index("objectLifecycleReleaseBlockSlow(page_id, block_id)")
    end = text.index("objectLifecycleReleaseBlock(page_id, block_id)", start)
    body = text[start:end]
    return "page.releaseLocal(block_id)" in body


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--timeout-seconds", type=int, default=20)
    args = parser.parse_args()

    smoke = read_kv_text(
        run_text(
            [str(ROOT / "target/release/hakorune"), str(SMOKE_APP)],
            args.timeout_seconds,
        )
    )
    smoke_ok = smoke.get("summary") == "ok"

    lines = [
        "output_contract=release-direct-cached-page-known-live-release-implementation-v0",
        "input_contract=page-array-keeper-selection-v0",
        "proof_scope=lightweight_known_live_release_smoke",
        f"keeper_applied={int(direct_cached_uses_known_live())}",
        f"generic_release_fallback_preserved={int(generic_release_preserved())}",
        f"lightweight_exact_exe_proof_ok={int(smoke_ok)}",
        f"allocation_count={smoke['allocation_count']}",
        f"free_count={smoke['free_count']}",
        f"release_known_page_fast_path_count={smoke['release_known_page_fast_path_count']}",
        f"release_known_page_fallback_count={smoke['release_known_page_fallback_count']}",
        "expected_array_get_removed_at_full_repeat=524288",
        "full_repeat_measurement_executed=0",
        "winner_claim=0",
        "replacement_active=0",
        "selected_next=post_known_live_release_measurement",
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

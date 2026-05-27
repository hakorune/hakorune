#!/usr/bin/env python3
"""Select the next page-acquire fast path keeper."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
MIR_SHAPE = ROOT / "tools/mir_check/method_shape_report.py"
ROW150_TOOL = ROOT / "tools/allocator/post_known_live_release_source_mir_refresh.py"


def read_kv_text(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
        if line and "=" in line:
            key, value = line.split("=", 1)
            values[key] = value
    return values


def run_text(cmd: list[str]) -> str:
    return subprocess.run(
        cmd,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        check=True,
    ).stdout


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    row150 = read_kv_text(run_text([str(ROW150_TOOL)]))

    with tempfile.TemporaryDirectory(prefix="hakorune_page_acquire_keeper_selection.") as tmp:
        mir_json = Path(tmp) / "app.mir.json"
        subprocess.run(
            [
                str(ROOT / "target/release/hakorune"),
                "--backend",
                "mir",
                "--emit-mir-json",
                str(mir_json),
                str(APP),
            ],
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            check=True,
        )
        acquire_usize = read_kv_text(
            run_text(
                [
                    "python3",
                    str(MIR_SHAPE),
                    "--mir-json",
                    str(mir_json),
                    "--method",
                    "HakoAllocPageModel.acquire_usize/1",
                ]
            )
        )
        acquire_fresh = read_kv_text(
            run_text(
                [
                    "python3",
                    str(MIR_SHAPE),
                    "--mir-json",
                    str(mir_json),
                    "--method",
                    "HakoAllocPageModel.acquireFreshSmall/1",
                ]
            )
        )

    lines = [
        "output_contract=page-acquire-fast-path-keeper-selection-v0",
        "input_contract=post-known-live-release-source-mir-refresh-v0",
        "active_owner=allocator_page_array_surface",
        f"baseline_page_acquire_mir_instruction_count={row150['page_acquire_mir_instruction_count']}",
        f"baseline_page_acquire_call_count={row150['page_acquire_call_count']}",
        f"baseline_page_acquire_copy_count={row150['page_acquire_copy_count']}",
        f"baseline_page_acquire_array_get_call_count={row150['page_acquire_array_get_call_count']}",
        f"baseline_page_acquire_array_set_call_count={row150['page_acquire_array_set_call_count']}",
        "candidate_0=small_alloc_page_acquire_usize_fast_path",
        "candidate_0_method=HakoAllocPageModel.acquire_usize/1",
        f"candidate_0_mir_instruction_count={acquire_usize['mir_instruction_count']}",
        f"candidate_0_call_count={acquire_usize['call_count']}",
        f"candidate_0_copy_count={acquire_usize['copy_count']}",
        f"candidate_0_array_get_call_count={acquire_usize['array_get_call_count']}",
        "candidate_0_semantics=preserves_retired_decommitted_size_checks_and_generic_acquire_fallback",
        "candidate_1=small_alloc_page_acquire_fresh_small_fast_path",
        "candidate_1_method=HakoAllocPageModel.acquireFreshSmall/1",
        f"candidate_1_mir_instruction_count={acquire_fresh['mir_instruction_count']}",
        f"candidate_1_call_count={acquire_fresh['call_count']}",
        f"candidate_1_copy_count={acquire_fresh['copy_count']}",
        f"candidate_1_array_get_call_count={acquire_fresh['array_get_call_count']}",
        "candidate_1_semantics=drops_retired_decommitted_checks_and_local_free_collect_fallback",
        "selected_keeper=small_alloc_page_acquire_usize_fast_path",
        "keeper_owner=object_lifecycle_facade_small_alloc_page_acquire_callsite",
        "keeper_kind=box_count",
        "fallback_preservation=generic_page_acquire_preserved_when_free_top_is_zero",
        "rejected_keeper=small_alloc_page_acquire_fresh_small_fast_path",
        "rejected_reason=too_narrow_for_first_keeper_semantics",
        "selected_next=small_alloc_page_acquire_usize_fast_path_implementation",
        "winner_claim=0",
        "replacement_active=0",
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

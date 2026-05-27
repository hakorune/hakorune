#!/usr/bin/env python3
"""Refresh source/MIR observation after static-scalar call lowering."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
SOURCE = ROOT / "lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
HAKO_CHECK = ROOT / "tools/hako_check/perf_surface_inventory.py"
MIR_SHAPE = ROOT / "tools/mir_check/method_shape_report.py"

METHODS = (
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    "HakoAllocPageModel.acquire/1",
    "HakoAllocPageModel.releaseLocal/1",
    "HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0",
)


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


def load_json(path: Path) -> dict[str, Any]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be object")
    return data


def find_function(data: dict[str, Any], name: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    for fn in functions:
        if isinstance(fn, dict) and fn.get("name") == name:
            return fn
    raise SystemExit(f"missing function: {name}")


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("name", ""))


def call_counter(function: dict[str, Any]) -> Counter[str]:
    calls: Counter[str] = Counter()
    for block in function.get("blocks", []):
        if not isinstance(block, dict):
            continue
        for inst in block.get("instructions", []):
            if isinstance(inst, dict) and inst.get("op") == "mir_call":
                calls[callee_name(inst)] += 1
    return calls


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    with tempfile.TemporaryDirectory(prefix="hakorune_post_static_scalar_refresh.") as tmp:
        tmp_dir = Path(tmp)
        mir_json = tmp_dir / "app.mir.json"
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
        data = load_json(mir_json)

        shape_reports = [
            read_kv_text(
                run_text(["python3", str(MIR_SHAPE), "--mir-json", str(mir_json), "--method", method])
            )
            for method in METHODS
        ]

        source_report = read_kv_text(
            run_text(
                [
                    "python3",
                    str(HAKO_CHECK),
                    "--contract-version",
                    "v1",
                    "--methods",
                    "objectLifecycleSmallAlloc,objectLifecycleReleaseBlock,objectLifecycleReleaseDirectCachedPage,objectLifecycleReleaseKnownPageIndex,objectLifecycleKnownPageIndexById",
                ]
            )
        )

    small_shape = shape_reports[0]
    release_shape = shape_reports[1]
    acquire_shape = shape_reports[2]
    release_local_shape = shape_reports[3]
    select_single_shape = shape_reports[4]
    small_calls = call_counter(find_function(data, METHODS[0]))
    release_calls = call_counter(find_function(data, METHODS[1]))
    acquire_calls = call_counter(find_function(data, METHODS[2]))

    lines = [
        "output_contract=post-static-scalar-source-mir-refresh-v0",
        "input_contract=post-static-scalar-call-lowering-measurement-v0",
        "selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "remaining_call_surface=facade_result_helpers_and_page_hotpath",
        f"small_alloc_mir_instruction_count={small_shape['mir_instruction_count']}",
        f"small_alloc_call_count={small_shape['call_count']}",
        f"small_alloc_copy_count={small_shape['copy_count']}",
        f"small_alloc_phi_count={small_shape['phi_count']}",
        f"small_alloc_field_get_count={small_shape['field_get_count']}",
        f"small_alloc_field_set_count={small_shape['field_set_count']}",
        f"small_alloc_record_failure_call_count={small_calls['recordSmallAllocFailure']}",
        f"small_alloc_record_success_call_count={small_calls['recordSmallAllocSuccess']}",
        f"small_alloc_page_acquire_call_count={small_calls['acquire']}",
        f"release_block_mir_instruction_count={release_shape['mir_instruction_count']}",
        f"release_block_call_count={release_shape['call_count']}",
        f"release_block_copy_count={release_shape['copy_count']}",
        f"release_block_phi_count={release_shape['phi_count']}",
        f"release_block_record_failure_call_count={release_calls['recordReleaseFailure']}",
        f"release_block_release_local_call_count={release_calls['releaseLocal']}",
        f"page_acquire_mir_instruction_count={acquire_shape['mir_instruction_count']}",
        f"page_acquire_call_count={acquire_shape['call_count']}",
        f"page_acquire_copy_count={acquire_shape['copy_count']}",
        f"page_acquire_phi_count={acquire_shape['phi_count']}",
        f"page_acquire_field_get_count={acquire_shape['field_get_count']}",
        f"page_acquire_field_set_count={acquire_shape['field_set_count']}",
        f"page_acquire_array_get_call_count={acquire_calls['get']}",
        f"page_acquire_array_set_call_count={acquire_calls['set']}",
        f"page_release_local_mir_instruction_count={release_local_shape['mir_instruction_count']}",
        f"page_release_local_call_count={release_local_shape['call_count']}",
        f"select_single_fast_path_mir_instruction_count={select_single_shape['mir_instruction_count']}",
        f"select_single_fast_path_call_count={select_single_shape['call_count']}",
        f"source_small_alloc_method_call_count={source_report['target_method_0_method_call_count']}",
        f"source_small_alloc_result_capsule_churn={source_report['target_method_0_result_capsule_churn']}",
        f"source_release_block_method_call_count={source_report['target_method_1_method_call_count']}",
        f"source_release_block_result_capsule_churn={source_report['target_method_1_result_capsule_churn']}",
        "gap_owner=compiler_lowering",
        "gap_confidence=medium",
        "next_diagnostic=small_alloc_call_copy_shape_deep_dive",
        "selected_next=small_alloc_call_copy_shape_deep_dive",
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

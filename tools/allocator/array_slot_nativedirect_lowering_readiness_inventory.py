#!/usr/bin/env python3
"""Inventory ArraySlot NativeDirect lowering readiness for one selected method."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
METHOD = "HakoAllocPageModel.acquire_usize/1"


def emit_mir_json(path: Path) -> None:
    subprocess.run(
        [
            str(ROOT / "target/release/hakorune"),
            "--backend",
            "mir",
            "--emit-mir-json",
            str(path),
            str(APP),
        ],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        check=True,
    )


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be object")
    return data


def find_function(data: dict[str, Any], method: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    matches = [fn for fn in functions if isinstance(fn, dict) and fn.get("name") == method]
    if len(matches) != 1:
        raise SystemExit(f"selected method not found exactly once: {method}")
    return matches[0]


def block_instructions(function: dict[str, Any]) -> list[tuple[int, list[dict[str, Any]]]]:
    out: list[tuple[int, list[dict[str, Any]]]] = []
    for block in function.get("blocks") or []:
        if not isinstance(block, dict):
            continue
        block_id = block.get("id")
        insts = block.get("instructions")
        if isinstance(block_id, int) and isinstance(insts, list):
            out.append((block_id, [inst for inst in insts if isinstance(inst, dict)]))
    return out


def callee(inst: dict[str, Any]) -> tuple[str, str, str]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return "", "", ""
    callee_obj = mir_call.get("callee")
    if not isinstance(callee_obj, dict):
        return "", "", ""
    return (
        str(callee_obj.get("box_name", "")),
        str(callee_obj.get("name", "")),
        str(callee_obj.get("certainty", "")),
    )


def direct_array_scaffolding_available() -> tuple[int, int, int]:
    direct_array = ROOT / "crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
    backend = ROOT / "crates/nyash_kernel/src/plugin/array_slot_backend.rs"
    direct_text = direct_array.read_text(encoding="utf-8")
    backend_text = backend.read_text(encoding="utf-8")
    buffer_ready = int(
        "struct DirectArrayI64BufferV0" in direct_text
        and "materialize_public_arraybox_snapshot_handle" in direct_text
    )
    route_closed = int(
        "DirectArrayI64Exact" in backend_text
        and "direct_array_i64_helper_route_closed" in backend_text
    )
    contiguous_i64 = int(
        "direct_array_i64_buffer_data_offset" in direct_text
        and "cast::<i64>()" in direct_text
    )
    return buffer_ready, route_closed, contiguous_i64


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path)
    parser.add_argument("--method", default=METHOD)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    subprocess.run(["cargo", "build", "--release", "--bin", "hakorune"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory(prefix="hakorune_array_nativedirect_readiness.") as tmp:
        mir_json = args.mir_json or Path(tmp) / "app.mir.json"
        if args.mir_json is None:
            emit_mir_json(mir_json)
        function = find_function(load_json(mir_json), args.method)

    array_get_count = 0
    array_set_count = 0
    phi_count = 0
    unknown_call_count = 0
    selected_block = 0
    same_block_get_set_pair = 0
    set_uses_get_result = 0

    for block_id, insts in block_instructions(function):
        block_get_count = 0
        block_set_count = 0
        get_result_carriers: set[int] = set()
        block_set_uses_get_result = 0
        for inst in insts:
            op = inst.get("op")
            if op == "phi":
                phi_count += 1
                continue
            if op == "copy" and inst.get("src") in get_result_carriers:
                dst = inst.get("dst")
                if isinstance(dst, int):
                    get_result_carriers.add(dst)
                continue
            if op != "mir_call":
                continue
            box_name, method_name, certainty = callee(inst)
            if box_name == "ArrayBox" and method_name == "get":
                block_get_count += 1
                array_get_count += 1
                dst = inst.get("dst")
                if isinstance(dst, int):
                    get_result_carriers.add(dst)
                continue
            if box_name == "ArrayBox" and method_name == "set":
                block_set_count += 1
                array_set_count += 1
                args_list = inst.get("mir_call", {}).get("args", [])
                if isinstance(args_list, list) and any(arg in get_result_carriers for arg in args_list):
                    block_set_uses_get_result += 1
                    set_uses_get_result += 1
                continue
            effects = inst.get("mir_call", {}).get("effects", [])
            if certainty != "Known" or effects:
                unknown_call_count += 1

        if block_get_count == 1 and block_set_count == 1 and block_set_uses_get_result == 1:
            same_block_get_set_pair = 1
            selected_block = block_id

    helper_count = array_get_count + array_set_count
    buffer_ready, route_closed, contiguous_i64 = direct_array_scaffolding_available()
    scaffolding_ready = int(buffer_ready == 1 and route_closed == 1 and contiguous_i64 == 1)
    planned_erased = helper_count
    planned_added = 0 if scaffolding_ready else helper_count
    planned_net = planned_erased - planned_added
    facts_available = int(
        same_block_get_set_pair == 1
        and set_uses_get_result == 1
        and scaffolding_ready == 1
    )

    lines = [
        "output_contract=array-slot-nativedirect-lowering-readiness-inventory-v0",
        "input_contract=direct-array-i64-helper-fallback-closeout-and-lowering-readiness-selection-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"selected_method={args.method}",
        "candidate_representation=NativeDirect",
        "storage_substrate=DirectArrayI64BufferV0",
        "direct_array_layout=repr_c_header_trailing_i64",
        "fallback_boundary=explicit_public_arraybox_snapshot_handle",
        f"selected_block={selected_block}",
        f"candidate_array_get_count={array_get_count}",
        f"candidate_array_set_count={array_set_count}",
        f"candidate_array_helper_count={helper_count}",
        f"same_block_get_set_pair={same_block_get_set_pair}",
        f"set_uses_get_result={set_uses_get_result}",
        "prior_array_residence_erased_get_set_helper_calls=2",
        "prior_array_residence_added_guard_helper_calls=1",
        "prior_array_residence_net_helper_call_delta=1",
        f"planned_erased_helper_ops={planned_erased}",
        f"planned_added_helper_ops={planned_added}",
        f"planned_net_helper_delta={planned_net}",
        f"planned_net_helper_delta_positive={1 if planned_net > 0 else 0}",
        f"direct_array_buffer_available={buffer_ready}",
        f"contiguous_i64_data_available={contiguous_i64}",
        "materialized_view_boundary_available=1",
        f"helper_free_bridge_available={scaffolding_ready}",
        f"index_and_bounds_facts_available={facts_available}",
        "append_policy_known=1",
        f"barrier_unknown_call_count={unknown_call_count}",
        f"barrier_phi_count={phi_count}",
        "fallback_materialization_boundary_known=1",
        "silent_fallback_allowed=0",
        "selected_next=array_slot_nativedirect_lowering_guard_surface",
        "implementation_open=0",
        "optimization_open=0",
        "llvm_lowering_open=0",
        "native_direct_open=0",
        "direct_load_store_open=0",
        "provider_activation=0",
        "host_replacement=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

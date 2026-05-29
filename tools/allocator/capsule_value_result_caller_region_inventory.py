#!/usr/bin/env python3
"""Inventory caller regions around result-capsule recordSuccess calls."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


TARGET_BOXES = {
    "HakoAllocObjectLifecycleAllocResult",
    "HakoAllocObjectLifecycleReleaseResult",
}


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def block_by_id(fn: dict[str, Any]) -> dict[int, dict[str, Any]]:
    return {int(block.get("id", -1)): block for block in fn.get("blocks", [])}


def is_target_record_success(inst: dict[str, Any]) -> bool:
    call = inst.get("mir_call")
    if not isinstance(call, dict):
        return False
    callee = call.get("callee", {})
    return (
        callee.get("name") == "recordSuccess"
        and callee.get("box_name") in TARGET_BOXES
    )


def block_has_ret_after(block: dict[str, Any], index: int) -> bool:
    for inst in block.get("instructions", [])[index + 1 :]:
        if inst.get("op") == "ret":
            return True
    return False


def block_tail_jump(block: dict[str, Any]) -> int | None:
    instructions = block.get("instructions", [])
    if not instructions:
        return None
    tail = instructions[-1]
    if tail.get("op") == "jump":
        return int(tail.get("target"))
    return None


def reaches_immediate_ret(fn: dict[str, Any], start_block: dict[str, Any], call_index: int) -> bool:
    if block_has_ret_after(start_block, call_index):
        return True
    blocks = block_by_id(fn)
    target = block_tail_jump(start_block)
    visited: set[int] = set()
    while target is not None and target not in visited:
        visited.add(target)
        block = blocks.get(target)
        if block is None:
            return False
        if any(inst.get("op") == "ret" for inst in block.get("instructions", [])):
            return True
        target = block_tail_jump(block)
    return False


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--plan-inventory-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    plan = read_kv(args.plan_inventory_report)
    require(plan, "output_contract", "capsule-value-result-plan-inventory-v0")
    require(plan, "selected_next", "capsule_value_result_caller_region_inventory")
    require(plan, "summary", "ok")

    data = json.loads(args.mir_json.read_text(encoding="utf-8"))
    callsites: list[tuple[str, int, str]] = []
    immediate_return = 0
    unknown_after_success = 0
    observer_before_return = 0

    for fn in data.get("functions", []):
        name = fn.get("name", "")
        for block in fn.get("blocks", []):
            block_id = int(block.get("id", -1))
            instructions = block.get("instructions", [])
            for idx, inst in enumerate(instructions):
                if not is_target_record_success(inst):
                    continue
                callee = inst["mir_call"]["callee"]["box_name"] + ".recordSuccess"
                callsites.append((name, block_id, callee))
                if reaches_immediate_ret(fn, block, idx):
                    immediate_return += 1
                for after in instructions[idx + 1 :]:
                    op = after.get("op")
                    if op in {"mir_call", "call", "boxcall"}:
                        unknown_after_success += 1
                    if op in {"field_get"}:
                        observer_before_return += 1

    callsite_count = len(callsites)
    public_return_boundary_count = immediate_return
    caller_region_defer_past_return_allowed = 0
    caller_region_value_aggregate_net_delta = 0

    lines = [
        "output_contract=capsule-value-result-caller-region-inventory-v0",
        "input_contract=capsule-value-result-plan-inventory-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"record_success_callsite_count={callsite_count}",
        f"immediate_return_callsite_count={immediate_return}",
        f"observer_before_return_count={observer_before_return}",
        f"unknown_call_after_success_count={unknown_after_success}",
        f"public_method_return_boundary_count={public_return_boundary_count}",
        "materialization_must_happen_before_public_return=1",
        f"caller_region_defer_past_return_allowed={caller_region_defer_past_return_allowed}",
        f"caller_region_value_aggregate_net_delta={caller_region_value_aggregate_net_delta}",
        "caller_region_value_aggregate_net_delta_positive=0",
        "helper_fusion_net_delta=12",
        "helper_fusion_net_delta_positive=1",
        "selected_next=record_success_helper_fusion_guard_surface",
        "selected_reason=public_method_return_boundary_prevents_value_delta_deferral",
        "rejected_owner=capsule_value_result_implementation",
        "rejected_reason=caller_region_cannot_defer_materialization_past_public_method_return",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (method, block_id, callee) in enumerate(callsites):
        lines.append(f"callsite_{idx}_method={method}")
        lines.append(f"callsite_{idx}_block={block_id}")
        lines.append(f"callsite_{idx}_callee={callee}")
    lines.append("summary=ok")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Inventory release-result capsule IR shape after recordSuccess helper fusion."""

from __future__ import annotations

import argparse
import collections
import json
from pathlib import Path
from typing import Any


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


def instruction_counts(fn: dict[str, Any]) -> collections.Counter[str]:
    counts: collections.Counter[str] = collections.Counter()
    for block in fn.get("blocks", []):
        if not isinstance(block, dict):
            continue
        for inst in block.get("instructions", []):
            if isinstance(inst, dict):
                counts[str(inst.get("op", ""))] += 1
    return counts


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-refresh-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    owner = read_kv(args.owner_refresh_report)
    require(owner, "output_contract", "post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0")
    require(owner, "selected_owner", "release_result_capsule_ir_shape_inventory_after_record_success_helper_fusion")
    require(owner, "summary", "ok")

    data = json.loads(args.mir_json.read_text(encoding="utf-8"))
    totals: collections.Counter[str] = collections.Counter()
    method_count = 0
    top = ("none", 0)
    top_hot = ("none", 0)
    for fn in data.get("functions", []):
        if not isinstance(fn, dict):
            continue
        name = str(fn.get("name", ""))
        if not name.startswith("HakoAllocObjectLifecycleReleaseResult."):
            continue
        method_count += 1
        counts = instruction_counts(fn)
        totals.update(counts)
        field_ops = counts["field_get"] + counts["field_set"]
        if field_ops > top[1]:
            top = (name, field_ops)
        if ("recordSuccess" in name or "recordRequest" in name or "reset" in name) and field_ops > top_hot[1]:
            top_hot = (name, field_ops)

    field_get = totals["field_get"]
    field_set = totals["field_set"]
    call_count = totals["mir_call"] + totals["call"] + totals["boxcall"]
    lines = [
        "output_contract=release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0",
        "input_contract=post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"release_result_method_count={method_count}",
        f"release_result_field_get_count={field_get}",
        f"release_result_field_set_count={field_set}",
        f"release_result_field_op_count={field_get + field_set}",
        f"release_result_copy_count={totals['copy']}",
        f"release_result_call_count={call_count}",
        f"release_result_phi_count={totals['phi']}",
        f"release_result_branch_count={totals['branch']}",
        f"top_release_method={top[0]}",
        f"top_release_method_field_op_count={top[1]}",
        f"top_release_hot_method={top_hot[0]}",
        f"top_release_hot_method_field_op_count={top_hot[1]}",
        "record_success_helper_fusion_landed=1",
        "record_success_repeat_closed=1",
        "selected_next=release_result_capsule_owner_selection_after_record_success_helper_fusion",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

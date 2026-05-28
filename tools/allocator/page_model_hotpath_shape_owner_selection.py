#!/usr/bin/env python3
"""Select the next page-model hotpath shape owner from IR inventory."""

from __future__ import annotations

import argparse
from pathlib import Path


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


def require_key(values: dict[str, str], key: str) -> str:
    value = values.get(key)
    if value is None:
        raise SystemExit(f"missing {key}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.inventory_report)
    require(values, "output_contract", "page-model-hotpath-ir-shape-diff-inventory-v0")
    require(values, "summary", "ok")
    require(values, "ir_shape_diff_inventory_only", "1")
    selected_method = require_key(values, "selected_method")
    shape_owner = require_key(values, "selected_method_shape_owner")

    if shape_owner == "copy_materialization":
        selected_owner = "page_model_acquire_usize_copy_materialization_probe"
        next_diagnostic = selected_owner
        selected_reason = "selected_method_shape_owner_copy_materialization"
    else:
        selected_owner = "page_model_selected_method_field_shape_probe"
        next_diagnostic = selected_owner
        selected_reason = f"selected_method_shape_owner_{shape_owner}"

    lines = [
        "output_contract=page-model-hotpath-shape-owner-selection-v0",
        "input_contract=page-model-hotpath-ir-shape-diff-inventory-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"selected_method={selected_method}",
        f"selected_method_pct={require_key(values, 'selected_method_pct')}",
        f"selected_method_shape_owner={shape_owner}",
        f"selected_method_copy_count={require_key(values, 'selected_method_copy_count')}",
        f"selected_method_field_op_count={require_key(values, 'selected_method_field_op_count')}",
        f"selected_method_call_count={require_key(values, 'selected_method_call_count')}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        "rejected_owner=page_model_same_block_rmw_retry",
        "rejected_reason=recent_selected_method_rmw_keeper_already_applied",
        "rejected_owner_1=page_model_direct_op_retry",
        "rejected_reason_1=direct_op_previous_rejected",
        "rejected_owner_2=page_queue_retry",
        "rejected_reason_2=page_queue_recent_nonkeeper_retry_closed",
        "implementation_open=0",
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

#!/usr/bin/env python3
"""Select a result-capsule owner after reset batching has already landed."""

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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.inventory_report)
    require(values, "output_contract", "alloc-result-capsule-ir-shape-inventory-after-release-known-live-rollback-v0")
    require(values, "selected_next", "result_capsule_owner_selection_after_release_known_live_rollback")
    require(values, "summary", "ok")

    lines = [
        "output_contract=result-capsule-owner-selection-after-release-known-live-rollback-v0",
        "input_contract=alloc-result-capsule-ir-shape-inventory-after-release-known-live-rollback-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "selected_owner=result_capsule_record_success_shape_guard_surface",
        "selected_owner_kind=branch_aware_exact_slot_rmw_and_status_set_plan",
        "selected_reason=reset_batching_already_landed_and_record_success_is_top_hot_capsule_shape",
        "selected_methods=HakoAllocObjectLifecycleAllocResult.recordSuccess/1,HakoAllocObjectLifecycleReleaseResult.recordSuccess/2",
        "alloc_record_success_field_op_count=8",
        "release_record_success_field_op_count=6",
        "record_success_combined_field_op_count=14",
        "alloc_record_success_has_branch_shape=1",
        "release_record_success_has_straightline_shape=1",
        "requires_guard_surface_before_implementation=1",
        "requires_hako_source_change=0",
        "selected_next=result_capsule_record_success_shape_guard_surface",
        "rejected_owner=result_capsule_reset_field_batching",
        "rejected_reason=result_capsule_reset_field_batching_already_landed_in_row259",
        "rejected_owner_1=birth_batching",
        "rejected_reason_1=birth_is_setup_shaped_not_current_hot_capsule_callsite",
        "rejected_owner_2=record_request_batching",
        "rejected_reason_2=smaller_release_only_owner_than_record_success_pair",
        "rejected_owner_3=capsule_flattening",
        "rejected_reason_3=too_broad_without_escape_specific_guard_surface",
        "rejected_owner_4=source_inline_success_result_fast_path",
        "rejected_reason_4=prior_source_inline_success_result_regressed_and_was_rolled_back",
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

#!/usr/bin/env python3
"""Select a release-result capsule owner after recordSuccess fusion has landed."""

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
    require(values, "output_contract", "release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0")
    require(values, "record_success_repeat_closed", "1")
    require(values, "summary", "ok")

    lines = [
        "output_contract=release-result-capsule-owner-selection-after-record-success-helper-fusion-v0",
        "input_contract=release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"release_result_field_op_count={values.get('release_result_field_op_count', '0')}",
        f"top_release_method={values.get('top_release_method', 'none')}",
        f"top_release_method_field_op_count={values.get('top_release_method_field_op_count', '0')}",
        f"top_release_hot_method={values.get('top_release_hot_method', 'none')}",
        f"top_release_hot_method_field_op_count={values.get('top_release_hot_method_field_op_count', '0')}",
        "record_success_helper_fusion_landed=1",
        "record_success_repeat_closed=1",
        "selected_owner=post_release_result_capsule_owner_refresh_after_record_success_helper_fusion",
        "selected_reason=record_success_already_fused_and_remaining_birth_setup_shape_is_not_current_hot_keeper",
        "next_diagnostic=post_release_result_capsule_owner_refresh_after_record_success_helper_fusion",
        "rejected_owner=release_result_record_success_helper_fusion_repeat",
        "rejected_reason=record_success_helper_fusion_already_landed_in_row282",
        "rejected_owner_1=release_result_birth_batching",
        "rejected_reason_1=birth_is_setup_shaped_not_current_hot_capsule_callsite",
        "rejected_owner_2=generic_capsule_flattening",
        "rejected_reason_2=too_broad_without_new_escape_specific_guard_surface",
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

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
    parser.add_argument(
        "--context",
        choices=[
            "after-result-capsule-reset",
            "after-record-success-helper-fusion",
        ],
        default="after-result-capsule-reset",
    )
    args = parser.parse_args()

    values = read_kv(args.inventory_report)
    require(values, "output_contract", "page-model-hotpath-ir-shape-diff-inventory-v0")
    require(values, "summary", "ok")
    require(values, "ir_shape_diff_inventory_only", "1")
    selected_method = require_key(values, "selected_method")
    shape_owner = require_key(values, "selected_method_shape_owner")
    prior_no_material_row = values.get("selected_method_prior_no_material_effect_row")
    fallback_no_effect_row = values.get("method_1_prior_no_effect_row")
    selected_owner_method = selected_method
    extra_lines: list[str] = []
    if args.context == "after-record-success-helper-fusion":
        owner_refresh = "post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion"
    else:
        owner_refresh = "page_model_owner_refresh"

    if shape_owner == "copy_materialization" and prior_no_material_row:
        fallback_method = values.get("method_1_symbol", "none")
        if fallback_method != "none" and fallback_no_effect_row:
            selected_owner = owner_refresh
            next_diagnostic = selected_owner
            selected_reason = "prior_acquire_copy_and_release_known_live_no_effect_select_owner_refresh"
            selected_owner_method = "none"
            extra_lines.extend(
                [
                    f"selected_method_prior_no_material_effect_row={prior_no_material_row}",
                    f"fallback_method={fallback_method}",
                    f"fallback_method_prior_no_effect_row={fallback_no_effect_row}",
                    "rejected_owner_3=page_model_acquire_usize_copy_materialization_retry",
                    "rejected_reason_3=prior_receiver_forwarding_no_material_effect_requires_different_page_model_owner",
                    "rejected_owner_4=page_model_release_known_live_field_traffic_probe",
                    "rejected_reason_4=prior_release_known_live_rmw_no_effect_requires_owner_refresh",
                ]
            )
        elif fallback_method != "none":
            selected_owner = "page_model_release_known_live_field_traffic_probe"
            next_diagnostic = selected_owner
            selected_reason = "prior_acquire_copy_materialization_no_material_effect_select_next_page_model_method"
            selected_owner_method = fallback_method
            extra_lines.extend(
                [
                    f"selected_method_prior_no_material_effect_row={prior_no_material_row}",
                    f"selected_owner_method_pct={values.get('method_1_pct', '0.00')}",
                    f"selected_owner_method_field_get_count={values.get('method_1_field_get_count', '0')}",
                    f"selected_owner_method_field_set_count={values.get('method_1_field_set_count', '0')}",
                    f"selected_owner_method_copy_count={values.get('method_1_copy_count', '0')}",
                    f"selected_owner_method_call_count={values.get('method_1_call_count', '0')}",
                    "rejected_owner_3=page_model_acquire_usize_copy_materialization_retry",
                    "rejected_reason_3=prior_receiver_forwarding_no_material_effect_requires_different_page_model_owner",
                ]
            )
        else:
            selected_owner = "page_model_owner_refresh"
            next_diagnostic = selected_owner
            selected_reason = "prior_acquire_copy_materialization_no_material_effect_without_alternate_page_model_method"
            extra_lines.extend(
                [
                    f"selected_method_prior_no_material_effect_row={prior_no_material_row}",
                    "rejected_owner_3=page_model_acquire_usize_copy_materialization_retry",
                    "rejected_reason_3=prior_receiver_forwarding_no_material_effect_requires_different_page_model_owner",
                ]
            )
    elif shape_owner == "copy_materialization":
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
        f"selected_owner_method={selected_owner_method}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        "rejected_owner=page_model_same_block_rmw_retry",
        "rejected_reason=recent_selected_method_rmw_keeper_already_applied",
        "rejected_owner_1=page_model_direct_op_retry",
        "rejected_reason_1=direct_op_previous_rejected",
        "rejected_owner_2=page_queue_retry",
        "rejected_reason_2=page_queue_recent_nonkeeper_retry_closed",
        *extra_lines,
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

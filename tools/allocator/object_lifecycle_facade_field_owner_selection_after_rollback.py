#!/usr/bin/env python3
"""Select the next owner from facade field inventory after rollback refresh."""

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


def int_key(values: dict[str, str], key: str) -> int:
    text = values.get(key)
    if text is None:
        raise SystemExit(f"missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} expected integer, got {text!r}") from exc


def str_key(values: dict[str, str], key: str) -> str:
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
            "after-release-known-live-rollback",
            "after-record-success-helper-fusion",
        ],
        default="after-release-known-live-rollback",
    )
    args = parser.parse_args()

    values = read_kv(args.inventory_report)
    require(values, "output_contract", "object-lifecycle-facade-exact-slot-field-inventory-v0")
    require(values, "input_contract", "weighted-exact-slot-owner-selection-v0")
    require(values, "selected_next", "facade_field_owner_selection")
    require(values, "summary", "ok")

    dominant = str_key(values, "dominant_field_family")
    facade_receiver = int_key(values, "field_family.facade_receiver_state_count")
    page_queue = int_key(values, "field_family.page_queue_bridge_count")
    alloc_result = int_key(values, "field_family.alloc_result_capsule_count")
    same_block = int_key(values, "pattern.same_block_get_set_count")
    repeated_get = int_key(values, "pattern.same_receiver_repeated_get_count")
    positive = int_key(values, "pattern.positive_net_cache_candidate_count")

    if args.context == "after-record-success-helper-fusion":
        output_contract = "facade-field-owner-selection-after-record-success-helper-fusion-v0"
        refresh_diagnostic = "post_facade_inventory_owner_refresh_after_record_success_helper_fusion"
    else:
        output_contract = "facade-field-owner-selection-after-release-known-live-rollback-v0"
        refresh_diagnostic = "post_facade_inventory_owner_refresh_after_release_known_live_rollback"

    if positive <= 4:
        selected_owner = "post_facade_inventory_owner_refresh"
        selected_reason = "selected_facade_fusion_already_landed_and_positive_net_surface_still_4"
        next_diagnostic = refresh_diagnostic
        rejected_owner = "repeat_selected_facade_same_block_get_set_fusion"
        rejected_reason = "same_block_get_set_candidate_count_3_already_exercised_by_row231_keeper"
    else:
        selected_owner = "facade_followon_owner_selection"
        selected_reason = "facade_positive_net_surface_grew_after_rollback"
        next_diagnostic = "facade_followon_owner_selection"
        rejected_owner = "post_facade_inventory_owner_refresh"
        rejected_reason = "facade_positive_net_surface_still_large"

    lines = [
        f"output_contract={output_contract}",
        "input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"dominant_field_family={dominant}",
        f"facade_receiver_state_count={facade_receiver}",
        f"page_queue_bridge_count={page_queue}",
        f"alloc_result_capsule_count={alloc_result}",
        f"same_block_get_set_count={same_block}",
        f"same_receiver_repeated_get_count={repeated_get}",
        f"positive_net_cache_candidate_count={positive}",
        "previous_selected_facade_get_set_keeper_landed=1",
        "previous_selected_facade_get_set_measurement_row=296x-231",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        f"rejected_owner={rejected_owner}",
        f"rejected_reason={rejected_reason}",
        "rejected_owner_1=generic_typed_field_residence_retry",
        "rejected_reason_1=no_new_family_specific_positive_net_plan",
        "rejected_owner_2=facade_method_local_scalar_cache",
        f"rejected_reason_2=same_receiver_repeated_get_count_{repeated_get}_too_small",
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

#!/usr/bin/env python3
"""Select the next owner from residual facade exact-slot field inventory."""

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
    value = values.get(key)
    if value is None:
        raise SystemExit(f"missing {key}")
    try:
        return int(value)
    except ValueError as exc:
        raise SystemExit(f"{key} expected integer, got {value!r}") from exc


def str_key(values: dict[str, str], key: str) -> str:
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
    require(values, "output_contract", "object-lifecycle-facade-residual-exact-slot-field-inventory-v0")
    require(values, "summary", "ok")

    positive_net = int_key(values, "pattern.positive_net_cache_candidate_count")
    page_queue_count = int_key(values, "field_family.page_queue_bridge_count")
    facade_receiver_count = int_key(values, "field_family.facade_receiver_state_count")
    dominant_family = str_key(values, "dominant_field_family")

    if positive_net <= 4 and page_queue_count > 0:
        selected_owner = "page_queue_exact_slot_field_inventory"
        selected_reason = "residual_facade_positive_net_surface_not_growing_and_page_queue_is_next_bridge_family"
        next_diagnostic = "page_queue_exact_slot_field_inventory"
        rejected_owner = "residual_facade_same_block_get_set_retry"
        rejected_reason = "selected_facade_get_set_fusion_already_landed_and_positive_net_candidate_count_still_4"
    else:
        selected_owner = "residual_facade_field_owner_selection_followon"
        selected_reason = "residual_facade_positive_net_surface_requires_followon_selection"
        next_diagnostic = "residual_facade_field_owner_selection_followon"
        rejected_owner = "page_queue_exact_slot_field_inventory"
        rejected_reason = "residual_facade_positive_net_surface_still_dominant"

    lines = [
        "output_contract=object-lifecycle-facade-residual-field-owner-selection-v0",
        "input_contract=object-lifecycle-facade-residual-exact-slot-field-inventory-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"dominant_field_family={dominant_family}",
        f"facade_receiver_state_count={facade_receiver_count}",
        f"page_queue_bridge_count={page_queue_count}",
        f"positive_net_cache_candidate_count={positive_net}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        f"rejected_owner={rejected_owner}",
        f"rejected_reason={rejected_reason}",
        "rejected_owner_1=generic_typed_field_residence_retry",
        "rejected_reason_1=no_family_specific_positive_net_plan",
        "rejected_owner_2=facade_method_local_scalar_cache",
        "rejected_reason_2=residual_repeated_get_surface_too_small",
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

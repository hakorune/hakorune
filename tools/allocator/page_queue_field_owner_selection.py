#!/usr/bin/env python3
"""Select one page queue field owner from exact-slot inventory."""

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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.inventory_report)
    require(values, "output_contract", "page-queue-exact-slot-field-inventory-v0")
    require(values, "summary", "ok")

    same_block = int_key(values, "pattern.same_block_get_set_count")
    repeated_get = int_key(values, "pattern.same_receiver_repeated_get_count")
    positive_net = int_key(values, "pattern.positive_net_cache_candidate_count")

    if same_block >= repeated_get:
        selected_owner = "selected_page_queue_same_block_get_set_fusion"
        selected_reason = "same_block_get_set_candidates_dominate_page_queue_positive_net_surface"
        next_diagnostic = "selected_page_queue_same_block_get_set_guard_surface"
        planned_erased = same_block * 2
        planned_added = same_block
        rejected_owner = "page_queue_method_local_scalar_cache"
        rejected_reason = "same_receiver_repeated_get_surface_smaller_than_same_block_get_set"
    else:
        selected_owner = "page_queue_method_local_scalar_cache"
        selected_reason = "same_receiver_repeated_get_candidates_dominate_page_queue_positive_net_surface"
        next_diagnostic = "page_queue_scalar_cache_guard_surface"
        planned_erased = repeated_get
        planned_added = 0
        rejected_owner = "selected_page_queue_same_block_get_set_fusion"
        rejected_reason = "same_block_get_set_surface_smaller_than_repeated_get"

    planned_net = planned_erased - planned_added
    lines = [
        "output_contract=page-queue-field-owner-selection-v0",
        "input_contract=page-queue-exact-slot-field-inventory-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"same_block_get_set_count={same_block}",
        f"same_receiver_repeated_get_count={repeated_get}",
        f"positive_net_cache_candidate_count={positive_net}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        f"planned_erased_get_set_helper_calls={planned_erased}",
        f"planned_added_fused_helper_calls={planned_added}",
        f"planned_net_helper_call_delta={planned_net}",
        f"planned_net_helper_call_delta_positive={1 if planned_net > 0 else 0}",
        f"rejected_owner={rejected_owner}",
        f"rejected_reason={rejected_reason}",
        "rejected_owner_1=generic_typed_field_residence_retry",
        "rejected_reason_1=no_page_queue_specific_residence_plan",
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

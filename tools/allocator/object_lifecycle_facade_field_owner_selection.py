#!/usr/bin/env python3
"""Select one facade field owner from exact-slot field inventory."""

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


def require_int(values: dict[str, str], key: str) -> int:
    text = values.get(key)
    if text is None:
        raise SystemExit(f"missing {key}")
    return int(text)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.inventory_report)
    require(values, "output_contract", "object-lifecycle-facade-exact-slot-field-inventory-v0")
    require(values, "selected_next", "facade_field_owner_selection")
    require(values, "summary", "ok")

    same_block = require_int(values, "pattern.same_block_get_set_count")
    repeated_get = require_int(values, "pattern.same_receiver_repeated_get_count")
    positive = require_int(values, "pattern.positive_net_cache_candidate_count")

    if same_block >= 3:
        selected_owner = "selected_facade_same_block_get_set_fusion"
        selected_reason = "same_block_get_set_candidates_dominate_positive_net_surface"
        next_diagnostic = "selected_facade_same_block_get_set_guard_surface"
        planned_erased = same_block * 2
        planned_added = same_block
    elif repeated_get >= 3:
        selected_owner = "facade_method_local_scalar_cache"
        selected_reason = "same_receiver_repeated_get_candidates_dominate_positive_net_surface"
        next_diagnostic = "facade_method_local_scalar_cache_guard_surface"
        planned_erased = repeated_get
        planned_added = 0
    else:
        selected_owner = "post_facade_inventory_owner_refresh"
        selected_reason = "positive_net_surface_too_small_for_keeper"
        next_diagnostic = "post_facade_inventory_owner_refresh"
        planned_erased = 0
        planned_added = 0

    planned_net = planned_erased - planned_added

    lines = [
        "output_contract=object-lifecycle-facade-field-owner-selection-v0",
        "input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"same_block_get_set_count={same_block}",
        f"same_receiver_repeated_get_count={repeated_get}",
        f"positive_net_cache_candidate_count={positive}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        f"planned_erased_get_set_helper_calls={planned_erased}",
        f"planned_added_fused_helper_calls={planned_added}",
        f"planned_net_helper_call_delta={planned_net}",
        f"planned_net_helper_call_delta_positive={1 if planned_net > 0 else 0}",
        "rejected_owner=generic_typed_field_residence_retry",
        "rejected_reason=no_family_specific_residence_plan",
        "rejected_owner_1=facade_method_local_scalar_cache",
        f"rejected_reason_1=same_receiver_repeated_get_count_{repeated_get}_too_small",
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

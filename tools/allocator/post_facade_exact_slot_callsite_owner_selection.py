#!/usr/bin/env python3
"""Select the next owner after post-facade exact-slot callsite attribution."""

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
    parser.add_argument("--attribution-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.attribution_report)
    require(values, "output_contract", "typed-object-exact-slot-callsite-attribution-v0")
    require(values, "input_contract", "post-selected-facade-get-set-owner-refresh-v0")
    require(values, "attribution_source", "perf_callgraph")
    require(values, "callgraph_attribution_available", "1")
    require(values, "summary", "ok")

    dominant_family = require_key(values, "dominant_family")
    top_callsite = require_key(values, "top_callsite_symbol")
    if dominant_family == "object_lifecycle_facade":
        selected_owner = "object_lifecycle_facade_residual_exact_slot_field_inventory"
        selected_reason = "dominant_facade_family_remains_after_selected_fusion"
        next_diagnostic = "object_lifecycle_facade_residual_exact_slot_field_inventory"
        rejected_owner = "repeat_selected_facade_same_block_get_set_fusion"
        rejected_reason = "selected_facade_get_set_fusion_already_landed_and_residual_shape_needs_inventory"
    else:
        selected_owner = "next_exact_slot_family_inventory"
        selected_reason = f"dominant_family_{dominant_family}"
        next_diagnostic = "next_exact_slot_family_inventory"
        rejected_owner = "object_lifecycle_facade_residual_exact_slot_field_inventory"
        rejected_reason = "object_lifecycle_facade_not_dominant_family"

    lines = [
        "output_contract=post-facade-exact-slot-callsite-owner-selection-v0",
        "input_contract=typed-object-exact-slot-callsite-attribution-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"dominant_family={dominant_family}",
        f"dominant_family_pct={require_key(values, 'dominant_family_pct')}",
        f"top_callsite_symbol={top_callsite}",
        f"top_callsite_pct={require_key(values, 'top_callsite_pct')}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        f"rejected_owner={rejected_owner}",
        f"rejected_reason={rejected_reason}",
        "rejected_owner_1=generic_typed_field_residence_retry",
        "rejected_reason_1=no_family_specific_positive_net_plan",
        "rejected_owner_2=page_queue_followon_keeper",
        "rejected_reason_2=page_queue_is_secondary_family_after_facade",
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

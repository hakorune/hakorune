#!/usr/bin/env python3
"""Select the next owner from weighted exact-slot attribution evidence."""

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


def owner_for_family(family: str) -> tuple[str, str]:
    if family == "page_model_hotpath":
        return "page_model_hotpath_ir_shape_diff_inventory", "page_model_hotpath_ir_shape_diff_inventory"
    if family == "object_lifecycle_facade":
        return "facade_exact_slot_ir_shape_diff_inventory", "facade_exact_slot_ir_shape_diff_inventory"
    if family in {"alloc_result_capsule", "release_result_capsule"}:
        return "result_capsule_ir_shape_diff_inventory", "result_capsule_ir_shape_diff_inventory"
    return "weighted_exact_slot_ir_shape_diff_inventory", "weighted_exact_slot_ir_shape_diff_inventory"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--weighted-attribution-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.weighted_attribution_report)
    require(values, "output_contract", "weighted-exact-slot-callsite-attribution-refresh-v0")
    require(values, "summary", "ok")
    require(values, "weighted_hot_candidate_score_required", "1")
    require(values, "ir_shape_diff_required_before_next_keeper", "1")
    require(values, "static_candidate_count_only_rejected", "1")

    dominant_family = require_key(values, "dominant_family")
    top_unblocked_family = require_key(values, "top_unblocked_family")
    recent_nonkeeper_family = require_key(values, "recent_nonkeeper_family")
    dominant_is_recent = values.get("dominant_family_is_recent_nonkeeper") == "1"

    if dominant_is_recent:
        selected_family = top_unblocked_family
        selected_reason = (
            "dominant_family_is_recent_nonkeeper_select_top_unblocked_family_with_ir_shape_diff"
        )
    else:
        selected_family = dominant_family
        selected_reason = "dominant_family_not_recent_nonkeeper"

    selected_owner, next_diagnostic = owner_for_family(selected_family)

    lines = [
        "output_contract=weighted-exact-slot-owner-selection-v0",
        "input_contract=weighted-exact-slot-callsite-attribution-refresh-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"dominant_family={dominant_family}",
        f"dominant_family_pct={require_key(values, 'dominant_family_pct')}",
        f"recent_nonkeeper_family={recent_nonkeeper_family}",
        f"recent_nonkeeper_row={require_key(values, 'recent_nonkeeper_row')}",
        f"recent_nonkeeper_candidate_count={require_key(values, 'recent_nonkeeper_candidate_count')}",
        f"recent_nonkeeper_hot_per_candidate_pct={require_key(values, 'recent_nonkeeper_hot_per_candidate_pct')}",
        f"dominant_family_is_recent_nonkeeper={1 if dominant_is_recent else 0}",
        f"top_unblocked_family={top_unblocked_family}",
        f"top_unblocked_family_pct={require_key(values, 'top_unblocked_family_pct')}",
        f"selected_family={selected_family}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        "rejected_owner=page_queue_immediate_retry",
        "rejected_reason=recent_nonkeeper_requires_ir_shape_diff_before_retry",
        "rejected_owner_1=static_candidate_count_only_selection",
        "rejected_reason_1=row241_candidate_count_prediction_failed",
        "rejected_owner_2=implementation_without_ir_shape_diff",
        "rejected_reason_2=ir_shape_diff_required_before_next_keeper",
        "weighted_hot_candidate_score_required=1",
        "ir_shape_diff_required_before_next_keeper=1",
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

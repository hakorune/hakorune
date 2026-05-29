#!/usr/bin/env python3
"""Refresh owner after facade inventory is rejected as too small."""

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
    parser.add_argument("--owner-refresh-report", type=Path, required=True)
    parser.add_argument("--facade-selection-report", type=Path, required=True)
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

    refresh = read_kv(args.owner_refresh_report)
    selection = read_kv(args.facade_selection_report)
    if args.context == "after-record-success-helper-fusion":
        refresh_contract = "post-record-success-helper-fusion-owner-refresh-v0"
        selection_contract = "facade-field-owner-selection-after-record-success-helper-fusion-v0"
        output_contract = "post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0"
        suffix = "after_record_success_helper_fusion"
        selected_reason = "top_unblocked_family_after_facade_small_surface_and_recent_page_queue_nonkeeper"
    else:
        refresh_contract = "post-release-known-live-rmw-rollback-owner-refresh-v0"
        selection_contract = "facade-field-owner-selection-after-release-known-live-rollback-v0"
        output_contract = "post-facade-inventory-owner-refresh-after-release-known-live-rollback-v0"
        suffix = "after_release_known_live_rollback"
        selected_reason = "top_unblocked_family_after_facade_small_surface_and_recent_page_model_nonkeeper"

    require(refresh, "output_contract", refresh_contract)
    require(refresh, "summary", "ok")
    require(selection, "output_contract", selection_contract)
    require(selection, "selected_owner", "post_facade_inventory_owner_refresh")
    require(selection, "summary", "ok")

    families: list[tuple[str, str]] = []
    for idx in range(16):
        name = refresh.get(f"family_{idx}_name")
        pct = refresh.get(f"family_{idx}_pct")
        if name is None or pct is None:
            continue
        families.append((name, pct))

    excluded = {
        "object_lifecycle_facade": "facade_positive_net_surface_already_exercised",
        refresh.get("recent_nonkeeper_family", ""): "recent_nonkeeper_requires_fresh_shape_before_retry",
    }

    selected_family = ""
    selected_pct = ""
    for name, pct in families:
        if name in excluded:
            continue
        selected_family = name
        selected_pct = pct
        break
    if not selected_family:
        raise SystemExit("no unblocked family found")

    selected_owner = f"{selected_family}_ir_shape_inventory_{suffix}"
    next_diagnostic = selected_owner

    lines = [
        f"output_contract={output_contract}",
        f"input_contract={selection_contract}",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"source_exact_slot_get_set_pct={refresh.get('exact_slot_get_set_pct', 'unknown')}",
        "excluded_family_0=object_lifecycle_facade",
        f"excluded_reason_0={excluded['object_lifecycle_facade']}",
        f"excluded_family_1={refresh.get('recent_nonkeeper_family', 'unknown')}",
        f"excluded_reason_1={excluded.get(refresh.get('recent_nonkeeper_family', ''), 'unknown')}",
        f"selected_family={selected_family}",
        f"selected_family_pct={selected_pct}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        "weighted_hot_candidate_score_required=1",
        "ir_shape_diff_required_before_next_keeper=1",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    for idx, (name, pct) in enumerate(families):
        lines.append(f"family_{idx}_name={name}")
        lines.append(f"family_{idx}_pct={pct}")
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

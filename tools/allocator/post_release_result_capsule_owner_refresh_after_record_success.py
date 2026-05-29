#!/usr/bin/env python3
"""Refresh exact-slot owner after release-result capsule repeat is rejected."""

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
    parser.add_argument("--release-selection-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    refresh = read_kv(args.owner_refresh_report)
    selection = read_kv(args.release_selection_report)
    require(refresh, "output_contract", "post-record-success-helper-fusion-owner-refresh-v0")
    require(refresh, "summary", "ok")
    require(selection, "output_contract", "release-result-capsule-owner-selection-after-record-success-helper-fusion-v0")
    require(selection, "selected_owner", "post_release_result_capsule_owner_refresh_after_record_success_helper_fusion")
    require(selection, "summary", "ok")

    families: list[tuple[str, str]] = []
    for idx in range(16):
        name = refresh.get(f"family_{idx}_name")
        pct = refresh.get(f"family_{idx}_pct")
        if name is None or pct is None:
            continue
        families.append((name, pct))

    excluded = {
        "page_queue_helpers": "recent_nonkeeper_requires_fresh_shape_before_retry",
        "object_lifecycle_facade": "facade_positive_net_surface_already_exercised",
        "page_model_hotpath": "page_model_subowners_already_exercised",
        "release_result_capsule": "release_result_record_success_repeat_closed",
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

    selected_owner = f"{selected_family}_ir_shape_inventory_after_record_success_helper_fusion"
    lines = [
        "output_contract=post-release-result-capsule-owner-refresh-after-record-success-helper-fusion-v0",
        "input_contract=release-result-capsule-owner-selection-after-record-success-helper-fusion-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"source_exact_slot_get_set_pct={refresh.get('exact_slot_get_set_pct', 'unknown')}",
        "excluded_family_0=page_queue_helpers",
        f"excluded_reason_0={excluded['page_queue_helpers']}",
        "excluded_family_1=object_lifecycle_facade",
        f"excluded_reason_1={excluded['object_lifecycle_facade']}",
        "excluded_family_2=page_model_hotpath",
        f"excluded_reason_2={excluded['page_model_hotpath']}",
        "excluded_family_3=release_result_capsule",
        f"excluded_reason_3={excluded['release_result_capsule']}",
        f"selected_family={selected_family}",
        f"selected_family_pct={selected_pct}",
        f"selected_owner={selected_owner}",
        "selected_reason=last_unblocked_family_after_known_nonkeeper_and_repeat_exclusions",
        f"next_diagnostic={selected_owner}",
        "remaining_family_is_small=1",
        "micro_helper_stop_line_near=1",
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

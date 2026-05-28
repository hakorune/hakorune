#!/usr/bin/env python3
"""Select one owner from releaseLocalKnownLive field-traffic evidence."""

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


def int_value(values: dict[str, str], key: str) -> int:
    try:
        return int(values[key])
    except KeyError as exc:
        raise SystemExit(f"missing {key}") from exc
    except ValueError as exc:
        raise SystemExit(f"{key} must be integer: {values[key]!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--probe-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.probe_report)
    require(values, "output_contract", "page-model-release-known-live-field-traffic-probe-v0")
    require(values, "summary", "ok")
    require(values, "implementation_open", "0")

    single_use_rmw = int_value(values, "rmw_single_use_candidate_count")
    multi_use_rmw = int_value(values, "rmw_multi_use_candidate_count")
    array_bridge = int_value(values, "array_bridge_field_get_count")

    if single_use_rmw > 0:
        selected_owner = "page_model_release_known_live_single_use_rmw_guard_surface"
        selected_reason = "single_use_rmw_candidates_have_positive_helper_call_delta"
        next_row = selected_owner
    elif array_bridge > 0:
        selected_owner = "page_model_release_known_live_array_bridge_probe"
        selected_reason = "array_bridge_field_gets_remain_after_no_single_use_rmw_candidates"
        next_row = selected_owner
    else:
        selected_owner = "page_model_owner_refresh"
        selected_reason = "no_positive_net_release_known_live_owner"
        next_row = selected_owner

    lines = [
        "output_contract=page-model-release-known-live-owner-selection-v0",
        "input_contract=page-model-release-known-live-field-traffic-probe-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"target_method={values.get('target_method', 'HakoAllocPageModel.releaseLocalKnownLive/1')}",
        f"target_method_pct={values.get('target_method_pct', '0.00')}",
        f"rmw_candidate_count={values.get('rmw_candidate_count', '0')}",
        f"rmw_single_use_candidate_count={single_use_rmw}",
        f"rmw_multi_use_candidate_count={multi_use_rmw}",
        f"array_bridge_field_get_count={array_bridge}",
        "multi_use_rmw_immediate_implementation_blocked=1",
        "array_bridge_immediate_implementation_blocked=1",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_row={next_row}",
        "rejected_owner=page_model_release_known_live_multi_use_rmw_fusion",
        "rejected_reason=multi_use_rmw_does_not_guarantee_positive_helper_call_delta",
        "rejected_owner_1=page_model_release_known_live_array_bridge_implementation",
        "rejected_reason_1=array_bridge_requires_separate_direct_slot_or_array_bridge_plan",
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

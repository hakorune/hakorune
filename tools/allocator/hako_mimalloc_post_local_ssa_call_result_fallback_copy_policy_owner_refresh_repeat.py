#!/usr/bin/env python3
"""Repeat owner refresh after the LocalSSA call-result fallback Copy keeper."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"{label}: missing {key}")
    return value


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = require_key(values, key, label)
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measurement", type=Path, required=True)
    parser.add_argument("--attribution", type=Path, required=True)
    parser.add_argument("--dynamic-weight", type=Path, required=True)
    parser.add_argument("--position", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    measurement = read_kv(args.measurement)
    attribution = read_kv(args.attribution)
    dynamic = read_kv(args.dynamic_weight)
    position = read_kv(args.position)

    require(
        measurement,
        "output_contract",
        "hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-measurement-v0",
        "measurement",
    )
    require(measurement, "selected_next_owner", "post_keeper_owner_unclear", "measurement")
    require(measurement, "selected_owner_confidence", "low", "measurement")
    require(attribution, "output_contract", "hako-mimalloc-callsite-copy-attribution-v0", "attribution")
    require(dynamic, "output_contract", "hako-mimalloc-local-ssa-dynamic-weight-probe-v0", "dynamic")
    require(position, "output_contract", "hako-mimalloc-local-ssa-copy-position-probe-v0", "position")

    copy_count = require_int(attribution, "copy_count", "attribution")
    page_hotpath_copies = require_int(attribution, "page_hotpath_helpers_attributed_copy_count", "attribution")
    result_copies = require_int(attribution, "owner_result_materialization_copy_count", "attribution")
    local_ssa_copies = require_int(attribution, "owner_local_ssa_copy_materialization_copy_count", "attribution")
    call_operand_copies = require_int(position, "call_operand_route_carrier_copy_count", "position")
    call_adjacent_copies = require_int(position, "call_adjacent_copy_count", "position")
    backend_route_carrier_copies = require_int(position, "backend_route_carrier_copy_count", "position")
    route_aware_candidates = require_int(position, "route_aware_candidate_copy_count", "position")

    dominant_copy_owner = require_key(attribution, "dominant_copy_owner", "attribution")
    dominant_dynamic_owner = require_key(dynamic, "dominant_dynamic_owner", "dynamic")
    dominant_position = require_key(position, "dominant_position", "position")
    dominant_route_role = require_key(position, "dominant_route_carrier_role", "position")

    selected_next_owner = "post_keeper_owner_unclear"
    confidence = "low"
    next_task = "post_keeper_owner_repeat"
    selected_reason = "owner_surface_still_ambiguous"

    if (
        dominant_copy_owner == "local_ssa_copy_materialization"
        and dominant_dynamic_owner == "local_ssa_copy_materialization"
        and dominant_position == "call_adjacent"
        and dominant_route_role == "call_operand"
        and call_operand_copies >= local_ssa_copies
    ):
        selected_next_owner = "call_operand_materialization_copy_chain_inventory"
        confidence = "medium"
        next_task = "call_operand_materialization_copy_chain_inventory"
        selected_reason = "same_current_mir_run_shows_call_operand_route_carrier_dominates_remaining_copy_surface"

    lines = [
        "output_contract=hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-owner-refresh-repeat-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-682",
        f"hako_body_elapsed_ns={require_key(measurement, 'hako_body_elapsed_ns', 'measurement')}",
        f"c_body_elapsed_ns={require_key(measurement, 'c_body_elapsed_ns', 'measurement')}",
        f"body_elapsed_ratio={require_key(measurement, 'body_elapsed_ratio', 'measurement')}",
        f"copy_count={copy_count}",
        f"local_ssa_copy_materialization_copy_count={local_ssa_copies}",
        f"call_adjacent_copy_count={call_adjacent_copies}",
        f"call_operand_route_carrier_copy_count={call_operand_copies}",
        f"backend_route_carrier_copy_count={backend_route_carrier_copies}",
        f"route_aware_candidate_copy_count={route_aware_candidates}",
        f"page_hotpath_helpers_attributed_copy_count={page_hotpath_copies}",
        f"result_materialization_copy_count={result_copies}",
        f"dominant_copy_owner={dominant_copy_owner}",
        f"dominant_dynamic_owner={dominant_dynamic_owner}",
        f"dominant_position={dominant_position}",
        f"dominant_route_carrier_role={dominant_route_role}",
        f"selected_next_owner={selected_next_owner}",
        f"selected_owner_confidence={confidence}",
        f"selected_reason={selected_reason}",
        f"next_task={next_task}",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Refresh the owner after call-operand receiver-root forwarding measurement."""

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
        raise SystemExit(f"{label}: {key} must be integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measurement", type=Path, required=True)
    parser.add_argument("--attribution", type=Path, required=True)
    parser.add_argument("--position", type=Path, required=True)
    parser.add_argument("--inventory", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    measurement = read_kv(args.measurement)
    attribution = read_kv(args.attribution)
    position = read_kv(args.position)
    inventory = read_kv(args.inventory)

    require(
        measurement,
        "output_contract",
        "hako-mimalloc-post-call-operand-materialization-forwarding-measurement-v0",
        "measurement",
    )
    require(measurement, "winner_claim", "0", "measurement")
    require(attribution, "output_contract", "hako-mimalloc-callsite-copy-attribution-v0", "attribution")
    require(position, "output_contract", "hako-mimalloc-local-ssa-copy-position-probe-v0", "position")
    require(inventory, "output_contract", "hako-mimalloc-call-operand-materialization-copy-chain-inventory-v0", "inventory")

    copy_count = require_int(attribution, "copy_count", "attribution")
    local_ssa_count = require_int(attribution, "owner_local_ssa_copy_materialization_copy_count", "attribution")
    call_adjacent_count = require_int(position, "call_adjacent_copy_count", "position")
    call_operand_count = require_int(position, "call_operand_route_carrier_copy_count", "position")
    call_operand_chain_count = require_int(inventory, "call_operand_chain_count", "inventory")
    arg_same_root = require_int(inventory, "arg_same_block_root_call_operand_chain_count", "inventory")
    dominance_required = require_int(inventory, "dominance_required_candidate_count", "inventory")
    unknown_root = require_int(inventory, "unknown_root_call_operand_chain_count", "inventory")
    receiver_cross_root = require_int(inventory, "receiver_cross_block_root_call_operand_chain_count", "inventory")

    dominant_copy_owner = require_key(attribution, "dominant_copy_owner", "attribution")
    dominant_position = require_key(position, "dominant_position", "position")
    dominant_route_role = require_key(position, "dominant_route_carrier_role", "position")

    selected_next_owner = "post_call_operand_owner_unclear"
    confidence = "low"
    next_task = "post_call_operand_owner_refresh_repeat"
    selected_reason = "measurement_nonkeeper_and_residual_owner_unclear"

    if (
        dominant_copy_owner == "local_ssa_copy_materialization"
        and dominant_position == "call_adjacent"
        and dominant_route_role == "call_operand"
        and call_operand_count >= 20
        and dominance_required >= arg_same_root
    ):
        selected_next_owner = "call_operand_residual_policy_selection"
        confidence = "medium"
        next_task = "call_operand_residual_policy_selection"
        selected_reason = "residual_call_operand_surface_remains_dominant_but_previous_tiny_keeper_was_not_a_body_time_keeper"

    lines = [
        "output_contract=hako-mimalloc-post-call-operand-materialization-forwarding-owner-refresh-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-688",
        f"hako_body_elapsed_ns={require_key(measurement, 'hako_body_elapsed_ns', 'measurement')}",
        f"c_body_elapsed_ns={require_key(measurement, 'c_body_elapsed_ns', 'measurement')}",
        f"body_elapsed_ratio={require_key(measurement, 'body_elapsed_ratio', 'measurement')}",
        f"copy_count={copy_count}",
        f"local_ssa_copy_materialization_copy_count={local_ssa_count}",
        f"call_operand_route_carrier_copy_count={call_operand_count}",
        f"call_adjacent_copy_count={call_adjacent_count}",
        f"call_operand_chain_count={call_operand_chain_count}",
        f"arg_same_block_root_call_operand_chain_count={arg_same_root}",
        f"dominance_required_candidate_count={dominance_required}",
        f"unknown_root_call_operand_chain_count={unknown_root}",
        f"receiver_cross_block_root_call_operand_chain_count={receiver_cross_root}",
        f"dominant_copy_owner={dominant_copy_owner}",
        f"dominant_position={dominant_position}",
        f"dominant_route_carrier_role={dominant_route_role}",
        f"selected_next_owner={selected_next_owner}",
        f"selected_owner_confidence={confidence}",
        f"selected_reason={selected_reason}",
        f"next_task={next_task}",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
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

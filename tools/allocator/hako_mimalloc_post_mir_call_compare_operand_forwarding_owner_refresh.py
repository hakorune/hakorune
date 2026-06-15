#!/usr/bin/env python3
"""Refresh the owner after MIR-call compare operand forwarding measurement."""

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
    parser.add_argument("--dynamic-weight", type=Path, required=True)
    parser.add_argument("--position", type=Path, required=True)
    parser.add_argument("--origin", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    measurement = read_kv(args.measurement)
    attribution = read_kv(args.attribution)
    dynamic = read_kv(args.dynamic_weight)
    position = read_kv(args.position)
    origin = read_kv(args.origin)

    require(
        measurement,
        "output_contract",
        "hako-mimalloc-mir-call-compare-operand-forwarding-measurement-v0",
        "measurement",
    )
    require(measurement, "winner_claim", "0", "measurement")
    require(attribution, "output_contract", "hako-mimalloc-callsite-copy-attribution-v0", "attribution")
    require(dynamic, "output_contract", "hako-mimalloc-local-ssa-dynamic-weight-probe-v0", "dynamic")
    require(position, "output_contract", "hako-mimalloc-local-ssa-copy-position-probe-v0", "position")
    require(origin, "output_contract", "hako-mimalloc-expression-materialization-copy-origin-probe-v0", "origin")

    copy_count = require_int(attribution, "copy_count", "attribution")
    local_ssa_count = require_int(attribution, "owner_local_ssa_copy_materialization_copy_count", "attribution")
    phi_edge_count = require_int(position, "phi_edge_copy_count", "position")
    block_entry_count = require_int(position, "block_entry_copy_count", "position")
    call_operand_count = require_int(position, "call_operand_route_carrier_copy_count", "position")
    compare_operand_count = require_int(position, "compare_operand_route_carrier_copy_count", "position")
    expression_count = require_int(position, "expression_materialization_copy_count", "position")
    mir_call_origin_count = require_int(origin, "mir_call_origin_copy_count", "origin")
    const_origin_count = require_int(origin, "origin_const_copy_count", "origin")

    selected_owner = "post_compare_operand_owner_unclear"
    confidence = "low"
    next_task = "post_compare_operand_owner_refresh_repeat"
    reason = "compare_operand_family_removed_but_next_owner_not_selected"
    implementation_allowed = "0"

    if (
        local_ssa_count >= 10
        and phi_edge_count >= 10
        and block_entry_count >= 8
        and compare_operand_count == 0
        and mir_call_origin_count == 0
        and expression_count <= 1
    ):
        selected_owner = "local_ssa_block_entry_phi_edge_copy_family"
        confidence = "medium"
        next_task = "local_ssa_block_entry_phi_edge_copy_design"
        reason = "compare_operand_family_removed_and_residue_moved_to_block_entry_phi_edge_copies"

    lines = [
        "output_contract=hako-mimalloc-post-mir-call-compare-operand-forwarding-owner-refresh-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-747",
        f"hako_body_elapsed_ns={require_key(measurement, 'hako_body_elapsed_ns', 'measurement')}",
        f"c_body_elapsed_ns={require_key(measurement, 'c_body_elapsed_ns', 'measurement')}",
        f"body_elapsed_ratio={require_key(measurement, 'body_elapsed_ratio', 'measurement')}",
        f"copy_count={copy_count}",
        f"local_ssa_copy_materialization_copy_count={local_ssa_count}",
        f"phi_edge_copy_count={phi_edge_count}",
        f"block_entry_copy_count={block_entry_count}",
        f"call_operand_route_carrier_copy_count={call_operand_count}",
        f"compare_operand_route_carrier_copy_count={compare_operand_count}",
        f"expression_materialization_copy_count={expression_count}",
        f"mir_call_origin_copy_count={mir_call_origin_count}",
        f"const_origin_copy_count={const_origin_count}",
        f"dominant_dynamic_owner={require_key(dynamic, 'dominant_dynamic_owner', 'dynamic')}",
        f"dominant_position={require_key(position, 'dominant_position', 'position')}",
        f"dominant_local_like_position={require_key(position, 'dominant_local_like_position', 'position')}",
        f"dominant_expression_origin={require_key(origin, 'dominant_expression_origin', 'origin')}",
        f"selected_next_owner={selected_owner}",
        f"selected_owner_confidence={confidence}",
        f"selected_reason={reason}",
        f"next_task={next_task}",
        f"implementation_allowed={implementation_allowed}",
        "design_required=1",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

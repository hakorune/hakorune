#!/usr/bin/env python3
"""Select the next local-SSA copy-kind policy without reopening optimization."""

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
    parser.add_argument("--dynamic-weight", type=Path, required=True)
    parser.add_argument("--position", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    dynamic = read_kv(args.dynamic_weight)
    position = read_kv(args.position)
    require(dynamic, "output_contract", "hako-mimalloc-local-ssa-dynamic-weight-probe-v0", "dynamic")
    require(dynamic, "dominant_dynamic_owner", "local_ssa_copy_materialization", "dynamic")
    require(dynamic, "summary", "ok", "dynamic")
    require(position, "output_contract", "hako-mimalloc-local-ssa-copy-position-probe-v0", "position")
    require(position, "target_method", "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1", "position")
    require(position, "summary", "ok", "position")

    expression_count = require_int(position, "expression_materialization_copy_count", "position")
    local_like_count = require_int(position, "local_like_copy_count", "position")
    if expression_count <= 0 or local_like_count <= 0:
        raise SystemExit("position: expression/local-like counts must be positive")
    selected = "expression_materialization_copy_policy"
    next_diagnostic = "expression_materialization_copy_origin_probe"
    confidence = "medium"
    reason = "dominant_local_like_position_under_dynamic_local_ssa_owner"
    if expression_count * 2 >= local_like_count:
        confidence = "high"
        reason = "expression_materialization_majority_of_local_like_copies"

    lines = [
        "output_contract=hako-mimalloc-local-ssa-copy-kind-policy-selection-v0",
        "input_contract=hako-mimalloc-local-ssa-dynamic-weight-probe-v0+hako-mimalloc-local-ssa-copy-position-probe-v0",
        f"target_method={require_key(position, 'target_method', 'position')}",
        f"method_invocation_count={require_key(dynamic, 'method_invocation_count', 'dynamic')}",
        f"dominant_dynamic_owner={require_key(dynamic, 'dominant_dynamic_owner', 'dynamic')}",
        f"dominant_local_like_position={require_key(position, 'dominant_local_like_position', 'position')}",
        f"local_like_copy_count={local_like_count}",
        f"expression_materialization_copy_count={expression_count}",
        f"field_set_value_copy_count={require_key(position, 'field_set_value_copy_count', 'position')}",
        f"branch_condition_copy_count={require_key(position, 'branch_condition_copy_count', 'position')}",
        f"block_entry_copy_count={require_key(position, 'block_entry_copy_count', 'position')}",
        f"call_adjacent_copy_count={require_key(position, 'call_adjacent_copy_count', 'position')}",
        f"phi_edge_copy_count={require_key(position, 'phi_edge_copy_count', 'position')}",
        f"selected_copy_kind_policy={selected}",
        f"selected_policy_confidence={confidence}",
        f"selected_policy_reason={reason}",
        "rejected_policy=local_ssa_same_block_field_get_reuse",
        "rejected_reason=recent_nonkeeper_regressed_exact_exe_body",
        "rejected_policy_scope=same_block_field_get_only",
        f"next_diagnostic={next_diagnostic}",
        "optimization_open=0",
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

#!/usr/bin/env python3
"""Select the policy owner for field_get expression copy chains."""

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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None or text == "":
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def get_int(values: dict[str, str], key: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"origin: {key} must be an integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--origin", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    origin = read_kv(args.origin)
    require(
        origin,
        "output_contract",
        "hako-mimalloc-expression-materialization-copy-origin-probe-v0",
        "origin",
    )
    require(origin, "selected_origin_policy", "field_get_expression_value_copy_chain", "origin")
    require(origin, "optimization_open", "0", "origin")

    expression_count = require_int(origin, "expression_materialization_copy_count", "origin")
    field_get_count = require_int(origin, "field_get_origin_copy_count", "origin")
    if expression_count <= 0:
        raise SystemExit("origin: expression_materialization_copy_count must be positive")
    ratio_bp = (field_get_count * 10000) // expression_count

    compare_count = (
        get_int(origin, "sink_compare_eq_copy_count")
        + get_int(origin, "sink_compare_lt_copy_count")
        + get_int(origin, "sink_compare_ne_copy_count")
        + get_int(origin, "sink_compare_gt_copy_count")
        + get_int(origin, "sink_compare_le_copy_count")
        + get_int(origin, "sink_compare_ge_copy_count")
    )
    binop_count = (
        get_int(origin, "sink_binop_add_copy_count")
        + get_int(origin, "sink_binop_sub_copy_count")
        + get_int(origin, "sink_binop_mul_copy_count")
        + get_int(origin, "sink_binop_div_copy_count")
    )
    field_set_count = 0
    for key in origin:
        if key.startswith("sink_field_set_") and key.endswith("_copy_count"):
            field_set_count += get_int(origin, key)

    selected = "field_get_direct_consumer_value_forwarding"
    next_diagnostic = "field_get_direct_consumer_forwarding_candidate_probe"
    confidence = "medium"
    reason = "field_get_origin_dominates_expression_materialization"
    if ratio_bp >= 9000 and compare_count >= binop_count:
        confidence = "high"
        reason = "field_get_origin_dominates_and_compare_sinks_are_primary"

    lines = [
        "output_contract=hako-mimalloc-field-get-expression-copy-chain-policy-selection-v0",
        "input_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0",
        f"target_method={origin.get('target_method', '')}",
        f"field_get_origin_copy_count={field_get_count}",
        f"expression_materialization_copy_count={expression_count}",
        f"field_get_origin_ratio_bp={ratio_bp}",
        f"compare_sink_copy_count={compare_count}",
        f"binop_sink_copy_count={binop_count}",
        f"field_set_sink_copy_count={field_set_count}",
        f"selected_chain_policy={selected}",
        f"selected_chain_policy_confidence={confidence}",
        f"selected_chain_policy_reason={reason}",
        "rejected_chain_policy=local_ssa_broad_copy_coalescing",
        "rejected_reason=owner_not_yet_narrow_enough_and_recent_local_ssa_reuse_nonkeeper",
        f"next_diagnostic={next_diagnostic}",
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

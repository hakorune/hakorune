#!/usr/bin/env python3
"""Select the policy owner for param-origin expression copy chains."""

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


def sum_pair_prefix(values: dict[str, str], prefix: str) -> int:
    total = 0
    for key in values:
        if key.startswith(prefix) and key.endswith("_copy_count"):
            total += get_int(values, key)
    return total


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
    require(origin, "selected_origin_policy", "param_expression_value_copy_chain", "origin")
    require(origin, "optimization_open", "0", "origin")

    expression_count = require_int(origin, "expression_materialization_copy_count", "origin")
    param_count = require_int(origin, "param_origin_copy_count", "origin")
    if expression_count <= 0:
        raise SystemExit("origin: expression_materialization_copy_count must be positive")
    ratio_bp = (param_count * 10000) // expression_count

    field_get_count = get_int(origin, "pair_param__field_get_copy_count")
    field_set_count = sum_pair_prefix(origin, "pair_param__field_set_")
    compare_count = (
        get_int(origin, "pair_param__compare_eq_copy_count")
        + get_int(origin, "pair_param__compare_ne_copy_count")
        + get_int(origin, "pair_param__compare_lt_copy_count")
        + get_int(origin, "pair_param__compare_gt_copy_count")
        + get_int(origin, "pair_param__compare_le_copy_count")
        + get_int(origin, "pair_param__compare_ge_copy_count")
    )
    direct_sink_count = field_get_count + field_set_count + compare_count

    selected = "param_direct_consumer_value_forwarding"
    next_diagnostic = "param_direct_consumer_forwarding_candidate_probe"
    confidence = "medium"
    reason = "param_origin_dominates_expression_materialization_with_mixed_direct_sinks"
    if ratio_bp >= 8000:
        confidence = "high"
        reason = "param_origin_strongly_dominates_expression_materialization"
    elif direct_sink_count < param_count:
        confidence = "low"
        reason = "param_origin_dominates_but_direct_sink_coverage_is_partial"

    lines = [
        "output_contract=hako-mimalloc-param-expression-copy-chain-policy-selection-v0",
        "input_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0",
        f"target_method={origin.get('target_method', '')}",
        f"param_origin_copy_count={param_count}",
        f"expression_materialization_copy_count={expression_count}",
        f"param_origin_ratio_bp={ratio_bp}",
        f"param_field_get_sink_copy_count={field_get_count}",
        f"param_field_set_sink_copy_count={field_set_count}",
        f"param_compare_sink_copy_count={compare_count}",
        f"param_direct_sink_copy_count={direct_sink_count}",
        f"selected_chain_policy={selected}",
        f"selected_chain_policy_confidence={confidence}",
        f"selected_chain_policy_reason={reason}",
        "rejected_chain_policy=field_get_expression_value_copy_chain",
        "rejected_reason=current_expression_origin_is_param_not_field_get",
        "rejected_chain_policy_2=local_ssa_broad_copy_coalescing",
        "rejected_reason_2=recent_local_ssa_same_block_reuse_nonkeeper",
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

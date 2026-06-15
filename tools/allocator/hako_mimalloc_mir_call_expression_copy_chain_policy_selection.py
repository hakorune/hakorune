#!/usr/bin/env python3
"""Select the policy owner for MIR-call-origin expression copy chains."""

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
    require(origin, "selected_origin_policy", "mir_call_expression_value_copy_chain", "origin")
    require(origin, "optimization_open", "0", "origin")

    expression_count = require_int(origin, "expression_materialization_copy_count", "origin")
    mir_call_count = require_int(origin, "mir_call_origin_copy_count", "origin")
    if expression_count <= 0:
        raise SystemExit("origin: expression_materialization_copy_count must be positive")
    ratio_bp = (mir_call_count * 10000) // expression_count

    compare_count = (
        get_int(origin, "pair_mir_call__compare_eq_copy_count")
        + get_int(origin, "pair_mir_call__compare_ne_copy_count")
        + get_int(origin, "pair_mir_call__compare_lt_copy_count")
        + get_int(origin, "pair_mir_call__compare_gt_copy_count")
        + get_int(origin, "pair_mir_call__compare_le_copy_count")
        + get_int(origin, "pair_mir_call__compare_ge_copy_count")
    )
    select_page_count = get_int(origin, "origin_detail_selectPage_copy_count")
    const_unused_count = get_int(origin, "pair_const__unused_or_phi_only_copy_count")
    chain_len0_count = get_int(origin, "origin_copy_chain_len_0_count")
    chain_len1_count = get_int(origin, "origin_copy_chain_len_1_count")

    selected = "mir_call_compare_operand_value_forwarding_candidate_probe"
    next_diagnostic = "mir_call_compare_operand_forwarding_candidate_probe"
    confidence = "medium"
    reason = "mir_call_origin_reaches_compare_operand_but_expression_count_is_small"
    if ratio_bp >= 8000 and compare_count == mir_call_count:
        confidence = "high"
        reason = "mir_call_origin_dominates_and_all_mir_call_copies_feed_compare_operands"
    elif compare_count < mir_call_count:
        confidence = "low"
        reason = "mir_call_origin_has_partial_compare_sink_coverage"

    lines = [
        "output_contract=hako-mimalloc-mir-call-expression-copy-chain-policy-selection-v0",
        "input_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0",
        f"target_method={origin.get('target_method', '')}",
        f"mir_call_origin_copy_count={mir_call_count}",
        f"expression_materialization_copy_count={expression_count}",
        f"mir_call_origin_ratio_bp={ratio_bp}",
        f"mir_call_compare_sink_copy_count={compare_count}",
        f"mir_call_select_page_origin_copy_count={select_page_count}",
        f"const_unused_copy_count={const_unused_count}",
        f"origin_copy_chain_len_0_count={chain_len0_count}",
        f"origin_copy_chain_len_1_count={chain_len1_count}",
        f"selected_chain_policy={selected}",
        f"selected_chain_policy_confidence={confidence}",
        f"selected_chain_policy_reason={reason}",
        "rejected_chain_policy=field_get_expression_value_copy_chain",
        "rejected_reason=current_expression_origin_is_mir_call_not_field_get",
        "rejected_chain_policy_2=param_direct_consumer_value_forwarding",
        "rejected_reason_2=current_expression_origin_is_mir_call_not_param",
        "rejected_chain_policy_3=local_ssa_broad_copy_coalescing",
        "rejected_reason_3=recent_local_ssa_same_block_reuse_nonkeeper",
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

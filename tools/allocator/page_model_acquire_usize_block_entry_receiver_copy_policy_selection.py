#!/usr/bin/env python3
"""Select policy for acquire_usize block-entry receiver copies."""

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


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key)
    if text is None:
        raise SystemExit(f"missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be int, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--probe-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.probe_report)
    require(values, "output_contract", "page-model-acquire-usize-copy-materialization-probe-v0")
    require(values, "selected_next", "page_model_acquire_usize_block_entry_receiver_copy_policy_selection")
    require(values, "summary", "ok")

    block_entry = as_int(values, "block_entry_copy_count")
    receiver = as_int(values, "block_entry_receiver_param_copy_count")
    local_ssa = as_int(values, "local_ssa_copy_count")
    phi_edge = as_int(values, "phi_edge_copy_count")

    if receiver > 0 and local_ssa == 0 and phi_edge == 0:
        selected_policy = "selected_method_receiver_block_entry_copy_forwarding_guard_surface"
        next_row = selected_policy
        selected_reason = "receiver_block_entry_copies_dominate_without_local_ssa_or_phi_edge_surface"
    else:
        selected_policy = "copy_owner_refresh"
        next_row = "page_model_copy_owner_refresh"
        selected_reason = "block_entry_receiver_surface_not_isolated"

    lines = [
        "output_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0",
        "input_contract=page-model-acquire-usize-copy-materialization-probe-v0",
        f"target_method={values.get('target_method', 'HakoAllocPageModel.acquire_usize/1')}",
        f"copy_count={as_int(values, 'copy_count')}",
        f"block_entry_copy_count={block_entry}",
        f"block_entry_receiver_param_copy_count={receiver}",
        f"local_ssa_copy_count={local_ssa}",
        f"phi_edge_copy_count={phi_edge}",
        f"selected_policy={selected_policy}",
        f"selected_reason={selected_reason}",
        f"next_row={next_row}",
        "policy_scope=selected_method_only",
        "policy_shape=receiver_param_block_entry_copy_forwarding",
        "broad_local_ssa_reuse=0",
        "cross_block_value_rewrite=0",
        "field_get_result_chain_rewrite=0",
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

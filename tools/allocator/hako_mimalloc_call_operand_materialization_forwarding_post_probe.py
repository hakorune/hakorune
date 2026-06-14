#!/usr/bin/env python3
"""Validate the call-operand materialization forwarding implementation surface."""

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
    parser.add_argument("--guard-surface", type=Path, required=True)
    parser.add_argument("--post-inventory", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    guard = read_kv(args.guard_surface)
    post = read_kv(args.post_inventory)
    require(guard, "output_contract", "hako-mimalloc-call-operand-materialization-forwarding-guard-surface-v0", "guard-surface")
    require(guard, "selected_keeper_shape", "same_block_root_receiver_operand_forwarding", "guard-surface")
    require(post, "output_contract", "hako-mimalloc-call-operand-materialization-copy-chain-inventory-v0", "post-inventory")
    require(post, "summary", "ok", "post-inventory")

    post_selected = require_int(post, "receiver_same_block_root_call_operand_chain_count", "post-inventory")
    post_unique = require_int(post, "call_operand_unique_copy_count", "post-inventory")
    upper_bound = require_int(guard, "post_call_operand_unique_copy_count_upper_bound", "guard-surface")
    target = require_int(guard, "post_selected_keeper_candidate_count_target", "guard-surface")
    if post_selected != target:
        raise SystemExit(f"post-inventory: selected keeper candidate count expected {target}, got {post_selected}")
    if post_unique > upper_bound:
        raise SystemExit(f"post-inventory: call operand unique copies expected <= {upper_bound}, got {post_unique}")

    lines = [
        "output_contract=hako-mimalloc-call-operand-materialization-forwarding-implementation-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-686",
        "selected_keeper_shape=same_block_root_receiver_operand_forwarding",
        f"pre_selected_keeper_candidate_count={require_key(guard, 'pre_selected_keeper_candidate_count', 'guard-surface')}",
        f"post_selected_keeper_candidate_count={post_selected}",
        f"post_call_operand_unique_copy_count={post_unique}",
        f"post_call_operand_unique_copy_count_upper_bound={upper_bound}",
        "arg_forwarding_enabled=0",
        "helper_name_special_case=0",
        "requires_dominance_guard=0",
        "variable_map_semantics_changed=0",
        "phi_lifecycle_changed=0",
        "implementation_started=1",
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

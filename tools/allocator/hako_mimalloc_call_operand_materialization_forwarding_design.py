#!/usr/bin/env python3
"""Design the narrow keeper for call-operand materialization forwarding."""

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
    parser.add_argument("--inventory", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    inv = read_kv(args.inventory)
    require(inv, "output_contract", "hako-mimalloc-call-operand-materialization-copy-chain-inventory-v0", "inventory")
    require(inv, "selected_next_owner", "call_operand_materialization_forwarding_design", "inventory")
    require(inv, "summary", "ok", "inventory")

    call_operand_chain_count = require_int(inv, "call_operand_chain_count", "inventory")
    safe_candidates = require_int(inv, "safe_forwarding_candidate_count", "inventory")
    dominance_required = require_int(inv, "dominance_required_candidate_count", "inventory")
    unknown_roots = require_int(inv, "unknown_root_call_operand_chain_count", "inventory")
    arg_chains = require_int(inv, "arg_operand_chain_count", "inventory")
    receiver_same_root = require_int(inv, "receiver_same_block_root_call_operand_chain_count", "inventory")
    arg_same_root = require_int(inv, "arg_same_block_root_call_operand_chain_count", "inventory")
    receiver_cross_root = require_int(inv, "receiver_cross_block_root_call_operand_chain_count", "inventory")
    receiver_unknown_root = require_int(inv, "receiver_unknown_root_call_operand_chain_count", "inventory")

    selected_keeper_candidate_count = receiver_same_root
    rejected_arg_forwarding_count = arg_chains
    rejected_unknown_root_count = unknown_roots
    rejected_dominance_required_count = dominance_required
    rejected_receiver_nonlocal_root_count = receiver_cross_root + receiver_unknown_root

    if safe_candidates != receiver_same_root + arg_same_root:
        raise SystemExit("inventory: safe forwarding candidate split does not match role/root counts")

    lines = [
        "output_contract=hako-mimalloc-call-operand-materialization-forwarding-design-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-684",
        f"call_operand_chain_count={call_operand_chain_count}",
        f"safe_forwarding_candidate_count={safe_candidates}",
        f"dominance_required_candidate_count={dominance_required}",
        f"unknown_root_call_operand_chain_count={unknown_roots}",
        "selected_keeper_shape=same_block_root_receiver_operand_forwarding",
        f"selected_keeper_candidate_count={selected_keeper_candidate_count}",
        f"receiver_same_block_root_candidate_count={receiver_same_root}",
        f"arg_same_block_root_candidate_count={arg_same_root}",
        f"rejected_arg_forwarding_count={rejected_arg_forwarding_count}",
        f"rejected_unknown_root_count={rejected_unknown_root_count}",
        f"rejected_dominance_required_count={rejected_dominance_required_count}",
        f"rejected_receiver_nonlocal_root_count={rejected_receiver_nonlocal_root_count}",
        "requires_dominance_guard=0",
        "arg_forwarding_enabled=0",
        "helper_name_special_case=0",
        "variable_map_semantics_changed=0",
        "phi_lifecycle_changed=0",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
        "next_task=call_operand_materialization_forwarding_guard_surface",
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

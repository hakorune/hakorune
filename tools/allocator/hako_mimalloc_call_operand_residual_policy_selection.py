#!/usr/bin/env python3
"""Select the next residual call-operand materialization policy family."""

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
    parser.add_argument("--owner-refresh", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    owner = read_kv(args.owner_refresh)
    require(
        owner,
        "output_contract",
        "hako-mimalloc-post-call-operand-materialization-forwarding-owner-refresh-v0",
        "owner-refresh",
    )
    require(owner, "selected_next_owner", "call_operand_residual_policy_selection", "owner-refresh")
    require(owner, "summary", "ok", "owner-refresh")

    arg_same_root = require_int(owner, "arg_same_block_root_call_operand_chain_count", "owner-refresh")
    dominance_required = require_int(owner, "dominance_required_candidate_count", "owner-refresh")
    unknown_root = require_int(owner, "unknown_root_call_operand_chain_count", "owner-refresh")
    receiver_cross_root = require_int(owner, "receiver_cross_block_root_call_operand_chain_count", "owner-refresh")

    selected_policy_family = "dominance_required_call_operand_forwarding"
    selected_policy_candidate_count = dominance_required
    rejected_policy_family = "arg_same_block_root_forwarding"
    rejected_policy_candidate_count = arg_same_root
    selected_next_owner = "call_operand_dominance_required_forwarding_design"
    next_task = "call_operand_dominance_required_forwarding_design"
    confidence = "medium"
    reason = "dominance_required_surface_is_larger_than_arg_safe_surface_and_previous_tiny_receiver_keeper_was_not_a_body_time_keeper"

    lines = [
        "output_contract=hako-mimalloc-call-operand-residual-policy-selection-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-689",
        f"arg_same_block_root_call_operand_chain_count={arg_same_root}",
        f"dominance_required_candidate_count={dominance_required}",
        f"unknown_root_call_operand_chain_count={unknown_root}",
        f"receiver_cross_block_root_call_operand_chain_count={receiver_cross_root}",
        f"selected_policy_family={selected_policy_family}",
        f"selected_policy_candidate_count={selected_policy_candidate_count}",
        f"rejected_policy_family={rejected_policy_family}",
        f"rejected_policy_candidate_count={rejected_policy_candidate_count}",
        f"selected_next_owner={selected_next_owner}",
        f"selected_owner_confidence={confidence}",
        f"selected_reason={reason}",
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

#!/usr/bin/env python3
"""Inventory call-operand materialization Copy chains in one MIR method."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


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


def find_function(data: dict[str, Any], selected_method: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    for function in functions:
        if isinstance(function, dict) and function.get("name") == selected_method:
            return function
    raise SystemExit(f"selected method not found: {selected_method}")


def block_instructions(function: dict[str, Any]) -> list[tuple[Any, list[dict[str, Any]]]]:
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    out: list[tuple[Any, list[dict[str, Any]]]] = []
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if not isinstance(insts, list):
            continue
        out.append((block.get("id"), [inst for inst in insts if isinstance(inst, dict)]))
    return out


def call_operands(inst: dict[str, Any]) -> list[tuple[str, Any]]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return []
    callee = mir_call.get("callee")
    operands: list[tuple[str, Any]] = []
    if isinstance(callee, dict):
        operands.append(("receiver", callee.get("receiver")))
    args = mir_call.get("args", [])
    if isinstance(args, list):
        operands.extend(("arg", arg) for arg in args)
    return operands


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return "unknown"
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return "unknown"
    return str(callee.get("name", "unknown"))


def trace_copy_chain(seed: Any, copy_dst_to_src: dict[Any, Any]) -> tuple[list[Any], Any]:
    chain: list[Any] = []
    current = seed
    seen: set[Any] = set()
    while current in copy_dst_to_src and current not in seen:
        seen.add(current)
        chain.append(current)
        current = copy_dst_to_src[current]
    return chain, current


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--source-evidence", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=12)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    evidence = read_kv(args.source_evidence)
    require(
        evidence,
        "output_contract",
        "hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-owner-refresh-repeat-v0",
        "source-evidence",
    )
    require(evidence, "selected_next_owner", "call_operand_materialization_copy_chain_inventory", "source-evidence")
    require(evidence, "summary", "ok", "source-evidence")

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)

    defs: dict[Any, tuple[Any, int, dict[str, Any]]] = {}
    copy_dst_to_src: dict[Any, Any] = {}
    for block_id, insts in blocks:
        for inst_index, inst in enumerate(insts):
            dst = inst.get("dst")
            if dst is not None:
                defs[dst] = (block_id, inst_index, inst)
            if inst.get("op") == "copy":
                copy_dst_to_src[dst] = inst.get("src")

    role_counts: Counter[str] = Counter()
    role_root_counts: Counter[str] = Counter()
    callee_counts: Counter[str] = Counter()
    unique_copy_values: set[Any] = set()
    same_block_chain_count = 0
    cross_block_chain_count = 0
    same_block_root_count = 0
    cross_block_root_count = 0
    root_unknown_count = 0
    max_chain_len = 0
    samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "mir_call":
                continue
            for role, operand in call_operands(inst):
                chain, root = trace_copy_chain(operand, copy_dst_to_src)
                if not chain:
                    continue
                unique_copy_values.update(chain)
                role_counts[role] += 1
                callee = callee_name(inst)
                callee_counts[callee] += 1
                max_chain_len = max(max_chain_len, len(chain))
                copy_defs_same_block = all(defs.get(value, (None,))[0] == block_id for value in chain)
                if copy_defs_same_block:
                    same_block_chain_count += 1
                else:
                    cross_block_chain_count += 1
                root_block = defs.get(root, (None,))[0]
                if root_block == block_id:
                    same_block_root_count += 1
                    role_root_counts[f"{role}_same_block_root"] += 1
                elif root_block is None:
                    root_unknown_count += 1
                    role_root_counts[f"{role}_unknown_root"] += 1
                else:
                    cross_block_root_count += 1
                    role_root_counts[f"{role}_cross_block_root"] += 1
                samples.append(
                    {
                        "block": block_id,
                        "inst_index": inst_index,
                        "callee": callee,
                        "role": role,
                        "operand": operand,
                        "chain_len": len(chain),
                        "chain": ",".join(str(value) for value in chain),
                        "root": root,
                        "root_block": "unknown" if root_block is None else f"block_{root_block}",
                    }
                )

    chain_count = sum(role_counts.values())
    safe_forwarding_candidate_count = same_block_root_count
    dominance_required_candidate_count = cross_block_root_count

    selected_next_owner = "call_operand_materialization_forwarding_design"
    confidence = "medium"
    if chain_count == 0:
        selected_next_owner = "post_keeper_owner_repeat"
        confidence = "low"

    samples.sort(key=lambda sample: (sample["chain_len"], str(sample["callee"]), sample["role"]), reverse=True)

    lines = [
        "output_contract=hako-mimalloc-call-operand-materialization-copy-chain-inventory-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-683",
        f"copy_count={require_key(evidence, 'copy_count', 'source-evidence')}",
        f"call_operand_route_carrier_copy_count={require_key(evidence, 'call_operand_route_carrier_copy_count', 'source-evidence')}",
        f"call_adjacent_copy_count={require_key(evidence, 'call_adjacent_copy_count', 'source-evidence')}",
        f"call_operand_chain_count={chain_count}",
        f"call_operand_unique_copy_count={len(unique_copy_values)}",
        f"same_block_call_operand_chain_count={same_block_chain_count}",
        f"cross_block_call_operand_chain_count={cross_block_chain_count}",
        f"same_block_root_call_operand_chain_count={same_block_root_count}",
        f"cross_block_root_call_operand_chain_count={cross_block_root_count}",
        f"unknown_root_call_operand_chain_count={root_unknown_count}",
        f"receiver_operand_chain_count={role_counts['receiver']}",
        f"arg_operand_chain_count={role_counts['arg']}",
        f"receiver_same_block_root_call_operand_chain_count={role_root_counts['receiver_same_block_root']}",
        f"arg_same_block_root_call_operand_chain_count={role_root_counts['arg_same_block_root']}",
        f"receiver_cross_block_root_call_operand_chain_count={role_root_counts['receiver_cross_block_root']}",
        f"arg_cross_block_root_call_operand_chain_count={role_root_counts['arg_cross_block_root']}",
        f"receiver_unknown_root_call_operand_chain_count={role_root_counts['receiver_unknown_root']}",
        f"arg_unknown_root_call_operand_chain_count={role_root_counts['arg_unknown_root']}",
        f"max_call_operand_chain_len={max_chain_len}",
        f"safe_forwarding_candidate_count={safe_forwarding_candidate_count}",
        f"dominance_required_candidate_count={dominance_required_candidate_count}",
        f"selected_next_owner={selected_next_owner}",
        f"selected_owner_confidence={confidence}",
        "next_task=call_operand_materialization_forwarding_design",
        "implementation_started=0",
        "optimization_open=0",
        "winner_claim=0",
    ]
    for idx, (callee, count) in enumerate(callee_counts.most_common(8)):
        lines.append(f"top_callee_{idx}_name={callee}")
        lines.append(f"top_callee_{idx}_call_operand_chain_count={count}")
    for idx, sample in enumerate(samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_block=block_{sample['block']}",
                f"{prefix}_inst_index={sample['inst_index']}",
                f"{prefix}_callee={sample['callee']}",
                f"{prefix}_role={sample['role']}",
                f"{prefix}_operand={sample['operand']}",
                f"{prefix}_chain_len={sample['chain_len']}",
                f"{prefix}_chain={sample['chain']}",
                f"{prefix}_root={sample['root']}",
                f"{prefix}_root_block={sample['root_block']}",
            ]
        )
    lines.append("summary=ok")

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

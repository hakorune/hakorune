#!/usr/bin/env python3
"""Classify remaining field_get result-chain copies by consumer and owner seam."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


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
        if isinstance(insts, list):
            out.append((block.get("id"), [inst for inst in insts if isinstance(inst, dict)]))
    return out


def direct_uses(function: dict[str, Any]) -> dict[Any, Counter[str]]:
    uses: dict[Any, Counter[str]] = defaultdict(Counter)
    for block_id, insts in block_instructions(function):
        for inst in insts:
            op = inst.get("op")
            if op == "field_get":
                uses[inst.get("box")][f"field_get_receiver:block_{block_id}"] += 1
            elif op == "field_set":
                uses[inst.get("box")][f"field_set_receiver:block_{block_id}"] += 1
                uses[inst.get("value")][f"field_set_value:block_{block_id}"] += 1
            elif op == "binop":
                uses[inst.get("lhs")][f"binop_operand:block_{block_id}"] += 1
                uses[inst.get("rhs")][f"binop_operand:block_{block_id}"] += 1
            elif op == "compare":
                uses[inst.get("lhs")][f"compare_operand:block_{block_id}"] += 1
                uses[inst.get("rhs")][f"compare_operand:block_{block_id}"] += 1
            elif op == "branch":
                uses[inst.get("cond")][f"branch_condition:block_{block_id}"] += 1
            elif op == "mir_call":
                mir_call = inst.get("mir_call")
                if not isinstance(mir_call, dict):
                    continue
                callee = mir_call.get("callee")
                if isinstance(callee, dict):
                    uses[callee.get("receiver")][f"call_receiver:block_{block_id}"] += 1
                args = mir_call.get("args", [])
                if isinstance(args, list):
                    for arg in args:
                        uses[arg][f"call_arg:block_{block_id}"] += 1
            elif op == "phi":
                for incoming in inst.get("incoming", []):
                    if isinstance(incoming, list) and incoming:
                        uses[incoming[0]][f"phi_incoming:block_{block_id}"] += 1
            elif op == "copy":
                uses[inst.get("src")][f"copy_source:block_{block_id}"] += 1
    return uses


def owner_kind(use_key: str) -> str:
    return use_key.split(":", 1)[0]


def copy_chain_origin(
    src: Any,
    dst_to_src: dict[Any, Any],
    dst_to_origin: dict[Any, tuple[str, Any]],
) -> tuple[str, Any]:
    cursor = src
    seen: set[Any] = set()
    while cursor not in seen:
        seen.add(cursor)
        if cursor in dst_to_origin:
            return dst_to_origin[cursor]
        if cursor not in dst_to_src:
            break
        cursor = dst_to_src[cursor]
    return ("unknown", None)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=10)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    blocks = block_instructions(function)
    uses = direct_uses(function)

    consumer_counts: Counter[str] = Counter()
    origin_field_counts: Counter[str] = Counter()
    block_counts: Counter[Any] = Counter()
    same_block_origin_count = 0
    field_get_chain_copy_count = 0
    samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        dst_to_src = {inst.get("dst"): inst.get("src") for inst in insts if inst.get("op") == "copy"}
        dst_to_origin: dict[Any, tuple[str, Any]] = {}
        for inst in insts:
            if inst.get("op") == "field_get":
                dst_to_origin[inst.get("dst")] = (str(inst.get("field", "unknown")), block_id)

        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            field_name, origin_block = copy_chain_origin(inst.get("src"), dst_to_src, dst_to_origin)
            if field_name == "unknown":
                continue
            field_get_chain_copy_count += 1
            block_counts[block_id] += 1
            origin_field_counts[field_name] += 1
            if origin_block == block_id:
                same_block_origin_count += 1

            direct = uses.get(inst.get("dst"), Counter())
            if direct:
                for use_key, count in direct.items():
                    consumer_counts[owner_kind(use_key)] += count
            else:
                consumer_counts["dead_or_chain_internal"] += 1

            samples.append(
                {
                    "block_id": block_id,
                    "inst_index": inst_index,
                    "field": field_name,
                    "dst": inst.get("dst"),
                    "src": inst.get("src"),
                    "uses": ",".join(sorted(direct)) if direct else "none",
                }
            )

    selected_owner = "local_ssa_same_block_field_get_reuse_probe"
    owner_confidence = "medium"
    owner_reason = "all_field_get_result_chain_copies_have_same_block_field_get_origins"
    if same_block_origin_count == field_get_chain_copy_count and consumer_counts["copy_source"] >= 10:
        selected_owner = "local_ssa_same_block_field_get_reuse_probe"
        owner_reason = "same_block_field_get_origins_and_internal_copy_chains_dominate"
    elif consumer_counts["phi_incoming"] > consumer_counts["field_get_receiver"]:
        selected_owner = "phi_incoming_field_get_copy_probe"
        owner_reason = "phi_incoming_consumers_dominate_field_get_result_chains"
    elif consumer_counts["compare_operand"] + consumer_counts["binop_operand"] > consumer_counts["field_get_receiver"]:
        selected_owner = "scalar_operand_field_get_copy_probe"
        owner_reason = "scalar_operand_consumers_dominate_field_get_result_chains"

    samples.sort(
        key=lambda item: (
            block_counts[item["block_id"]],
            origin_field_counts[item["field"]],
            -int(item["inst_index"]),
        ),
        reverse=True,
    )

    lines = [
        "output_contract=hako-mimalloc-field-get-result-chain-follow-on-probe-v0",
        "input_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0",
        f"target_method={function.get('name', args.method)}",
        f"field_get_result_chain_copy_count={field_get_chain_copy_count}",
        f"same_block_origin_copy_count={same_block_origin_count}",
        f"selected_owner={selected_owner}",
        f"owner_confidence={owner_confidence}",
        f"owner_reason={owner_reason}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key in (
        "copy_source",
        "field_get_receiver",
        "phi_incoming",
        "compare_operand",
        "binop_operand",
        "field_set_value",
        "branch_condition",
        "dead_or_chain_internal",
    ):
        lines.append(f"consumer_{key}_count={consumer_counts[key]}")
    for field, count in origin_field_counts.most_common():
        lines.append(f"origin_field_{field}_copy_count={count}")
    for idx, (block_id, count) in enumerate(block_counts.most_common(6)):
        lines.append(f"top_block_{idx}_id=block_{block_id}")
        lines.append(f"top_block_{idx}_field_get_chain_copy_count={count}")
    for idx, sample in enumerate(samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_block=block_{sample['block_id']}",
                f"{prefix}_inst_index={sample['inst_index']}",
                f"{prefix}_origin_field={sample['field']}",
                f"{prefix}_dst={sample['dst']}",
                f"{prefix}_src={sample['src']}",
                f"{prefix}_uses={sample['uses']}",
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

#!/usr/bin/env python3
"""Refresh field_get direct-consumer forwarding ownership on current MIR."""

from __future__ import annotations

import argparse
import importlib.util
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
ORIGIN_PROBE = ROOT / "tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py"
DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


def load_origin_module() -> Any:
    spec = importlib.util.spec_from_file_location("hako_expr_origin_probe", ORIGIN_PROBE)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load origin probe: {ORIGIN_PROBE}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


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


def is_real_consumer(sinks: list[str]) -> bool:
    for sink in sinks:
        if sink == "unused_or_phi_only":
            continue
        if sink.startswith(("compare_", "binop_", "field_set:", "field_get")):
            return True
    return False


def safe_key(text: str) -> str:
    out: list[str] = []
    for ch in text:
        if ch.isalnum() or ch == "_":
            out.append(ch)
        else:
            out.append("_")
    return "".join(out).strip("_") or "unknown"


def producer_maps(
    blocks: list[tuple[Any, list[dict[str, Any]]]],
) -> tuple[dict[Any, dict[str, Any]], dict[Any, Any], dict[Any, int]]:
    producers: dict[Any, dict[str, Any]] = {}
    producer_blocks: dict[Any, Any] = {}
    producer_indices: dict[Any, int] = {}
    for block_id, insts in blocks:
        for inst_index, inst in enumerate(insts):
            dst = inst.get("dst")
            if dst is None:
                continue
            producers[dst] = inst
            producer_blocks[dst] = block_id
            producer_indices[dst] = inst_index
    return producers, producer_blocks, producer_indices


def root_field_get(
    seed: Any,
    producers: dict[Any, dict[str, Any]],
    producer_blocks: dict[Any, Any],
    producer_indices: dict[Any, int],
) -> tuple[dict[str, Any] | None, Any, int | None, int, list[Any]]:
    current = seed
    seen: set[Any] = set()
    chain: list[Any] = []
    while current in producers and current not in seen:
        seen.add(current)
        inst = producers[current]
        op = inst.get("op")
        if op == "copy":
            chain.append(current)
            current = inst.get("src")
            continue
        if op == "field_get":
            return inst, producer_blocks.get(current), producer_indices.get(current), len(chain), chain
        return None, None, None, len(chain), chain
    return None, None, None, len(chain), chain


def dominant(counts: Counter[str]) -> str:
    if not counts:
        return "none"
    return max(sorted(counts), key=lambda key: counts[key])


def selected_owner(
    forwarding_count: int,
    covered_count: int,
    same_block_count: int,
    cross_block_count: int,
) -> tuple[str, str, str]:
    if forwarding_count == 0:
        return "none", "high", "return_to_kernel_front_selection"
    if covered_count == forwarding_count:
        return "existing_row182_same_block_field_get_forwarding", "high", "rerun_body_timing"
    if same_block_count >= cross_block_count:
        return (
            "same_block_copy_chain_after_field_get_forwarding_gap",
            "medium",
            "same_block_copy_chain_after_field_get_forwarding_design",
        )
    return (
        "cross_block_field_get_alias_copy_chain",
        "medium",
        "cross_block_field_get_alias_forwarding_design",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--chain-policy", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--topn", type=int, default=10)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    policy = read_kv(args.chain_policy)
    require(
        policy,
        "output_contract",
        "hako-mimalloc-field-get-expression-copy-chain-policy-selection-v0",
        "chain-policy",
    )
    require(policy, "selected_chain_policy", "field_get_direct_consumer_value_forwarding", "chain-policy")
    require(policy, "optimization_open", "0", "chain-policy")

    origin = load_origin_module()
    function = origin.find_function(origin.load_json(args.mir_json), args.method)
    blocks = origin.block_instructions(function)
    producers, producer_blocks, producer_indices = producer_maps(blocks)

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    forwarding_candidates: list[dict[str, Any]] = []
    field_get_expression_count = 0
    same_block_count = 0
    cross_block_count = 0
    covered_by_existing_count = 0
    copy_chain_len_counts: Counter[str] = Counter()
    sink_counts: Counter[str] = Counter()
    field_counts: Counter[str] = Counter()

    for block_id, insts in blocks:
        call_attributed = origin.collect_call_attributed_copy_dsts(insts)
        consumers: dict[Any, list[dict[str, Any]]] = defaultdict(list)
        for inst in insts:
            for value in origin.value_uses(inst):
                consumers[value].append(inst)

        for inst_index, inst in enumerate(insts):
            if inst.get("op") != "copy":
                continue
            category = origin.classify_copy(
                inst.get("dst"),
                inst.get("src"),
                inst_index,
                insts,
                phi_dsts,
                call_attributed,
            )
            if category != "expression_materialization":
                continue
            root_inst, origin_block, origin_index, chain_len, chain = root_field_get(
                inst.get("src"),
                producers,
                producer_blocks,
                producer_indices,
            )
            if root_inst is None:
                continue
            field_get_expression_count += 1
            sinks = sorted(set(origin.sink_labels(inst.get("dst"), consumers)))
            if not is_real_consumer(sinks) or chain_len <= 0:
                continue

            same_block = origin_block == block_id
            source_inst = producers.get(inst.get("src"))
            covered_by_existing = (
                same_block
                and source_inst is not None
                and source_inst.get("op") == "field_get"
            )
            if same_block:
                same_block_count += 1
            else:
                cross_block_count += 1
            if covered_by_existing:
                covered_by_existing_count += 1

            for sink in sinks:
                if sink != "unused_or_phi_only":
                    sink_counts[sink] += 1
            field = str(root_inst.get("field", "unknown"))
            field_counts[field] += 1
            copy_chain_len_counts[str(chain_len)] += 1
            forwarding_candidates.append(
                {
                    "candidate_block": block_id,
                    "candidate_inst_index": inst_index,
                    "candidate_dst": inst.get("dst"),
                    "candidate_src": inst.get("src"),
                    "origin_field": field,
                    "origin_block": origin_block,
                    "origin_inst_index": origin_index,
                    "same_block_origin": int(same_block),
                    "copy_chain_len": chain_len,
                    "consumer_family": "+".join(sinks),
                    "covered_by_existing_rule": int(covered_by_existing),
                    "copy_chain": ",".join(str(value) for value in chain),
                }
            )

    forwarding_count = len(forwarding_candidates)
    owner, confidence, next_task = selected_owner(
        forwarding_count,
        covered_by_existing_count,
        same_block_count,
        cross_block_count,
    )

    lines = [
        "output_contract=hako-mimalloc-field-get-direct-consumer-refresh-v2",
        "input_contract=hako-mimalloc-field-get-expression-copy-chain-policy-selection-v0",
        f"target_method={function.get('name', args.method)}",
        f"field_get_expression_copy_count={field_get_expression_count}",
        f"forwarding_candidate_copy_count={forwarding_count}",
        f"same_block_candidate_count={same_block_count}",
        f"cross_block_candidate_count={cross_block_count}",
        f"covered_by_existing_rule_count={covered_by_existing_count}",
        f"dominant_candidate_sink={dominant(sink_counts)}",
        f"dominant_candidate_field={dominant(field_counts)}",
        f"selected_owner={owner}",
        f"selected_owner_confidence={confidence}",
        f"next_task={next_task}",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key, count in sorted(copy_chain_len_counts.items(), key=lambda item: int(item[0])):
        lines.append(f"copy_chain_len_{key}_candidate_count={count}")
    for key, count in sink_counts.most_common(8):
        lines.append(f"candidate_sink_{safe_key(key)}_count={count}")
    for key, count in field_counts.most_common(8):
        lines.append(f"candidate_field_{safe_key(key)}_count={count}")
    for idx, sample in enumerate(forwarding_candidates[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_origin_field={sample['origin_field']}",
                f"{prefix}_origin_block=block_{sample['origin_block']}",
                f"{prefix}_origin_inst_index={sample['origin_inst_index']}",
                f"{prefix}_candidate_block=block_{sample['candidate_block']}",
                f"{prefix}_candidate_inst_index={sample['candidate_inst_index']}",
                f"{prefix}_candidate_dst={sample['candidate_dst']}",
                f"{prefix}_candidate_src={sample['candidate_src']}",
                f"{prefix}_same_block_origin={sample['same_block_origin']}",
                f"{prefix}_copy_chain_len={sample['copy_chain_len']}",
                f"{prefix}_consumer_family={sample['consumer_family']}",
                f"{prefix}_covered_by_existing_rule={sample['covered_by_existing_rule']}",
                f"{prefix}_copy_chain={sample['copy_chain']}",
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

#!/usr/bin/env python3
"""Count MIR-call-result expression copies that feed compare operands."""

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


def is_compare_sink(sink: str) -> bool:
    return sink.startswith("compare_")


def is_forwardable_compare_sinks(sinks: list[str]) -> bool:
    real_sinks = [sink for sink in sinks if sink != "unused_or_phi_only"]
    return bool(real_sinks) and all(is_compare_sink(sink) for sink in real_sinks)


def dominant(counts: Counter[str]) -> str:
    if not counts:
        return "none"
    return max(sorted(counts), key=lambda key: counts[key])


def safe_key(text: str) -> str:
    out: list[str] = []
    for ch in text:
        if ch.isalnum() or ch == "_":
            out.append(ch)
        else:
            out.append("_")
    return "".join(out).strip("_") or "unknown"


def block_successors(insts: list[dict[str, Any]]) -> set[Any]:
    out: set[Any] = set()
    for inst in insts:
        if inst.get("op") == "branch":
            if inst.get("then") is not None:
                out.add(inst.get("then"))
            if inst.get("else") is not None:
                out.add(inst.get("else"))
        elif inst.get("op") == "jump" and inst.get("target") is not None:
            out.add(inst.get("target"))
    return out


def compute_dominators(blocks: list[tuple[Any, list[dict[str, Any]]]]) -> dict[Any, set[Any]]:
    block_ids = [block_id for block_id, _ in blocks]
    if not block_ids:
        return {}
    all_blocks = set(block_ids)
    entry = block_ids[0]
    preds: dict[Any, set[Any]] = {block_id: set() for block_id in block_ids}
    for block_id, insts in blocks:
        for succ in block_successors(insts):
            if succ in preds:
                preds[succ].add(block_id)

    doms: dict[Any, set[Any]] = {block_id: set(all_blocks) for block_id in block_ids}
    doms[entry] = {entry}
    changed = True
    while changed:
        changed = False
        for block_id in block_ids[1:]:
            if preds[block_id]:
                new = set(all_blocks)
                for pred in preds[block_id]:
                    new &= doms[pred]
            else:
                new = set()
            new.add(block_id)
            if new != doms[block_id]:
                doms[block_id] = new
                changed = True
    return doms


def mir_call_root_block(
    seed: Any,
    producers: dict[Any, tuple[Any, dict[str, Any]]],
) -> Any | None:
    current = seed
    seen: set[Any] = set()
    for _ in range(8):
        if current in seen:
            return None
        seen.add(current)
        item = producers.get(current)
        if item is None:
            return None
        block_id, inst = item
        op = inst.get("op")
        if op == "copy":
            current = inst.get("src")
            continue
        if op in {"mir_call", "call"}:
            return block_id
        return None
    return None


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
        "hako-mimalloc-mir-call-expression-copy-chain-policy-selection-v0",
        "chain-policy",
    )
    require(
        policy,
        "selected_chain_policy",
        "mir_call_compare_operand_value_forwarding_candidate_probe",
        "chain-policy",
    )
    require(policy, "optimization_open", "0", "chain-policy")

    origin = load_origin_module()
    function = origin.find_function(origin.load_json(args.mir_json), args.method)
    blocks = origin.block_instructions(function)
    all_producers = {
        inst.get("dst"): inst
        for _, insts in blocks
        for inst in insts
        if inst.get("dst") is not None
    }
    producers_with_block = {
        inst.get("dst"): (block_id, inst)
        for block_id, insts in blocks
        for inst in insts
        if inst.get("dst") is not None
    }
    dominators = compute_dominators(blocks)

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    mir_call_expression_count = 0
    compare_candidate_count = 0
    unsafe_candidate_count = 0
    same_block_candidate_count = 0
    dominance_required_candidate_count = 0
    root_dominates_candidate_count = 0
    chain_len_counts: Counter[str] = Counter()
    sink_counts: Counter[str] = Counter()
    origin_detail_counts: Counter[str] = Counter()
    unsafe_sink_counts: Counter[str] = Counter()
    samples: list[dict[str, Any]] = []

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
            origin_kind, origin_detail, chain_len = origin.origin_label(
                inst.get("src"), all_producers
            )
            if origin_kind != "mir_call":
                continue

            mir_call_expression_count += 1
            sinks = sorted(set(origin.sink_labels(inst.get("dst"), consumers)))
            chain_len_counts[str(chain_len)] += 1
            origin_detail_counts[origin_detail] += 1
            for sink in sinks:
                sink_counts[sink] += 1

            safe = is_forwardable_compare_sinks(sinks)
            if safe:
                compare_candidate_count += 1
                root_block = mir_call_root_block(inst.get("src"), producers_with_block)
                if root_block == block_id:
                    same_block_candidate_count += 1
                else:
                    dominance_required_candidate_count += 1
                if root_block is not None and root_block in dominators.get(block_id, set()):
                    root_dominates_candidate_count += 1
            else:
                unsafe_candidate_count += 1
                for sink in sinks:
                    unsafe_sink_counts[sink] += 1

            samples.append(
                {
                    "block": block_id,
                    "inst_index": inst_index,
                    "dst": inst.get("dst"),
                    "src": inst.get("src"),
                    "origin_detail": origin_detail,
                    "sink": "+".join(sinks),
                    "copy_chain_len": chain_len,
                    "safe": int(safe),
                    "root_block": mir_call_root_block(inst.get("src"), producers_with_block),
                }
            )

    selected_owner = "0"
    confidence = "low"
    next_task = "mir_call_compare_operand_forwarding_policy_recheck"
    if (
        mir_call_expression_count > 0
        and compare_candidate_count == mir_call_expression_count
        and root_dominates_candidate_count == compare_candidate_count
    ):
        selected_owner = "dominance_guarded_mir_call_compare_operand_forwarding"
        confidence = "medium"
        next_task = "mir_call_compare_operand_forwarding_guard_surface"

    lines = [
        "output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-candidate-probe-v0",
        "input_contract=hako-mimalloc-mir-call-expression-copy-chain-policy-selection-v0",
        f"target_method={function.get('name', args.method)}",
        f"mir_call_expression_copy_count={mir_call_expression_count}",
        f"compare_operand_forwarding_candidate_count={compare_candidate_count}",
        f"same_block_candidate_count={same_block_candidate_count}",
        f"dominance_required_candidate_count={dominance_required_candidate_count}",
        f"root_dominates_candidate_count={root_dominates_candidate_count}",
        f"unsafe_candidate_count={unsafe_candidate_count}",
        f"dominant_candidate_sink={dominant(sink_counts)}",
        f"dominant_origin_detail={dominant(origin_detail_counts)}",
        f"selected_optimization_owner={selected_owner}",
        f"selected_owner_confidence={confidence}",
        f"next_task={next_task}",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key, count in sorted(chain_len_counts.items(), key=lambda item: int(item[0])):
        lines.append(f"copy_chain_len_{key}_count={count}")
    for key, count in sink_counts.most_common(8):
        lines.append(f"sink_{safe_key(key)}_copy_count={count}")
    for key, count in origin_detail_counts.most_common(8):
        lines.append(f"origin_detail_{safe_key(key)}_copy_count={count}")
    for key, count in unsafe_sink_counts.most_common(8):
        lines.append(f"unsafe_sink_{safe_key(key)}_copy_count={count}")
    for idx, sample in enumerate(samples[: max(0, args.topn)]):
        prefix = f"sample_{idx}"
        lines.extend(
            [
                f"{prefix}_block=block_{sample['block']}",
                f"{prefix}_inst_index={sample['inst_index']}",
                f"{prefix}_dst={sample['dst']}",
                f"{prefix}_src={sample['src']}",
                f"{prefix}_origin_detail={sample['origin_detail']}",
                f"{prefix}_sink={sample['sink']}",
                f"{prefix}_copy_chain_len={sample['copy_chain_len']}",
                f"{prefix}_root_block=block_{sample['root_block']}",
                f"{prefix}_safe_forward={sample['safe']}",
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

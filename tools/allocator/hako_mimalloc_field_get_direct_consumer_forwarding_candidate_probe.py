#!/usr/bin/env python3
"""Count field_get direct-consumer forwarding candidates in one MIR method."""

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

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    field_get_expression_count = 0
    consumer_reachable_count = 0
    forwarding_candidate_count = 0
    max_chain_len = 0
    sink_counts: Counter[str] = Counter()
    detail_counts: Counter[str] = Counter()
    samples: list[dict[str, Any]] = []

    for block_id, insts in blocks:
        call_attributed = origin.collect_call_attributed_copy_dsts(insts)
        producers = {inst.get("dst"): inst for inst in insts if inst.get("dst") is not None}
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
            origin_kind, origin_detail, chain_len = origin.origin_label(inst.get("src"), producers)
            if origin_kind != "field_get":
                continue
            field_get_expression_count += 1
            sinks = sorted(set(origin.sink_labels(inst.get("dst"), consumers)))
            if is_real_consumer(sinks):
                consumer_reachable_count += 1
            if chain_len > 0 and is_real_consumer(sinks):
                forwarding_candidate_count += 1
                max_chain_len = max(max_chain_len, chain_len)
                detail_counts[origin_detail] += 1
                for sink in sinks:
                    if sink != "unused_or_phi_only":
                        sink_counts[sink] += 1
                samples.append(
                    {
                        "block": block_id,
                        "inst_index": inst_index,
                        "dst": inst.get("dst"),
                        "src": inst.get("src"),
                        "origin_detail": origin_detail,
                        "sink": "+".join(sinks),
                        "copy_chain_len": chain_len,
                    }
                )

    lines = [
        "output_contract=hako-mimalloc-field-get-direct-consumer-forwarding-candidate-probe-v0",
        "input_contract=hako-mimalloc-field-get-expression-copy-chain-policy-selection-v0",
        f"target_method={function.get('name', args.method)}",
        f"field_get_expression_copy_count={field_get_expression_count}",
        f"consumer_reachable_copy_count={consumer_reachable_count}",
        f"forwarding_candidate_copy_count={forwarding_candidate_count}",
        f"max_forwarding_chain_len={max_chain_len}",
        f"dominant_candidate_sink={dominant(sink_counts)}",
        f"dominant_candidate_field={dominant(detail_counts)}",
        "selected_optimization_owner=mir_builder_expression_materialization_forwarding",
        "next_diagnostic=field_get_direct_consumer_forwarding_keeper_design",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for key, count in sink_counts.most_common(8):
        lines.append(f"candidate_sink_{safe_key(key)}_copy_count={count}")
    for key, count in detail_counts.most_common(8):
        lines.append(f"candidate_field_{safe_key(key)}_copy_count={count}")
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

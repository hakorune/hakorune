#!/usr/bin/env python3
"""Classify param-origin direct-consumer forwarding candidates in one MIR method."""

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


def sink_family(sink: str) -> str:
    if sink == "field_get":
        return "field_get"
    if sink.startswith("field_set:"):
        return "field_set"
    if sink.startswith("compare_"):
        return "compare"
    if sink.startswith("binop_"):
        return "binop"
    if sink == "unused_or_phi_only":
        return "unused"
    return "other"


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
        "hako-mimalloc-param-expression-copy-chain-policy-selection-v0",
        "chain-policy",
    )
    require(policy, "selected_chain_policy", "param_direct_consumer_value_forwarding", "chain-policy")
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

    phi_dsts: set[Any] = set()
    for _, insts in blocks:
        for inst in insts:
            if inst.get("op") == "phi" and inst.get("dst") is not None:
                phi_dsts.add(inst.get("dst"))

    param_candidate_count = 0
    safe_forward_count = 0
    unsafe_forward_count = 0
    safe_family_counts: Counter[str] = Counter()
    sink_counts: Counter[str] = Counter()
    chain_len_counts: Counter[str] = Counter()
    unsafe_reason_counts: Counter[str] = Counter()
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
            if origin_kind != "param":
                continue

            param_candidate_count += 1
            sinks = sorted(set(origin.sink_labels(inst.get("dst"), consumers)))
            families = sorted({sink_family(sink) for sink in sinks})
            direct_families = {"field_get", "field_set", "compare"}
            safe = bool(families) and all(family in direct_families for family in families)
            chain_len_counts[str(chain_len)] += 1
            for sink in sinks:
                sink_counts[sink] += 1
            if safe:
                safe_forward_count += 1
                for family in families:
                    safe_family_counts[family] += 1
            else:
                unsafe_forward_count += 1
                for family in families:
                    unsafe_reason_counts[family] += 1
            samples.append(
                {
                    "block": block_id,
                    "inst_index": inst_index,
                    "dst": inst.get("dst"),
                    "src": inst.get("src"),
                    "origin_detail": origin_detail,
                    "sink": "+".join(sinks),
                    "families": "+".join(families),
                    "copy_chain_len": chain_len,
                    "safe": int(safe),
                }
            )

    selected_owner = "0"
    confidence = "low"
    next_task = "param_direct_consumer_forwarding_owner_selection"
    if param_candidate_count > 0 and unsafe_forward_count == 0:
        selected_owner = "mir_builder_param_direct_consumer_value_forwarding"
        confidence = "medium"
        next_task = "param_direct_consumer_forwarding_guard_surface"

    lines = [
        "output_contract=hako-mimalloc-param-direct-consumer-forwarding-candidate-probe-v0",
        "input_contract=hako-mimalloc-param-expression-copy-chain-policy-selection-v0",
        f"target_method={function.get('name', args.method)}",
        f"param_candidate_copy_count={param_candidate_count}",
        f"safe_forward_total_count={safe_forward_count}",
        f"safe_forward_field_get_count={safe_family_counts['field_get']}",
        f"safe_forward_field_set_count={safe_family_counts['field_set']}",
        f"safe_forward_compare_count={safe_family_counts['compare']}",
        f"unsafe_forward_count={unsafe_forward_count}",
        f"dominant_param_sink={dominant(sink_counts)}",
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
    for key, count in sink_counts.most_common(8):
        lines.append(f"sink_{safe_key(key)}_copy_count={count}")
    for key, count in sorted(chain_len_counts.items(), key=lambda item: int(item[0])):
        lines.append(f"copy_chain_len_{key}_count={count}")
    for key, count in unsafe_reason_counts.most_common(8):
        lines.append(f"unsafe_family_{safe_key(key)}_copy_count={count}")
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
                f"{prefix}_sink_families={sample['families']}",
                f"{prefix}_copy_chain_len={sample['copy_chain_len']}",
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
